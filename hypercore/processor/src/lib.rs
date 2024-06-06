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

//! Program's execution service for eGPU.

use anyhow::Result;
use db::Database;
use gear_core::{
    ids::{prelude::CodeIdExt as _, ProgramId},
    message::IncomingMessage,
};
use gprimitives::{CodeId, H256};
use host::InstanceWrapper;
use hypercore_db::CASDatabase;
use hypercore_observer::Event;
use std::collections::HashMap;

pub(crate) mod db;
pub mod host;
pub(crate) mod state;

pub struct Processor {
    db: Database,
}

impl Processor {
    pub fn new(db: Box<dyn CASDatabase>) -> Self {
        Self {
            db: Database::new(db),
        }
    }

    pub fn new_code(&mut self, hash: CodeId, code: Vec<u8>) -> Result<bool> {
        if hash != CodeId::generate(&code) {
            return Ok(false);
        }

        let mut executor = InstanceWrapper::new()?;

        let res = executor.verify(&code)?;

        if res {
            self.db.write_code(hash, &code)
        }

        Ok(res)
    }

    pub fn instrument_code(&mut self, code_id: CodeId) -> Result<Option<H256>> {
        let code = self.db.read_code(code_id).unwrap();

        let mut instance_wrapper = host::InstanceWrapper::new()?;

        if let Some(instrumented) = instance_wrapper.instrument(code)? {
            let hash = self.db.write_instrumented_code(&instrumented);

            Ok(Some(hash))
        } else {
            Ok(None)
        }
    }

    // TODO: use proper `Dispatch` type here instead of db's.
    pub fn run(
        &mut self,
        _chain_head: H256,
        _programs: Vec<ProgramId>,
        _messages: HashMap<ProgramId, Vec<IncomingMessage>>,
    ) -> Result<()> {
        Ok(())
    }

    pub fn process_observer_event(&mut self, event: &Event) -> Result<()> {
        match event {
            Event::UploadCode { code_id, .. } => {
                log::debug!("Processing upload code {code_id:?}");
            }
            Event::Block {
                ref block_hash,
                events: _,
            } => {
                log::debug!("Processing events for {block_hash:?}");
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use core_processor::common::JournalNote;
    use gear_core::message::{DispatchKind, Payload};
    use hypercore_db::MemDb;
    use hypercore_runtime_native::{
        process_program,
        state::{Dispatch, MaybeHash, MessageQueue, ProgramState},
        NativeRuntimeInterface,
    };
    use wabt::wat2wasm;

    fn valid_code() -> Vec<u8> {
        let wat = r#"
            (module
            (import "env" "memory" (memory 1))
            (import "env" "gr_reply" (func $reply (param i32 i32 i32 i32)))
            (export "init" (func $init))
            (func $init
                (call $reply (i32.const 0) (i32.const 32) (i32.const 222) (i32.const 333))
            )
        )"#;

        wat2wasm(wat).unwrap()
    }

    fn init_logger() {
        let _ = env_logger::Builder::from_default_env()
            .format_module_path(false)
            .format_level(true)
            .try_init();
    }

    #[test]
    fn verify_code() {
        init_logger();

        let db = MemDb::default();
        let mut processor = Processor::new(db.clone_boxed());

        let valid = valid_code();
        let valid_id = CodeId::generate(&valid);

        assert!(processor.db.read_code(valid_id).is_none());
        assert!(processor.new_code(valid_id, valid).unwrap());
        assert!(processor.db.read_code(valid_id).is_some());

        let invalid = vec![0; 42];
        let invalid_id = CodeId::generate(&invalid);

        assert!(processor.db.read_code(invalid_id).is_none());
        assert!(!processor.new_code(invalid_id, invalid).unwrap());
        assert!(processor.db.read_code(invalid_id).is_none());
    }

    #[test]
    fn bad_hash() {
        init_logger();

        let db = MemDb::default();
        let mut processor = Processor::new(db.clone_boxed());

        let valid = valid_code();
        let valid_id = CodeId::generate(&valid).into_bytes().into();

        assert!(processor.new_code(valid_id, valid.clone()).unwrap());
        assert!(!processor.new_code(H256::zero().into(), valid).unwrap());
    }

    #[test]
    fn instrument_code() {
        init_logger();

        let db = MemDb::default();
        let mut processor = Processor::new(db.clone_boxed());

        let code = valid_code();
        let code_len = code.len();
        let id = CodeId::generate(&code);

        assert!(processor.new_code(id, code).unwrap());

        let hash = processor.instrument_code(id).unwrap().unwrap();
        let instrumented = processor.db.read_instrumented_code(hash).unwrap();

        assert_eq!(instrumented.original_code_len() as usize, code_len);
        assert!(instrumented.code().len() > code_len);
    }

    #[test]
    fn ping_pong() {
        init_logger();

        let db = MemDb::default();
        let mut processor = Processor::new(db.clone_boxed());

        let code = demo_ping::WASM_BINARY;
        let code_id = CodeId::generate(code);
        assert!(processor.new_code(code_id, code.to_vec()).unwrap());

        let instrumented_code_hash = processor.instrument_code(code_id).unwrap().unwrap();

        let payload = Payload::try_from(b"PING".to_vec()).unwrap();
        let payload_hash = processor.db.write(&payload);

        let dispatch = Dispatch {
            id: Default::default(),
            kind: DispatchKind::Handle,
            source: Default::default(),
            payload_hash: payload_hash.into(),
            gas_limit: 1_000_000_000,
            value: 1,
            details: None,
            context: None,
        };

        let queue = MessageQueue(vec![dispatch]);
        let queue_hash = processor.db.write(&queue);

        let program_id = ProgramId::default();
        let program_state = ProgramState {
            queue_hash: queue_hash.into(),
            allocations_hash: MaybeHash::Empty,
            pages_hash: MaybeHash::Empty,
            original_code_hash: H256::from(code_id.into_bytes()).into(),
            instrumented_code_hash: instrumented_code_hash.into(),
            gas_reservation_map_hash: MaybeHash::Empty,
            memory_infix: Default::default(),
            balance: 0,
        };

        let mut ri = NativeRuntimeInterface::new(processor.db.inner());
        let journal = process_program(program_id, &program_state, &mut ri);
        for note in journal {
            if let JournalNote::SendDispatch { dispatch, .. } = note {
                assert_eq!(
                    String::from_utf8(dispatch.message().payload_bytes().to_vec()),
                    Ok("PONG".to_string())
                );
                return;
            }
        }
        panic!("No SendDispatch note found");
    }
}
