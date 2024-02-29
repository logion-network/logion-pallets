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
	fn import_host_legal_officer() -> Weight;
	fn import_guest_legal_officer() -> Weight;
}

/// Weights for pallet_logion_loc using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `LoAuthorityList::LegalOfficerSet` (r:101 w:1)
	/// Proof: `LoAuthorityList::LegalOfficerSet` (`max_values`: None, `max_size`: Some(2185), added: 4660, mode: `MaxEncodedLen`)
	/// Storage: `LoAuthorityList::LegalOfficerNodes` (r:0 w:1)
	/// Proof: `LoAuthorityList::LegalOfficerNodes` (`max_values`: Some(1), `max_size`: Some(13002), added: 13497, mode: `MaxEncodedLen`)
	fn add_legal_officer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `9309`
		//  Estimated: `471650`
		// Minimum execution time: 418_485_000 picoseconds.
		Weight::from_parts(426_943_000, 0)
			.saturating_add(Weight::from_parts(0, 471650))
			.saturating_add(T::DbWeight::get().reads(101))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `LoAuthorityList::LegalOfficerSet` (r:101 w:1)
	/// Proof: `LoAuthorityList::LegalOfficerSet` (`max_values`: None, `max_size`: Some(2185), added: 4660, mode: `MaxEncodedLen`)
	/// Storage: `LoAuthorityList::LegalOfficerNodes` (r:0 w:1)
	/// Proof: `LoAuthorityList::LegalOfficerNodes` (`max_values`: Some(1), `max_size`: Some(13002), added: 13497, mode: `MaxEncodedLen`)
	fn remove_legal_officer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `9400`
		//  Estimated: `471650`
		// Minimum execution time: 598_565_000 picoseconds.
		Weight::from_parts(621_236_000, 0)
			.saturating_add(Weight::from_parts(0, 471650))
			.saturating_add(T::DbWeight::get().reads(101))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `LoAuthorityList::LegalOfficerSet` (r:101 w:1)
	/// Proof: `LoAuthorityList::LegalOfficerSet` (`max_values`: None, `max_size`: Some(2185), added: 4660, mode: `MaxEncodedLen`)
	/// Storage: `LoAuthorityList::LegalOfficerNodes` (r:0 w:1)
	/// Proof: `LoAuthorityList::LegalOfficerNodes` (`max_values`: Some(1), `max_size`: Some(13002), added: 13497, mode: `MaxEncodedLen`)
	fn update_legal_officer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `9400`
		//  Estimated: `471650`
		// Minimum execution time: 410_435_000 picoseconds.
		Weight::from_parts(427_006_000, 0)
			.saturating_add(Weight::from_parts(0, 471650))
			.saturating_add(T::DbWeight::get().reads(101))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `LoAuthorityList::LegalOfficerSet` (r:101 w:1)
	/// Proof: `LoAuthorityList::LegalOfficerSet` (`max_values`: None, `max_size`: Some(2185), added: 4660, mode: `MaxEncodedLen`)
	/// Storage: `LoAuthorityList::LegalOfficerNodes` (r:0 w:1)
	/// Proof: `LoAuthorityList::LegalOfficerNodes` (`max_values`: Some(1), `max_size`: Some(13002), added: 13497, mode: `MaxEncodedLen`)
	fn import_host_legal_officer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `9309`
		//  Estimated: `471650`
		// Minimum execution time: 406_380_000 picoseconds.
		Weight::from_parts(432_396_000, 0)
			.saturating_add(Weight::from_parts(0, 471650))
			.saturating_add(T::DbWeight::get().reads(101))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `LoAuthorityList::LegalOfficerSet` (r:2 w:1)
	/// Proof: `LoAuthorityList::LegalOfficerSet` (`max_values`: None, `max_size`: Some(2185), added: 4660, mode: `MaxEncodedLen`)
	fn import_guest_legal_officer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `878`
		//  Estimated: `10310`
		// Minimum execution time: 23_398_000 picoseconds.
		Weight::from_parts(27_347_000, 0)
			.saturating_add(Weight::from_parts(0, 10310))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
