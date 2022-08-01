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

//! This `gstd` module provides async messaging functions.

use crate::{
    async_runtime::{signals, ReplyPoll},
    errors::{ContractError, Result},
    prelude::{convert::AsRef, Vec},
    ActorId, CodeHash, MessageId,
};
use codec::{Decode, Encode};
use core::{
    future::Future,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};
use futures::future::FusedFuture;

/// To interrupt a program execution waiting for a reply on a previous message,
/// one needs to call an `.await` expression.
/// The initial message that requires a reply is sent instantly.
/// Function `send_for_reply` returns `CodecMessageFuture` which
/// implements `Future` trait. Program interrupts until the reply is received.
/// As soon as the reply is received, the function checks it's exit code and
/// returns `Ok()` with decoded structure inside or `Err()` in case of exit code
/// does not equal 0. For decode-related errors (<https://docs.rs/parity-scale-codec/2.3.1/parity_scale_codec/struct.Error.html>),
/// Gear returns the native one after decode.
pub struct CodecMessageFuture<T> {
    /// Waiting reply to this the message id
    pub waiting_reply_to: MessageId,
    /// Marker
    ///
    /// # Note
    ///
    /// Need to `pub` this field because we are constructing this
    /// field in other files
    pub(super) _marker: PhantomData<T>,
}

impl<D: Decode> Future for CodecMessageFuture<D> {
    type Output = Result<D>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let fut = &mut self;
        match signals().poll(fut.waiting_reply_to, cx) {
            ReplyPoll::None => panic!("Somebody created CodecMessageFuture with the MessageId that never ended in static replies!"),
            ReplyPoll::Pending => Poll::Pending,
            ReplyPoll::Some((actual_reply, exit_code)) => {
                if exit_code != 0 {
                    return Poll::Ready(Err(ContractError::ExitCode(exit_code)));
                }

                Poll::Ready(D::decode(&mut actual_reply.as_ref()).map_err(ContractError::Decode))
            },
        }
    }
}

impl<D: Decode> FusedFuture for CodecMessageFuture<D> {
    fn is_terminated(&self) -> bool {
        !signals().waits_for(self.waiting_reply_to)
    }
}

/// To interrupt a program execution waiting for a reply on a previous message,
/// one needs to call an `.await` expression.
/// The initial message that requires a reply is sent instantly.
/// Function `send_bytes_for_reply` returns `MessageFuture` which
/// implements `Future` trait. Program interrupts until the reply is received.
/// As soon as the reply is received, the function checks it's exit code and
/// returns `Ok()` with raw bytes inside or `Err()` in case of exit code does
/// not equal 0. For decode-related errors (<https://docs.rs/parity-scale-codec/2.3.1/parity_scale_codec/struct.Error.html>),
/// Gear returns the native one after decode.
pub struct MessageFuture {
    /// Waiting reply to this the message id
    pub waiting_reply_to: MessageId,
}

impl Future for MessageFuture {
    type Output = Result<Vec<u8>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let fut = &mut *self;
        match signals().poll(fut.waiting_reply_to, cx) {
            ReplyPoll::None => panic!("Somebody created MessageFuture with the MessageId that never ended in static replies!"),
            ReplyPoll::Pending => Poll::Pending,
            ReplyPoll::Some((actual_reply, exit_code)) => {
                if exit_code != 0 {
                    return Poll::Ready(Err(ContractError::ExitCode(exit_code)));
                }

                Poll::Ready(Ok(actual_reply))
            },
        }
    }
}

impl FusedFuture for MessageFuture {
    fn is_terminated(&self) -> bool {
        !signals().waits_for(self.waiting_reply_to)
    }
}

/// Send a message and wait for reply.
///
/// This function works similarly to [`send_bytes_and_wait_for_reply`],
/// with one difference - it takes the structure in, then encodes it
/// and sends it in bytes. When receiving the message, it decodes the bytes.
/// So the input should be SCALE codeс encodable, output - decodable
/// (<https://docs.substrate.io/v3/advanced/scale-codec/>).
/// The program will be interrupted (waiting for a reply) if an `.await`
/// has been called on the `CodecMessageFuture` object returned by the function.
#[deprecated(note = "please use `gstd::msg::basic::send_for_reply` instead")]
pub fn send_and_wait_for_reply<D: Decode, E: Encode>(
    program: ActorId,
    payload: E,
    value: u128,
) -> Result<CodecMessageFuture<D>> {
    let waiting_reply_to = crate::msg::send(program, payload, value)?;
    signals().register_signal(waiting_reply_to);

    Ok(CodecMessageFuture::<D> {
        waiting_reply_to,
        _marker: PhantomData,
    })
}

/// Send a message and wait for reply.
///
/// This function works similarly to [`send_and_wait_for_reply`],
/// with one difference - it works with raw bytes as a payload.
/// The program will be interrupted (waiting for a reply) if an `.await`
/// has been called on the `MessageFuture` object returned by the function.
#[deprecated(note = "please use `gstd::msg::basic::send_bytes_for_reply` instead")]
pub fn send_bytes_and_wait_for_reply<T: AsRef<[u8]>>(
    program: ActorId,
    payload: T,
    value: u128,
) -> Result<MessageFuture> {
    let waiting_reply_to = crate::msg::send_bytes(program, payload, value)?;
    signals().register_signal(waiting_reply_to);

    Ok(MessageFuture { waiting_reply_to })
}

/// Create a program and wait for reply.
///
/// This function works similarly to
/// [`create_program_wbytes_for_reply`], with one difference - it takes
/// the structure in, then encodes it and sends it in bytes. The program will be
/// interrupted (waiting for a reply) if an `.await` has been called on the
/// `CodecMessageFuture` object returned by the function.
pub fn create_program_for_reply<E: Encode, D: Decode>(
    code_hash: CodeHash,
    payload: E,
    value: u128,
) -> Result<(ActorId, CodecMessageFuture<D>)> {
    let (actor_id, init_message_id) =
        crate::prog::ProgramGenerator::create_program(code_hash, payload.encode(), value)?;
    signals().register_signal(init_message_id);

    Ok((
        actor_id,
        CodecMessageFuture::<D> {
            waiting_reply_to: init_message_id,
            _marker: PhantomData,
        },
    ))
}

/// Create a program with bytes payload and wait for reply.
///
/// This function works similarly to [`create_program_for_reply`],
/// with one difference - it works with raw bytes as a payload.
/// The program will be interrupted (waiting for a reply) if an `.await`
/// has been called on the `MessageFuture` object returned by the function.
pub fn create_program_wbytes_for_reply<T: AsRef<[u8]>>(
    code_hash: CodeHash,
    payload: T,
    value: u128,
) -> Result<(ActorId, MessageFuture)> {
    let (actor_id, init_message_id) =
        crate::prog::ProgramGenerator::create_program(code_hash, payload, value)?;
    signals().register_signal(init_message_id);
    Ok((
        actor_id,
        MessageFuture {
            waiting_reply_to: init_message_id,
        },
    ))
}

/// Create a program with gas and wait for reply.
///
/// This function works similarly to
/// [`create_program_wgas_wbytes_for_reply`], with one difference - it
/// takes the structure in, then encodes it and sends it in bytes. The program
/// will be interrupted (waiting for a reply) if an `.await` has been called on
/// the `CodecMessageFuture` object returned by the function.
pub fn create_program_wgas_for_reply<E: Encode, D: Decode>(
    code_hash: CodeHash,
    payload: E,
    gas_limit: u64,
    value: u128,
) -> Result<(ActorId, CodecMessageFuture<D>)> {
    let (actor_id, init_message_id) = crate::prog::ProgramGenerator::create_program_with_gas(
        code_hash,
        payload.encode(),
        gas_limit,
        value,
    )?;
    signals().register_signal(init_message_id);

    Ok((
        actor_id,
        CodecMessageFuture {
            waiting_reply_to: init_message_id,
            _marker: PhantomData,
        },
    ))
}

/// Create a program with gas with init message payload in bytes and wait for
/// reply.
///
/// This function works similarly to [`create_program_wgas_for_reply`],
/// with one difference - it works with raw bytes as a payload.
/// The program will be interrupted (waiting for a reply) if an `.await`
/// has been called on the `MessageFuture` object returned by the function.
pub fn create_program_wgas_wbytes_for_reply<T: AsRef<[u8]>>(
    code_hash: CodeHash,
    payload: T,
    gas_limit: u64,
    value: u128,
) -> Result<(ActorId, MessageFuture)> {
    let (actor_id, init_message_id) = crate::prog::ProgramGenerator::create_program_with_gas(
        code_hash, payload, gas_limit, value,
    )?;
    signals().register_signal(init_message_id);

    Ok((
        actor_id,
        MessageFuture {
            waiting_reply_to: init_message_id,
        },
    ))
}

/// Create a program with salt and wait for reply.
///
/// This function works similarly to
/// [`create_program_wsalt_wbytes_for_reply`], with one difference - it takes
/// the structure in, then encodes it and sends it in bytes. The program will be
/// interrupted (waiting for a reply) if an `.await` has been called on the
/// `CodecMessageFuture` object returned by the function.
pub fn create_program_wsalt_for_reply<E: Encode, D: Decode, T: AsRef<[u8]>>(
    code_hash: CodeHash,
    salt: T,
    payload: E,
    value: u128,
) -> Result<(ActorId, CodecMessageFuture<D>)> {
    let (actor_id, init_message_id) =
        crate::prog::create_program(code_hash, salt, payload.encode(), value)?;
    signals().register_signal(init_message_id);

    Ok((
        actor_id,
        CodecMessageFuture::<D> {
            waiting_reply_to: init_message_id,
            _marker: PhantomData,
        },
    ))
}

/// Create a program with salt with bytes payload and wait for reply.
///
/// This function works similarly to [`create_program_wsalt_for_reply`],
/// with one difference - it works with raw bytes as a payload.
/// The program will be interrupted (waiting for a reply) if an `.await`
/// has been called on the `MessageFuture` object returned by the function.
pub fn create_program_wsalt_wbytes_for_reply<T1: AsRef<[u8]>, T2: AsRef<[u8]>>(
    code_hash: CodeHash,
    salt: T1,
    payload: T2,
    value: u128,
) -> Result<(ActorId, MessageFuture)> {
    let (actor_id, init_message_id) = crate::prog::create_program(code_hash, salt, payload, value)?;
    signals().register_signal(init_message_id);
    Ok((
        actor_id,
        MessageFuture {
            waiting_reply_to: init_message_id,
        },
    ))
}

/// Create a program with salt with gas and wait for reply.
///
/// This function works similarly to
/// [`create_program_wgas_wsalt_wbytes_for_reply`], with one difference - it
/// takes the structure in, then encodes it and sends it in bytes. The program
/// will be interrupted (waiting for a reply) if an `.await` has been called on
/// the `CodecMessageFuture` object returned by the function.
pub fn create_program_wsalt_wgas_for_reply<E: Encode, D: Decode, T: AsRef<[u8]>>(
    code_hash: CodeHash,
    salt: T,
    payload: E,
    gas_limit: u64,
    value: u128,
) -> Result<(ActorId, CodecMessageFuture<D>)> {
    let (actor_id, init_message_id) =
        crate::prog::create_program_with_gas(code_hash, salt, payload.encode(), gas_limit, value)?;
    signals().register_signal(init_message_id);

    Ok((
        actor_id,
        CodecMessageFuture {
            waiting_reply_to: init_message_id,
            _marker: PhantomData,
        },
    ))
}

/// Create a program with salt with gas with init message payload in bytes and
/// wait for reply.
///
/// This function works similarly to [`create_program_wsalt_wgas_for_reply`],
/// with one difference - it works with raw bytes as a payload.
/// The program will be interrupted (waiting for a reply) if an `.await`
/// has been called on the `MessageFuture` object returned by the function.
pub fn create_program_wsalt_wgas_wbytes_for_reply<T1: AsRef<[u8]>, T2: AsRef<[u8]>>(
    code_hash: CodeHash,
    salt: T1,
    payload: T2,
    gas_limit: u64,
    value: u128,
) -> Result<(ActorId, MessageFuture)> {
    let (actor_id, init_message_id) =
        crate::prog::create_program_with_gas(code_hash, salt, payload, gas_limit, value)?;
    signals().register_signal(init_message_id);

    Ok((
        actor_id,
        MessageFuture {
            waiting_reply_to: init_message_id,
        },
    ))
}
