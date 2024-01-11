#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_logion_loc.
pub trait WeightInfo {
    fn create_vote_for_all_legal_officers() -> Weight;
    fn vote() -> Weight;
}

/// Weights for pallet_logion_loc using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `LoAuthorityList::LegalOfficerSet` (r:3 w:0)
	/// Proof: `LoAuthorityList::LegalOfficerSet` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::LocMap` (r:1 w:0)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Vote::LastVoteId` (r:1 w:1)
	/// Proof: `Vote::LastVoteId` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Vote::Votes` (r:0 w:1)
	/// Proof: `Vote::Votes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn create_vote_for_all_legal_officers() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `537`
		//  Estimated: `8952`
		// Minimum execution time: 28_024_000 picoseconds.
		Weight::from_parts(28_642_000, 0)
			.saturating_add(Weight::from_parts(0, 8952))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Vote::Votes` (r:1 w:1)
	/// Proof: `Vote::Votes` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn vote() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `166`
		//  Estimated: `3631`
		// Minimum execution time: 17_829_000 picoseconds.
		Weight::from_parts(18_854_000, 0)
			.saturating_add(Weight::from_parts(0, 3631))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
