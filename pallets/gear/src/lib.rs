// This file is part of Gear.

// Copyright (C) 2021-2022 Gear Technologies Inc.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub use pallet::*;
pub use weights::WeightInfo;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
mod ext;
pub mod manager;
pub mod weights;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub type Authorship<T> = pallet_authorship::Pallet<T>;

use pallet_gear_program::Pallet as GearProgramPallet;

pub trait DebugInfo {
    fn is_remap_id_enabled() -> bool;
    fn remap_id();
    fn do_snapshot();
    fn is_enabled() -> bool;
}

impl DebugInfo for () {
    fn is_remap_id_enabled() -> bool {
        false
    }
    fn remap_id() {}
    fn do_snapshot() {}
    fn is_enabled() -> bool {
        false
    }
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    use alloc::format;
    use common::{self, CodeMetadata, DAGBasedLedger, GasPrice, Origin, Program, ProgramState, CodeStorageTrait, ExistingCode};
    use core_processor::{
        common::{DispatchOutcome as CoreDispatchOutcome, ExecutableActor, JournalNote},
        configs::BlockInfo,
    };
    use frame_support::{
        dispatch::{DispatchError, DispatchResultWithPostInfo},
        pallet_prelude::*,
        traits::{BalanceStatus, Currency, Get, LockableCurrency, ReservableCurrency},
    };
    use frame_system::pallet_prelude::*;
    use gear_backend_sandbox::SandboxEnvironment;
    use gear_core::{
        code::{CheckedCode, CheckedCodeWithHash},
        ids::{CodeId, MessageId, ProgramId},
        message::*,
    };
    use primitive_types::H256;
    use scale_info::TypeInfo;
    use sp_runtime::traits::UniqueSaturatedInto;
    use sp_std::{collections::btree_map::BTreeMap, prelude::*};

    use crate::manager::{ExtManager, HandleKind, AddedCode};

    #[pallet::config]
    pub trait Config:
        frame_system::Config
        + pallet_authorship::Config
        + pallet_timestamp::Config
        + pallet_gear_program::Config<Currency = <Self as Config>::Currency>
    {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        /// Gas and value transfer currency
        type Currency: LockableCurrency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

        /// Gas to Currency converter
        type GasPrice: GasPrice<Balance = BalanceOf<Self>>;

        /// Implementation of a ledger to account for gas creation and consumption
        type GasHandler: DAGBasedLedger<ExternalOrigin = H256, Key = H256, Balance = u64>;

        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;

        /// The maximum amount of gas that can be used within a single block.
        #[pallet::constant]
        type BlockGasLimit: Get<u64>;

        /// The cost for a message to spend one block in the wait list
        #[pallet::constant]
        type WaitListFeePerBlock: Get<u64>;

        type DebugInfo: DebugInfo;

        type CodeStorage: CodeStorageTrait;
    }

    type BalanceOf<T> =
        <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::pallet]
    #[pallet::without_storage_info]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Log event from the specific program.
        Log(StoredMessage),
        /// Program created and an init message enqueued.
        InitMessageEnqueued(MessageInfo),
        /// Program initialization error.
        InitFailure(MessageInfo, Reason),
        /// Program initialized.
        InitSuccess(MessageInfo),
        /// Dispatch message with a specific ID enqueued for processing.
        DispatchMessageEnqueued(MessageInfo),
        /// Dispatched message has resulted in an outcome
        MessageDispatched(DispatchOutcome),
        /// Some number of messages processed.
        // TODO: will be replaced by more comprehensive stats
        MessagesDequeued(u32),
        /// Value and gas has been claimed from a message in mailbox by the addressee
        ClaimedValueFromMailbox(H256),
        /// A message has been added to the wait list
        AddedToWaitList(StoredDispatch),
        /// A message has been removed from the wait list
        RemovedFromWaitList(H256),
        /// Program code with a calculated code hash is saved to the storage
        CodeSaved(H256),
        /// Pallet associated storage has been wiped.
        DatabaseWiped,
        /// Message was not executed
        MessageNotExecuted(H256),
    }

    // Gear pallet error.
    #[pallet::error]
    pub enum Error<T> {
        /// Not enough balance to reserve.
        ///
        /// Usually occurs when gas_limit specified is such that origin account can't afford the message.
        NotEnoughBalanceForReserve,
        /// Gas limit too high.
        ///
        /// Occurs when an extrinsic's declared `gas_limit` is greater than a block's maximum gas limit.
        GasLimitTooHigh,
        /// Program already exists.
        ///
        /// Occurs if a program with some specific program id already exists in program storage.
        ProgramAlreadyExists,
        /// No message in the mailbox.
        ///
        /// The user tried to reply on message that was not found in his personal mailbox.
        NoMessageInMailbox,
        /// Program is terminated
        ///
        /// Program init ended up with failure, so such message destination is unavailable anymore
        ProgramIsTerminated,
        /// Message gas tree is not found.
        ///
        /// When message claimed from mailbox has a corrupted or non-extant gas tree associated.
        NoMessageTree,
        /// Code already exists
        ///
        /// Occurs when trying to save to storage a program code, that has been saved there.
        CodeAlreadyExists,
        /// Failed to create a program.
        FailedToConstructProgram,
        /// Value doesnt cover ExistenceDeposit
        ValueLessThanMinimal,
        /// The instance of code storage cannot be acquired
        FailedToAcquireCodeStorage,
    }

    #[derive(Debug, Encode, Decode, Clone, PartialEq, TypeInfo)]
    pub enum Reason {
        Error,
        ValueTransfer,
        Dispatch(Vec<u8>),
    }

    #[derive(Debug, Encode, Decode, Clone, PartialEq, TypeInfo)]
    pub enum ExecutionResult {
        Success,
        Failure(Vec<u8>),
    }

    #[derive(Debug, Encode, Decode, Clone, PartialEq, TypeInfo)]
    pub struct DispatchOutcome {
        pub message_id: H256,
        pub outcome: ExecutionResult,
    }

    #[derive(Debug, Encode, Decode, Clone, PartialEq, TypeInfo)]
    pub struct MessageInfo {
        pub message_id: H256,
        pub program_id: H256,
        pub origin: H256,
    }

    #[pallet::type_value]
    pub fn DefaultForGasLimit<T: Config>() -> u64 {
        T::BlockGasLimit::get()
    }

    #[pallet::storage]
    #[pallet::getter(fn gas_allowance)]
    pub type GasAllowance<T> = StorageValue<_, u64, ValueQuery, DefaultForGasLimit<T>>;

    #[pallet::type_value]
    pub fn Zero() -> u128 {
        0
    }

    #[pallet::storage]
    #[pallet::getter(fn messages_sent)]
    pub type MessagesSent<T: Config> = StorageValue<_, u128, ValueQuery, Zero>;

    #[pallet::storage]
    #[pallet::getter(fn mailbox)]
    pub type Mailbox<T: Config> =
        StorageMap<_, Identity, T::AccountId, BTreeMap<H256, StoredMessage>>;

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T>
    where
        T::AccountId: Origin,
    {
        /// Initialization
        fn on_initialize(_bn: BlockNumberFor<T>) -> Weight {
            // Reset block gas allowance
            GasAllowance::<T>::put(T::BlockGasLimit::get());
            MessagesSent::<T>::put(0);
            T::DbWeight::get().writes(1)
        }

        /// Finalization
        fn on_finalize(_bn: BlockNumberFor<T>) {}

        /// Queue processing occurs after all normal extrinsics in the block
        ///
        /// There should always remain enough weight for this hook to be invoked
        fn on_idle(bn: BlockNumberFor<T>, remaining_weight: Weight) -> Weight {
            log::debug!(
                target: "runtime::gear",
                "{} of weight remains in block {:?} after normal extrinsics have been processed",
                remaining_weight,
                bn,
            );
            // Adjust the block gas allowance based on actual remaining weight
            GasAllowance::<T>::put(remaining_weight);
            MessagesSent::<T>::put(0);
            let mut weight = T::DbWeight::get().writes(1);
            weight += Self::process_queue();

            weight
        }
    }

    impl<T: Config> Pallet<T>
    where
        T::AccountId: Origin,
    {
        // Messages have only two options to be inserted in mailbox:
        // 1. While message processing called `gr_wait`.
        // 2. While message addressed to program, that hadn't finished it's initialization.
        //
        // This means that program always exists in storage in active or terminated status.
        //
        // We also remove messages from mailbox for cases of out of rent (in `pallet-usage`)
        // and once program initialized or failed it's inititalization.
        pub fn insert_to_mailbox(user: H256, message: StoredMessage) {
            let user_id = &<T::AccountId as Origin>::from_origin(user);

            <Mailbox<T>>::mutate(user_id, |value| {
                value
                    .get_or_insert(BTreeMap::new())
                    .insert(message.id().into_origin(), message)
            });
        }

        pub fn get_from_mailbox(user: H256, message_id: H256) -> Option<StoredMessage> {
            let user_id = &<T::AccountId as Origin>::from_origin(user);

            <Mailbox<T>>::try_get(user_id)
                .ok()
                .and_then(|mut messages| messages.remove(&message_id))
        }

        pub fn remove_from_mailbox(user: H256, message_id: H256) -> Option<StoredMessage> {
            let user_id = &<T::AccountId as Origin>::from_origin(user);

            <Mailbox<T>>::try_mutate(user_id, |value| match value {
                Some(ref mut btree) => Ok(btree.remove(&message_id)),
                None => Err(()),
            })
            .ok()
            .flatten()
        }

        pub fn remove_and_claim_from_mailbox(
            user_id: &T::AccountId,
            message_id: H256,
        ) -> Result<StoredMessage, DispatchError> {
            let message = Self::remove_from_mailbox(user_id.clone().into_origin(), message_id)
                .ok_or(Error::<T>::NoMessageInMailbox)?;

            if message.value() > 0 {
                // Assuming the programs has enough balance
                <T as Config>::Currency::repatriate_reserved(
                    &<T::AccountId as Origin>::from_origin(message.source().into_origin()),
                    user_id,
                    message.value().unique_saturated_into(),
                    BalanceStatus::Free,
                )?;
            }

            Ok(message)
        }

        pub fn get_gas_spent(
            source: H256,
            kind: HandleKind,
            payload: Vec<u8>,
            value: u128,
        ) -> Result<u64, Vec<u8>> {
            let mut ext_manager = ExtManager::<T>::default();

            let root_message_id = Self::next_message_id(source);

            let dispatch = match kind {
                HandleKind::Init(ref code) => {
                    let program_id = ProgramId::generate(CodeId::generate(code), b"salt");
                    let code = CheckedCode::try_new(code.clone())
                        .map_err(|_| b"Program failed to load: {}".to_vec())?;

                    let checked_code_hash = CheckedCodeWithHash::new(code);
                    let hash = checked_code_hash.hash();
                    let static_pages = checked_code_hash.code().static_pages();

                    let code_storage = T::CodeStorage::try_new().ok_or(b"Failed to acquire CodeStorage".to_vec())?;

                    let added_code = code_storage.exists(hash).map_right(|s| {
                        let code_hash = s.add_code(checked_code_hash, Self::get_metadata(source));
                        Self::deposit_event(Event::CodeSaved(code_hash.hash().into_origin()));
                        code_hash
                    }).into_inner();

                    ExtManager::<T>::default().set_program(
                        program_id,
                        &added_code,
                        static_pages,
                        root_message_id,
                    );

                    Dispatch::new(
                        DispatchKind::Init,
                        Message::new(
                            MessageId::from_origin(root_message_id),
                            ProgramId::from_origin(source),
                            program_id,
                            payload,
                            Some(u64::MAX),
                            value,
                            None,
                        ),
                    )
                }
                HandleKind::Handle(dest) => Dispatch::new(
                    DispatchKind::Handle,
                    Message::new(
                        MessageId::from_origin(root_message_id),
                        ProgramId::from_origin(source),
                        ProgramId::from_origin(dest),
                        payload,
                        Some(u64::MAX),
                        value,
                        None,
                    ),
                ),
                HandleKind::Reply(msg_id, exit_code) => {
                    let msg = Self::get_from_mailbox(source, msg_id).ok_or_else(|| {
                        b"Internal error: unable to find message in mailbox".to_vec()
                    })?;
                    Dispatch::new(
                        DispatchKind::Reply,
                        Message::new(
                            MessageId::from_origin(root_message_id),
                            ProgramId::from_origin(source),
                            msg.source(),
                            payload,
                            Some(u64::MAX),
                            value,
                            Some((msg.id(), exit_code)),
                        ),
                    )
                }
            };

            let initial_gas = T::BlockGasLimit::get();
            T::GasHandler::create(source.into_origin(), root_message_id, initial_gas)
                .map_err(|_| b"Internal error: unable to create gas handler".to_vec())?;

            let dispatch = dispatch.into_stored();

            common::clear_dispatch_queue();
            common::queue_dispatch(dispatch);

            let block_info = BlockInfo {
                height: <frame_system::Pallet<T>>::block_number().unique_saturated_into(),
                timestamp: <pallet_timestamp::Pallet<T>>::get().unique_saturated_into(),
            };

            let existential_deposit =
                <T as Config>::Currency::minimum_balance().unique_saturated_into();

            let mut max_gas_spent = 0;

            while let Some(queued_dispatch) = common::dequeue_dispatch() {
                let actor_id = queued_dispatch.destination();
                let actor = ext_manager
                    .get_executable_actor(actor_id.into_origin())
                    .ok_or_else(|| b"Program not found in the storage".to_vec())?;

                let journal = core_processor::process::<
                    ext::LazyPagesExt,
                    SandboxEnvironment<ext::LazyPagesExt>,
                >(
                    Some(actor),
                    queued_dispatch.into_incoming(initial_gas),
                    block_info,
                    existential_deposit,
                    ProgramId::from_origin(source),
                    actor_id,
                );

                core_processor::handle_journal(journal.clone(), &mut ext_manager);

                let (remaining_gas, _) =
                    T::GasHandler::get_limit(root_message_id).ok_or_else(|| {
                        b"Internal error: unable to get gas limit after execution".to_vec()
                    })?;

                // TODO: Check whether we charge gas fee for submitting code after #646
                for note in journal {
                    match note {
                        JournalNote::SendDispatch { .. }
                        | JournalNote::WaitDispatch(..)
                        | JournalNote::MessageConsumed(..) => {
                            let gas_spent = initial_gas.saturating_sub(remaining_gas);
                            if gas_spent > max_gas_spent {
                                max_gas_spent = gas_spent;
                            }
                        }
                        JournalNote::MessageDispatched(CoreDispatchOutcome::MessageTrap {
                            trap,
                            ..
                        }) => {
                            return Err(format!(
                                "Program terminated with a trap: {}",
                                trap.unwrap_or("No reason")
                            )
                            .into_bytes());
                        }
                        _ => (),
                    }
                }
            }

            Ok(max_gas_spent)
        }

        pub(crate) fn decrease_gas_allowance(gas_charge: u64) {
            GasAllowance::<T>::mutate(|x| *x = x.saturating_sub(gas_charge));
        }

        /// Returns true if a program has been successfully initialized
        pub fn is_initialized(program_id: H256) -> bool {
            common::get_program(program_id)
                .map(|p| p.is_initialized())
                .unwrap_or(false)
        }

        /// Returns true if a program has terminated status
        pub fn is_terminated(program_id: H256) -> bool {
            common::get_program(program_id)
                .map(|p| p.is_terminated())
                .unwrap_or(false)
        }

        /// Returns MessageId for newly created user message.
        pub fn next_message_id(user_id: H256) -> H256 {
            let nonce = Self::messages_sent();
            MessagesSent::<T>::mutate(|x| *x = x.saturating_add(1));
            let block_number = <frame_system::Pallet<T>>::block_number().unique_saturated_into();
            let user_id = ProgramId::from_origin(user_id);

            MessageId::generate_from_user(block_number, user_id, nonce).into_origin()
        }

        /// Message Queue processing.
        ///
        /// Can emit the following events:
        /// - `InitSuccess(MessageInfo)` when initialization message is processed successfully;
        /// - `InitFailure(MessageInfo, Reason)` when initialization message fails;
        /// - `Log(Message)` when a dispatched message spawns other messages (including replies);
        /// - `MessageDispatched(H256)` when a dispatch message has been processed with some outcome.
        pub fn process_queue() -> Weight {
            let mut ext_manager = ExtManager::<T>::default();

            let mut weight = Self::gas_allowance() as Weight;
            let mut total_handled = 0u32;

            let block_info = BlockInfo {
                height: <frame_system::Pallet<T>>::block_number().unique_saturated_into(),
                timestamp: <pallet_timestamp::Pallet<T>>::get().unique_saturated_into(),
            };

            let existential_deposit =
                <T as Config>::Currency::minimum_balance().unique_saturated_into();

            if T::DebugInfo::is_remap_id_enabled() {
                T::DebugInfo::remap_id();
            }
            while let Some(dispatch) = common::dequeue_dispatch() {
                // Update message gas limit for it may have changed in the meantime

                let msg_id = dispatch.id().into_origin();
                let (gas_limit, _) = if let Some(limit) = T::GasHandler::get_limit(msg_id) {
                    limit
                } else {
                    log::debug!(
                        target: "essential",
                        "No gas handler for message: {:?} to {:?}",
                        dispatch.id(),
                        dispatch.destination(),
                    );

                    common::queue_dispatch(dispatch);

                    // Since we requeue the message without GasHandler we have to take
                    // into account that there can left only such messages in the queue.
                    // So stop processing when there is not enough gas/weight.
                    let consumed = T::DbWeight::get().reads(1) + T::DbWeight::get().writes(1);
                    Self::decrease_gas_allowance(consumed);
                    if Self::gas_allowance() < consumed {
                        break;
                    }

                    continue;
                };

                log::debug!(
                    "Processing message: {:?} to {:?} / gas_limit: {}",
                    dispatch.id(),
                    dispatch.destination(),
                    gas_limit
                );

                // Check whether we have enough of gas allowed for message processing
                if gas_limit > GasAllowance::<T>::get() {
                    log::debug!(
                        "Not enought gas for processing: gas_limit = {}, allowance = {}",
                        gas_limit,
                        GasAllowance::<T>::get(),
                    );
                    common::queue_dispatch(dispatch);
                    break;
                }

                let program_id = dispatch.destination();
                let maybe_active_actor = if let Some(maybe_active_program) =
                    common::get_program(program_id.into_origin())
                {
                    let current_message_id = dispatch.id();
                    let maybe_message_reply = dispatch.reply();

                    // Check whether message should be added to the wait list
                    if let Program::Active(ref prog) = maybe_active_program {
                        let is_for_wait_list = maybe_message_reply.is_none()
                            && matches!(prog.state, ProgramState::Uninitialized {message_id} if message_id != current_message_id.into_origin());
                        if is_for_wait_list {
                            Self::deposit_event(Event::AddedToWaitList(dispatch.clone()));
                            common::waiting_init_append_message_id(
                                program_id.into_origin(),
                                current_message_id.into_origin(),
                            );
                            common::insert_waiting_message(
                                program_id.into_origin(),
                                current_message_id.into_origin(),
                                dispatch,
                                block_info.height,
                            );

                            continue;
                        }
                    }

                    manager::try_into_native::<T>(maybe_active_program, program_id.into_origin())
                        .ok()
                        .map(|program| {
                            let balance = <T as Config>::Currency::free_balance(
                                &<T::AccountId as Origin>::from_origin(program_id.into_origin()),
                            )
                            .unique_saturated_into();

                            ExecutableActor { program, balance }
                        })
                } else {
                    None
                };

                let program_id = dispatch.destination();
                let origin = <T as Config>::GasHandler::get_origin(msg_id)
                    .expect("Gas node is guaranteed to exist for the key due to earlier checks");

                let journal = core_processor::process::<
                    ext::LazyPagesExt,
                    SandboxEnvironment<ext::LazyPagesExt>,
                >(
                    maybe_active_actor,
                    dispatch.into_incoming(gas_limit),
                    block_info,
                    existential_deposit,
                    ProgramId::from_origin(origin),
                    program_id,
                );

                core_processor::handle_journal(journal, &mut ext_manager);

                total_handled += 1;

                if T::DebugInfo::is_enabled() {
                    T::DebugInfo::do_snapshot();
                }

                if T::DebugInfo::is_remap_id_enabled() {
                    T::DebugInfo::remap_id();
                }
            }

            if total_handled > 0 {
                Self::deposit_event(Event::MessagesDequeued(total_handled));
            }

            weight = weight.saturating_sub(Self::gas_allowance());
            weight
        }

        /// Sets `code` and metadata, if code doesn't exist in storage.
        ///
        /// On success returns Blake256 hash of the `code`. If code already
        /// exists (*so, metadata exists as well*), returns unit `CodeAlreadyExists` error.
        ///
        /// # Note
        /// Code existence in storage means that metadata is there too.
        /* fn set_code_with_metadata(
            code_hash: CheckedCodeWithHash,
            who: H256,
            code_storage: &mut T::CodeStorage,
        ) -> Result<ExistingCode<T::CodeStorage>, Error<T>> {
            if code_storage.exists(code_hash.hash()).is_some() {
                return Err(Error::<T>::CodeAlreadyExists);
            }

            let metadata = {
                let block_number =
                    <frame_system::Pallet<T>>::block_number().unique_saturated_into();
                CodeMetadata::new(who, block_number)
            };

            Ok(code_storage.add_code(code_hash))
        } */

        fn get_metadata(who: H256) -> CodeMetadata {
            let block_number =
                    <frame_system::Pallet<T>>::block_number().unique_saturated_into();
            CodeMetadata::new(who, block_number)
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T>
    where
        T::AccountId: Origin,
    {
        /// Saves program `code` in storage.
        ///
        /// The extrinsic was created to provide _deploy program from program_ functionality.
        /// Anyone who wants to define a "factory" logic in program should first store the code and metadata for the "child"
        /// program in storage. So the code for the child will be initialized by program initialization request only if it exists in storage.
        ///
        /// More precisely, the code and its metadata are actually saved in the storage under the hash of the `code`. The code hash is computed
        /// as Blake256 hash. At the time of the call the `code` hash should not be in the storage. If it was stored previously, call will end up
        /// with an `CodeAlreadyExists` error. In this case user can be sure, that he can actually use the hash of his program's code bytes to define
        /// "program factory" logic in his program.
        ///
        /// Parameters
        /// - `code`: wasm code of a program as a byte vector.
        ///
        /// Emits the following events:
        /// - `SavedCode(H256)` - when the code is saved in storage.
        #[pallet::weight(
            <T as Config>::WeightInfo::submit_code(code.len() as u32)
        )]
        pub fn submit_code(origin: OriginFor<T>, code: Vec<u8>) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            let code = CheckedCode::try_new(code).map_err(|e| {
                log::debug!("Program failed to load: {}", e);
                Error::<T>::FailedToConstructProgram
            })?;

            let code_storage = T::CodeStorage::try_new().ok_or(Error::<T>::FailedToAcquireCodeStorage)?;

            let code_hash = CheckedCodeWithHash::new(code);
            code_storage.exists(code_hash.hash())
                .map_left(|_| Err(Error::<T>::CodeAlreadyExists.into()))
                .map_right(|s| {
                    let code_hash = s.add_code(code_hash, Self::get_metadata(who.into_origin()));
                    Self::deposit_event(Event::CodeSaved(code_hash.hash().into_origin()));
                    Ok(().into())
                }).into_inner()
        }

        /// Creates program initialization request (message), that is scheduled to be run in the same block.
        ///
        /// There are no guarantees that initialization message will be run in the same block due to block
        /// gas limit restrictions. For example, when it will be the message's turn, required gas limit for it
        /// could be more than remaining block gas limit. Therefore, the message processing will be postponed
        /// until the next block.
        ///
        /// `ProgramId` is computed as Blake256 hash of concatenated bytes of `code` + `salt`. (todo #512 `code_hash` + `salt`)
        /// Such `ProgramId` must not exist in the Program Storage at the time of this call.
        ///
        /// There is the same guarantee here as in `submit_code`. That is, future program's
        /// `code` and metadata are stored before message was added to the queue and processed.
        ///
        /// The origin must be Signed and the sender must have sufficient funds to pay
        /// for `gas` and `value` (in case the latter is being transferred).
        ///
        /// Parameters:
        /// - `code`: wasm code of a program as a byte vector.
        /// - `salt`: randomness term (a seed) to allow programs with identical code
        ///   to be created independently.
        /// - `init_payload`: encoded parameters of the wasm module `init` function.
        /// - `gas_limit`: maximum amount of gas the program can spend before it is halted.
        /// - `value`: balance to be transferred to the program once it's been created.
        ///
        /// Emits the following events:
        /// - `InitMessageEnqueued(MessageInfo)` when init message is placed in the queue.
        ///
        /// # Note
        /// Faulty (uninitialized) programs still have a valid addresses (program ids) that can deterministically be derived on the
        /// caller's side upfront. It means that if messages are sent to such an address, they might still linger in the queue.
        ///
        /// In order to mitigate the risk of users' funds being sent to an address,
        /// where a valid program should have resided, while it's not,
        /// such "failed-to-initialize" programs are not silently deleted from the
        /// program storage but rather marked as "ghost" programs.
        /// Ghost program can be removed by their original author via an explicit call.
        /// The funds stored by a ghost program will be release to the author once the program
        /// has been removed.
        #[pallet::weight(
            <T as Config>::WeightInfo::submit_program(code.len() as u32, init_payload.len() as u32)
        )]
        pub fn submit_program(
            origin: OriginFor<T>,
            code: Vec<u8>,
            salt: Vec<u8>,
            init_payload: Vec<u8>,
            gas_limit: u64,
            value: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            // Check that provided `gas_limit` value does not exceed the block gas limit
            ensure!(
                gas_limit <= T::BlockGasLimit::get(),
                Error::<T>::GasLimitTooHigh
            );

            let code = CheckedCode::try_new(code).map_err(|e| {
                log::debug!("Program failed to load: {}", e);
                Error::<T>::FailedToConstructProgram
            })?;

            let static_pages = code.static_pages();
            let checked_code_hash = CheckedCodeWithHash::new(code);
            let hash = checked_code_hash.hash();

            let packet = InitPacket::new_with_gas(
                hash,
                salt,
                init_payload,
                gas_limit,
                value.unique_saturated_into(),
            );

            let program_id = packet.destination();
            let id = program_id.into_origin();
            // Make sure there is no program with such id in program storage
            ensure!(
                !GearProgramPallet::<T>::program_exists(id),
                Error::<T>::ProgramAlreadyExists
            );

            let reserve_fee = T::GasPrice::gas_price(gas_limit);

            // First we reserve enough funds on the account to pay for `gas_limit`
            // and to transfer declared value.
            <T as Config>::Currency::reserve(&who, reserve_fee + value)
                .map_err(|_| Error::<T>::NotEnoughBalanceForReserve)?;

            let code_storage = T::CodeStorage::try_new().ok_or(Error::<T>::FailedToAcquireCodeStorage)?;

            let origin = who.into_origin();

            // By that call we follow the guarantee that we have in `Self::submit_code` -
            // if there's code in storage, there's also metadata for it.
            let added_code = code_storage.exists(hash).map_right(|s| {
                let code_hash = s.add_code(checked_code_hash, Self::get_metadata(origin));
                Self::deposit_event(Event::CodeSaved(code_hash.hash().into_origin()));
                code_hash
            }).into_inner();

            let message_id = Self::next_message_id(origin).into_origin();
            ExtManager::<T>::default().set_program(program_id, &added_code, static_pages, message_id);

            let _ =
                T::GasHandler::create(origin, message_id, packet.gas_limit().expect("Can't fail"));

            let message = InitMessage::from_packet(MessageId::from_origin(message_id), packet);
            let dispatch = message
                .into_dispatch(ProgramId::from_origin(origin))
                .into_stored();

            common::queue_dispatch(dispatch);

            Self::deposit_event(Event::InitMessageEnqueued(MessageInfo {
                message_id,
                program_id: id,
                origin,
            }));

            Ok(().into())
        }

        /// Sends a message to a program or to another account.
        ///
        /// The origin must be Signed and the sender must have sufficient funds to pay
        /// for `gas` and `value` (in case the latter is being transferred).
        ///
        /// To avoid an undefined behavior a check is made that the destination address
        /// is not a program in uninitialized state. If the opposite holds true,
        /// the message is not enqueued for processing.
        ///
        /// Parameters:
        /// - `destination`: the message destination.
        /// - `payload`: in case of a program destination, parameters of the `handle` function.
        /// - `gas_limit`: maximum amount of gas the program can spend before it is halted.
        /// - `value`: balance to be transferred to the program once it's been created.
        ///
        /// Emits the following events:
        /// - `DispatchMessageEnqueued(MessageInfo)` when dispatch message is placed in the queue.
        #[frame_support::transactional]
        #[pallet::weight(<T as Config>::WeightInfo::send_message(payload.len() as u32))]
        pub fn send_message(
            origin: OriginFor<T>,
            destination: H256,
            payload: Vec<u8>,
            gas_limit: u64,
            value: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            let numeric_value: u128 = value.unique_saturated_into();
            let minimum: u128 = <T as Config>::Currency::minimum_balance().unique_saturated_into();

            // Check that provided `gas_limit` value does not exceed the block gas limit
            ensure!(
                gas_limit <= T::BlockGasLimit::get(),
                Error::<T>::GasLimitTooHigh
            );

            // Check that provided `value` equals 0 or greater than existential deposit
            ensure!(
                0 == numeric_value || numeric_value >= minimum,
                Error::<T>::ValueLessThanMinimal
            );

            ensure!(
                !Self::is_terminated(destination),
                Error::<T>::ProgramIsTerminated
            );

            // Message is not guaranteed to be executed, that's why value is not immediately transferred.
            // That's because destination can fail to be initialized, while this dispatch message is next
            // in the queue.
            <T as Config>::Currency::reserve(&who, value.unique_saturated_into())
                .map_err(|_| Error::<T>::NotEnoughBalanceForReserve)?;

            let origin = who.clone().into_origin();

            let message_id = Self::next_message_id(origin);
            let packet = HandlePacket::new_with_gas(
                ProgramId::from_origin(destination),
                payload,
                gas_limit,
                value.unique_saturated_into(),
            );
            let message = HandleMessage::from_packet(MessageId::from_origin(message_id), packet);

            if GearProgramPallet::<T>::program_exists(destination) {
                let gas_limit_reserve = T::GasPrice::gas_price(gas_limit);

                // First we reserve enough funds on the account to pay for `gas_limit`
                <T as Config>::Currency::reserve(&who, gas_limit_reserve)
                    .map_err(|_| Error::<T>::NotEnoughBalanceForReserve)?;

                let origin = who.into_origin();
                let _ = T::GasHandler::create(origin, message_id.into_origin(), gas_limit);

                common::queue_dispatch(
                    message.into_stored_dispatch(ProgramId::from_origin(origin)),
                );

                Self::deposit_event(Event::DispatchMessageEnqueued(MessageInfo {
                    message_id: message_id.into_origin(),
                    origin,
                    program_id: destination,
                }));
            } else {
                // Message in mailbox is not meant for any processing, hence 0 gas limit
                // and no gas tree needs to be created
                let origin = who.into_origin();
                let message = message.into_stored(ProgramId::from_origin(origin));
                Self::insert_to_mailbox(destination.into_origin(), message.clone());
                Self::deposit_event(Event::Log(message));
            }

            Ok(().into())
        }

        /// Sends a reply message.
        ///
        /// The origin must be Signed and the sender must have sufficient funds to pay
        /// for `gas` and `value` (in case the latter is being transferred).
        ///
        /// Parameters:
        /// - `reply_to_id`: the original message id.
        /// - `payload`: data expected by the original sender.
        /// - `gas_limit`: maximum amount of gas the program can spend before it is halted.
        /// - `value`: balance to be transferred to the program once it's been created.
        ///
        /// - `DispatchMessageEnqueued(H256)` when dispatch message is placed in the queue.
        #[frame_support::transactional]
        #[pallet::weight(<T as Config>::WeightInfo::send_reply(payload.len() as u32))]
        pub fn send_reply(
            origin: OriginFor<T>,
            reply_to_id: H256,
            payload: Vec<u8>,
            gas_limit: u64,
            value: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            let numeric_value: u128 = value.unique_saturated_into();
            let minimum: u128 = <T as Config>::Currency::minimum_balance().unique_saturated_into();

            // Ensure the `gas_limit` allows the extrinsic to fit into a block
            ensure!(
                gas_limit <= T::BlockGasLimit::get(),
                Error::<T>::GasLimitTooHigh
            );

            // Check that provided `value` equals 0 or greater than existential deposit
            ensure!(
                0 == numeric_value || numeric_value >= minimum,
                Error::<T>::ValueLessThanMinimal
            );

            // Claim outstanding value from the original message first
            let original_message = Self::remove_and_claim_from_mailbox(&who, reply_to_id)?;
            let destination = original_message.source();

            // Message is not guaranteed to be executed, that's why value is not immediately transferred.
            // That's because destination can fail to be initialized, while this dispatch message is next
            // in the queue.
            <T as Config>::Currency::reserve(&who, value.unique_saturated_into())
                .map_err(|_| Error::<T>::NotEnoughBalanceForReserve)?;

            let message_id = MessageId::generate_reply(original_message.id(), 0);
            let packet =
                ReplyPacket::new_with_gas(payload, gas_limit, value.unique_saturated_into());
            let message = ReplyMessage::from_packet(message_id, packet);

            if GearProgramPallet::<T>::program_exists(destination.into_origin()) {
                let gas_limit_reserve = T::GasPrice::gas_price(gas_limit);

                // First we reserve enough funds on the account to pay for `gas_limit`
                <T as Config>::Currency::reserve(&who, gas_limit_reserve)
                    .map_err(|_| Error::<T>::NotEnoughBalanceForReserve)?;

                let origin = who.into_origin();
                let _ = T::GasHandler::create(origin, message_id.into_origin(), gas_limit);

                common::queue_dispatch(message.into_stored_dispatch(
                    ProgramId::from_origin(origin),
                    destination,
                    original_message.id(),
                ));

                Self::deposit_event(Event::DispatchMessageEnqueued(MessageInfo {
                    message_id: message_id.into_origin(),
                    origin,
                    program_id: destination.into_origin(),
                }));
            } else {
                // Message in mailbox is not meant for any processing, hence 0 gas limit
                // and no gas tree needs to be created
                let origin = who.into_origin();

                let message = message.into_stored(
                    ProgramId::from_origin(origin),
                    destination,
                    original_message.id(),
                );
                Self::insert_to_mailbox(destination.into_origin(), message.clone());
                Self::deposit_event(Event::Log(message));
            }

            Ok(().into())
        }

        #[frame_support::transactional]
        #[pallet::weight(T::DbWeight::get().writes(1))]
        pub fn claim_value_from_mailbox(
            origin: OriginFor<T>,
            message_id: H256,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            let _ = Self::remove_and_claim_from_mailbox(&who, message_id)?;

            Self::deposit_event(Event::ClaimedValueFromMailbox(message_id));

            Ok(().into())
        }

        /// Reset all pallet associated storage.
        #[pallet::weight(0)]
        pub fn reset(origin: OriginFor<T>) -> DispatchResult {
            ensure_root(origin)?;
            <Mailbox<T>>::remove_all(None);
            GearProgramPallet::<T>::reset_storage();
            common::reset_storage();

            Self::deposit_event(Event::DatabaseWiped);

            Ok(())
        }
    }

    impl<T: Config> common::PaymentProvider<T::AccountId> for Pallet<T>
    where
        T::AccountId: Origin,
    {
        type Balance = BalanceOf<T>;

        fn withhold_reserved(
            source: H256,
            dest: &T::AccountId,
            amount: Self::Balance,
        ) -> Result<(), DispatchError> {
            let _ = <T as Config>::Currency::repatriate_reserved(
                &<T::AccountId as Origin>::from_origin(source),
                dest,
                amount,
                BalanceStatus::Free,
            )?;

            Ok(())
        }
    }
}
