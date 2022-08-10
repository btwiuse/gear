// This file is part of Gear.

// Copyright (C) 2022 Gear Technologies Inc.
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

//! Autogenerated weights for pallet_gear
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-08-06, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/gear-node benchmark pallet --chain=dev --steps=50 --repeat=20 --pallet=pallet_gear --extrinsic=* --execution=wasm --wasm-execution=compiled --heap-pages=4096 --output=./pallets/gear/src/weights.rs --template=./.maintain/gear-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_gear.
pub trait WeightInfo {
	fn allocation_cost() -> Weight;
	fn grow_cost() -> Weight;
	fn initial_cost() -> Weight;
	fn load_cost() -> Weight;
	fn claim_value() -> Weight;
	fn upload_code(c: u32, ) -> Weight;
	fn create_program(s: u32, ) -> Weight;
	fn upload_program(c: u32, s: u32, ) -> Weight;
	fn send_message(p: u32, ) -> Weight;
	fn send_reply(p: u32, ) -> Weight;
	fn initial_allocation(q: u32, ) -> Weight;
	fn alloc_in_handle(q: u32, ) -> Weight;
	fn reinstrument(c: u32, ) -> Weight;
	fn alloc(r: u32, ) -> Weight;
	fn gas(r: u32, ) -> Weight;
	fn gr_gas_available(r: u32, ) -> Weight;
	fn gr_msg_id(r: u32, ) -> Weight;
	fn gr_origin(r: u32, ) -> Weight;
	fn gr_program_id(r: u32, ) -> Weight;
	fn gr_source(r: u32, ) -> Weight;
	fn gr_value(r: u32, ) -> Weight;
	fn gr_value_available(r: u32, ) -> Weight;
	fn gr_size(r: u32, ) -> Weight;
	fn gr_read(r: u32, ) -> Weight;
	fn gr_read_per_kb(n: u32, ) -> Weight;
	fn gr_block_height(r: u32, ) -> Weight;
	fn gr_block_timestamp(r: u32, ) -> Weight;
	fn gr_send_init(r: u32, ) -> Weight;
	fn gr_send_push(r: u32, ) -> Weight;
	fn gr_send_push_per_kb(n: u32, ) -> Weight;
	fn gr_send_commit(r: u32, ) -> Weight;
	fn gr_send_commit_per_kb(n: u32, ) -> Weight;
	fn gr_reply_commit(r: u32, ) -> Weight;
	fn gr_reply_commit_per_kb(n: u32, ) -> Weight;
	fn gr_reply_push(r: u32, ) -> Weight;
	fn gr_reply_push_per_kb(n: u32, ) -> Weight;
	fn gr_reply_to(r: u32, ) -> Weight;
	fn gr_debug(r: u32, ) -> Weight;
	fn gr_exit_code(r: u32, ) -> Weight;
	fn gr_exit(r: u32, ) -> Weight;
	fn gr_leave(r: u32, ) -> Weight;
	fn gr_wait(r: u32, ) -> Weight;
	fn gr_wake(r: u32, ) -> Weight;
	fn gr_create_program_wgas(r: u32, ) -> Weight;
	fn gr_create_program_wgas_per_kb(n: u32, ) -> Weight;
	fn instr_i64const(r: u32, ) -> Weight;
	fn instr_i64load(r: u32, ) -> Weight;
	fn instr_i64store(r: u32, ) -> Weight;
	fn instr_select(r: u32, ) -> Weight;
	fn instr_if(r: u32, ) -> Weight;
	fn instr_br(r: u32, ) -> Weight;
	fn instr_br_if(r: u32, ) -> Weight;
	fn instr_br_table(r: u32, ) -> Weight;
	fn instr_br_table_per_entry(e: u32, ) -> Weight;
	fn instr_call(r: u32, ) -> Weight;
	fn instr_call_indirect(r: u32, ) -> Weight;
	fn instr_call_indirect_per_param(p: u32, ) -> Weight;
	fn instr_local_get(r: u32, ) -> Weight;
	fn instr_local_set(r: u32, ) -> Weight;
	fn instr_local_tee(r: u32, ) -> Weight;
	fn instr_global_get(r: u32, ) -> Weight;
	fn instr_global_set(r: u32, ) -> Weight;
	fn instr_memory_current(r: u32, ) -> Weight;
	fn instr_i64clz(r: u32, ) -> Weight;
	fn instr_i64ctz(r: u32, ) -> Weight;
	fn instr_i64popcnt(r: u32, ) -> Weight;
	fn instr_i64eqz(r: u32, ) -> Weight;
	fn instr_i64extendsi32(r: u32, ) -> Weight;
	fn instr_i64extendui32(r: u32, ) -> Weight;
	fn instr_i32wrapi64(r: u32, ) -> Weight;
	fn instr_i64eq(r: u32, ) -> Weight;
	fn instr_i64ne(r: u32, ) -> Weight;
	fn instr_i64lts(r: u32, ) -> Weight;
	fn instr_i64ltu(r: u32, ) -> Weight;
	fn instr_i64gts(r: u32, ) -> Weight;
	fn instr_i64gtu(r: u32, ) -> Weight;
	fn instr_i64les(r: u32, ) -> Weight;
	fn instr_i64leu(r: u32, ) -> Weight;
	fn instr_i64ges(r: u32, ) -> Weight;
	fn instr_i64geu(r: u32, ) -> Weight;
	fn instr_i64add(r: u32, ) -> Weight;
	fn instr_i64sub(r: u32, ) -> Weight;
	fn instr_i64mul(r: u32, ) -> Weight;
	fn instr_i64divs(r: u32, ) -> Weight;
	fn instr_i64divu(r: u32, ) -> Weight;
	fn instr_i64rems(r: u32, ) -> Weight;
	fn instr_i64remu(r: u32, ) -> Weight;
	fn instr_i64and(r: u32, ) -> Weight;
	fn instr_i64or(r: u32, ) -> Weight;
	fn instr_i64xor(r: u32, ) -> Weight;
	fn instr_i64shl(r: u32, ) -> Weight;
	fn instr_i64shrs(r: u32, ) -> Weight;
	fn instr_i64shru(r: u32, ) -> Weight;
	fn instr_i64rotl(r: u32, ) -> Weight;
	fn instr_i64rotr(r: u32, ) -> Weight;
}

/// Weights for pallet_gear using the Gear node and recommended hardware.
pub struct GearWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for GearWeight<T> {
	fn allocation_cost() -> Weight {
		// To be changed with the proper value.
		T::DbWeight::get().writes(1 as Weight)
	}
	fn grow_cost() -> Weight {
		// To be changed with the proper value.
		T::DbWeight::get().writes(1 as Weight)
	}
	fn initial_cost() -> Weight {
		T::DbWeight::get().writes(1 as Weight)
	}
	fn load_cost() -> Weight {
		T::DbWeight::get().reads(1 as Weight)
	}
	fn claim_value() -> Weight {
		(107_761_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	fn upload_code(c: u32, ) -> Weight {
		(88_657_000 as Weight)
			// Standard Error: 0
			.saturating_add((58_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn create_program(s: u32, ) -> Weight {
		(92_781_000 as Weight)
			// Standard Error: 0
			.saturating_add((3_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}
	fn upload_program(c: u32, s: u32, ) -> Weight {
		(56_972_000 as Weight)
			// Standard Error: 0
			.saturating_add((60_000 as Weight).saturating_mul(c as Weight))
			// Standard Error: 0
			.saturating_add((3_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().writes(11 as Weight))
	}
	fn send_message(p: u32, ) -> Weight {
		(85_774_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(10 as Weight))
			.saturating_add(T::DbWeight::get().writes(8 as Weight))
	}
	fn send_reply(p: u32, ) -> Weight {
		(137_573_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(14 as Weight))
			.saturating_add(T::DbWeight::get().writes(11 as Weight))
	}
	fn initial_allocation(_q: u32, ) -> Weight {
		(243_339_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(28 as Weight))
			.saturating_add(T::DbWeight::get().writes(26 as Weight))
	}
	fn alloc_in_handle(_q: u32, ) -> Weight {
		(361_323_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(26 as Weight))
			.saturating_add(T::DbWeight::get().writes(24 as Weight))
	}
	fn reinstrument(c: u32, ) -> Weight {
		(31_859_000 as Weight)
			// Standard Error: 0
			.saturating_add((61_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn alloc(r: u32, ) -> Weight {
		(129_918_000 as Weight)
			// Standard Error: 330_000
			.saturating_add((245_760_000 as Weight).saturating_mul(r as Weight))
	}
	fn gas(r: u32, ) -> Weight {
		(118_943_000 as Weight)
			// Standard Error: 141_000
			.saturating_add((61_679_000 as Weight).saturating_mul(r as Weight))
	}
	fn gr_gas_available(r: u32, ) -> Weight {
		(119_180_000 as Weight)
			// Standard Error: 174_000
			.saturating_add((60_163_000 as Weight).saturating_mul(r as Weight))
	}
	fn gr_msg_id(r: u32, ) -> Weight {
		(165_468_000 as Weight)
			// Standard Error: 256_000
			.saturating_add((73_176_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	fn gr_origin(r: u32, ) -> Weight {
		(159_107_000 as Weight)
			// Standard Error: 198_000
			.saturating_add((73_350_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	fn gr_program_id(r: u32, ) -> Weight {
		(160_573_000 as Weight)
			// Standard Error: 203_000
			.saturating_add((73_481_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	fn gr_source(r: u32, ) -> Weight {
		(157_808_000 as Weight)
			// Standard Error: 208_000
			.saturating_add((76_480_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	fn gr_value(r: u32, ) -> Weight {
		(168_022_000 as Weight)
			// Standard Error: 238_000
			.saturating_add((80_771_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	fn gr_value_available(r: u32, ) -> Weight {
		(162_591_000 as Weight)
			// Standard Error: 274_000
			.saturating_add((81_381_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	fn gr_size(r: u32, ) -> Weight {
		(116_813_000 as Weight)
			// Standard Error: 200_000
			.saturating_add((51_107_000 as Weight).saturating_mul(r as Weight))
	}
	fn gr_read(r: u32, ) -> Weight {
		(148_006_000 as Weight)
			// Standard Error: 163_000
			.saturating_add((71_441_000 as Weight).saturating_mul(r as Weight))
	}
	fn gr_read_per_kb(n: u32, ) -> Weight {
		(176_272_000 as Weight)
			// Standard Error: 162_000
			.saturating_add((33_383_000 as Weight).saturating_mul(n as Weight))
	}
	fn gr_block_height(r: u32, ) -> Weight {
		(117_042_000 as Weight)
			// Standard Error: 156_000
			.saturating_add((51_770_000 as Weight).saturating_mul(r as Weight))
	}
	fn gr_block_timestamp(r: u32, ) -> Weight {
		(117_851_000 as Weight)
			// Standard Error: 155_000
			.saturating_add((60_184_000 as Weight).saturating_mul(r as Weight))
	}
	fn gr_send_init(r: u32, ) -> Weight {
		(149_532_000 as Weight)
			// Standard Error: 254_000
			.saturating_add((88_915_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	fn gr_send_push(r: u32, ) -> Weight {
		(155_076_000 as Weight)
			// Standard Error: 295_000
			.saturating_add((176_430_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	fn gr_send_push_per_kb(n: u32, ) -> Weight {
		(333_042_000 as Weight)
			// Standard Error: 94_000
			.saturating_add((58_777_000 as Weight).saturating_mul(n as Weight))
	}
	fn gr_send_commit(r: u32, ) -> Weight {
		(164_137_000 as Weight)
			// Standard Error: 221_000
			.saturating_add((52_561_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	fn gr_send_commit_per_kb(n: u32, ) -> Weight {
		(169_583_000 as Weight)
			// Standard Error: 14_000
			.saturating_add((3_685_000 as Weight).saturating_mul(n as Weight))
	}
	fn gr_reply_commit(r: u32, ) -> Weight {
		(173_202_000 as Weight)
			// Standard Error: 298_000
			.saturating_add((100_781_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	fn gr_reply_commit_per_kb(n: u32, ) -> Weight {
		(287_723_000 as Weight)
			// Standard Error: 11_000
			.saturating_add((163_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
	}
	fn gr_reply_push(r: u32, ) -> Weight {
		(135_355_000 as Weight)
			// Standard Error: 184_000
			.saturating_add((78_342_000 as Weight).saturating_mul(r as Weight))
	}
	fn gr_reply_push_per_kb(n: u32, ) -> Weight {
		(230_699_000 as Weight)
			// Standard Error: 68_000
			.saturating_add((59_669_000 as Weight).saturating_mul(n as Weight))
	}
	fn gr_reply_to(r: u32, ) -> Weight {
		(149_297_000 as Weight)
			// Standard Error: 212_000
			.saturating_add((73_702_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
	}
	fn gr_debug(r: u32, ) -> Weight {
		(116_535_000 as Weight)
			// Standard Error: 188_000
			.saturating_add((72_010_000 as Weight).saturating_mul(r as Weight))
	}
	fn gr_exit_code(r: u32, ) -> Weight {
		(118_518_000 as Weight)
			// Standard Error: 78_000
			.saturating_add((51_738_000 as Weight).saturating_mul(r as Weight))
	}
	fn gr_exit(r: u32, ) -> Weight {
		(133_299_000 as Weight)
			// Standard Error: 477_000
			.saturating_add((53_109_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
	}
	fn gr_leave(r: u32, ) -> Weight {
		(111_822_000 as Weight)
			// Standard Error: 373_000
			.saturating_add((23_394_000 as Weight).saturating_mul(r as Weight))
	}
	fn gr_wait(r: u32, ) -> Weight {
		(112_271_000 as Weight)
			// Standard Error: 280_000
			.saturating_add((23_250_000 as Weight).saturating_mul(r as Weight))
	}
	fn gr_wake(r: u32, ) -> Weight {
		(170_137_000 as Weight)
			// Standard Error: 317_000
			.saturating_add((125_334_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
	}
	fn gr_create_program_wgas(r: u32, ) -> Weight {
		(135_364_000 as Weight)
			// Standard Error: 354_000
			.saturating_add((61_678_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
	}
	fn gr_create_program_wgas_per_kb(n: u32, ) -> Weight {
		(204_895_000 as Weight)
			// Standard Error: 20_000
			.saturating_add((3_732_000 as Weight).saturating_mul(n as Weight))
	}
	fn instr_i64const(r: u32, ) -> Weight {
		(5_087_000 as Weight)
			// Standard Error: 0
			.saturating_add((3_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64load(r: u32, ) -> Weight {
		(41_075_000 as Weight)
			// Standard Error: 14_000
			.saturating_add((346_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64store(r: u32, ) -> Weight {
		(59_949_000 as Weight)
			// Standard Error: 22_000
			.saturating_add((602_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_select(r: u32, ) -> Weight {
		(5_188_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((643_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_if(r: u32, ) -> Weight {
		(5_042_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((666_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_br(r: u32, ) -> Weight {
		(5_205_000 as Weight)
			// Standard Error: 0
			.saturating_add((397_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_br_if(r: u32, ) -> Weight {
		(6_296_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((503_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_br_table(r: u32, ) -> Weight {
		(5_936_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((1_600_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_br_table_per_entry(e: u32, ) -> Weight {
		(6_878_000 as Weight)
			// Standard Error: 0
			.saturating_add((18_000 as Weight).saturating_mul(e as Weight))
	}
	fn instr_call(r: u32, ) -> Weight {
		(5_662_000 as Weight)
			// Standard Error: 0
			.saturating_add((594_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_call_indirect(r: u32, ) -> Weight {
		(9_983_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((1_713_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_call_indirect_per_param(p: u32, ) -> Weight {
		(7_744_000 as Weight)
			// Standard Error: 0
			.saturating_add((158_000 as Weight).saturating_mul(p as Weight))
	}
	fn instr_local_get(r: u32, ) -> Weight {
		(5_302_000 as Weight)
			// Standard Error: 0
			.saturating_add((61_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_local_set(r: u32, ) -> Weight {
		(5_448_000 as Weight)
			// Standard Error: 0
			.saturating_add((129_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_local_tee(r: u32, ) -> Weight {
		(5_506_000 as Weight)
			// Standard Error: 0
			.saturating_add((126_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_global_get(r: u32, ) -> Weight {
		(20_521_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((145_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_global_set(r: u32, ) -> Weight {
		(22_401_000 as Weight)
			// Standard Error: 0
			.saturating_add((208_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_memory_current(r: u32, ) -> Weight {
		(10_027_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((1_249_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64clz(r: u32, ) -> Weight {
		(5_438_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((603_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64ctz(r: u32, ) -> Weight {
		(6_568_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((553_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64popcnt(r: u32, ) -> Weight {
		(5_240_000 as Weight)
			// Standard Error: 0
			.saturating_add((106_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64eqz(r: u32, ) -> Weight {
		(5_013_000 as Weight)
			// Standard Error: 0
			.saturating_add((240_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64extendsi32(r: u32, ) -> Weight {
		(5_157_000 as Weight)
			// Standard Error: 0
			.saturating_add((74_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64extendui32(r: u32, ) -> Weight {
		(5_111_000 as Weight)
			// Standard Error: 0
			.saturating_add((48_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i32wrapi64(r: u32, ) -> Weight {
		(5_115_000 as Weight)
			// Standard Error: 0
			.saturating_add((49_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64eq(r: u32, ) -> Weight {
		(5_205_000 as Weight)
			// Standard Error: 0
			.saturating_add((238_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64ne(r: u32, ) -> Weight {
		(5_230_000 as Weight)
			// Standard Error: 0
			.saturating_add((233_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64lts(r: u32, ) -> Weight {
		(5_189_000 as Weight)
			// Standard Error: 0
			.saturating_add((236_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64ltu(r: u32, ) -> Weight {
		(5_185_000 as Weight)
			// Standard Error: 0
			.saturating_add((237_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64gts(r: u32, ) -> Weight {
		(5_115_000 as Weight)
			// Standard Error: 0
			.saturating_add((237_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64gtu(r: u32, ) -> Weight {
		(5_144_000 as Weight)
			// Standard Error: 0
			.saturating_add((238_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64les(r: u32, ) -> Weight {
		(5_177_000 as Weight)
			// Standard Error: 0
			.saturating_add((236_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64leu(r: u32, ) -> Weight {
		(5_271_000 as Weight)
			// Standard Error: 0
			.saturating_add((234_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64ges(r: u32, ) -> Weight {
		(5_235_000 as Weight)
			// Standard Error: 0
			.saturating_add((236_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64geu(r: u32, ) -> Weight {
		(5_208_000 as Weight)
			// Standard Error: 0
			.saturating_add((234_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64add(r: u32, ) -> Weight {
		(5_198_000 as Weight)
			// Standard Error: 0
			.saturating_add((180_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64sub(r: u32, ) -> Weight {
		(5_157_000 as Weight)
			// Standard Error: 0
			.saturating_add((181_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64mul(r: u32, ) -> Weight {
		(5_168_000 as Weight)
			// Standard Error: 0
			.saturating_add((230_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64divs(r: u32, ) -> Weight {
		(6_467_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((932_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64divu(r: u32, ) -> Weight {
		(6_004_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((858_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64rems(r: u32, ) -> Weight {
		(3_088_000 as Weight)
			// Standard Error: 6_000
			.saturating_add((1_902_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64remu(r: u32, ) -> Weight {
		(6_873_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((856_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64and(r: u32, ) -> Weight {
		(5_165_000 as Weight)
			// Standard Error: 0
			.saturating_add((181_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64or(r: u32, ) -> Weight {
		(5_120_000 as Weight)
			// Standard Error: 0
			.saturating_add((182_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64xor(r: u32, ) -> Weight {
		(5_115_000 as Weight)
			// Standard Error: 0
			.saturating_add((182_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64shl(r: u32, ) -> Weight {
		(5_164_000 as Weight)
			// Standard Error: 0
			.saturating_add((162_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64shrs(r: u32, ) -> Weight {
		(5_162_000 as Weight)
			// Standard Error: 0
			.saturating_add((162_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64shru(r: u32, ) -> Weight {
		(5_136_000 as Weight)
			// Standard Error: 0
			.saturating_add((164_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64rotl(r: u32, ) -> Weight {
		(5_178_000 as Weight)
			// Standard Error: 0
			.saturating_add((161_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64rotr(r: u32, ) -> Weight {
		(5_206_000 as Weight)
			// Standard Error: 0
			.saturating_add((162_000 as Weight).saturating_mul(r as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn allocation_cost() -> Weight {
		// To be changed with the proper value.
		RocksDbWeight::get().writes(1 as Weight)
	}
	fn grow_cost() -> Weight {
		// To be changed with the proper value.
		RocksDbWeight::get().writes(1 as Weight)
	}
	fn initial_cost() -> Weight {
		RocksDbWeight::get().writes(1 as Weight)
	}
	fn load_cost() -> Weight {
		RocksDbWeight::get().reads(1 as Weight)
	}
	fn claim_value() -> Weight {
		(107_761_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(8 as Weight))
			.saturating_add(RocksDbWeight::get().writes(6 as Weight))
	}
	fn upload_code(c: u32, ) -> Weight {
		(88_657_000 as Weight)
			// Standard Error: 0
			.saturating_add((58_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	fn create_program(s: u32, ) -> Weight {
		(92_781_000 as Weight)
			// Standard Error: 0
			.saturating_add((3_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(10 as Weight))
			.saturating_add(RocksDbWeight::get().writes(8 as Weight))
	}
	fn upload_program(c: u32, s: u32, ) -> Weight {
		(56_972_000 as Weight)
			// Standard Error: 0
			.saturating_add((60_000 as Weight).saturating_mul(c as Weight))
			// Standard Error: 0
			.saturating_add((3_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(10 as Weight))
			.saturating_add(RocksDbWeight::get().writes(11 as Weight))
	}
	fn send_message(p: u32, ) -> Weight {
		(85_774_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(10 as Weight))
			.saturating_add(RocksDbWeight::get().writes(8 as Weight))
	}
	fn send_reply(p: u32, ) -> Weight {
		(137_573_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(14 as Weight))
			.saturating_add(RocksDbWeight::get().writes(11 as Weight))
	}
	fn initial_allocation(_q: u32, ) -> Weight {
		(243_339_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(28 as Weight))
			.saturating_add(RocksDbWeight::get().writes(26 as Weight))
	}
	fn alloc_in_handle(_q: u32, ) -> Weight {
		(361_323_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(26 as Weight))
			.saturating_add(RocksDbWeight::get().writes(24 as Weight))
	}
	fn reinstrument(c: u32, ) -> Weight {
		(31_859_000 as Weight)
			// Standard Error: 0
			.saturating_add((61_000 as Weight).saturating_mul(c as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn alloc(r: u32, ) -> Weight {
		(129_918_000 as Weight)
			// Standard Error: 330_000
			.saturating_add((245_760_000 as Weight).saturating_mul(r as Weight))
	}
	fn gas(r: u32, ) -> Weight {
		(118_943_000 as Weight)
			// Standard Error: 141_000
			.saturating_add((61_679_000 as Weight).saturating_mul(r as Weight))
	}
	fn gr_gas_available(r: u32, ) -> Weight {
		(119_180_000 as Weight)
			// Standard Error: 174_000
			.saturating_add((60_163_000 as Weight).saturating_mul(r as Weight))
	}
	fn gr_msg_id(r: u32, ) -> Weight {
		(165_468_000 as Weight)
			// Standard Error: 256_000
			.saturating_add((73_176_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
	}
	fn gr_origin(r: u32, ) -> Weight {
		(159_107_000 as Weight)
			// Standard Error: 198_000
			.saturating_add((73_350_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
	}
	fn gr_program_id(r: u32, ) -> Weight {
		(160_573_000 as Weight)
			// Standard Error: 203_000
			.saturating_add((73_481_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
	}
	fn gr_source(r: u32, ) -> Weight {
		(157_808_000 as Weight)
			// Standard Error: 208_000
			.saturating_add((76_480_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
	}
	fn gr_value(r: u32, ) -> Weight {
		(168_022_000 as Weight)
			// Standard Error: 238_000
			.saturating_add((80_771_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
	}
	fn gr_value_available(r: u32, ) -> Weight {
		(162_591_000 as Weight)
			// Standard Error: 274_000
			.saturating_add((81_381_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
	}
	fn gr_size(r: u32, ) -> Weight {
		(116_813_000 as Weight)
			// Standard Error: 200_000
			.saturating_add((51_107_000 as Weight).saturating_mul(r as Weight))
	}
	fn gr_read(r: u32, ) -> Weight {
		(148_006_000 as Weight)
			// Standard Error: 163_000
			.saturating_add((71_441_000 as Weight).saturating_mul(r as Weight))
	}
	fn gr_read_per_kb(n: u32, ) -> Weight {
		(176_272_000 as Weight)
			// Standard Error: 162_000
			.saturating_add((33_383_000 as Weight).saturating_mul(n as Weight))
	}
	fn gr_block_height(r: u32, ) -> Weight {
		(117_042_000 as Weight)
			// Standard Error: 156_000
			.saturating_add((51_770_000 as Weight).saturating_mul(r as Weight))
	}
	fn gr_block_timestamp(r: u32, ) -> Weight {
		(117_851_000 as Weight)
			// Standard Error: 155_000
			.saturating_add((60_184_000 as Weight).saturating_mul(r as Weight))
	}
	fn gr_send_init(r: u32, ) -> Weight {
		(149_532_000 as Weight)
			// Standard Error: 254_000
			.saturating_add((88_915_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
	}
	fn gr_send_push(r: u32, ) -> Weight {
		(155_076_000 as Weight)
			// Standard Error: 295_000
			.saturating_add((176_430_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
	}
	fn gr_send_push_per_kb(n: u32, ) -> Weight {
		(333_042_000 as Weight)
			// Standard Error: 94_000
			.saturating_add((58_777_000 as Weight).saturating_mul(n as Weight))
	}
	fn gr_send_commit(r: u32, ) -> Weight {
		(164_137_000 as Weight)
			// Standard Error: 221_000
			.saturating_add((52_561_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
	}
	fn gr_send_commit_per_kb(n: u32, ) -> Weight {
		(169_583_000 as Weight)
			// Standard Error: 14_000
			.saturating_add((3_685_000 as Weight).saturating_mul(n as Weight))
	}
	fn gr_reply_commit(r: u32, ) -> Weight {
		(173_202_000 as Weight)
			// Standard Error: 298_000
			.saturating_add((100_781_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
	}
	fn gr_reply_commit_per_kb(n: u32, ) -> Weight {
		(287_723_000 as Weight)
			// Standard Error: 11_000
			.saturating_add((163_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
	}
	fn gr_reply_push(r: u32, ) -> Weight {
		(135_355_000 as Weight)
			// Standard Error: 184_000
			.saturating_add((78_342_000 as Weight).saturating_mul(r as Weight))
	}
	fn gr_reply_push_per_kb(n: u32, ) -> Weight {
		(230_699_000 as Weight)
			// Standard Error: 68_000
			.saturating_add((59_669_000 as Weight).saturating_mul(n as Weight))
	}
	fn gr_reply_to(r: u32, ) -> Weight {
		(149_297_000 as Weight)
			// Standard Error: 212_000
			.saturating_add((73_702_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
	}
	fn gr_debug(r: u32, ) -> Weight {
		(116_535_000 as Weight)
			// Standard Error: 188_000
			.saturating_add((72_010_000 as Weight).saturating_mul(r as Weight))
	}
	fn gr_exit_code(r: u32, ) -> Weight {
		(118_518_000 as Weight)
			// Standard Error: 78_000
			.saturating_add((51_738_000 as Weight).saturating_mul(r as Weight))
	}
	fn gr_exit(r: u32, ) -> Weight {
		(133_299_000 as Weight)
			// Standard Error: 477_000
			.saturating_add((53_109_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
	}
	fn gr_leave(r: u32, ) -> Weight {
		(111_822_000 as Weight)
			// Standard Error: 373_000
			.saturating_add((23_394_000 as Weight).saturating_mul(r as Weight))
	}
	fn gr_wait(r: u32, ) -> Weight {
		(112_271_000 as Weight)
			// Standard Error: 280_000
			.saturating_add((23_250_000 as Weight).saturating_mul(r as Weight))
	}
	fn gr_wake(r: u32, ) -> Weight {
		(170_137_000 as Weight)
			// Standard Error: 317_000
			.saturating_add((125_334_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
	}
	fn gr_create_program_wgas(r: u32, ) -> Weight {
		(135_364_000 as Weight)
			// Standard Error: 354_000
			.saturating_add((61_678_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
	}
	fn gr_create_program_wgas_per_kb(n: u32, ) -> Weight {
		(204_895_000 as Weight)
			// Standard Error: 20_000
			.saturating_add((3_732_000 as Weight).saturating_mul(n as Weight))
	}
	fn instr_i64const(r: u32, ) -> Weight {
		(5_087_000 as Weight)
			// Standard Error: 0
			.saturating_add((3_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64load(r: u32, ) -> Weight {
		(41_075_000 as Weight)
			// Standard Error: 14_000
			.saturating_add((346_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64store(r: u32, ) -> Weight {
		(59_949_000 as Weight)
			// Standard Error: 22_000
			.saturating_add((602_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_select(r: u32, ) -> Weight {
		(5_188_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((643_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_if(r: u32, ) -> Weight {
		(5_042_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((666_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_br(r: u32, ) -> Weight {
		(5_205_000 as Weight)
			// Standard Error: 0
			.saturating_add((397_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_br_if(r: u32, ) -> Weight {
		(6_296_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((503_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_br_table(r: u32, ) -> Weight {
		(5_936_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((1_600_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_br_table_per_entry(e: u32, ) -> Weight {
		(6_878_000 as Weight)
			// Standard Error: 0
			.saturating_add((18_000 as Weight).saturating_mul(e as Weight))
	}
	fn instr_call(r: u32, ) -> Weight {
		(5_662_000 as Weight)
			// Standard Error: 0
			.saturating_add((594_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_call_indirect(r: u32, ) -> Weight {
		(9_983_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((1_713_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_call_indirect_per_param(p: u32, ) -> Weight {
		(7_744_000 as Weight)
			// Standard Error: 0
			.saturating_add((158_000 as Weight).saturating_mul(p as Weight))
	}
	fn instr_local_get(r: u32, ) -> Weight {
		(5_302_000 as Weight)
			// Standard Error: 0
			.saturating_add((61_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_local_set(r: u32, ) -> Weight {
		(5_448_000 as Weight)
			// Standard Error: 0
			.saturating_add((129_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_local_tee(r: u32, ) -> Weight {
		(5_506_000 as Weight)
			// Standard Error: 0
			.saturating_add((126_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_global_get(r: u32, ) -> Weight {
		(20_521_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((145_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_global_set(r: u32, ) -> Weight {
		(22_401_000 as Weight)
			// Standard Error: 0
			.saturating_add((208_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_memory_current(r: u32, ) -> Weight {
		(10_027_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((1_249_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64clz(r: u32, ) -> Weight {
		(5_438_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((603_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64ctz(r: u32, ) -> Weight {
		(6_568_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((553_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64popcnt(r: u32, ) -> Weight {
		(5_240_000 as Weight)
			// Standard Error: 0
			.saturating_add((106_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64eqz(r: u32, ) -> Weight {
		(5_013_000 as Weight)
			// Standard Error: 0
			.saturating_add((240_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64extendsi32(r: u32, ) -> Weight {
		(5_157_000 as Weight)
			// Standard Error: 0
			.saturating_add((74_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64extendui32(r: u32, ) -> Weight {
		(5_111_000 as Weight)
			// Standard Error: 0
			.saturating_add((48_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i32wrapi64(r: u32, ) -> Weight {
		(5_115_000 as Weight)
			// Standard Error: 0
			.saturating_add((49_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64eq(r: u32, ) -> Weight {
		(5_205_000 as Weight)
			// Standard Error: 0
			.saturating_add((238_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64ne(r: u32, ) -> Weight {
		(5_230_000 as Weight)
			// Standard Error: 0
			.saturating_add((233_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64lts(r: u32, ) -> Weight {
		(5_189_000 as Weight)
			// Standard Error: 0
			.saturating_add((236_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64ltu(r: u32, ) -> Weight {
		(5_185_000 as Weight)
			// Standard Error: 0
			.saturating_add((237_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64gts(r: u32, ) -> Weight {
		(5_115_000 as Weight)
			// Standard Error: 0
			.saturating_add((237_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64gtu(r: u32, ) -> Weight {
		(5_144_000 as Weight)
			// Standard Error: 0
			.saturating_add((238_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64les(r: u32, ) -> Weight {
		(5_177_000 as Weight)
			// Standard Error: 0
			.saturating_add((236_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64leu(r: u32, ) -> Weight {
		(5_271_000 as Weight)
			// Standard Error: 0
			.saturating_add((234_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64ges(r: u32, ) -> Weight {
		(5_235_000 as Weight)
			// Standard Error: 0
			.saturating_add((236_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64geu(r: u32, ) -> Weight {
		(5_208_000 as Weight)
			// Standard Error: 0
			.saturating_add((234_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64add(r: u32, ) -> Weight {
		(5_198_000 as Weight)
			// Standard Error: 0
			.saturating_add((180_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64sub(r: u32, ) -> Weight {
		(5_157_000 as Weight)
			// Standard Error: 0
			.saturating_add((181_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64mul(r: u32, ) -> Weight {
		(5_168_000 as Weight)
			// Standard Error: 0
			.saturating_add((230_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64divs(r: u32, ) -> Weight {
		(6_467_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((932_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64divu(r: u32, ) -> Weight {
		(6_004_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((858_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64rems(r: u32, ) -> Weight {
		(3_088_000 as Weight)
			// Standard Error: 6_000
			.saturating_add((1_902_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64remu(r: u32, ) -> Weight {
		(6_873_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((856_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64and(r: u32, ) -> Weight {
		(5_165_000 as Weight)
			// Standard Error: 0
			.saturating_add((181_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64or(r: u32, ) -> Weight {
		(5_120_000 as Weight)
			// Standard Error: 0
			.saturating_add((182_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64xor(r: u32, ) -> Weight {
		(5_115_000 as Weight)
			// Standard Error: 0
			.saturating_add((182_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64shl(r: u32, ) -> Weight {
		(5_164_000 as Weight)
			// Standard Error: 0
			.saturating_add((162_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64shrs(r: u32, ) -> Weight {
		(5_162_000 as Weight)
			// Standard Error: 0
			.saturating_add((162_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64shru(r: u32, ) -> Weight {
		(5_136_000 as Weight)
			// Standard Error: 0
			.saturating_add((164_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64rotl(r: u32, ) -> Weight {
		(5_178_000 as Weight)
			// Standard Error: 0
			.saturating_add((161_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64rotr(r: u32, ) -> Weight {
		(5_206_000 as Weight)
			// Standard Error: 0
			.saturating_add((162_000 as Weight).saturating_mul(r as Weight))
	}
}
