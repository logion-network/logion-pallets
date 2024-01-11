//! Benchmarking setup for pallet-logion-loc
#![cfg(feature = "runtime-benchmarks")]
use super::*;

use crate::{ItemsParams, MetadataItemParamsOf, FileParamsOf, LocLinkParamsOf, Pallet as LogionLoc};

use frame_benchmarking::{account, impl_benchmark_test_suite, v2::*, BenchmarkError};
use frame_support::assert_ok;
use frame_system::RawOrigin;

use logion_shared::IsLegalOfficer;

use sp_core::{Get, hash::H256};
use sp_io::hashing::sha2_256;
use sp_runtime::traits::Bounded;


pub trait LocIdFactory<LocId> {

	fn loc_id(id: u32) -> LocId;
}
impl<LocId: From<u32>> LocIdFactory<LocId> for () {

	fn loc_id(id: u32) -> LocId {
		id.into()
	}
}

pub trait CollectionItemIdFactory<CollectionItemId> {

	fn collection_item_id(id: u8) -> CollectionItemId;
}
impl CollectionItemIdFactory<H256> for () {

	fn collection_item_id(id: u8) -> H256 {
		let bytes = sha2_256(&[id]);
		H256(bytes)
	}
}

pub trait TokensRecordIdFactory<TokensRecordId> {

	fn tokens_record_id(id: u8) -> TokensRecordId;
}
impl TokensRecordIdFactory<H256> for () {

	fn tokens_record_id(id: u8) -> H256 {
		let bytes = sha2_256(&[id]);
		H256(bytes)
	}
}

pub trait EthereumAddressFactory<EthereumAddress> {

	fn address(id: u8) -> EthereumAddress;
}
impl EthereumAddressFactory<H160> for () {

	fn address(id: u8) -> H160 {
		let bytes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, id];
		H160(bytes)
	}
}

pub trait SponsorshipIdFactory<SponsorshipId> {

	fn sponsorship_id(id: u32) -> SponsorshipId;
}
impl<SponsorshipId: From<u32>> SponsorshipIdFactory<SponsorshipId> for () {

	fn sponsorship_id(id: u32) -> SponsorshipId {
		id.into()
	}
}

#[benchmarks]
mod benchmarks {
	use super::*;

	// Benchmark `create_polkadot_identity_loc` extrinsic with the worst possible conditions:
	// * LOC with "many" files, metadata and links.
	//
	// TODO: put a limit on the number of items (otherwise, no bounded worst case)
	#[benchmark]
	fn create_polkadot_identity_loc() -> Result<(), BenchmarkError> {
		let legal_officer_id = any_legal_officer::<T>();
		let requester: T::AccountId = account("requester", 1, SEED);
		create_closed_polkadot_identity_loc::<T>(T::LocIdFactory::loc_id(REQUESTER_IDENTITY_LOC_ID), &legal_officer_id, &requester);
		ensure_enough_funds::<T>(&requester);
		let items = many_items::<T>(&requester);
		let next_loc_id: T::LocId = T::LocIdFactory::loc_id(NEXT_LOC_ID);

		#[extrinsic_call]
		_(
			RawOrigin::Signed(requester),
			next_loc_id,
			legal_officer_id,
			0u32.into(),
			items,
		);

		assert!(LogionLoc::<T>::loc(next_loc_id).is_some());

		Ok(())
	}

	// Benchmark `create_logion_identity_loc` extrinsic.
	#[benchmark]
	fn create_logion_identity_loc() -> Result<(), BenchmarkError> {
		let legal_officer_id = any_legal_officer::<T>();
		let loc_id: T::LocId = T::LocIdFactory::loc_id(0);

		#[extrinsic_call]
		_(
			RawOrigin::Signed(legal_officer_id),
			loc_id,
		);

		assert!(LogionLoc::<T>::loc(loc_id).is_some());

		Ok(())
	}

	// Benchmark `create_polkadot_transaction_loc` extrinsic with the worst possible conditions:
	// * LOC with "many" files, metadata and links.
	//
	// TODO: put a limit on the number of items (otherwise, no bounded worst case)
	#[benchmark]
	fn create_polkadot_transaction_loc() -> Result<(), BenchmarkError> {
		let legal_officer_id = any_legal_officer::<T>();
		let requester: T::AccountId = account("requester", 1, SEED);
		create_closed_polkadot_identity_loc::<T>(T::LocIdFactory::loc_id(REQUESTER_IDENTITY_LOC_ID), &legal_officer_id, &requester);
		ensure_enough_funds::<T>(&requester);
		let items = many_items::<T>(&requester);
		let next_loc_id: T::LocId = T::LocIdFactory::loc_id(NEXT_LOC_ID);

		#[extrinsic_call]
		_(
			RawOrigin::Signed(requester),
			next_loc_id,
			legal_officer_id,
			0u32.into(),
			items,
		);

		assert!(LogionLoc::<T>::loc(next_loc_id).is_some());

		Ok(())
	}

	// Benchmark `create_logion_transaction_loc` extrinsic.
	#[benchmark]
	fn create_logion_transaction_loc() -> Result<(), BenchmarkError> {
		let legal_officer_id = any_legal_officer::<T>();
		let identity_loc_id: T::LocId = T::LocIdFactory::loc_id(0);
		assert_ok!(LogionLoc::<T>::create_logion_identity_loc(
			<T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(legal_officer_id.clone())),
			identity_loc_id.into(),
		));
		assert_ok!(LogionLoc::<T>::close(
			<T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(legal_officer_id.clone())),
			identity_loc_id.into(),
			None,
			false,
		));
		let loc_id: T::LocId = T::LocIdFactory::loc_id(1);

		#[extrinsic_call]
		_(
			RawOrigin::Signed(legal_officer_id),
			loc_id,
			identity_loc_id.clone(),
		);

		assert!(LogionLoc::<T>::loc(loc_id).is_some());

		Ok(())
	}

	// Benchmark `create_collection_loc` extrinsic with the worst possible conditions:
	// * LOC with "many" files, metadata and links.
	//
	// TODO: put a limit on the number of items (otherwise, no bounded worst case)
	#[benchmark]
	fn create_collection_loc() -> Result<(), BenchmarkError> {
		let legal_officer_id = any_legal_officer::<T>();
		let requester: T::AccountId = account("requester", 1, SEED);
		create_closed_polkadot_identity_loc::<T>(T::LocIdFactory::loc_id(REQUESTER_IDENTITY_LOC_ID), &legal_officer_id, &requester);
		ensure_enough_funds::<T>(&requester);
		let items = many_items::<T>(&requester);
		let next_loc_id: T::LocId = T::LocIdFactory::loc_id(NEXT_LOC_ID);

		#[extrinsic_call]
		_(
			RawOrigin::Signed(requester),
			next_loc_id,
			legal_officer_id,
			None,
			Some(100),
			true,
			0u32.into(),
			0u32.into(),
			0u32.into(),
			0u32.into(),
			items,
		);

		assert!(LogionLoc::<T>::loc(next_loc_id).is_some());

		Ok(())
	}

	// Benchmark `add_metadata` extrinsic with the worst possible conditions:
	// * LOC has already "many" metadata items
	//
	// TODO: put a limit on the number of items (otherwise, no bounded worst case)
	#[benchmark]
	fn add_metadata() -> Result<(), BenchmarkError> {
		let (loc_id, requester) = setup_empty_loc::<T>();
		add_many_metadata::<T>(&loc_id, &requester);

		#[extrinsic_call]
		_(
			RawOrigin::Signed(requester.clone()),
			loc_id,
			metadata_item::<T>(MANY_ITEMS, &requester),
		);

		Ok(())
	}

	// Benchmark `add_file` extrinsic with the worst possible conditions:
	// * LOC has already "many" files
	//
	// TODO: put a limit on the number of items (otherwise, no bounded worst case)
	#[benchmark]
	fn add_file() -> Result<(), BenchmarkError> {
		let (loc_id, requester) = setup_empty_loc::<T>();
		add_many_files::<T>(&loc_id, &requester);

		#[extrinsic_call]
		_(
			RawOrigin::Signed(requester.clone()),
			loc_id,
			file::<T>(MANY_ITEMS, &requester),
		);

		Ok(())
	}

	// Benchmark `add_link` extrinsic with the worst possible conditions:
	// * LOC has already "many" files
	//
	// TODO: put a limit on the number of items (otherwise, no bounded worst case)
	#[benchmark]
	fn add_link() -> Result<(), BenchmarkError> {
		let legal_officer_id = any_legal_officer::<T>();
		let requester: T::AccountId = account("requester", 1, SEED);
		create_closed_polkadot_identity_loc::<T>(T::LocIdFactory::loc_id(REQUESTER_IDENTITY_LOC_ID), &legal_officer_id, &requester);
		ensure_enough_funds::<T>(&requester);
		create_locs_to_link_to::<T>(&requester); // Targets of the many links
		create_loc::<T>(T::LocIdFactory::loc_id(MANY_ITEMS), &legal_officer_id, &requester); // New target

		let loc_id: T::LocId = T::LocIdFactory::loc_id(MANY_ITEMS + 1);
		create_loc::<T>(loc_id, &legal_officer_id, &requester);
		add_many_links::<T>(&loc_id, &requester);

		#[extrinsic_call]
		_(
			RawOrigin::Signed(requester.clone()),
			loc_id,
			loc_link::<T>(MANY_ITEMS, &requester),
		);

		Ok(())
	}

	// Benchmark `make_void` extrinsic.
	#[benchmark]
	fn make_void() -> Result<(), BenchmarkError> {
		let (loc_id, _) = setup_empty_loc::<T>();
		let legal_officer_id = any_legal_officer::<T>();

		#[extrinsic_call]
		_(
			RawOrigin::Signed(legal_officer_id.clone()),
			loc_id,
		);

		Ok(())
	}

	// Benchmark `make_void_and_replace` extrinsic.
	#[benchmark]
	fn make_void_and_replace() -> Result<(), BenchmarkError> {
		let (loc_id, requester) = setup_empty_loc::<T>();
		let legal_officer_id = any_legal_officer::<T>();
		let replacer_id: T::LocId = T::LocIdFactory::loc_id(1);
		create_loc::<T>(replacer_id, &legal_officer_id, &requester);

		#[extrinsic_call]
		_(
			RawOrigin::Signed(legal_officer_id.clone()),
			loc_id,
			replacer_id,
		);

		Ok(())
	}

	// Benchmark `add_collection_item` extrinsic with the worst possible conditions:
	// * LOC has already "many" files
	//
	// TODO: put a limit on the number of files (otherwise, no bounded worst case)
	#[benchmark]
	fn add_collection_item() -> Result<(), BenchmarkError> {
		let legal_officer_id = any_legal_officer::<T>();
		let requester: T::AccountId = account("requester", 1, SEED);
		create_closed_polkadot_identity_loc::<T>(T::LocIdFactory::loc_id(REQUESTER_IDENTITY_LOC_ID), &legal_officer_id, &requester);
		ensure_enough_funds::<T>(&requester);

		let loc_id: T::LocId = T::LocIdFactory::loc_id(0);
		assert_ok!(LogionLoc::<T>::create_collection_loc(
			<T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(requester.clone())),
			loc_id,
			legal_officer_id.clone(),
			None,
			Some(100),
			true,
			0u32.into(),
			0u32.into(),
			0u32.into(),
			0u32.into(),
			ItemsParams {
				metadata: Vec::new(),
				files: Vec::new(),
				links: Vec::new(),
			},
		));
		assert_ok!(LogionLoc::<T>::close(
			<T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(legal_officer_id.clone())),
			loc_id,
			None,
			false,
		));

		let item_id: T::CollectionItemId = T::CollectionItemIdFactory::collection_item_id(0);
		let item_description = T::Hasher::hash(&Vec::from([0u8]));

		#[extrinsic_call]
		_(
			RawOrigin::Signed(requester.clone()),
			loc_id,
			item_id,
			item_description,
			Vec::new(),
			None,
			false,
			Vec::new(),
		);

		Ok(())
	}

	// Benchmark `nominate_issuer` extrinsic.
	#[benchmark]
	fn nominate_issuer() -> Result<(), BenchmarkError> {
		let legal_officer_id = any_legal_officer::<T>();
		let requester: T::AccountId = account("requester", 1, SEED);
		ensure_enough_funds::<T>(&requester);

		let loc_id: T::LocId = T::LocIdFactory::loc_id(0);
		assert_ok!(LogionLoc::<T>::create_polkadot_identity_loc(
			<T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(requester.clone())),
			loc_id,
			legal_officer_id.clone(),
			0u32.into(),
			ItemsParams {
				metadata: Vec::new(),
				files: Vec::new(),
				links: Vec::new(),
			},
		));
		assert_ok!(LogionLoc::<T>::close(
			<T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(legal_officer_id.clone())),
			loc_id,
			None,
			false,
		));

		#[extrinsic_call]
		_(
			RawOrigin::Signed(legal_officer_id.clone()),
			requester.clone(),
			loc_id,
		);

		Ok(())
	}

	// Benchmark `dismiss_issuer` extrinsic.
	#[benchmark]
	fn dismiss_issuer() -> Result<(), BenchmarkError> {
		let legal_officer_id = any_legal_officer::<T>();
		let requester: T::AccountId = account("requester", 1, SEED);
		ensure_enough_funds::<T>(&requester);

		let loc_id: T::LocId = T::LocIdFactory::loc_id(0);
		assert_ok!(LogionLoc::<T>::create_polkadot_identity_loc(
			<T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(requester.clone())),
			loc_id,
			legal_officer_id.clone(),
			0u32.into(),
			ItemsParams {
				metadata: Vec::new(),
				files: Vec::new(),
				links: Vec::new(),
			},
		));
		assert_ok!(LogionLoc::<T>::close(
			<T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(legal_officer_id.clone())),
			loc_id,
			None,
			false,
		));
		assert_ok!(LogionLoc::<T>::nominate_issuer(
			<T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(legal_officer_id.clone())),
			requester.clone(),
			loc_id,
		));

		#[extrinsic_call]
		_(
			RawOrigin::Signed(legal_officer_id.clone()),
			requester.clone(),
		);

		Ok(())
	}

	// Benchmark `set_issuer_selection` extrinsic.
	#[benchmark]
	fn set_issuer_selection() -> Result<(), BenchmarkError> {
		let legal_officer_id = any_legal_officer::<T>();
		let requester: T::AccountId = account("requester", 1, SEED);
		ensure_enough_funds::<T>(&requester);

		let loc_id: T::LocId = T::LocIdFactory::loc_id(0);
		assert_ok!(LogionLoc::<T>::create_polkadot_identity_loc(
			<T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(requester.clone())),
			loc_id,
			legal_officer_id.clone(),
			0u32.into(),
			ItemsParams {
				metadata: Vec::new(),
				files: Vec::new(),
				links: Vec::new(),
			},
		));
		assert_ok!(LogionLoc::<T>::close(
			<T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(legal_officer_id.clone())),
			loc_id,
			None,
			false,
		));
		assert_ok!(LogionLoc::<T>::nominate_issuer(
			<T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(legal_officer_id.clone())),
			requester.clone(),
			loc_id,
		));

		#[extrinsic_call]
		_(
			RawOrigin::Signed(legal_officer_id.clone()),
			loc_id,
			requester.clone(),
			true,
		);

		Ok(())
	}

	// Benchmark `add_tokens_record` extrinsic with the worst possible conditions:
	// * Max files
	#[benchmark]
	fn add_tokens_record() -> Result<(), BenchmarkError> {
		let legal_officer_id = any_legal_officer::<T>();
		let requester: T::AccountId = account("requester", 1, SEED);
		create_closed_polkadot_identity_loc::<T>(T::LocIdFactory::loc_id(REQUESTER_IDENTITY_LOC_ID), &legal_officer_id, &requester);
		ensure_enough_funds::<T>(&requester);

		let loc_id: T::LocId = T::LocIdFactory::loc_id(0);
		assert_ok!(LogionLoc::<T>::create_collection_loc(
			<T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(requester.clone())),
			loc_id,
			legal_officer_id.clone(),
			None,
			Some(100),
			true,
			0u32.into(),
			0u32.into(),
			0u32.into(),
			0u32.into(),
			ItemsParams {
				metadata: Vec::new(),
				files: Vec::new(),
				links: Vec::new(),
			},
		));
		assert_ok!(LogionLoc::<T>::close(
			<T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(legal_officer_id.clone())),
			loc_id,
			None,
			false,
		));

		let record_id: T::TokensRecordId = T::TokensRecordIdFactory::tokens_record_id(0);
		let description = T::Hasher::hash(&Vec::from([0u8]));

		#[extrinsic_call]
		_(
			RawOrigin::Signed(requester.clone()),
			loc_id,
			record_id,
			description,
			max_tokens_record_files::<T>(),
		);

		Ok(())
	}

	// Benchmark `create_other_identity_loc` extrinsic.
	#[benchmark]
	fn create_other_identity_loc() -> Result<(), BenchmarkError> {
		let legal_officer_id = any_legal_officer::<T>();
		let requester = OtherAccountId::Ethereum(T::EthereumAddressFactory::address(0));
		let loc_id = T::LocIdFactory::loc_id(0);
		let sponsorship_id = T::SponsorshipIdFactory::sponsorship_id(0);

		assert_ok!(LogionLoc::<T>::sponsor(
			<T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(legal_officer_id.clone())),
			sponsorship_id,
			SupportedAccountId::Other(requester),
			legal_officer_id.clone(),
		));

		#[extrinsic_call]
		_(
			RawOrigin::Signed(legal_officer_id.clone()),
			loc_id,
			requester,
			sponsorship_id,
			0u32.into(),
		);

		assert!(LogionLoc::<T>::loc(loc_id).is_some());

		Ok(())
	}

	// Benchmark `sponsor` extrinsic.
	#[benchmark]
	fn sponsor() -> Result<(), BenchmarkError> {
		let legal_officer_id = any_legal_officer::<T>();
		let requester = OtherAccountId::Ethereum(T::EthereumAddressFactory::address(0));
		let sponsorship_id = T::SponsorshipIdFactory::sponsorship_id(0);

		#[extrinsic_call]
		_(
			RawOrigin::Signed(legal_officer_id.clone()),
			sponsorship_id,
			SupportedAccountId::Other(requester),
			legal_officer_id.clone(),
		);

		Ok(())
	}

	// Benchmark `withdraw_sponsorship` extrinsic.
	#[benchmark]
	fn withdraw_sponsorship() -> Result<(), BenchmarkError> {
		let legal_officer_id = any_legal_officer::<T>();
		let requester = OtherAccountId::Ethereum(T::EthereumAddressFactory::address(0));
		let sponsorship_id = T::SponsorshipIdFactory::sponsorship_id(0);

		assert_ok!(LogionLoc::<T>::sponsor(
			<T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(legal_officer_id.clone())),
			sponsorship_id,
			SupportedAccountId::Other(requester),
			legal_officer_id.clone(),
		));

		#[extrinsic_call]
		_(
			RawOrigin::Signed(legal_officer_id.clone()),
			sponsorship_id,
		);

		Ok(())
	}

	// Benchmark `acknowledge_metadata` extrinsic with the worst possible conditions:
	// * LOC with "many" metadata items.
	// * Acknowledge last item.
	//
	// TODO: put a limit on the number of items (otherwise, no bounded worst case)
	#[benchmark]
	fn acknowledge_metadata() -> Result<(), BenchmarkError> {
		let legal_officer_id = any_legal_officer::<T>();
		let requester: T::AccountId = account("requester", 1, SEED);
		create_closed_polkadot_identity_loc::<T>(T::LocIdFactory::loc_id(REQUESTER_IDENTITY_LOC_ID), &legal_officer_id, &requester);
		ensure_enough_funds::<T>(&requester);

		let loc_id: T::LocId = T::LocIdFactory::loc_id(0);
		let items = ItemsParams {
			metadata: many_metadata::<T>(&requester),
			files: Vec::new(),
			links: Vec::new(),
		};
		assert_ok!(LogionLoc::<T>::create_polkadot_transaction_loc(
			<T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(requester.clone())),
			loc_id,
			legal_officer_id.clone(),
			0u32.into(),
			items,
		));

		#[extrinsic_call]
		_(
			RawOrigin::Signed(legal_officer_id.clone()),
			loc_id,
			T::Hasher::hash(&Vec::from([(MANY_ITEMS - 1) as u8])),
		);

		Ok(())
	}

	// Benchmark `acknowledge_file` extrinsic with the worst possible conditions:
	// * LOC with "many" files.
	// * Acknowledge last item.
	//
	// TODO: put a limit on the number of items (otherwise, no bounded worst case)
	#[benchmark]
	fn acknowledge_file() -> Result<(), BenchmarkError> {
		let legal_officer_id = any_legal_officer::<T>();
		let requester: T::AccountId = account("requester", 1, SEED);
		create_closed_polkadot_identity_loc::<T>(T::LocIdFactory::loc_id(REQUESTER_IDENTITY_LOC_ID), &legal_officer_id, &requester);
		ensure_enough_funds::<T>(&requester);

		let loc_id: T::LocId = T::LocIdFactory::loc_id(0);
		let items = ItemsParams {
			metadata: Vec::new(),
			files: many_files::<T>(&requester),
			links: Vec::new(),
		};
		assert_ok!(LogionLoc::<T>::create_polkadot_transaction_loc(
			<T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(requester.clone())),
			loc_id,
			legal_officer_id.clone(),
			0u32.into(),
			items,
		));

		#[extrinsic_call]
		_(
			RawOrigin::Signed(legal_officer_id.clone()),
			loc_id,
			T::Hasher::hash(&Vec::from([(MANY_ITEMS - 1) as u8])),
		);

		Ok(())
	}

	// Benchmark `acknowledge_link` extrinsic with the worst possible conditions:
	// * LOC with "many" files.
	// * Acknowledge last item.
	//
	// TODO: put a limit on the number of items (otherwise, no bounded worst case)
	#[benchmark]
	fn acknowledge_link() -> Result<(), BenchmarkError> {
		let legal_officer_id = any_legal_officer::<T>();
		let requester: T::AccountId = account("requester", 1, SEED);
		create_closed_polkadot_identity_loc::<T>(T::LocIdFactory::loc_id(REQUESTER_IDENTITY_LOC_ID), &legal_officer_id, &requester);
		ensure_enough_funds::<T>(&requester);
		create_locs_to_link_to::<T>(&requester); // Targets of the many links

		let loc_id: T::LocId = T::LocIdFactory::loc_id(MANY_ITEMS);
		let items = ItemsParams {
			metadata: Vec::new(),
			files: Vec::new(),
			links: many_loc_links::<T>(&requester),
		};
		assert_ok!(LogionLoc::<T>::create_polkadot_transaction_loc(
			<T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(requester.clone())),
			loc_id,
			legal_officer_id.clone(),
			0u32.into(),
			items,
		));

		#[extrinsic_call]
		_(
			RawOrigin::Signed(legal_officer_id.clone()),
			loc_id,
			T::LocIdFactory::loc_id(MANY_ITEMS - 1),
		);

		Ok(())
	}

	// Benchmark `close` extrinsic with the worst possible conditions:
	// * LOC with "many" items.
	// * With auto-ack.
	//
	// TODO: put a limit on the number of items (otherwise, no bounded worst case)
	#[benchmark]
	fn close() -> Result<(), BenchmarkError> {
		let legal_officer_id = any_legal_officer::<T>();
		let requester: T::AccountId = account("requester", 1, SEED);
		create_closed_polkadot_identity_loc::<T>(T::LocIdFactory::loc_id(REQUESTER_IDENTITY_LOC_ID), &legal_officer_id, &requester);
		ensure_enough_funds::<T>(&requester);
		let items = many_items::<T>(&requester);
		let next_loc_id: T::LocId = T::LocIdFactory::loc_id(NEXT_LOC_ID);

		assert_ok!(LogionLoc::<T>::create_polkadot_identity_loc(
			<T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(requester.clone())),
			next_loc_id,
			legal_officer_id.clone(),
			0u32.into(),
			items,
		));

		#[extrinsic_call]
		_(
			RawOrigin::Signed(legal_officer_id.clone()),
			next_loc_id,
			None,
			true,
		);

		Ok(())
	}

	// Benchmark `set_invited_contributor_selection` extrinsic.
	#[benchmark]
	fn set_invited_contributor_selection() -> Result<(), BenchmarkError> {
		let legal_officer_id = any_legal_officer::<T>();
		let requester: T::AccountId = account("requester", 1, SEED);
		create_closed_polkadot_identity_loc::<T>(T::LocIdFactory::loc_id(REQUESTER_IDENTITY_LOC_ID), &legal_officer_id, &requester);

		ensure_enough_funds::<T>(&requester);

		let invited_contributor: T::AccountId = account("invited_contributor", 1, SEED);
		create_closed_polkadot_identity_loc::<T>(T::LocIdFactory::loc_id(INVITED_CONTRIBUTOR_IDENTITY_LOC_ID), &legal_officer_id, &invited_contributor);

		let loc_id: T::LocId = T::LocIdFactory::loc_id(0);
		assert_ok!(LogionLoc::<T>::create_collection_loc(
			<T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(requester.clone())),
			loc_id,
			legal_officer_id.clone(),
			None,
			Some(100),
			true,
			0u32.into(),
			0u32.into(),
			0u32.into(),
			0u32.into(),
			ItemsParams {
				metadata: Vec::new(),
				files: Vec::new(),
				links: Vec::new(),
			},
		));
		assert_ok!(LogionLoc::<T>::close(
			<T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(legal_officer_id.clone())),
			loc_id,
			None,
			false,
		));

		#[extrinsic_call]
		_(
			RawOrigin::Signed(requester.clone()),
			loc_id,
			requester.clone(),
			true,
		);

		Ok(())
	}

	impl_benchmark_test_suite! {
		LogionLoc,
		crate::mock::new_test_ext(),
		crate::mock::Test,
	}
}

fn any_legal_officer<T: pallet::Config>() -> T::AccountId {
	let legal_officers = T::IsLegalOfficer::legal_officers();
	legal_officers[0].clone()
}

const SEED: u32 = 0;

fn ensure_enough_funds<T: pallet::Config>(account_id: &T::AccountId) {
	T::Currency::make_free_balance_be(account_id, BalanceOf::<T>::max_value());
}

fn many_items<T: pallet::Config>(requester: &T::AccountId) -> ItemsParamsOf<T> {
	create_locs_to_link_to::<T>(requester);

	let metadata = many_metadata::<T>(requester);
	let files = many_files::<T>(requester);
	let links = many_loc_links::<T>(requester);
	ItemsParams {
		metadata,
		files,
		links,
	}
}

fn many_metadata<T: pallet::Config>(requester: &T::AccountId) -> Vec<MetadataItemParamsOf<T>> {
	let mut metadata: Vec<MetadataItemParamsOf<T>> = Vec::new();
	for i in 0..MANY_ITEMS {
		metadata.push(metadata_item::<T>(i, requester));
	}
	metadata
}

fn many_files<T: pallet::Config>(requester: &T::AccountId) -> Vec<FileParamsOf<T>> {
	let mut files: Vec<FileParamsOf<T>> = Vec::new();
	for i in 0..MANY_ITEMS {
		files.push(file::<T>(i, requester));
	}
	files
}

fn many_loc_links<T: pallet::Config>(requester: &T::AccountId) -> Vec<LocLinkParamsOf<T>> {
	let mut links: Vec<LocLinkParamsOf<T>> = Vec::new();
	for i in 0..MANY_ITEMS {
		links.push(loc_link::<T>(i, requester));
	}
	links
}

fn metadata_item<T: pallet::Config>(i: u32, submitter: &T::AccountId) -> MetadataItemParamsOf<T> {
	MetadataItemParams {
		name: T::Hasher::hash(&Vec::from([i as u8])),
		value: T::Hasher::hash(&Vec::from([i as u8])),
		submitter: SupportedAccountId::Polkadot(submitter.clone()),
	}
}

fn file<T: pallet::Config>(i: u32, submitter: &T::AccountId) -> FileParamsOf<T> {
	FileParams {
		hash: T::Hasher::hash(&Vec::from([i as u8])),
		nature: T::Hasher::hash(&Vec::from([i as u8])),
		submitter: SupportedAccountId::Polkadot(submitter.clone()),
		size: 0,
	}
}

fn loc_link<T: pallet::Config>(i: u32, submitter: &T::AccountId) -> LocLinkParamsOf<T> {
	LocLinkParams {
		id: T::LocIdFactory::loc_id(i),
		nature: T::Hasher::hash(&Vec::from([i as u8])),
		submitter: SupportedAccountId::Polkadot(submitter.clone()),
	}
}

fn create_locs_to_link_to<T: pallet::Config>(requester: &T::AccountId) {
	let legal_officer_id = any_legal_officer::<T>();
	for i in 0..MANY_ITEMS {
		create_loc::<T>(T::LocIdFactory::loc_id(i), &legal_officer_id, requester);
	}
}

fn create_loc<T: pallet::Config>(loc_id: T::LocId, legal_officer_id: &T::AccountId, requester: &T::AccountId) {
	assert_ok!(LogionLoc::<T>::create_polkadot_transaction_loc(
		<T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(requester.clone())),
		loc_id,
		legal_officer_id.clone(),
		0u32.into(),
		ItemsParams {
			metadata: Vec::new(),
			files: Vec::new(),
			links: Vec::new(),
		},
	));
}

fn create_closed_polkadot_identity_loc<T: pallet::Config>(loc_id: T::LocId, legal_officer_id: &T::AccountId, requester: &T::AccountId) {
	assert_ok!(LogionLoc::<T>::create_polkadot_identity_loc(
		<T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(requester.clone())),
		loc_id,
		legal_officer_id.clone(),
		0u32.into(),
		ItemsParams {
			metadata: Vec::new(),
			files: Vec::new(),
			links: Vec::new(),
		},
	));
	assert_ok!(LogionLoc::<T>::close(
		<T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(legal_officer_id.clone())),
		loc_id,
		None,
		false,
	));
}

const MANY_ITEMS: u32 = 10;
const NEXT_LOC_ID: u32 = MANY_ITEMS;
const REQUESTER_IDENTITY_LOC_ID: u32 = NEXT_LOC_ID + 2;
const INVITED_CONTRIBUTOR_IDENTITY_LOC_ID: u32 = NEXT_LOC_ID + 3;

fn setup_empty_loc<T: pallet::Config>() -> (T::LocId, T::AccountId) {
	let legal_officer_id = any_legal_officer::<T>();
	let requester: T::AccountId = account("requester", 1, SEED);
	create_closed_polkadot_identity_loc::<T>(T::LocIdFactory::loc_id(REQUESTER_IDENTITY_LOC_ID), &legal_officer_id, &requester);
	ensure_enough_funds::<T>(&requester);
	let loc_id: T::LocId = T::LocIdFactory::loc_id(0);
	create_loc::<T>(loc_id, &legal_officer_id, &requester);
	(loc_id, requester)
}

fn add_many_metadata<T: pallet::Config>(loc_id: &T::LocId, requester: &T::AccountId) {
	for i in 0..MANY_ITEMS {
		assert_ok!(LogionLoc::<T>::add_metadata(
			<T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(requester.clone())),
			*loc_id,
			metadata_item::<T>(i, requester),
		));
	}
}

fn add_many_files<T: pallet::Config>(loc_id: &T::LocId, requester: &T::AccountId) {
	for i in 0..MANY_ITEMS {
		assert_ok!(LogionLoc::<T>::add_file(
			<T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(requester.clone())),
			*loc_id,
			file::<T>(i, requester),
		));
	}
}

fn add_many_links<T: pallet::Config>(loc_id: &T::LocId, requester: &T::AccountId) {
	for i in 0..MANY_ITEMS {
		assert_ok!(LogionLoc::<T>::add_link(
			<T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(requester.clone())),
			*loc_id,
			loc_link::<T>(i, requester),
		));
	}
}

fn max_tokens_record_files<T: pallet::Config>() -> Vec<TokensRecordFileOf<T>> {
	let mut files = Vec::with_capacity(T::MaxTokensRecordFiles::get().try_into().unwrap());
	for i in 0..files.capacity() {
		files.push(TokensRecordFile {
			name: T::Hasher::hash(&Vec::from([i as u8])),
			content_type: T::Hasher::hash(&Vec::from([i as u8])),
			size: 0,
			hash: T::Hasher::hash(&Vec::from([i as u8])),
		});
	}
	files
}
