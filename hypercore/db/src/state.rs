// This file is part of Gear.
//
// Copyright (C) 2024 Gear Technologies Inc.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! State-related data structures.

use gear_core::ids::{self, ProgramId};
use gprimitives::H256;

#[derive(Clone, Debug)]
pub struct Message {
    pub sender: ProgramId,
    pub gas_limit: u64,
    pub value: u128,
    pub data: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct Page {
    pub index: u32,
    pub data: Vec<u8>,
}

/// Hypercore program state.
#[derive(Clone, Debug)]
pub struct State {
    /// Program ID.
    pub program_id: ProgramId,
    pub queue: Vec<Message>,
    pub pages: Vec<Page>,
}

impl State {
    pub fn hash(&self) -> H256 {
        let mut array = Vec::new();
        array.extend_from_slice(self.program_id.as_ref());

        for queue_item in &self.queue {
            array.extend_from_slice(&queue_item.hash().0);
        }

        for page in &self.pages {
            array.extend_from_slice(&page.hash().0);
        }

        ids::hash(&array).into()
    }
}

impl Page {
    pub fn hash(&self) -> H256 {
        let mut array = Vec::new();
        array.extend_from_slice(&self.data);
        array.extend_from_slice(&self.index.to_le_bytes());

        ids::hash(&array).into()
    }
}

impl Message {
    pub fn hash(&self) -> H256 {
        let mut array = Vec::new();
        array.extend_from_slice(self.sender.as_ref());
        array.extend_from_slice(&self.gas_limit.to_le_bytes());
        array.extend_from_slice(&self.value.to_le_bytes());
        array.extend_from_slice(&self.data);

        ids::hash(&array).into()
    }
}