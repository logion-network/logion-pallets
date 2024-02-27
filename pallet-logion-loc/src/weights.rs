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
		// Minimum execution time: 219_560_000 picoseconds.
		Weight::from_parts(225_456_000, 0)
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
		// Minimum execution time: 16_939_000 picoseconds.
		Weight::from_parts(17_543_000, 0)
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
		// Minimum execution time: 222_215_000 picoseconds.
		Weight::from_parts(225_991_000, 0)
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
		// Minimum execution time: 19_804_000 picoseconds.
		Weight::from_parts(20_441_000, 0)
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
		// Minimum execution time: 226_216_000 picoseconds.
		Weight::from_parts(235_017_000, 0)
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
		// Minimum execution time: 37_750_000 picoseconds.
		Weight::from_parts(39_493_000, 0)
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
		// Minimum execution time: 41_667_000 picoseconds.
		Weight::from_parts(43_786_000, 0)
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
		// Minimum execution time: 35_155_000 picoseconds.
		Weight::from_parts(36_721_000, 0)
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
		// Minimum execution time: 17_045_000 picoseconds.
		Weight::from_parts(17_458_000, 0)
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
		// Minimum execution time: 23_174_000 picoseconds.
		Weight::from_parts(23_704_000, 0)
			.saturating_add(Weight::from_parts(0, 34966))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `LogionLoc::LocMap` (r:1 w:0)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: Some(14513), added: 16988, mode: `MaxEncodedLen`)
	/// Storage: `LogionLoc::CollectionItemsMap` (r:1 w:1)
	/// Proof: `LogionLoc::CollectionItemsMap` (`max_values`: None, `max_size`: Some(1988), added: 4463, mode: `MaxEncodedLen`)
	/// Storage: `LogionLoc::CollectionSizeMap` (r:1 w:1)
	/// Proof: `LogionLoc::CollectionSizeMap` (`max_values`: None, `max_size`: Some(36), added: 2511, mode: `MaxEncodedLen`)
	fn add_collection_item() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `410`
		//  Estimated: `17978`
		// Minimum execution time: 30_755_000 picoseconds.
		Weight::from_parts(31_714_000, 0)
			.saturating_add(Weight::from_parts(0, 17978))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `LoAuthorityList::LegalOfficerSet` (r:1 w:0)
	/// Proof: `LoAuthorityList::LegalOfficerSet` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::LocMap` (r:1 w:0)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: Some(14513), added: 16988, mode: `MaxEncodedLen`)
	/// Storage: `LogionLoc::VerifiedIssuersMap` (r:1 w:1)
	/// Proof: `LogionLoc::VerifiedIssuersMap` (`max_values`: None, `max_size`: Some(112), added: 2587, mode: `MaxEncodedLen`)
	fn nominate_issuer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `506`
		//  Estimated: `17978`
		// Minimum execution time: 18_632_000 picoseconds.
		Weight::from_parts(18_997_000, 0)
			.saturating_add(Weight::from_parts(0, 17978))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LoAuthorityList::LegalOfficerSet` (r:1 w:0)
	/// Proof: `LoAuthorityList::LegalOfficerSet` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::VerifiedIssuersMap` (r:1 w:1)
	/// Proof: `LogionLoc::VerifiedIssuersMap` (`max_values`: None, `max_size`: Some(112), added: 2587, mode: `MaxEncodedLen`)
	/// Storage: `LogionLoc::LocsByVerifiedIssuerMap` (r:1 w:0)
	/// Proof: `LogionLoc::LocsByVerifiedIssuerMap` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn dismiss_issuer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456`
		//  Estimated: `3921`
		// Minimum execution time: 23_122_000 picoseconds.
		Weight::from_parts(23_933_000, 0)
			.saturating_add(Weight::from_parts(0, 3921))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LoAuthorityList::LegalOfficerSet` (r:1 w:0)
	/// Proof: `LoAuthorityList::LegalOfficerSet` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::LocMap` (r:1 w:0)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: Some(14513), added: 16988, mode: `MaxEncodedLen`)
	/// Storage: `LogionLoc::VerifiedIssuersMap` (r:1 w:0)
	/// Proof: `LogionLoc::VerifiedIssuersMap` (`max_values`: None, `max_size`: Some(112), added: 2587, mode: `MaxEncodedLen`)
	/// Storage: `LogionLoc::VerifiedIssuersByLocMap` (r:1 w:1)
	/// Proof: `LogionLoc::VerifiedIssuersByLocMap` (`max_values`: None, `max_size`: Some(80), added: 2555, mode: `MaxEncodedLen`)
	/// Storage: `LogionLoc::LocsByVerifiedIssuerMap` (r:0 w:1)
	/// Proof: `LogionLoc::LocsByVerifiedIssuerMap` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn set_issuer_selection() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `693`
		//  Estimated: `17978`
		// Minimum execution time: 29_416_000 picoseconds.
		Weight::from_parts(30_773_000, 0)
			.saturating_add(Weight::from_parts(0, 17978))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `LogionLoc::LocMap` (r:1 w:0)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: Some(14513), added: 16988, mode: `MaxEncodedLen`)
	/// Storage: `LogionLoc::TokensRecordsMap` (r:1 w:1)
	/// Proof: `LogionLoc::TokensRecordsMap` (`max_values`: None, `max_size`: Some(1145), added: 3620, mode: `MaxEncodedLen`)
	fn add_tokens_record() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `343`
		//  Estimated: `17978`
		// Minimum execution time: 26_687_000 picoseconds.
		Weight::from_parts(30_478_000, 0)
			.saturating_add(Weight::from_parts(0, 17978))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LoAuthorityList::LegalOfficerSet` (r:1 w:0)
	/// Proof: `LoAuthorityList::LegalOfficerSet` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::LocMap` (r:1 w:1)
	/// Proof: `LogionLoc::LocMap` (`max_values`: None, `max_size`: Some(14513), added: 16988, mode: `MaxEncodedLen`)
	/// Storage: `LogionLoc::SponsorshipMap` (r:1 w:1)
	/// Proof: `LogionLoc::SponsorshipMap` (`max_values`: None, `max_size`: Some(146), added: 2621, mode: `MaxEncodedLen`)
	fn create_other_identity_loc() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `414`
		//  Estimated: `17978`
		// Minimum execution time: 31_218_000 picoseconds.
		Weight::from_parts(34_689_000, 0)
			.saturating_add(Weight::from_parts(0, 17978))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `LoAuthorityList::LegalOfficerSet` (r:1 w:0)
	/// Proof: `LoAuthorityList::LegalOfficerSet` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `LogionLoc::SponsorshipMap` (r:1 w:1)
	/// Proof: `LogionLoc::SponsorshipMap` (`max_values`: None, `max_size`: Some(146), added: 2621, mode: `MaxEncodedLen`)
	fn sponsor() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `271`
		//  Estimated: `3736`
		// Minimum execution time: 17_286_000 picoseconds.
		Weight::from_parts(17_813_000, 0)
			.saturating_add(Weight::from_parts(0, 3736))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `LogionLoc::SponsorshipMap` (r:1 w:1)
	/// Proof: `LogionLoc::SponsorshipMap` (`max_values`: None, `max_size`: Some(146), added: 2621, mode: `MaxEncodedLen`)
	fn withdraw_sponsorship() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `209`
		//  Estimated: `3611`
		// Minimum execution time: 12_400_000 picoseconds.
		Weight::from_parts(12_885_000, 0)
			.saturating_add(Weight::from_parts(0, 3611))
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
		// Minimum execution time: 35_401_000 picoseconds.
		Weight::from_parts(39_657_000, 0)
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
		// Minimum execution time: 32_135_000 picoseconds.
		Weight::from_parts(33_047_000, 0)
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
		// Minimum execution time: 31_840_000 picoseconds.
		Weight::from_parts(33_039_000, 0)
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
		// Minimum execution time: 67_467_000 picoseconds.
		Weight::from_parts(69_635_000, 0)
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
		// Minimum execution time: 22_996_000 picoseconds.
		Weight::from_parts(25_631_000, 0)
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
		// Minimum execution time: 63_741_000 picoseconds.
		Weight::from_parts(66_251_000, 0)
			.saturating_add(Weight::from_parts(0, 17978))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
