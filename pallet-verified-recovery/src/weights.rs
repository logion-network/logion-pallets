#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_verified_recovery.
pub trait WeightInfo {
    fn create_recovery() -> Weight;
}

/// Default weights
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `LogionLoc::AccountLocsMap` (r:1 w:0)
	/// Proof: `LogionLoc::AccountLocsMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::LocMap` (r:2 w:0)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Recovery::Recoverable` (r:1 w:1)
	/// Proof: `Recovery::Recoverable` (`max_values`: None, `max_size`: Some(159), added: 2634, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn create_recovery() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `641`
		//  Estimated: `6581`
		// Minimum execution time: 49_324_000 picoseconds.
		Weight::from_parts(51_145_000, 0)
			.saturating_add(Weight::from_parts(0, 6581))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}

}
