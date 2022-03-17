
//! Autogenerated weights for `pallet_vesting`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-03-01, STEPS: `20`, REPEAT: 64, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/oak-collator
// benchmark
// --chain
// dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet_vesting
// --extrinsic
// *
// --repeat
// 64
// --steps
// 20
// --raw
// --output
// ./pallets/vesting/src/raw-weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_vest.
pub trait WeightInfo {
	fn vest(v: u32, ) -> Weight;
}

/// Weight functions for `pallet_vesting`.
pub struct VestingWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for VestingWeight<T> {
	// Storage: Timestamp Now (r:1 w:0)
	// Storage: Vesting VestingSchedule (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn vest(v: u32, ) -> Weight {
		(9_540_000 as Weight)
			// Standard Error: 13_000
			.saturating_add((17_041_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(v as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(v as Weight)))
	}
}