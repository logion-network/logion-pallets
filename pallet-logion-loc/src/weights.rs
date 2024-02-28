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
	fn import_loc() -> Weight;
	fn import_collection_item() -> Weight;
	fn import_tokens_record() -> Weight;
	fn import_invited_contributor_selection() -> Weight;
	fn import_verified_issuer() -> Weight;
	fn import_verified_issuer_selection() -> Weight;
	fn import_sponsorship() -> Weight;
}

/// Weights for pallet_logion_loc using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `LoAuthorityList::LegalOfficerSet` (r:1 w:0)
	/// Proof: `LoAuthorityList::LegalOfficerSet` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::LocMap` (r:51 w:1)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: Some(14513), added: 16988, mode: `MaxEncodedLen`)
	/// Storage: `LogionLoc::AccountLocsMap` (r:1 w:1)
	/// Proof: `LogionLoc::AccountLocsMap` (`max_values`: None, `max_size`: Some(3250), added: 5725, mode: `MaxEncodedLen`)
	fn create_polkadot_identity_loc() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `10293`
		//  Estimated: `867378`
		// Minimum execution time: 226_327_000 picoseconds.
		Weight::from_parts(233_772_000, 0)
			.saturating_add(Weight::from_parts(0, 867378))
			.saturating_add(T::DbWeight::get().reads(53))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `LoAuthorityList::LegalOfficerSet` (r:1 w:0)
	/// Proof: `LoAuthorityList::LegalOfficerSet` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::LocMap` (r:1 w:1)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: Some(14513), added: 16988, mode: `MaxEncodedLen`)
	fn create_logion_identity_loc() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `271`
		//  Estimated: `17978`
		// Minimum execution time: 18_671_000 picoseconds.
		Weight::from_parts(18_969_000, 0)
			.saturating_add(Weight::from_parts(0, 17978))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LoAuthorityList::LegalOfficerSet` (r:1 w:0)
	/// Proof: `LoAuthorityList::LegalOfficerSet` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::LocMap` (r:52 w:1)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: Some(14513), added: 16988, mode: `MaxEncodedLen`)
	/// Storage: `LogionLoc::AccountLocsMap` (r:1 w:1)
	/// Proof: `LogionLoc::AccountLocsMap` (`max_values`: None, `max_size`: Some(3250), added: 5725, mode: `MaxEncodedLen`)
	fn create_polkadot_transaction_loc() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `10440`
		//  Estimated: `884366`
		// Minimum execution time: 231_602_000 picoseconds.
		Weight::from_parts(249_136_000, 0)
			.saturating_add(Weight::from_parts(0, 884366))
			.saturating_add(T::DbWeight::get().reads(54))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `LoAuthorityList::LegalOfficerSet` (r:1 w:0)
	/// Proof: `LoAuthorityList::LegalOfficerSet` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::LocMap` (r:2 w:1)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: Some(14513), added: 16988, mode: `MaxEncodedLen`)
	fn create_logion_transaction_loc() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `440`
		//  Estimated: `34966`
		// Minimum execution time: 20_135_000 picoseconds.
		Weight::from_parts(20_624_000, 0)
			.saturating_add(Weight::from_parts(0, 34966))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LoAuthorityList::LegalOfficerSet` (r:1 w:0)
	/// Proof: `LoAuthorityList::LegalOfficerSet` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::AccountLocsMap` (r:1 w:1)
	/// Proof: `LogionLoc::AccountLocsMap` (`max_values`: None, `max_size`: Some(3250), added: 5725, mode: `MaxEncodedLen`)
	/// Storage: `LogionLoc::LocMap` (r:52 w:1)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: Some(14513), added: 16988, mode: `MaxEncodedLen`)
	fn create_collection_loc() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `10440`
		//  Estimated: `884366`
		// Minimum execution time: 231_018_000 picoseconds.
		Weight::from_parts(236_069_000, 0)
			.saturating_add(Weight::from_parts(0, 884366))
			.saturating_add(T::DbWeight::get().reads(54))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `LogionLoc::LocMap` (r:1 w:1)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: Some(14513), added: 16988, mode: `MaxEncodedLen`)
	fn add_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `5190`
		//  Estimated: `17978`
		// Minimum execution time: 36_129_000 picoseconds.
		Weight::from_parts(37_419_000, 0)
			.saturating_add(Weight::from_parts(0, 17978))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LogionLoc::LocMap` (r:1 w:1)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: Some(14513), added: 16988, mode: `MaxEncodedLen`)
	fn add_file() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `5386`
		//  Estimated: `17978`
		// Minimum execution time: 42_500_000 picoseconds.
		Weight::from_parts(45_224_000, 0)
			.saturating_add(Weight::from_parts(0, 17978))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LogionLoc::LocMap` (r:2 w:1)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: Some(14513), added: 16988, mode: `MaxEncodedLen`)
	fn add_link() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `5249`
		//  Estimated: `34966`
		// Minimum execution time: 35_292_000 picoseconds.
		Weight::from_parts(36_338_000, 0)
			.saturating_add(Weight::from_parts(0, 34966))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LogionLoc::LocMap` (r:1 w:1)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: Some(14513), added: 16988, mode: `MaxEncodedLen`)
	fn make_void() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `339`
		//  Estimated: `17978`
		// Minimum execution time: 16_935_000 picoseconds.
		Weight::from_parts(17_499_000, 0)
			.saturating_add(Weight::from_parts(0, 17978))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LogionLoc::LocMap` (r:2 w:2)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: Some(14513), added: 16988, mode: `MaxEncodedLen`)
	fn make_void_and_replace() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `521`
		//  Estimated: `34966`
		// Minimum execution time: 24_661_000 picoseconds.
		Weight::from_parts(27_556_000, 0)
			.saturating_add(Weight::from_parts(0, 34966))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `LogionLoc::LocMap` (r:1 w:0)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: Some(14513), added: 16988, mode: `MaxEncodedLen`)
	/// Storage: `LogionLoc::CollectionItemsMap` (r:1 w:1)
	/// Proof: `LogionLoc::CollectionItemsMap` (`max_values`: None, `max_size`: Some(1989), added: 4464, mode: `MaxEncodedLen`)
	/// Storage: `LogionLoc::CollectionSizeMap` (r:1 w:1)
	/// Proof: `LogionLoc::CollectionSizeMap` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	fn add_collection_item() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `410`
		//  Estimated: `17978`
		// Minimum execution time: 65_162_000 picoseconds.
		Weight::from_parts(72_543_000, 0)
			.saturating_add(Weight::from_parts(0, 17978))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `LoAuthorityList::LegalOfficerSet` (r:1 w:0)
	/// Proof: `LoAuthorityList::LegalOfficerSet` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::LocMap` (r:1 w:0)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: Some(14513), added: 16988, mode: `MaxEncodedLen`)
	/// Storage: `LogionLoc::VerifiedIssuersMap` (r:1 w:1)
	/// Proof: `LogionLoc::VerifiedIssuersMap` (`max_values`: None, `max_size`: Some(113), added: 2588, mode: `MaxEncodedLen`)
	fn nominate_issuer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `506`
		//  Estimated: `17978`
		// Minimum execution time: 18_969_000 picoseconds.
		Weight::from_parts(19_436_000, 0)
			.saturating_add(Weight::from_parts(0, 17978))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LoAuthorityList::LegalOfficerSet` (r:1 w:0)
	/// Proof: `LoAuthorityList::LegalOfficerSet` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::VerifiedIssuersMap` (r:1 w:1)
	/// Proof: `LogionLoc::VerifiedIssuersMap` (`max_values`: None, `max_size`: Some(113), added: 2588, mode: `MaxEncodedLen`)
	/// Storage: `LogionLoc::LocsByVerifiedIssuerMap` (r:1 w:0)
	/// Proof: `LogionLoc::LocsByVerifiedIssuerMap` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn dismiss_issuer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `457`
		//  Estimated: `3922`
		// Minimum execution time: 22_722_000 picoseconds.
		Weight::from_parts(24_055_000, 0)
			.saturating_add(Weight::from_parts(0, 3922))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LoAuthorityList::LegalOfficerSet` (r:1 w:0)
	/// Proof: `LoAuthorityList::LegalOfficerSet` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::LocMap` (r:1 w:0)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: Some(14513), added: 16988, mode: `MaxEncodedLen`)
	/// Storage: `LogionLoc::VerifiedIssuersMap` (r:1 w:0)
	/// Proof: `LogionLoc::VerifiedIssuersMap` (`max_values`: None, `max_size`: Some(113), added: 2588, mode: `MaxEncodedLen`)
	/// Storage: `LogionLoc::VerifiedIssuersByLocMap` (r:1 w:1)
	/// Proof: `LogionLoc::VerifiedIssuersByLocMap` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// Storage: `LogionLoc::LocsByVerifiedIssuerMap` (r:0 w:1)
	/// Proof: `LogionLoc::LocsByVerifiedIssuerMap` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn set_issuer_selection() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `694`
		//  Estimated: `17978`
		// Minimum execution time: 29_004_000 picoseconds.
		Weight::from_parts(30_911_000, 0)
			.saturating_add(Weight::from_parts(0, 17978))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `LogionLoc::LocMap` (r:1 w:0)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: Some(14513), added: 16988, mode: `MaxEncodedLen`)
	/// Storage: `LogionLoc::TokensRecordsMap` (r:1 w:1)
	/// Proof: `LogionLoc::TokensRecordsMap` (`max_values`: None, `max_size`: Some(1146), added: 3621, mode: `MaxEncodedLen`)
	fn add_tokens_record() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `343`
		//  Estimated: `17978`
		// Minimum execution time: 26_889_000 picoseconds.
		Weight::from_parts(28_123_000, 0)
			.saturating_add(Weight::from_parts(0, 17978))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LoAuthorityList::LegalOfficerSet` (r:1 w:0)
	/// Proof: `LoAuthorityList::LegalOfficerSet` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::LocMap` (r:1 w:1)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: Some(14513), added: 16988, mode: `MaxEncodedLen`)
	/// Storage: `LogionLoc::SponsorshipMap` (r:1 w:1)
	/// Proof: `LogionLoc::SponsorshipMap` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	fn create_other_identity_loc() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `415`
		//  Estimated: `17978`
		// Minimum execution time: 30_304_000 picoseconds.
		Weight::from_parts(31_096_000, 0)
			.saturating_add(Weight::from_parts(0, 17978))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `LoAuthorityList::LegalOfficerSet` (r:1 w:0)
	/// Proof: `LoAuthorityList::LegalOfficerSet` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::SponsorshipMap` (r:1 w:1)
	/// Proof: `LogionLoc::SponsorshipMap` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	fn sponsor() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `271`
		//  Estimated: `3736`
		// Minimum execution time: 17_133_000 picoseconds.
		Weight::from_parts(17_641_000, 0)
			.saturating_add(Weight::from_parts(0, 3736))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LogionLoc::SponsorshipMap` (r:1 w:1)
	/// Proof: `LogionLoc::SponsorshipMap` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	fn withdraw_sponsorship() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `210`
		//  Estimated: `3612`
		// Minimum execution time: 12_343_000 picoseconds.
		Weight::from_parts(13_411_000, 0)
			.saturating_add(Weight::from_parts(0, 3612))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LogionLoc::LocMap` (r:1 w:1)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: Some(14513), added: 16988, mode: `MaxEncodedLen`)
	/// Storage: `LogionLoc::VerifiedIssuersByLocMap` (r:1 w:0)
	/// Proof: `LogionLoc::VerifiedIssuersByLocMap` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	fn acknowledge_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `5356`
		//  Estimated: `17978`
		// Minimum execution time: 34_384_000 picoseconds.
		Weight::from_parts(35_457_000, 0)
			.saturating_add(Weight::from_parts(0, 17978))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LogionLoc::LocMap` (r:1 w:1)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: Some(14513), added: 16988, mode: `MaxEncodedLen`)
	/// Storage: `LogionLoc::VerifiedIssuersByLocMap` (r:1 w:0)
	/// Proof: `LogionLoc::VerifiedIssuersByLocMap` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	fn acknowledge_file() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `5556`
		//  Estimated: `17978`
		// Minimum execution time: 32_574_000 picoseconds.
		Weight::from_parts(33_820_000, 0)
			.saturating_add(Weight::from_parts(0, 17978))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LogionLoc::LocMap` (r:1 w:1)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: Some(14513), added: 16988, mode: `MaxEncodedLen`)
	/// Storage: `LogionLoc::VerifiedIssuersByLocMap` (r:1 w:0)
	/// Proof: `LogionLoc::VerifiedIssuersByLocMap` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	fn acknowledge_link() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `5056`
		//  Estimated: `17978`
		// Minimum execution time: 31_786_000 picoseconds.
		Weight::from_parts(33_044_000, 0)
			.saturating_add(Weight::from_parts(0, 17978))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LogionLoc::LocMap` (r:1 w:1)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: Some(14513), added: 16988, mode: `MaxEncodedLen`)
	fn close() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `15089`
		//  Estimated: `17978`
		// Minimum execution time: 69_538_000 picoseconds.
		Weight::from_parts(72_659_000, 0)
			.saturating_add(Weight::from_parts(0, 17978))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LogionLoc::LocMap` (r:2 w:0)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: Some(14513), added: 16988, mode: `MaxEncodedLen`)
	/// Storage: `LogionLoc::AccountLocsMap` (r:1 w:0)
	/// Proof: `LogionLoc::AccountLocsMap` (`max_values`: None, `max_size`: Some(3250), added: 5725, mode: `MaxEncodedLen`)
	/// Storage: `LogionLoc::InvitedContributorsByLocMap` (r:1 w:1)
	/// Proof: `LogionLoc::InvitedContributorsByLocMap` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	fn set_invited_contributor_selection() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `633`
		//  Estimated: `34966`
		// Minimum execution time: 23_595_000 picoseconds.
		Weight::from_parts(24_072_000, 0)
			.saturating_add(Weight::from_parts(0, 34966))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LogionLoc::LocMap` (r:1 w:1)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: Some(14513), added: 16988, mode: `MaxEncodedLen`)
	/// Storage: `LogionLoc::AccountLocsMap` (r:1 w:1)
	/// Proof: `LogionLoc::AccountLocsMap` (`max_values`: None, `max_size`: Some(3250), added: 5725, mode: `MaxEncodedLen`)
	fn import_loc() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `66`
		//  Estimated: `17978`
		// Minimum execution time: 66_516_000 picoseconds.
		Weight::from_parts(67_546_000, 0)
			.saturating_add(Weight::from_parts(0, 17978))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `LogionLoc::CollectionItemsMap` (r:1 w:1)
	/// Proof: `LogionLoc::CollectionItemsMap` (`max_values`: None, `max_size`: Some(1989), added: 4464, mode: `MaxEncodedLen`)
	/// Storage: `LogionLoc::CollectionSizeMap` (r:1 w:1)
	/// Proof: `LogionLoc::CollectionSizeMap` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	fn import_collection_item() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `66`
		//  Estimated: `5454`
		// Minimum execution time: 22_633_000 picoseconds.
		Weight::from_parts(23_501_000, 0)
			.saturating_add(Weight::from_parts(0, 5454))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `LogionLoc::TokensRecordsMap` (r:1 w:1)
	/// Proof: `LogionLoc::TokensRecordsMap` (`max_values`: None, `max_size`: Some(1146), added: 3621, mode: `MaxEncodedLen`)
	fn import_tokens_record() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `66`
		//  Estimated: `4611`
		// Minimum execution time: 17_527_000 picoseconds.
		Weight::from_parts(18_025_000, 0)
			.saturating_add(Weight::from_parts(0, 4611))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LogionLoc::InvitedContributorsByLocMap` (r:1 w:1)
	/// Proof: `LogionLoc::InvitedContributorsByLocMap` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	fn import_invited_contributor_selection() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `66`
		//  Estimated: `3545`
		// Minimum execution time: 8_832_000 picoseconds.
		Weight::from_parts(9_897_000, 0)
			.saturating_add(Weight::from_parts(0, 3545))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LogionLoc::VerifiedIssuersMap` (r:1 w:1)
	/// Proof: `LogionLoc::VerifiedIssuersMap` (`max_values`: None, `max_size`: Some(113), added: 2588, mode: `MaxEncodedLen`)
	fn import_verified_issuer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `66`
		//  Estimated: `3578`
		// Minimum execution time: 9_395_000 picoseconds.
		Weight::from_parts(9_642_000, 0)
			.saturating_add(Weight::from_parts(0, 3578))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LogionLoc::VerifiedIssuersByLocMap` (r:1 w:1)
	/// Proof: `LogionLoc::VerifiedIssuersByLocMap` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// Storage: `LogionLoc::LocsByVerifiedIssuerMap` (r:0 w:1)
	/// Proof: `LogionLoc::LocsByVerifiedIssuerMap` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn import_verified_issuer_selection() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `66`
		//  Estimated: `3545`
		// Minimum execution time: 12_877_000 picoseconds.
		Weight::from_parts(13_142_000, 0)
			.saturating_add(Weight::from_parts(0, 3545))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `LogionLoc::SponsorshipMap` (r:1 w:1)
	/// Proof: `LogionLoc::SponsorshipMap` (`max_values`: None, `max_size`: Some(147), added: 2622, mode: `MaxEncodedLen`)
	fn import_sponsorship() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `66`
		//  Estimated: `3612`
		// Minimum execution time: 11_420_000 picoseconds.
		Weight::from_parts(13_370_000, 0)
			.saturating_add(Weight::from_parts(0, 3612))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
