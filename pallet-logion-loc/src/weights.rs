#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_logion_loc.
pub trait WeightInfo {
    fn create_polkadot_identity_loc() -> Weight;
    fn create_logion_identity_loc() -> Weight;
    fn create_polkadot_transaction_loc() -> Weight;
    fn create_logion_transaction_loc() -> Weight;
    fn add_metadata() -> Weight;
    fn add_file() -> Weight;
    fn add_link() -> Weight;
    fn close() -> Weight;
    fn make_void() -> Weight;
    fn make_void_and_replace() -> Weight;
    fn create_collection_loc() -> Weight;
    fn add_collection_item() -> Weight;
    fn nominate_issuer() -> Weight;
    fn dismiss_issuer() -> Weight;
    fn set_issuer_selection() -> Weight;
    fn add_tokens_record() -> Weight;
    fn create_other_identity_loc() -> Weight;
    fn sponsor() -> Weight;
	fn withdraw_sponsorship() -> Weight;
    fn acknowledge_metadata() -> Weight;
    fn acknowledge_file() -> Weight;
    fn acknowledge_link() -> Weight;
    fn set_invited_contributor_selection() -> Weight;
}

/// Weights for pallet_logion_loc using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `LoAuthorityList::LegalOfficerSet` (r:1 w:0)
	/// Proof: `LoAuthorityList::LegalOfficerSet` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::LocMap` (r:11 w:1)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::AccountLocsMap` (r:1 w:1)
	/// Proof: `LogionLoc::AccountLocsMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn create_polkadot_identity_loc() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2403`
		//  Estimated: `30618`
		// Minimum execution time: 95_940_000 picoseconds.
		Weight::from_parts(97_163_000, 0)
			.saturating_add(Weight::from_parts(0, 30618))
			.saturating_add(T::DbWeight::get().reads(13))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `LoAuthorityList::LegalOfficerSet` (r:1 w:0)
	/// Proof: `LoAuthorityList::LegalOfficerSet` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::LocMap` (r:1 w:1)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn create_logion_identity_loc() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `247`
		//  Estimated: `3712`
		// Minimum execution time: 22_428_000 picoseconds.
		Weight::from_parts(22_829_000, 0)
			.saturating_add(Weight::from_parts(0, 3712))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LoAuthorityList::LegalOfficerSet` (r:1 w:0)
	/// Proof: `LoAuthorityList::LegalOfficerSet` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::LocMap` (r:12 w:1)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::AccountLocsMap` (r:1 w:1)
	/// Proof: `LogionLoc::AccountLocsMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn create_polkadot_transaction_loc() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2515`
		//  Estimated: `33205`
		// Minimum execution time: 100_998_000 picoseconds.
		Weight::from_parts(101_851_000, 0)
			.saturating_add(Weight::from_parts(0, 33205))
			.saturating_add(T::DbWeight::get().reads(14))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `LoAuthorityList::LegalOfficerSet` (r:1 w:0)
	/// Proof: `LoAuthorityList::LegalOfficerSet` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::LocMap` (r:2 w:1)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::IdentityLocLocsMap` (r:1 w:1)
	/// Proof: `LogionLoc::IdentityLocLocsMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn create_logion_transaction_loc() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `416`
		//  Estimated: `6356`
		// Minimum execution time: 31_105_000 picoseconds.
		Weight::from_parts(31_613_000, 0)
			.saturating_add(Weight::from_parts(0, 6356))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `LoAuthorityList::LegalOfficerSet` (r:1 w:0)
	/// Proof: `LoAuthorityList::LegalOfficerSet` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::AccountLocsMap` (r:1 w:1)
	/// Proof: `LogionLoc::AccountLocsMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::LocMap` (r:12 w:1)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn create_collection_loc() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2515`
		//  Estimated: `33205`
		// Minimum execution time: 100_963_000 picoseconds.
		Weight::from_parts(102_063_000, 0)
			.saturating_add(Weight::from_parts(0, 33205))
			.saturating_add(T::DbWeight::get().reads(14))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `LogionLoc::LocMap` (r:1 w:1)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn add_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1305`
		//  Estimated: `4770`
		// Minimum execution time: 25_992_000 picoseconds.
		Weight::from_parts(26_327_000, 0)
			.saturating_add(Weight::from_parts(0, 4770))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LogionLoc::LocMap` (r:1 w:1)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn add_file() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1345`
		//  Estimated: `4810`
		// Minimum execution time: 34_977_000 picoseconds.
		Weight::from_parts(35_543_000, 0)
			.saturating_add(Weight::from_parts(0, 4810))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LogionLoc::LocMap` (r:2 w:1)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn add_link() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1483`
		//  Estimated: `7423`
		// Minimum execution time: 27_481_000 picoseconds.
		Weight::from_parts(27_937_000, 0)
			.saturating_add(Weight::from_parts(0, 7423))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LogionLoc::LocMap` (r:1 w:1)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn make_void() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `315`
		//  Estimated: `3780`
		// Minimum execution time: 23_023_000 picoseconds.
		Weight::from_parts(23_238_000, 0)
			.saturating_add(Weight::from_parts(0, 3780))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LogionLoc::LocMap` (r:2 w:2)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn make_void_and_replace() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `496`
		//  Estimated: `6436`
		// Minimum execution time: 30_497_000 picoseconds.
		Weight::from_parts(30_990_000, 0)
			.saturating_add(Weight::from_parts(0, 6436))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `LogionLoc::LocMap` (r:1 w:0)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::CollectionItemsMap` (r:1 w:1)
	/// Proof: `LogionLoc::CollectionItemsMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::CollectionSizeMap` (r:1 w:1)
	/// Proof: `LogionLoc::CollectionSizeMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn add_collection_item() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `386`
		//  Estimated: `3851`
		// Minimum execution time: 39_486_000 picoseconds.
		Weight::from_parts(40_257_000, 0)
			.saturating_add(Weight::from_parts(0, 3851))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `LoAuthorityList::LegalOfficerSet` (r:1 w:0)
	/// Proof: `LoAuthorityList::LegalOfficerSet` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::LocMap` (r:1 w:0)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::VerifiedIssuersMap` (r:1 w:1)
	/// Proof: `LogionLoc::VerifiedIssuersMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn nominate_issuer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `482`
		//  Estimated: `3947`
		// Minimum execution time: 24_249_000 picoseconds.
		Weight::from_parts(24_607_000, 0)
			.saturating_add(Weight::from_parts(0, 3947))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LoAuthorityList::LegalOfficerSet` (r:1 w:0)
	/// Proof: `LoAuthorityList::LegalOfficerSet` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::VerifiedIssuersMap` (r:1 w:1)
	/// Proof: `LogionLoc::VerifiedIssuersMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::LocsByVerifiedIssuerMap` (r:1 w:0)
	/// Proof: `LogionLoc::LocsByVerifiedIssuerMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn dismiss_issuer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456`
		//  Estimated: `3921`
		// Minimum execution time: 31_239_000 picoseconds.
		Weight::from_parts(31_902_000, 0)
			.saturating_add(Weight::from_parts(0, 3921))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LoAuthorityList::LegalOfficerSet` (r:1 w:0)
	/// Proof: `LoAuthorityList::LegalOfficerSet` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::LocMap` (r:1 w:0)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::VerifiedIssuersMap` (r:1 w:0)
	/// Proof: `LogionLoc::VerifiedIssuersMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::VerifiedIssuersByLocMap` (r:1 w:1)
	/// Proof: `LogionLoc::VerifiedIssuersByLocMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::LocsByVerifiedIssuerMap` (r:0 w:1)
	/// Proof: `LogionLoc::LocsByVerifiedIssuerMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn set_issuer_selection() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `669`
		//  Estimated: `4134`
		// Minimum execution time: 36_909_000 picoseconds.
		Weight::from_parts(37_441_000, 0)
			.saturating_add(Weight::from_parts(0, 4134))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `LogionLoc::LocMap` (r:1 w:0)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::TokensRecordsMap` (r:1 w:1)
	/// Proof: `LogionLoc::TokensRecordsMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn add_tokens_record() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `319`
		//  Estimated: `3784`
		// Minimum execution time: 35_524_000 picoseconds.
		Weight::from_parts(35_917_000, 0)
			.saturating_add(Weight::from_parts(0, 3784))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LoAuthorityList::LegalOfficerSet` (r:1 w:0)
	/// Proof: `LoAuthorityList::LegalOfficerSet` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::LocMap` (r:1 w:1)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::SponsorshipMap` (r:1 w:1)
	/// Proof: `LogionLoc::SponsorshipMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::OtherAccountLocsMap` (r:1 w:1)
	/// Proof: `LogionLoc::OtherAccountLocsMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn create_other_identity_loc() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `394`
		//  Estimated: `3859`
		// Minimum execution time: 45_376_000 picoseconds.
		Weight::from_parts(46_065_000, 0)
			.saturating_add(Weight::from_parts(0, 3859))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `LoAuthorityList::LegalOfficerSet` (r:1 w:0)
	/// Proof: `LoAuthorityList::LegalOfficerSet` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::SponsorshipMap` (r:1 w:1)
	/// Proof: `LogionLoc::SponsorshipMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn sponsor() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `247`
		//  Estimated: `3712`
		// Minimum execution time: 23_082_000 picoseconds.
		Weight::from_parts(23_505_000, 0)
			.saturating_add(Weight::from_parts(0, 3712))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LogionLoc::SponsorshipMap` (r:1 w:1)
	/// Proof: `LogionLoc::SponsorshipMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn withdraw_sponsorship() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `189`
		//  Estimated: `3654`
		// Minimum execution time: 16_804_000 picoseconds.
		Weight::from_parts(17_167_000, 0)
			.saturating_add(Weight::from_parts(0, 3654))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LogionLoc::LocMap` (r:1 w:1)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::VerifiedIssuersByLocMap` (r:1 w:0)
	/// Proof: `LogionLoc::VerifiedIssuersByLocMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn acknowledge_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1372`
		//  Estimated: `4837`
		// Minimum execution time: 28_104_000 picoseconds.
		Weight::from_parts(28_485_000, 0)
			.saturating_add(Weight::from_parts(0, 4837))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LogionLoc::LocMap` (r:1 w:1)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::VerifiedIssuersByLocMap` (r:1 w:0)
	/// Proof: `LogionLoc::VerifiedIssuersByLocMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn acknowledge_file() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1412`
		//  Estimated: `4877`
		// Minimum execution time: 27_103_000 picoseconds.
		Weight::from_parts(27_694_000, 0)
			.saturating_add(Weight::from_parts(0, 4877))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LogionLoc::LocMap` (r:1 w:1)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::VerifiedIssuersByLocMap` (r:1 w:0)
	/// Proof: `LogionLoc::VerifiedIssuersByLocMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn acknowledge_link() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1480`
		//  Estimated: `4945`
		// Minimum execution time: 26_542_000 picoseconds.
		Weight::from_parts(26_905_000, 0)
			.saturating_add(Weight::from_parts(0, 4945))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LogionLoc::LocMap` (r:1 w:1)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn close() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3433`
		//  Estimated: `6898`
		// Minimum execution time: 37_542_000 picoseconds.
		Weight::from_parts(38_053_000, 0)
			.saturating_add(Weight::from_parts(0, 6898))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LogionLoc::LocMap` (r:2 w:0)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::AccountLocsMap` (r:1 w:0)
	/// Proof: `LogionLoc::AccountLocsMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::InvitedContributorsByLocMap` (r:1 w:1)
	/// Proof: `LogionLoc::InvitedContributorsByLocMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn set_invited_contributor_selection() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `608`
		//  Estimated: `6548`
		// Minimum execution time: 30_176_000 picoseconds.
		Weight::from_parts(30_444_000, 0)
			.saturating_add(Weight::from_parts(0, 6548))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
