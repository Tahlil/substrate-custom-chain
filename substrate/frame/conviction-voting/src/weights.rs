// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_conviction_voting
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-16, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-e8ezs4ez-project-145-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/substrate
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_conviction_voting
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/conviction-voting/src/weights.rs
// --header=./HEADER-APACHE2
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_conviction_voting.
pub trait WeightInfo {
	fn vote_new() -> Weight;
	fn vote_existing() -> Weight;
	fn remove_vote() -> Weight;
	fn remove_other_vote() -> Weight;
	fn delegate(r: u32, ) -> Weight;
	fn undelegate(r: u32, ) -> Weight;
	fn unlock() -> Weight;
}

/// Weights for pallet_conviction_voting using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(366), added: 2841, mode: MaxEncodedLen)
	/// Storage: ConvictionVoting VotingFor (r:1 w:1)
	/// Proof: ConvictionVoting VotingFor (max_values: None, max_size: Some(27241), added: 29716, mode: MaxEncodedLen)
	/// Storage: ConvictionVoting ClassLocksFor (r:1 w:1)
	/// Proof: ConvictionVoting ClassLocksFor (max_values: None, max_size: Some(59), added: 2534, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(107022), added: 109497, mode: MaxEncodedLen)
	fn vote_new() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `13074`
		//  Estimated: `219984`
		// Minimum execution time: 112_936_000 picoseconds.
		Weight::from_parts(116_972_000, 219984)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(366), added: 2841, mode: MaxEncodedLen)
	/// Storage: ConvictionVoting VotingFor (r:1 w:1)
	/// Proof: ConvictionVoting VotingFor (max_values: None, max_size: Some(27241), added: 29716, mode: MaxEncodedLen)
	/// Storage: ConvictionVoting ClassLocksFor (r:1 w:1)
	/// Proof: ConvictionVoting ClassLocksFor (max_values: None, max_size: Some(59), added: 2534, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(107022), added: 109497, mode: MaxEncodedLen)
	fn vote_existing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `20216`
		//  Estimated: `219984`
		// Minimum execution time: 291_971_000 picoseconds.
		Weight::from_parts(301_738_000, 219984)
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(6_u64))
	}
	/// Storage: ConvictionVoting VotingFor (r:1 w:1)
	/// Proof: ConvictionVoting VotingFor (max_values: None, max_size: Some(27241), added: 29716, mode: MaxEncodedLen)
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(366), added: 2841, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(107022), added: 109497, mode: MaxEncodedLen)
	fn remove_vote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `19968`
		//  Estimated: `219984`
		// Minimum execution time: 262_582_000 picoseconds.
		Weight::from_parts(270_955_000, 219984)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: ConvictionVoting VotingFor (r:1 w:1)
	/// Proof: ConvictionVoting VotingFor (max_values: None, max_size: Some(27241), added: 29716, mode: MaxEncodedLen)
	/// Storage: Referenda ReferendumInfoFor (r:1 w:0)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(366), added: 2841, mode: MaxEncodedLen)
	fn remove_other_vote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `12675`
		//  Estimated: `30706`
		// Minimum execution time: 52_909_000 picoseconds.
		Weight::from_parts(56_365_000, 30706)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: ConvictionVoting VotingFor (r:2 w:2)
	/// Proof: ConvictionVoting VotingFor (max_values: None, max_size: Some(27241), added: 29716, mode: MaxEncodedLen)
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(366), added: 2841, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(107022), added: 109497, mode: MaxEncodedLen)
	/// Storage: ConvictionVoting ClassLocksFor (r:1 w:1)
	/// Proof: ConvictionVoting ClassLocksFor (max_values: None, max_size: Some(59), added: 2534, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 1]`.
	fn delegate(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `240 + r * (1627 ±0)`
		//  Estimated: `109992 + r * (109992 ±0)`
		// Minimum execution time: 54_640_000 picoseconds.
		Weight::from_parts(57_185_281, 109992)
			// Standard Error: 193_362
			.saturating_add(Weight::from_parts(44_897_418, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(4_u64))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_parts(0, 109992).saturating_mul(r.into()))
	}
	/// Storage: ConvictionVoting VotingFor (r:2 w:2)
	/// Proof: ConvictionVoting VotingFor (max_values: None, max_size: Some(27241), added: 29716, mode: MaxEncodedLen)
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(366), added: 2841, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(107022), added: 109497, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 1]`.
	fn undelegate(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `406 + r * (1376 ±0)`
		//  Estimated: `109992 + r * (109992 ±0)`
		// Minimum execution time: 26_514_000 picoseconds.
		Weight::from_parts(28_083_732, 109992)
			// Standard Error: 104_905
			.saturating_add(Weight::from_parts(40_722_467, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(2_u64))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_parts(0, 109992).saturating_mul(r.into()))
	}
	/// Storage: ConvictionVoting VotingFor (r:1 w:1)
	/// Proof: ConvictionVoting VotingFor (max_values: None, max_size: Some(27241), added: 29716, mode: MaxEncodedLen)
	/// Storage: ConvictionVoting ClassLocksFor (r:1 w:1)
	/// Proof: ConvictionVoting ClassLocksFor (max_values: None, max_size: Some(59), added: 2534, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	fn unlock() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `11734`
		//  Estimated: `30706`
		// Minimum execution time: 71_140_000 picoseconds.
		Weight::from_parts(77_388_000, 30706)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(366), added: 2841, mode: MaxEncodedLen)
	/// Storage: ConvictionVoting VotingFor (r:1 w:1)
	/// Proof: ConvictionVoting VotingFor (max_values: None, max_size: Some(27241), added: 29716, mode: MaxEncodedLen)
	/// Storage: ConvictionVoting ClassLocksFor (r:1 w:1)
	/// Proof: ConvictionVoting ClassLocksFor (max_values: None, max_size: Some(59), added: 2534, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(107022), added: 109497, mode: MaxEncodedLen)
	fn vote_new() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `13074`
		//  Estimated: `219984`
		// Minimum execution time: 112_936_000 picoseconds.
		Weight::from_parts(116_972_000, 219984)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(366), added: 2841, mode: MaxEncodedLen)
	/// Storage: ConvictionVoting VotingFor (r:1 w:1)
	/// Proof: ConvictionVoting VotingFor (max_values: None, max_size: Some(27241), added: 29716, mode: MaxEncodedLen)
	/// Storage: ConvictionVoting ClassLocksFor (r:1 w:1)
	/// Proof: ConvictionVoting ClassLocksFor (max_values: None, max_size: Some(59), added: 2534, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(107022), added: 109497, mode: MaxEncodedLen)
	fn vote_existing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `20216`
		//  Estimated: `219984`
		// Minimum execution time: 291_971_000 picoseconds.
		Weight::from_parts(301_738_000, 219984)
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().writes(6_u64))
	}
	/// Storage: ConvictionVoting VotingFor (r:1 w:1)
	/// Proof: ConvictionVoting VotingFor (max_values: None, max_size: Some(27241), added: 29716, mode: MaxEncodedLen)
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(366), added: 2841, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(107022), added: 109497, mode: MaxEncodedLen)
	fn remove_vote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `19968`
		//  Estimated: `219984`
		// Minimum execution time: 262_582_000 picoseconds.
		Weight::from_parts(270_955_000, 219984)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: ConvictionVoting VotingFor (r:1 w:1)
	/// Proof: ConvictionVoting VotingFor (max_values: None, max_size: Some(27241), added: 29716, mode: MaxEncodedLen)
	/// Storage: Referenda ReferendumInfoFor (r:1 w:0)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(366), added: 2841, mode: MaxEncodedLen)
	fn remove_other_vote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `12675`
		//  Estimated: `30706`
		// Minimum execution time: 52_909_000 picoseconds.
		Weight::from_parts(56_365_000, 30706)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: ConvictionVoting VotingFor (r:2 w:2)
	/// Proof: ConvictionVoting VotingFor (max_values: None, max_size: Some(27241), added: 29716, mode: MaxEncodedLen)
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(366), added: 2841, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(107022), added: 109497, mode: MaxEncodedLen)
	/// Storage: ConvictionVoting ClassLocksFor (r:1 w:1)
	/// Proof: ConvictionVoting ClassLocksFor (max_values: None, max_size: Some(59), added: 2534, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 1]`.
	fn delegate(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `240 + r * (1627 ±0)`
		//  Estimated: `109992 + r * (109992 ±0)`
		// Minimum execution time: 54_640_000 picoseconds.
		Weight::from_parts(57_185_281, 109992)
			// Standard Error: 193_362
			.saturating_add(Weight::from_parts(44_897_418, 0).saturating_mul(r.into()))
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().reads((3_u64).saturating_mul(r.into())))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
			.saturating_add(RocksDbWeight::get().writes((3_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_parts(0, 109992).saturating_mul(r.into()))
	}
	/// Storage: ConvictionVoting VotingFor (r:2 w:2)
	/// Proof: ConvictionVoting VotingFor (max_values: None, max_size: Some(27241), added: 29716, mode: MaxEncodedLen)
	/// Storage: Referenda ReferendumInfoFor (r:1 w:1)
	/// Proof: Referenda ReferendumInfoFor (max_values: None, max_size: Some(366), added: 2841, mode: MaxEncodedLen)
	/// Storage: Scheduler Agenda (r:2 w:2)
	/// Proof: Scheduler Agenda (max_values: None, max_size: Some(107022), added: 109497, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 1]`.
	fn undelegate(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `406 + r * (1376 ±0)`
		//  Estimated: `109992 + r * (109992 ±0)`
		// Minimum execution time: 26_514_000 picoseconds.
		Weight::from_parts(28_083_732, 109992)
			// Standard Error: 104_905
			.saturating_add(Weight::from_parts(40_722_467, 0).saturating_mul(r.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().reads((3_u64).saturating_mul(r.into())))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
			.saturating_add(RocksDbWeight::get().writes((3_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_parts(0, 109992).saturating_mul(r.into()))
	}
	/// Storage: ConvictionVoting VotingFor (r:1 w:1)
	/// Proof: ConvictionVoting VotingFor (max_values: None, max_size: Some(27241), added: 29716, mode: MaxEncodedLen)
	/// Storage: ConvictionVoting ClassLocksFor (r:1 w:1)
	/// Proof: ConvictionVoting ClassLocksFor (max_values: None, max_size: Some(59), added: 2534, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	fn unlock() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `11734`
		//  Estimated: `30706`
		// Minimum execution time: 71_140_000 picoseconds.
		Weight::from_parts(77_388_000, 30706)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
}
