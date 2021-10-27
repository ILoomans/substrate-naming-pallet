// This file is part of Substrate.

// Copyright (C) 2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_identity
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-06-19, STEPS: `[50, ]`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// target/release/substrate
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_identity
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/identity/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_identity.
pub trait WeightInfo {
	fn add_registrar(r: u32, ) -> Weight;
	fn set_identity(r: u32, x: u32, ) -> Weight;
	fn set_subs_new(s: u32, ) -> Weight;
	fn set_subs_old(p: u32, ) -> Weight;
	fn clear_identity(r: u32, s: u32, x: u32, ) -> Weight;
	fn request_judgement(r: u32, x: u32, ) -> Weight;
	fn cancel_request(r: u32, x: u32, ) -> Weight;
	fn set_fee(r: u32, ) -> Weight;
	fn set_account_id(r: u32, ) -> Weight;
	fn set_fields(r: u32, ) -> Weight;
	fn provide_judgement(r: u32, x: u32, ) -> Weight;
	fn kill_identity(r: u32, s: u32, x: u32, ) -> Weight;
	fn add_sub(s: u32, ) -> Weight;
	fn rename_sub(s: u32, ) -> Weight;
	fn remove_sub(s: u32, ) -> Weight;
	fn quit_sub(s: u32, ) -> Weight;
}

/// Weights for pallet_identity using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn add_registrar(r: u32, ) -> Weight {
		(21_825_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((288_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_identity(r: u32, x: u32, ) -> Weight {
		(53_354_000 as Weight)
			// Standard Error: 15_000
			.saturating_add((274_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 2_000
			.saturating_add((939_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_subs_new(s: u32, ) -> Weight {
		(42_017_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((6_457_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(s as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	fn set_subs_old(p: u32, ) -> Weight {
		(41_605_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_157_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(p as Weight)))
	}
	fn clear_identity(r: u32, s: u32, x: u32, ) -> Weight {
		(51_811_000 as Weight)
			// Standard Error: 5_000
			.saturating_add((202_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 0
			.saturating_add((2_157_000 as Weight).saturating_mul(s as Weight))
			// Standard Error: 0
			.saturating_add((618_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	fn request_judgement(r: u32, x: u32, ) -> Weight {
		(54_657_000 as Weight)
			// Standard Error: 5_000
			.saturating_add((381_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 0
			.saturating_add((1_153_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn cancel_request(r: u32, x: u32, ) -> Weight {
		(50_895_000 as Weight)
			// Standard Error: 6_000
			.saturating_add((267_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 0
			.saturating_add((1_141_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_fee(r: u32, ) -> Weight {
		(8_036_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((281_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_account_id(r: u32, ) -> Weight {
		(9_001_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((288_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn set_fields(r: u32, ) -> Weight {
		(8_039_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((286_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn provide_judgement(r: u32, x: u32, ) -> Weight {
		(35_746_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((346_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 0
			.saturating_add((1_164_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn kill_identity(r: u32, s: u32, x: u32, ) -> Weight {
		(65_304_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((149_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 0
			.saturating_add((2_118_000 as Weight).saturating_mul(s as Weight))
			// Standard Error: 0
			.saturating_add((6_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	fn add_sub(s: u32, ) -> Weight {
		(55_491_000 as Weight)
			// Standard Error: 0
			.saturating_add((220_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn rename_sub(s: u32, ) -> Weight {
		(17_564_000 as Weight)
			// Standard Error: 0
			.saturating_add((84_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn remove_sub(s: u32, ) -> Weight {
		(56_535_000 as Weight)
			// Standard Error: 0
			.saturating_add((209_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn quit_sub(s: u32, ) -> Weight {
		(35_369_000 as Weight)
			// Standard Error: 0
			.saturating_add((200_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn add_registrar(r: u32, ) -> Weight {
		(21_825_000 as Weight)
			// Standard Error: 3_000
			.saturating_add((288_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn set_identity(r: u32, x: u32, ) -> Weight {
		(53_354_000 as Weight)
			// Standard Error: 15_000
			.saturating_add((274_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 2_000
			.saturating_add((939_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn set_subs_new(s: u32, ) -> Weight {
		(42_017_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((6_457_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().reads((1 as Weight).saturating_mul(s as Weight)))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	fn set_subs_old(p: u32, ) -> Weight {
		(41_605_000 as Weight)
			// Standard Error: 0
			.saturating_add((2_157_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(p as Weight)))
	}
	fn clear_identity(r: u32, s: u32, x: u32, ) -> Weight {
		(51_811_000 as Weight)
			// Standard Error: 5_000
			.saturating_add((202_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 0
			.saturating_add((2_157_000 as Weight).saturating_mul(s as Weight))
			// Standard Error: 0
			.saturating_add((618_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	fn request_judgement(r: u32, x: u32, ) -> Weight {
		(54_657_000 as Weight)
			// Standard Error: 5_000
			.saturating_add((381_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 0
			.saturating_add((1_153_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn cancel_request(r: u32, x: u32, ) -> Weight {
		(50_895_000 as Weight)
			// Standard Error: 6_000
			.saturating_add((267_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 0
			.saturating_add((1_141_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn set_fee(r: u32, ) -> Weight {
		(8_036_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((281_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn set_account_id(r: u32, ) -> Weight {
		(9_001_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((288_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn set_fields(r: u32, ) -> Weight {
		(8_039_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((286_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn provide_judgement(r: u32, x: u32, ) -> Weight {
		(35_746_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((346_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 0
			.saturating_add((1_164_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn kill_identity(r: u32, s: u32, x: u32, ) -> Weight {
		(65_304_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((149_000 as Weight).saturating_mul(r as Weight))
			// Standard Error: 0
			.saturating_add((2_118_000 as Weight).saturating_mul(s as Weight))
			// Standard Error: 0
			.saturating_add((6_000 as Weight).saturating_mul(x as Weight))
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes((1 as Weight).saturating_mul(s as Weight)))
	}
	fn add_sub(s: u32, ) -> Weight {
		(55_491_000 as Weight)
			// Standard Error: 0
			.saturating_add((220_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	fn rename_sub(s: u32, ) -> Weight {
		(17_564_000 as Weight)
			// Standard Error: 0
			.saturating_add((84_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	fn remove_sub(s: u32, ) -> Weight {
		(56_535_000 as Weight)
			// Standard Error: 0
			.saturating_add((209_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	fn quit_sub(s: u32, ) -> Weight {
		(35_369_000 as Weight)
			// Standard Error: 0
			.saturating_add((200_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
}