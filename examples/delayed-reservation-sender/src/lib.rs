// This file is part of Gear.

// Copyright (C) 2021-2023 Gear Technologies Inc.
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

#![no_std]

use gstd::codec::{Decode, Encode};

#[cfg(feature = "std")]
mod code {
    include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));
}

#[cfg(feature = "std")]
pub use code::WASM_BINARY_OPT as WASM_BINARY;

pub const SENDING_EXPECT: &str = "Failed to send delayed message from reservation";

#[derive(Encode, Decode, Debug, Clone, Copy)]
#[codec(crate = gstd::codec)]
pub enum ReservationSendingShowcase {
    ToSourceInPlace {
        reservation_amount: u64,
        reservation_delay: u32,
        sending_delay: u32,
    },
    ToSourceAfterWait {
        reservation_amount: u64,
        reservation_delay: u32,
        wait_for: u32,
        sending_delay: u32,
    },
}

#[cfg(not(feature = "std"))]
mod wasm;