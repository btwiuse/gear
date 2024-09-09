// This file is part of Gear.

// Copyright (C) 2021-2024 Gear Technologies Inc.
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

use crate::{
    default_users_list,
    error::usage_panic,
    manager::ExtManager,
    state::actors::{Actors, GenuineProgram, Program as InnerProgram, TestActor},
    system::System,
    Result, Value, GAS_ALLOWANCE,
};
use codec::{Codec, Decode, Encode};
use gear_core::{
    code::{Code, CodeAndId, InstrumentedCode, InstrumentedCodeAndId},
    gas_metering::Schedule,
    ids::{prelude::*, CodeId, MessageId, ProgramId},
    message::{Dispatch, DispatchKind, Message},
};
use gear_utils::{MemoryPageDump, ProgramMemoryDump};
use path_clean::PathClean;
use std::{
    cell::RefCell,
    convert::TryInto,
    env,
    ffi::OsStr,
    fmt::Debug,
    fs,
    path::{Path, PathBuf},
    str::FromStr,
};

/// Gas for gear programs.
#[derive(
    Default,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    derive_more::Add,
    derive_more::AddAssign,
    derive_more::Sub,
    derive_more::SubAssign,
    derive_more::Mul,
    derive_more::MulAssign,
    derive_more::Div,
    derive_more::DivAssign,
    derive_more::Display,
)]
pub struct Gas(pub(crate) u64);

impl Gas {
    /// Gas with value zero.
    pub const fn zero() -> Self {
        Self(0)
    }

    /// Computes a + b, saturating at numeric bounds.
    pub const fn saturating_add(self, rhs: Self) -> Self {
        Self(self.0.saturating_add(rhs.0))
    }

    /// Computes a - b, saturating at numeric bounds.
    pub const fn saturating_sub(self, rhs: Self) -> Self {
        Self(self.0.saturating_sub(rhs.0))
    }

    /// Computes a * b, saturating at numeric bounds.
    pub const fn saturating_mul(self, rhs: Self) -> Self {
        Self(self.0.saturating_mul(rhs.0))
    }

    /// Computes a / b, saturating at numeric bounds.
    pub const fn saturating_div(self, rhs: Self) -> Self {
        Self(self.0.saturating_div(rhs.0))
    }
}

/// Trait for mocking gear programs.
///
/// See [`Program`] and [`Program::mock`] for the usages.
pub trait WasmProgram: Debug {
    /// Init wasm program with given `payload`.
    ///
    /// Returns `Ok(Some(payload))` if program has reply logic
    /// with given `payload`.
    ///
    /// If error occurs, the program will be terminated which
    /// means that `handle` and `handle_reply` will not be
    /// called.
    fn init(&mut self, payload: Vec<u8>) -> Result<Option<Vec<u8>>, &'static str>;
    /// Message handler with given `payload`.
    ///
    /// Returns `Ok(Some(payload))` if program has reply logic.
    fn handle(&mut self, payload: Vec<u8>) -> Result<Option<Vec<u8>>, &'static str>;
    /// Reply message handler with given `payload`.
    fn handle_reply(&mut self, payload: Vec<u8>) -> Result<(), &'static str>;
    /// Signal handler with given `payload`.
    fn handle_signal(&mut self, payload: Vec<u8>) -> Result<(), &'static str>;
    /// State of wasm program.
    ///
    /// See [`Program::read_state`] for the usage.
    fn state(&mut self) -> Result<Vec<u8>, &'static str>;
    /// Emit debug message in program with given `data`.
    ///
    /// Logging target `gwasm` is used in this method.
    fn debug(&mut self, data: &str) {
        log::debug!(target: "gwasm", "DEBUG: {data}");
    }
}

/// Wrapper for program id.
#[derive(Clone, Debug)]
pub struct ProgramIdWrapper(pub(crate) ProgramId);

impl<T: Into<ProgramIdWrapper> + Clone> PartialEq<T> for ProgramIdWrapper {
    fn eq(&self, other: &T) -> bool {
        self.0.eq(&other.clone().into().0)
    }
}

impl From<ProgramId> for ProgramIdWrapper {
    fn from(other: ProgramId) -> Self {
        Self(other)
    }
}

impl From<u64> for ProgramIdWrapper {
    fn from(other: u64) -> Self {
        Self(other.into())
    }
}

impl From<[u8; 32]> for ProgramIdWrapper {
    fn from(other: [u8; 32]) -> Self {
        Self(other.into())
    }
}

impl From<&[u8]> for ProgramIdWrapper {
    fn from(other: &[u8]) -> Self {
        ProgramId::try_from(other)
            .expect("invalid identifier")
            .into()
    }
}

impl From<Vec<u8>> for ProgramIdWrapper {
    fn from(other: Vec<u8>) -> Self {
        other[..].into()
    }
}

impl From<&Vec<u8>> for ProgramIdWrapper {
    fn from(other: &Vec<u8>) -> Self {
        other[..].into()
    }
}

impl From<String> for ProgramIdWrapper {
    fn from(other: String) -> Self {
        other[..].into()
    }
}

impl From<&str> for ProgramIdWrapper {
    fn from(other: &str) -> Self {
        ProgramId::from_str(other)
            .expect("invalid identifier")
            .into()
    }
}

/// Construct state arguments.
///
/// Used for reading and decoding the program’s transformed state,
/// see [`Program::read_state_using_wasm`] for example.
#[macro_export]
macro_rules! state_args {
    () => {
        Option::<()>::None
    };
    ($single:expr) => {
        Some($single)
    };
    ($($multiple:expr),*) => {
        Some(($($multiple,)*))
    };
}

/// Construct encoded state arguments.
///
/// Used for reading the program’s transformed state as a byte vector,
/// see [`Program::read_state_bytes_using_wasm`] for example.
#[macro_export]
macro_rules! state_args_encoded {
    () => {
        Option::<Vec<u8>>::None
    };
    ($single:expr) => {
        {
            use $crate::codec::Encode;
            Some(($single).encode())
        }
    };
    ($($multiple:expr),*) => {
        {
            use $crate::codec::Encode;
            Some((($($multiple,)*)).encode())
        }
    };
}

/// Builder for [`Program`].
#[must_use = "`build()` must be called at the end"]
#[derive(Debug, Clone)]
pub struct ProgramBuilder {
    code: Vec<u8>,
    meta: Option<Vec<u8>>,
    id: Option<ProgramIdWrapper>,
}

impl ProgramBuilder {
    /// Create program from WASM binary.
    pub fn from_binary(code: impl Into<Vec<u8>>) -> Self {
        Self {
            code: code.into(),
            meta: None,
            id: None,
        }
    }

    /// Create a program instance from wasm file.
    pub fn from_file(path: impl AsRef<Path>) -> Self {
        Self::from_binary(fs::read(path).expect("Failed to read WASM file"))
    }

    fn wasm_path(optimized: bool) -> PathBuf {
        Self::wasm_path_from_binpath(optimized)
            .unwrap_or_else(|| gbuild::wasm_path().expect("Unable to find built wasm"))
    }

    fn wasm_path_from_binpath(optimized: bool) -> Option<PathBuf> {
        let cwd = env::current_dir().expect("Unable to get current dir");
        let extension = if optimized { "opt.wasm" } else { "wasm" };
        let path_file = cwd.join(".binpath");
        let path_bytes = fs::read(path_file).ok()?;
        let mut relative_path: PathBuf =
            String::from_utf8(path_bytes).expect("Invalid path").into();
        relative_path.set_extension(extension);
        Some(cwd.join(relative_path))
    }

    fn inner_current(optimized: bool) -> Self {
        let path = env::current_dir()
            .expect("Unable to get root directory of the project")
            .join(Self::wasm_path(optimized))
            // TODO: consider to use `.canonicalize()` instead
            .clean();

        let filename = path.file_name().and_then(OsStr::to_str).unwrap_or_default();
        assert!(
            filename.ends_with(".wasm"),
            "File must have `.wasm` extension"
        );

        let code = fs::read(&path).unwrap_or_else(|_| panic!("Failed to read file {:?}", path));

        Self {
            code,
            meta: None,
            id: None,
        }
    }

    /// Get program of the root crate with provided `system`.
    ///
    /// It looks up the wasm binary of the root crate that contains
    /// the current test, uploads it to the testing system, then
    /// returns the program instance.
    pub fn current() -> Self {
        Self::inner_current(false)
    }

    /// Get optimized program of the root crate with provided `system`,
    ///
    /// See also [`ProgramBuilder::current`].
    pub fn current_opt() -> Self {
        Self::inner_current(true)
    }

    /// Set ID for future program.
    pub fn with_id(mut self, id: impl Into<ProgramIdWrapper>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set metadata for future program.
    pub fn with_meta(mut self, meta: impl Into<Vec<u8>>) -> Self {
        self.meta = Some(meta.into());
        self
    }

    /// Set metadata for future program from file.
    ///
    /// See also [`ProgramBuilder::with_meta`].
    pub fn with_meta_file(self, path: impl AsRef<Path>) -> Self {
        self.with_meta(fs::read(path).expect("Failed to read metadata file"))
    }

    /// Build program with set parameters.
    pub fn build(self, system: &System) -> Program {
        let id = self
            .id
            .unwrap_or_else(|| system.0.borrow_mut().free_id_nonce().into());

        let (code, code_id) = Self::build_instrumented_code_and_id(self.code);

        if let Some(metadata) = self.meta {
            system
                .0
                .borrow_mut()
                .meta_binaries
                .insert(code_id, metadata);
        }

        Program::program_with_id(
            system,
            id,
            InnerProgram::Genuine(GenuineProgram {
                code,
                code_id,
                allocations: Default::default(),
                pages_data: Default::default(),
                gas_reservation_map: Default::default(),
            }),
        )
    }

    pub(crate) fn build_instrumented_code_and_id(
        original_code: Vec<u8>,
    ) -> (InstrumentedCode, CodeId) {
        let schedule = Schedule::default();
        let code = Code::try_new(
            original_code,
            schedule.instruction_weights.version,
            |module| schedule.rules(module),
            schedule.limits.stack_height,
            schedule.limits.data_segments_amount.into(),
            schedule.limits.table_number.into(),
        )
        .expect("Failed to create Program from provided code");

        let c: InstrumentedCodeAndId = CodeAndId::new(code).into();
        c.into_parts()
    }
}

/// Gear program instance.
///
/// ```ignore
/// use gtest::{System, Program};
///
/// // Create a testing system.
/// let system = System::new();
///
/// // Get the current program of the testing system.
/// let program = Program::current(&system);
///
/// // Initialize the program from user 42 with message "init program".
/// let _result = program.send(42, "init program");
/// ```
pub struct Program<'a> {
    pub(crate) manager: &'a RefCell<ExtManager>,
    pub(crate) id: ProgramId,
}

impl<'a> Program<'a> {
    fn program_with_id<I: Into<ProgramIdWrapper> + Clone + Debug>(
        system: &'a System,
        id: I,
        program: InnerProgram,
    ) -> Self {
        let program_id = id.clone().into().0;

        if default_users_list().contains(&(program_id.into_bytes()[0] as u64)) {
            usage_panic!(
                "Can't create program with id {id:?}, because it's reserved for default users.\
                Please, use another id."
            )
        }

        if system
            .0
            .borrow_mut()
            .store_new_actor(program_id, program, None)
            .is_some()
        {
            usage_panic!(
                "Can't create program with id {id:?}, because Program with this id already exists. \
                Please, use another id."
            )
        }

        Self {
            manager: &system.0,
            id: program_id,
        }
    }

    /// Get the program of the root crate with provided `system`.
    ///
    /// See [`ProgramBuilder::current`]
    pub fn current(system: &'a System) -> Self {
        ProgramBuilder::current().build(system)
    }

    /// Get the program of the root crate with provided `system` and
    /// initialize it with given `id`.
    ///
    /// See also [`Program::current`].
    pub fn current_with_id<I: Into<ProgramIdWrapper> + Clone + Debug>(
        system: &'a System,
        id: I,
    ) -> Self {
        ProgramBuilder::current().with_id(id).build(system)
    }

    /// Get optimized program of the root crate with provided `system`,
    ///
    /// See also [`Program::current`].
    pub fn current_opt(system: &'a System) -> Self {
        ProgramBuilder::current_opt().build(system)
    }

    /// Create a program instance from wasm file.
    ///
    /// See also [`Program::current`].
    pub fn from_file<P: AsRef<Path>>(system: &'a System, path: P) -> Self {
        ProgramBuilder::from_file(path).build(system)
    }

    /// Create a program instance from wasm file with given ID.
    ///
    /// See also [`Program::from_file`].
    pub fn from_binary_with_id<ID, B>(system: &'a System, id: ID, binary: B) -> Self
    where
        ID: Into<ProgramIdWrapper> + Clone + Debug,
        B: Into<Vec<u8>>,
    {
        ProgramBuilder::from_binary(binary)
            .with_id(id)
            .build(system)
    }

    /// Mock a program with provided `system` and `mock`.
    ///
    /// See [`WasmProgram`] for more details.
    pub fn mock<T: WasmProgram + 'static>(system: &'a System, mock: T) -> Self {
        let nonce = system.0.borrow_mut().free_id_nonce();

        Self::mock_with_id(system, nonce, mock)
    }

    /// Create a mock program with provided `system` and `mock`,
    /// and initialize it with provided `id`.
    ///
    /// See also [`Program::mock`].
    pub fn mock_with_id<ID, T>(system: &'a System, id: ID, mock: T) -> Self
    where
        T: WasmProgram + 'static,
        ID: Into<ProgramIdWrapper> + Clone + Debug,
    {
        Self::program_with_id(system, id, InnerProgram::new_mock(mock))
    }

    /// Send message to the program.
    pub fn send<ID, C>(&self, from: ID, payload: C) -> MessageId
    where
        ID: Into<ProgramIdWrapper>,
        C: Codec,
    {
        self.send_with_value(from, payload, 0)
    }

    /// Send message to the program with value.
    pub fn send_with_value<ID, C>(&self, from: ID, payload: C, value: u128) -> MessageId
    where
        ID: Into<ProgramIdWrapper>,
        C: Codec,
    {
        self.send_bytes_with_value(from, payload.encode(), value)
    }

    /// Send message to the program with gas limit and value.
    pub fn send_with_gas<ID, P>(
        &self,
        from: ID,
        payload: P,
        gas_limit: u64,
        value: u128,
    ) -> MessageId
    where
        ID: Into<ProgramIdWrapper>,
        P: Encode,
    {
        self.send_bytes_with_gas_and_value(from, payload.encode(), gas_limit, value)
    }

    /// Send message to the program with bytes payload.
    pub fn send_bytes<ID, T>(&self, from: ID, payload: T) -> MessageId
    where
        ID: Into<ProgramIdWrapper>,
        T: Into<Vec<u8>>,
    {
        self.send_bytes_with_value(from, payload, 0)
    }

    /// Send the message to the program with bytes payload and value.
    pub fn send_bytes_with_value<ID, T>(&self, from: ID, payload: T, value: u128) -> MessageId
    where
        ID: Into<ProgramIdWrapper>,
        T: Into<Vec<u8>>,
    {
        self.send_bytes_with_gas_and_value(from, payload, GAS_ALLOWANCE, value)
    }

    /// Send the message to the program with bytes payload, gas limit and value.
    pub fn send_bytes_with_gas<ID, T>(
        &self,
        from: ID,
        payload: T,
        gas_limit: u64,
        value: u128,
    ) -> MessageId
    where
        ID: Into<ProgramIdWrapper>,
        T: Into<Vec<u8>>,
    {
        self.send_bytes_with_gas_and_value(from, payload, gas_limit, value)
    }

    fn send_bytes_with_gas_and_value<ID, T>(
        &self,
        from: ID,
        payload: T,
        gas_limit: u64,
        value: u128,
    ) -> MessageId
    where
        ID: Into<ProgramIdWrapper>,
        T: Into<Vec<u8>>,
    {
        let mut system = self.manager.borrow_mut();

        let source = from.into().0;

        // The current block number is always a block number of the "executed" block.
        // So before sending any messages and triggering a block run the block number
        // equals to 0 (curr). So any new message sent by user goes to a new block,
        // that will be executed, i.e. block with number curr + 1.
        let block_number = system.block_height() + 1;
        let message = Message::new(
            MessageId::generate_from_user(
                block_number,
                source,
                system.fetch_inc_message_nonce() as u128,
            ),
            source,
            self.id,
            payload.into().try_into().unwrap(),
            Some(gas_limit),
            value,
            None,
        );

        let kind = Actors::modify(self.id, |actor| {
            let actor = actor.expect("Can't fail");
            if let TestActor::Uninitialized(id @ None, _) = actor {
                *id = Some(message.id());
                DispatchKind::Init
            } else {
                DispatchKind::Handle
            }
        });

        system.validate_and_route_dispatch(Dispatch::new(kind, message))
    }

    /// Get program id.
    pub fn id(&self) -> ProgramId {
        self.id
    }

    /// Reads the program’s state as a byte vector.
    pub fn read_state_bytes(&self, payload: Vec<u8>) -> Result<Vec<u8>> {
        self.manager
            .borrow_mut()
            .read_state_bytes(payload, &self.id)
    }

    /// Reads the program’s transformed state as a byte vector. The transformed
    /// state is a result of applying the `fn_name` function from the `wasm`
    /// binary with the optional `argument`.
    ///
    /// # Usage
    /// You can pass arguments as `Option<(arg1, arg2, ...).encode()>` or by
    /// using [`state_args_encoded`] macro.
    ///
    /// # Examples
    ///
    /// ```
    /// # use gtest::{state_args_encoded, Program, System, WasmProgram, Result};
    /// # use codec::Encode;
    /// # fn doctest() -> Result<()> {
    /// # #[derive(Debug)]
    /// # struct MockWasm {}
    /// #
    /// # impl WasmProgram for MockWasm {
    /// #     fn init(&mut self, _payload: Vec<u8>) -> Result<Option<Vec<u8>>, &'static str> { unimplemented!() }
    /// #     fn handle(&mut self, _payload: Vec<u8>) -> Result<Option<Vec<u8>>, &'static str> { unimplemented!() }
    /// #     fn handle_reply(&mut self, _payload: Vec<u8>) -> Result<(), &'static str> {unimplemented!() }
    /// #     fn handle_signal(&mut self, _payload: Vec<u8>) -> Result<(), &'static str> { unimplemented!()  }
    /// #     fn state(&mut self) -> Result<Vec<u8>, &'static str> { unimplemented!()  }
    /// #  }
    /// # let system = System::new();
    /// # let program = Program::mock(&system, MockWasm { });
    /// # let ARG_1 = 0u8;
    /// # let ARG_2 = 0u8;
    /// //Read state bytes with no arguments passed to wasm.
    /// # let WASM = vec![];
    /// let _ = program.read_state_bytes_using_wasm(Default::default(), "fn_name", WASM, Option::<Vec<u8>>::None)?;
    /// # let WASM = vec![];
    /// let _ = program.read_state_bytes_using_wasm(Default::default(), "fn_name", WASM, state_args_encoded!())?;
    /// // Read state bytes with one argument passed to wasm.
    /// # let WASM = vec![];
    /// let _ = program.read_state_bytes_using_wasm(Default::default(), "fn_name", WASM, Some(ARG_1.encode()))?;
    /// # let WASM = vec![];
    /// let _ = program.read_state_bytes_using_wasm(Default::default(), "fn_name", WASM, state_args_encoded!(ARG_1))?;
    /// // Read state bytes with multiple arguments passed to wasm.
    /// # let WASM = vec![];
    /// let _ = program.read_state_bytes_using_wasm(Default::default(), "fn_name", WASM, Some((ARG_1, ARG_2).encode()))?;
    /// # let WASM = vec![];
    /// let _ = program.read_state_bytes_using_wasm(Default::default(), "fn_name", WASM, state_args_encoded!(ARG_1, ARG_2))?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn read_state_bytes_using_wasm(
        &self,
        payload: Vec<u8>,
        fn_name: &str,
        wasm: Vec<u8>,
        args: Option<Vec<u8>>,
    ) -> Result<Vec<u8>> {
        self.manager
            .borrow_mut()
            .read_state_bytes_using_wasm(payload, &self.id, fn_name, wasm, args)
    }

    /// Reads and decodes the program's state .
    pub fn read_state<D: Decode, P: Encode>(&self, payload: P) -> Result<D> {
        let state_bytes = self.read_state_bytes(payload.encode())?;
        D::decode(&mut state_bytes.as_ref()).map_err(Into::into)
    }

    /// Reads and decodes the program’s transformed state. The transformed state
    /// is a result of applying the `fn_name` function from the `wasm`
    /// binary with the optional `argument`.
    ///
    /// # Usage
    /// You can pass arguments as `Option<(arg1, arg2, ...)>` or by
    /// using [`state_args`] macro.
    ///
    /// # Examples
    ///
    /// ```
    /// # use gtest::{state_args, Program, System, WasmProgram, Result};
    /// # fn doctest() -> Result<()> {
    /// # #[derive(Debug)]
    /// # struct MockWasm;
    /// #
    /// # impl WasmProgram for MockWasm {
    /// #     fn init(&mut self, _payload: Vec<u8>) -> Result<Option<Vec<u8>>, &'static str> { unimplemented!() }
    /// #     fn handle(&mut self, _payload: Vec<u8>) -> Result<Option<Vec<u8>>, &'static str> { unimplemented!() }
    /// #     fn handle_reply(&mut self, _payload: Vec<u8>) -> Result<(), &'static str> {unimplemented!() }
    /// #     fn handle_signal(&mut self, _payload: Vec<u8>) -> Result<(), &'static str> { unimplemented!()  }
    /// #     fn state(&mut self) -> Result<Vec<u8>, &'static str> { unimplemented!()  }
    /// #  }
    /// # let system = System::new();
    /// # let program = Program::mock(&system, MockWasm);
    /// # let ARG_1 = 0u8;
    /// # let ARG_2 = 0u8;
    /// //Read state bytes with no arguments passed to wasm.
    /// # let WASM = vec![];
    /// let _ = program.read_state_using_wasm(Vec::<u8>::default(), "fn_name", WASM, Option::<()>::None)?;
    /// # let WASM = vec![];
    /// let _ = program.read_state_using_wasm(Vec::<u8>::default(), "fn_name", WASM, state_args!())?;
    /// // Read state bytes with one argument passed to wasm.
    /// # let WASM = vec![];
    /// let _ = program.read_state_using_wasm(Vec::<u8>::default(), "fn_name", WASM, Some(ARG_1))?;
    /// # let WASM = vec![];
    /// let _ = program.read_state_using_wasm(Vec::<u8>::default(), "fn_name", WASM, state_args!(ARG_1))?;
    /// // Read state bytes with multiple arguments passed to wasm.
    /// # let WASM = vec![];
    /// let _ = program.read_state_using_wasm(Vec::<u8>::default(), "fn_name", WASM, Some((ARG_1, ARG_2)))?;
    /// # let WASM = vec![];
    /// let _ = program.read_state_using_wasm(Vec::<u8>::default(), "fn_name", WASM, state_args!(ARG_1, ARG_2))?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn read_state_using_wasm<E: Encode, P: Encode, D: Decode>(
        &self,
        payload: P,
        fn_name: &str,
        wasm: Vec<u8>,
        argument: Option<E>,
    ) -> Result<D> {
        let argument_bytes = argument.map(|arg| arg.encode());
        let state_bytes =
            self.read_state_bytes_using_wasm(payload.encode(), fn_name, wasm, argument_bytes)?;
        D::decode(&mut state_bytes.as_ref()).map_err(Into::into)
    }

    /// Returns the balance of the account.
    pub fn balance(&self) -> Value {
        self.manager.borrow().balance_of(&self.id())
    }

    /// Save the program's memory to path.
    pub fn save_memory_dump(&self, path: impl AsRef<Path>) {
        let manager = self.manager.borrow();
        let mem = manager.read_memory_pages(&self.id);
        let balance = manager.balance_of(&self.id);

        ProgramMemoryDump {
            balance,
            reserved_balance: 0,
            pages: mem
                .iter()
                .map(|(page_number, page_data)| {
                    MemoryPageDump::new(*page_number, page_data.clone())
                })
                .collect(),
        }
        .save_to_file(path);
    }

    /// Load the program's memory from path.
    pub fn load_memory_dump(&mut self, path: impl AsRef<Path>) {
        let memory_dump = ProgramMemoryDump::load_from_file(path);
        let mem = memory_dump
            .pages
            .into_iter()
            .map(MemoryPageDump::into_gear_page)
            .collect();

        // @TODO : add support for gas reservation when implemented
        let balance = memory_dump
            .balance
            .saturating_add(memory_dump.reserved_balance);

        self.manager
            .borrow_mut()
            .update_storage_pages(&self.id, mem);
        self.manager
            .borrow_mut()
            .override_balance(&self.id, balance);
    }
}

/// Calculate program id from code id and salt.
pub fn calculate_program_id(code_id: CodeId, salt: &[u8], id: Option<MessageId>) -> ProgramId {
    if let Some(id) = id {
        ProgramId::generate_from_program(id, code_id, salt)
    } else {
        ProgramId::generate_from_user(code_id, salt)
    }
}

/// `cargo-gbuild` utils
pub mod gbuild {
    use crate::{
        error::{usage_panic, TestError as Error},
        Result,
    };
    use cargo_toml::Manifest;
    use std::{path::PathBuf, process::Command};

    /// Search program wasm from
    ///
    /// - `target/gbuild`
    /// - `$WORKSPACE_ROOT/target/gbuild`
    ///
    /// NOTE: Release or Debug is decided by the users
    /// who run the command `cargo-gbuild`.
    pub fn wasm_path() -> Result<PathBuf> {
        let manifest_path = etc::find_up("Cargo.toml")
            .map_err(|_| Error::GbuildArtifactNotFound("Could not find manifest".into()))?;
        let manifest = Manifest::from_path(&manifest_path)
            .map_err(|_| Error::GbuildArtifactNotFound("Could not parse manifest".into()))?;
        let target = etc::find_up("target").unwrap_or(
            manifest_path
                .parent()
                .ok_or(Error::GbuildArtifactNotFound(
                    "Could not parse target directory".into(),
                ))?
                .to_path_buf(),
        );

        let artifact = target
            .join(format!(
                "gbuild/{}",
                manifest.package().name().replace('-', "_")
            ))
            .with_extension("wasm");

        if artifact.exists() {
            Ok(artifact)
        } else {
            Err(Error::GbuildArtifactNotFound(format!(
                "Program artifact not exist, {artifact:?}"
            )))
        }
    }

    /// Ensure the current project has been built by `cargo-gbuild`.
    pub fn ensure_gbuild(rebuild: bool) {
        if wasm_path().is_err() || rebuild {
            let manifest = etc::find_up("Cargo.toml").expect("Unable to find project manifest.");
            let mut kargo = Command::new("cargo");
            kargo.args(["gbuild", "-m"]).arg(&manifest);

            #[cfg(not(debug_assertions))]
            kargo.arg("--release");

            if !kargo
                .status()
                .expect("cargo-gbuild is not installed, try `cargo install cargo-gbuild` first.")
                .success()
            {
                usage_panic!(
                    "Error occurs while compiling the current program, please run `cargo gbuild` directly for the current project to detect the problem, \
                    manifest path: {manifest:?}"
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Program;

    use crate::{Log, ProgramIdWrapper, System, Value, DEFAULT_USER_ALICE, EXISTENTIAL_DEPOSIT};
    use demo_constructor::{Arg, Scheme};
    use gear_common::Origin;

    use gear_core::ids::ActorId;
    use gear_core_errors::{ErrorReplyReason, ReplyCode, SimpleExecutionError};

    #[test]
    fn test_handle_signal() {
        use demo_constructor::{Calls, Scheme, WASM_BINARY};
        let sys = System::new();
        sys.init_logger();

        let user_id = DEFAULT_USER_ALICE;
        let message = "Signal handle";
        let panic_message = "Gotcha!";

        let scheme = Scheme::predefined(
            Calls::builder().noop(),
            Calls::builder()
                .system_reserve_gas(4_000_000_000)
                .panic(panic_message),
            Calls::builder().noop(),
            Calls::builder().send(
                Arg::new(ProgramIdWrapper::from(user_id).0.into_bytes()),
                Arg::bytes(message),
            ),
        );
        let prog = Program::from_binary_with_id(&sys, 137, WASM_BINARY);
        let msg_id = prog.send(user_id, scheme);
        let res = sys.run_next_block();
        assert!(res.succeed.contains(&msg_id));
        let msg_id = prog.send(user_id, *b"Hello");
        let res = sys.run_next_block();
        res.assert_panicked_with(msg_id, panic_message);
        let log = Log::builder().payload_bytes(message);
        let value = sys.get_mailbox(user_id).claim_value(log);
        assert!(value.is_ok(), "not okay: {:?}", value);
    }

    #[test]
    fn test_handle_messages_to_failing_program() {
        let sys = System::new();
        sys.init_logger();

        let user_id = DEFAULT_USER_ALICE;

        let prog = Program::from_binary_with_id(&sys, 137, demo_futures_unordered::WASM_BINARY);

        let init_msg_payload = String::from("InvalidInput");
        let failed_mid = prog.send(user_id, init_msg_payload);
        let skipped_mid = prog.send(user_id, String::from("should_be_skipped"));

        let res = sys.run_next_block();

        res.assert_panicked_with(failed_mid, "Failed to load destination: Decode(Error)");

        let expected_log = Log::error_builder(ErrorReplyReason::InactiveActor)
            .source(prog.id())
            .dest(user_id);

        assert!(res.not_executed.contains(&skipped_mid));
        assert!(res.contains(&expected_log));
    }

    #[test]
    fn simple_balance() {
        let sys = System::new();
        sys.init_logger();

        let user_id = 42;
        let mut user_spent_balance = 0;
        sys.mint_to(user_id, 12 * EXISTENTIAL_DEPOSIT);
        assert_eq!(sys.balance_of(user_id), 12 * EXISTENTIAL_DEPOSIT);

        let program_id = 137;
        let prog = Program::from_binary_with_id(&sys, program_id, demo_ping::WASM_BINARY);

        sys.transfer(user_id, program_id, 2 * EXISTENTIAL_DEPOSIT, true);
        assert_eq!(prog.balance(), 2 * EXISTENTIAL_DEPOSIT);

        prog.send_with_value(user_id, "init".to_string(), EXISTENTIAL_DEPOSIT);
        // Note: ED is charged upon program creation if its balance is not 0.
        user_spent_balance += sys.run_next_block().spent_value() + EXISTENTIAL_DEPOSIT;
        assert_eq!(
            prog.balance(),
            3 * EXISTENTIAL_DEPOSIT + EXISTENTIAL_DEPOSIT
        );
        assert_eq!(
            sys.balance_of(user_id),
            9 * EXISTENTIAL_DEPOSIT - user_spent_balance
        );

        prog.send_with_value(user_id, "PING".to_string(), 2 * EXISTENTIAL_DEPOSIT);
        user_spent_balance += sys.run_next_block().spent_value();

        assert_eq!(
            prog.balance(),
            5 * EXISTENTIAL_DEPOSIT + EXISTENTIAL_DEPOSIT
        );
        assert_eq!(
            sys.balance_of(user_id),
            7 * EXISTENTIAL_DEPOSIT - user_spent_balance
        );
    }

    #[test]
    fn piggy_bank() {
        let sys = System::new();
        sys.init_logger();

        let receiver = 42;
        let sender0 = 43;
        let sender1 = 44;
        let sender2 = 45;

        // Top-up senders balances
        sys.mint_to(sender0, 20 * EXISTENTIAL_DEPOSIT);
        sys.mint_to(sender1, 20 * EXISTENTIAL_DEPOSIT);
        sys.mint_to(sender2, 20 * EXISTENTIAL_DEPOSIT);

        // Top-up receiver balance
        let mut receiver_expected_balance = 10 * EXISTENTIAL_DEPOSIT;
        sys.mint_to(receiver, receiver_expected_balance);

        let prog = Program::from_binary_with_id(&sys, 137, demo_piggy_bank::WASM_BINARY);

        prog.send_bytes(receiver, b"init");
        receiver_expected_balance -= sys.run_next_block().spent_value() + EXISTENTIAL_DEPOSIT;
        assert_eq!(prog.balance(), EXISTENTIAL_DEPOSIT);

        // Send values to the program
        prog.send_bytes_with_value(sender0, b"insert", 2 * EXISTENTIAL_DEPOSIT);
        let sender0_spent_value = sys.run_next_block().spent_value();
        assert_eq!(
            sys.balance_of(sender0),
            18 * EXISTENTIAL_DEPOSIT - sender0_spent_value
        );
        prog.send_bytes_with_value(sender1, b"insert", 4 * EXISTENTIAL_DEPOSIT);
        let sender1_spent_value = sys.run_next_block().spent_value();
        assert_eq!(
            sys.balance_of(sender1),
            16 * EXISTENTIAL_DEPOSIT - sender1_spent_value
        );
        prog.send_bytes_with_value(sender2, b"insert", 6 * EXISTENTIAL_DEPOSIT);
        let sender2_spent_value = sys.run_next_block().spent_value();
        assert_eq!(
            sys.balance_of(sender2),
            14 * EXISTENTIAL_DEPOSIT - sender2_spent_value
        );

        // Check program's balance
        assert_eq!(
            prog.balance(),
            (2 + 4 + 6) * EXISTENTIAL_DEPOSIT + EXISTENTIAL_DEPOSIT
        );

        // Request to smash the piggy bank and send the value to the receiver address
        prog.send_bytes(receiver, b"smash");
        let res = sys.run_next_block();
        receiver_expected_balance -= res.spent_value();
        let reply_to_id = {
            let log = res.log();
            // 1 auto reply and 1 message from program
            assert_eq!(log.len(), 2);

            let core_log = log
                .iter()
                .find(|&core_log| {
                    core_log.eq(&Log::builder().dest(receiver).payload_bytes(b"send"))
                })
                .expect("message not found");

            core_log.id()
        };

        assert!(sys
            .get_mailbox(receiver)
            .claim_value(Log::builder().reply_to(reply_to_id))
            .is_ok());
        assert_eq!(
            sys.balance_of(receiver),
            (2 + 4 + 6) * EXISTENTIAL_DEPOSIT + receiver_expected_balance
        );
        // Program is alive and holds the ED
        assert_eq!(prog.balance(), EXISTENTIAL_DEPOSIT);
    }

    #[test]
    #[should_panic(
        expected = "Failed to increase balance: the sum (1) of the total balance (0) and the value (1) \
        cannot be lower than the existential deposit (1000000000000)"
    )]
    fn mint_less_than_deposit() {
        System::new().mint_to(1, 1);
    }

    #[test]
    #[should_panic(
        expected = "Insufficient balance: user (0x0500000000000000000000000000000000000000000000000000000000000000) \
    tries to send (1000000000001) value, (4500000000000) gas and ED (1000000000000), while his balance (1000000000000)"
    )]
    fn fails_on_insufficient_balance() {
        let sys = System::new();

        let user = 5;
        let prog = Program::from_binary_with_id(&sys, 6, demo_piggy_bank::WASM_BINARY);

        assert_eq!(sys.balance_of(user), 0);
        sys.mint_to(user, EXISTENTIAL_DEPOSIT);
        assert_eq!(sys.balance_of(user), EXISTENTIAL_DEPOSIT);

        prog.send_bytes_with_value(user, b"init", EXISTENTIAL_DEPOSIT + 1);
        sys.run_next_block();
    }

    #[test]
    fn claim_zero_value() {
        let sys = System::new();
        sys.init_logger();

        const RECEIVER_INITIAL_BALANCE: Value = 10 * EXISTENTIAL_DEPOSIT;

        let sender = 42;
        let receiver = 84;
        let mut receiver_expected_balance = RECEIVER_INITIAL_BALANCE;

        sys.mint_to(sender, 20 * EXISTENTIAL_DEPOSIT);
        sys.mint_to(receiver, RECEIVER_INITIAL_BALANCE);

        let prog = Program::from_binary_with_id(&sys, 137, demo_piggy_bank::WASM_BINARY);

        prog.send_bytes(receiver, b"init");
        receiver_expected_balance -= sys.run_next_block().spent_value() + EXISTENTIAL_DEPOSIT;

        // Get zero value to the receiver's mailbox
        prog.send_bytes(receiver, b"smash");
        receiver_expected_balance -= sys.run_next_block().spent_value();

        let receiver_mailbox = sys.get_mailbox(receiver);
        assert!(receiver_mailbox
            .claim_value(Log::builder().dest(receiver).payload_bytes(b"send"))
            .is_ok());
        assert_eq!(sys.balance_of(receiver), receiver_expected_balance);

        // Get the value > ED to the receiver's mailbox
        prog.send_bytes_with_value(sender, b"insert", 2 * EXISTENTIAL_DEPOSIT);
        sys.run_next_block();
        prog.send_bytes(receiver, b"smash");
        receiver_expected_balance -= sys.run_next_block().spent_value();

        // Check receiver's balance
        assert!(receiver_mailbox
            .claim_value(Log::builder().dest(receiver).payload_bytes(b"send"))
            .is_ok());
        assert_eq!(
            sys.balance_of(receiver),
            2 * EXISTENTIAL_DEPOSIT + receiver_expected_balance
        );
        // Program is alive and holds the ED
        assert_eq!(prog.balance(), EXISTENTIAL_DEPOSIT);
    }

    struct CleanupFolderOnDrop {
        path: String,
    }

    impl Drop for CleanupFolderOnDrop {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.path).expect("Failed to cleanup after test")
        }
    }

    #[test]
    fn save_load_memory_dump() {
        use demo_custom::{InitMessage, WASM_BINARY};
        let sys = System::new();
        sys.init_logger();

        let mut prog = Program::from_binary_with_id(&sys, 420, WASM_BINARY);

        let signer = DEFAULT_USER_ALICE;
        let signer_mailbox = sys.get_mailbox(signer);

        // Init capacitor with limit = 15
        prog.send(signer, InitMessage::Capacitor("15".to_string()));
        sys.run_next_block();

        // Charge capacitor with charge = 10
        dbg!(prog.send_bytes(signer, b"10"));
        let res = sys.run_next_block();
        let log = Log::builder()
            .source(prog.id())
            .dest(signer)
            .payload_bytes([]);
        assert!(res.contains(&log));

        let cleanup = CleanupFolderOnDrop {
            path: "./296c6962726".to_string(),
        };
        prog.save_memory_dump("./296c6962726/demo_custom.dump");

        // Charge capacitor with charge = 10
        prog.send_bytes(signer, b"10");
        let res = sys.run_next_block();
        let log = Log::builder()
            .source(prog.id())
            .dest(signer)
            .payload_bytes("Discharged: 20");
        // dbg!(log.clone());
        assert!(res.contains(&log));
        assert!(signer_mailbox.claim_value(log).is_ok());

        prog.load_memory_dump("./296c6962726/demo_custom.dump");
        drop(cleanup);

        // Charge capacitor with charge = 10
        prog.send_bytes(signer, b"10");
        let res = sys.run_next_block();
        let log = Log::builder()
            .source(prog.id())
            .dest(signer)
            .payload_bytes("Discharged: 20");
        assert!(res.contains(&log));
        assert!(signer_mailbox.claim_value(log).is_ok());
    }

    #[test]
    fn process_wait_for() {
        use demo_custom::{InitMessage, WASM_BINARY};
        let sys = System::new();
        sys.init_logger();

        let prog = Program::from_binary_with_id(&sys, 420, WASM_BINARY);

        let signer = DEFAULT_USER_ALICE;

        // Init simple waiter
        prog.send(signer, InitMessage::SimpleWaiter);
        sys.run_next_block();

        // Invoke `exec::wait_for` when running for the first time
        prog.send_bytes(signer, b"doesn't matter");
        let result = sys.run_next_block();

        // No log entries as the program is waiting
        assert!(result.log().is_empty());

        // Run task pool to make the waiter to wake up
        let _ = sys.run_scheduled_tasks(20);
        let res = sys.run_next_block();

        let log = Log::builder()
            .source(prog.id())
            .dest(signer)
            .payload_bytes("hello");
        assert!(res.contains(&log));
    }

    // Test for issue#3699
    #[test]
    fn reservations_limit() {
        use demo_custom::{InitMessage, WASM_BINARY};
        let sys = System::new();

        let prog = Program::from_binary_with_id(&sys, 420, WASM_BINARY);

        let signer = DEFAULT_USER_ALICE;

        // Init reserver
        prog.send(signer, InitMessage::Reserver);
        sys.run_next_block();

        for _ in 0..258 {
            // Reserve
            let msg_id = prog.send_bytes(signer, b"reserve");
            let result = sys.run_next_block();
            assert!(result.succeed.contains(&msg_id));

            // Spend
            let msg_id = prog.send_bytes(signer, b"send from reservation");
            let result = sys.run_next_block();
            assert!(result.succeed.contains(&msg_id));
        }
    }

    #[test]
    fn test_handle_exit_with_zero_balance() {
        use demo_constructor::{demo_exit_handle, WASM_BINARY};

        let sys = System::new();
        sys.init_logger();

        let user_id = 42;
        let mut user_balance = 4 * EXISTENTIAL_DEPOSIT;
        sys.mint_to(user_id, user_balance);

        let prog_id = 137;
        assert_eq!(sys.balance_of(prog_id), 0);
        let prog = Program::from_binary_with_id(&sys, prog_id, WASM_BINARY);

        let msg_id = prog.send_with_gas(user_id, demo_exit_handle::scheme(), 10_000_000_000, 0);
        let result = sys.run_next_block();
        user_balance -= result.spent_value() + EXISTENTIAL_DEPOSIT;

        assert!(result.succeed.contains(&msg_id));
        assert_eq!(sys.balance_of(prog_id), EXISTENTIAL_DEPOSIT);
        assert_eq!(sys.balance_of(user_id), user_balance);

        let msg_id = prog.send_bytes_with_gas(user_id, [], 10_000_000_000, 0);
        let result = sys.run_next_block();
        user_balance -= result.spent_value();

        // ED returned upon program exit
        user_balance += EXISTENTIAL_DEPOSIT;
        assert!(result.succeed.contains(&msg_id));
        assert_eq!(sys.balance_of(prog_id), 0);
        assert_eq!(sys.balance_of(user_id), user_balance);
    }

    #[test]
    fn test_insufficient_gas() {
        let sys = System::new();
        sys.init_logger();

        let prog = Program::from_binary_with_id(&sys, 137, demo_ping::WASM_BINARY);

        let user_id = ActorId::zero();
        sys.mint_to(user_id, EXISTENTIAL_DEPOSIT * 2);

        // set insufficient gas for execution
        let msg_id = prog.send_with_gas(user_id, "init".to_string(), 1, 0);
        let res = sys.run_next_block();

        let expected_log =
            Log::builder()
                .source(prog.id())
                .dest(user_id)
                .reply_code(ReplyCode::Error(ErrorReplyReason::Execution(
                    SimpleExecutionError::RanOutOfGas,
                )));

        assert!(res.contains(&expected_log));
        assert!(res.failed.contains(&msg_id));
    }

    #[test]
    fn test_create_delete_reservation() {
        use demo_constructor::{Calls, WASM_BINARY};

        let sys = System::new();
        sys.init_logger();

        let user_id = DEFAULT_USER_ALICE;
        let prog = Program::from_binary_with_id(&sys, 4242, WASM_BINARY);

        // Initialize program
        let msg_id = prog.send(user_id, Scheme::empty());
        let res = sys.run_next_block();
        assert!(res.succeed.contains(&msg_id));

        // Reserve gas handle
        let handle = Calls::builder().reserve_gas(1_000_000, 10);
        let msg_id = prog.send(user_id, handle);
        let res = sys.run_next_block();
        assert!(res.succeed.contains(&msg_id));

        // Get reservation id from program
        let reservation_id = sys
            .0
            .borrow_mut()
            .update_genuine_program(prog.id(), |genuine_prog| {
                assert_eq!(genuine_prog.gas_reservation_map.len(), 1);
                genuine_prog
                    .gas_reservation_map
                    .iter()
                    .next()
                    .map(|(&id, _)| id)
                    .expect("reservation exists, checked upper; qed.")
            })
            .expect("internal error: existing prog not found");

        // Check reservation exists in the tree
        assert!(sys.0.borrow().gas_tree.exists(reservation_id));

        // Unreserve gas handle
        let handle = Calls::builder().unreserve_gas(reservation_id.into_bytes());
        let msg_id = prog.send(user_id, handle);
        let res = sys.run_next_block();
        assert!(res.succeed.contains(&msg_id));

        // Check reservation is removed from the tree
        assert!(!sys.0.borrow().gas_tree.exists(reservation_id));
    }

    #[test]
    fn test_delete_expired_reservation() {
        use demo_constructor::{Calls, WASM_BINARY};

        let sys = System::new();
        sys.init_logger();

        let user_id = DEFAULT_USER_ALICE;
        let prog = Program::from_binary_with_id(&sys, 4242, WASM_BINARY);

        // Initialize program
        let msg_id = prog.send(user_id, Scheme::empty());
        let res = sys.run_next_block();
        assert!(res.succeed.contains(&msg_id));

        // Reserve gas handle
        let handle = Calls::builder().reserve_gas(1_000_000, 1);
        let msg_id = prog.send(user_id, handle);
        let res = sys.run_next_block();
        assert!(res.succeed.contains(&msg_id));

        // Get reservation id from program
        let reservation_id = sys
            .0
            .borrow_mut()
            .update_genuine_program(prog.id(), |genuine_prog| {
                assert_eq!(genuine_prog.gas_reservation_map.len(), 1);
                genuine_prog
                    .gas_reservation_map
                    .iter()
                    .next()
                    .map(|(&id, _)| id)
                    .expect("reservation exists, checked upper; qed.")
            })
            .expect("internal error: existing prog not found");

        // Check reservation exists in the tree
        assert!(sys.0.borrow().gas_tree.exists(reservation_id));

        sys.run_next_block();

        assert!(!sys.0.borrow().gas_tree.exists(reservation_id));
    }

    #[test]
    fn test_reservation_send() {
        use demo_constructor::{Calls, WASM_BINARY};

        let sys = System::new();
        sys.init_logger();

        let user_id = DEFAULT_USER_ALICE;
        let prog_id = 4242;
        let prog = Program::from_binary_with_id(&sys, prog_id, WASM_BINARY);

        // Initialize program
        let msg_id = prog.send(user_id, Scheme::empty());
        let res = sys.run_next_block();
        assert!(res.succeed.contains(&msg_id));

        // Send user message from reservation
        let payload = b"to_user".to_vec();
        let handle = Calls::builder()
            .reserve_gas(10_000_000_000, 5)
            .store("reservation")
            .reservation_send_value("reservation", user_id.into_origin().0, payload.clone(), 0);
        let msg_id = prog.send(user_id, handle);
        let res = sys.run_next_block();
        assert!(res.succeed.contains(&msg_id));

        // Check user message in mailbox
        let mailbox = sys.get_mailbox(user_id);
        assert!(mailbox.contains(&Log::builder().payload(payload).source(prog_id)));

        // Initialize another program for another test
        let new_prog_id = 4343;
        let new_program = Program::from_binary_with_id(&sys, new_prog_id, WASM_BINARY);
        let payload = b"sup!".to_vec();
        let handle = Calls::builder().send(user_id.into_origin().0, payload.clone());
        let scheme = Scheme::predefined(
            Calls::builder().noop(),
            handle,
            Calls::builder().noop(),
            Calls::builder().noop(),
        );
        let msg_id = new_program.send(user_id, scheme);
        let res = sys.run_next_block();
        assert!(res.succeed.contains(&msg_id));

        // Send program message from reservation
        let handle = Calls::builder()
            .reserve_gas(10_000_000_000, 5)
            .store("reservation")
            .reservation_send_value("reservation", new_prog_id.into_origin().0, [], 0);
        let msg_id = prog.send(user_id, handle);
        let res = sys.run_next_block();
        assert!(res.succeed.contains(&msg_id));
        assert!(mailbox.contains(&Log::builder().payload_bytes(payload).source(new_prog_id)));
    }

    #[test]
    fn tests_unused_gas_value_not_transferred() {
        let sys = System::new();
        sys.init_verbose_logger();

        let user = 42;
        sys.mint_to(user, 2 * EXISTENTIAL_DEPOSIT);

        let prog = Program::from_binary_with_id(&sys, 69, demo_piggy_bank::WASM_BINARY);
        prog.send_bytes_with_gas(user, b"init", 1_000_000_000, 0);
        sys.run_next_block();

        // Unspent gas is not returned to the user's balance when the sum of these is
        // lower than ED
        assert_eq!(sys.balance_of(user), 0)
    }
}
