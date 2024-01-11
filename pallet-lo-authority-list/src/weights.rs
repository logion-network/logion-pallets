#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_logion_loc.
pub trait WeightInfo {
    fn add_legal_officer() -> Weight;
    fn remove_legal_officer() -> Weight;
    fn update_legal_officer() -> Weight;
}

/// Weights for pallet_logion_loc using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `LoAuthorityList::LegalOfficerSet` (r:54 w:1)
	/// Proof: `LoAuthorityList::LegalOfficerSet` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LoAuthorityList::LegalOfficerNodes` (r:0 w:1)
	/// Proof: `LoAuthorityList::LegalOfficerNodes` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn add_legal_officer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4910`
		//  Estimated: `139550`
		// Minimum execution time: 243_589_000 picoseconds.
		Weight::from_parts(258_521_000, 0)
			.saturating_add(Weight::from_parts(0, 139550))
			.saturating_add(T::DbWeight::get().reads(54))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `LoAuthorityList::LegalOfficerSet` (r:53 w:1)
	/// Proof: `LoAuthorityList::LegalOfficerSet` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LoAuthorityList::LegalOfficerNodes` (r:0 w:1)
	/// Proof: `LoAuthorityList::LegalOfficerNodes` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn remove_legal_officer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4910`
		//  Estimated: `137075`
		// Minimum execution time: 353_828_000 picoseconds.
		Weight::from_parts(364_906_000, 0)
			.saturating_add(Weight::from_parts(0, 137075))
			.saturating_add(T::DbWeight::get().reads(53))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `LoAuthorityList::LegalOfficerSet` (r:53 w:1)
	/// Proof: `LoAuthorityList::LegalOfficerSet` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LoAuthorityList::LegalOfficerNodes` (r:0 w:1)
	/// Proof: `LoAuthorityList::LegalOfficerNodes` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn update_legal_officer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4910`
		//  Estimated: `137075`
		// Minimum execution time: 244_677_000 picoseconds.
		Weight::from_parts(253_044_000, 0)
			.saturating_add(Weight::from_parts(0, 137075))
			.saturating_add(T::DbWeight::get().reads(53))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
