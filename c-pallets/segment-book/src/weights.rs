// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
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

//! Autogenerated weights for pallet_segment_book
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-08-02, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("cess-staking-testnet"), DB CACHE: 1024

// Executed Command:
// ./target/release/cess-node
// benchmark
// --chain
// cess-staking-testnet
// --execution=wasm
// --wasm-execution=compiled
// --pallet
// pallet_segment_book
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --template=./.maintain/frame-weight-template.hbs
// --output=./c-pallets/segment-book/src/weights

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_segment_book.
pub trait WeightInfo {
	fn submit_challenge_prove(v: u32, ) -> Weight;
	fn verify_proof(v: u32, ) -> Weight;
}

/// Weights for pallet_segment_book using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: System Digest (r:1 w:0)
	// Storage: FileMap SchedulerMap (r:1 w:0)
	// Storage: SegmentBook ChallengeMap (r:1 w:1)
	// Storage: SegmentBook UnVerifyProof (r:1 w:1)
	fn submit_challenge_prove(v: u32, ) -> Weight {
		(116_351_000 as Weight)
			// Standard Error: 82_000
			.saturating_add((6_532_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: FileMap SchedulerMap (r:1 w:0)
	// Storage: SegmentBook UnVerifyProof (r:1 w:1)
	fn verify_proof(v: u32, ) -> Weight {
		(144_028_000 as Weight)
			// Standard Error: 120_000
			.saturating_add((15_284_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: System Digest (r:1 w:0)
	// Storage: FileMap SchedulerMap (r:1 w:0)
	// Storage: SegmentBook ChallengeMap (r:1 w:1)
	// Storage: SegmentBook UnVerifyProof (r:1 w:1)
	fn submit_challenge_prove(v: u32, ) -> Weight {
		(116_351_000 as Weight)
			// Standard Error: 82_000
			.saturating_add((6_532_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: FileMap SchedulerMap (r:1 w:0)
	// Storage: SegmentBook UnVerifyProof (r:1 w:1)
	fn verify_proof(v: u32, ) -> Weight {
		(144_028_000 as Weight)
			// Standard Error: 120_000
			.saturating_add((15_284_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
}
