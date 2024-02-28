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
	// * LOC with max files, metadata and links.
	#[benchmark]
	fn create_polkadot_identity_loc() -> Result<(), BenchmarkError> {
		let legal_officer_id = any_legal_officer::<T>();
		let requester: T::AccountId = account("requester", 1, SEED);
		create_closed_polkadot_identity_loc::<T>(T::LocIdFactory::loc_id(requester_identity_loc::<T>()), &legal_officer_id, &requester);
		ensure_enough_funds::<T>(&requester);
		let items = max_items::<T>(&requester);
		let loc_id: T::LocId = T::LocIdFactory::loc_id(next_loc_id::<T>());

		#[extrinsic_call]
		_(
			RawOrigin::Signed(requester),
			loc_id,
			legal_officer_id,
			0u32.into(),
			items,
		);

		assert!(LogionLoc::<T>::loc(loc_id).is_some());

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
	// * LOC with max files, metadata and links.
	#[benchmark]
	fn create_polkadot_transaction_loc() -> Result<(), BenchmarkError> {
		let legal_officer_id = any_legal_officer::<T>();
		let requester: T::AccountId = account("requester", 1, SEED);
		create_closed_polkadot_identity_loc::<T>(T::LocIdFactory::loc_id(requester_identity_loc::<T>()), &legal_officer_id, &requester);
		ensure_enough_funds::<T>(&requester);
		let items = max_items::<T>(&requester);
		let next_loc_id: T::LocId = T::LocIdFactory::loc_id(next_loc_id::<T>());

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
	// * LOC with max files, metadata and links.
	#[benchmark]
	fn create_collection_loc() -> Result<(), BenchmarkError> {
		let legal_officer_id = any_legal_officer::<T>();
		let requester: T::AccountId = account("requester", 1, SEED);
		create_closed_polkadot_identity_loc::<T>(T::LocIdFactory::loc_id(requester_identity_loc::<T>()), &legal_officer_id, &requester);
		ensure_enough_funds::<T>(&requester);
		let items = max_items::<T>(&requester);
		let next_loc_id: T::LocId = T::LocIdFactory::loc_id(next_loc_id::<T>());

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
	// * LOC has already max metadata items
	#[benchmark]
	fn add_metadata() -> Result<(), BenchmarkError> {
		let (loc_id, requester) = setup_empty_loc::<T>();
		add_many_metadata::<T>(&loc_id, &requester, 1);

		#[extrinsic_call]
		_(
			RawOrigin::Signed(requester.clone()),
			loc_id,
			metadata_item::<T>(T::MaxLocMetadata::get() - 1, &requester),
		);

		Ok(())
	}

	// Benchmark `add_file` extrinsic with the worst possible conditions:
	// * LOC has already max files
	#[benchmark]
	fn add_file() -> Result<(), BenchmarkError> {
		let (loc_id, requester) = setup_empty_loc::<T>();
		add_many_files::<T>(&loc_id, &requester, 1);

		#[extrinsic_call]
		_(
			RawOrigin::Signed(requester.clone()),
			loc_id,
			file::<T>(T::MaxLocFiles::get() - 1, &requester),
		);

		Ok(())
	}

	// Benchmark `add_link` extrinsic with the worst possible conditions:
	// * LOC has already max files
	#[benchmark]
	fn add_link() -> Result<(), BenchmarkError> {
		let legal_officer_id = any_legal_officer::<T>();
		let requester: T::AccountId = account("requester", 1, SEED);
		create_closed_polkadot_identity_loc::<T>(T::LocIdFactory::loc_id(requester_identity_loc::<T>()), &legal_officer_id, &requester);
		ensure_enough_funds::<T>(&requester);
		create_locs_to_link_to::<T>(&requester); // Targets of the many links
		create_loc::<T>(T::LocIdFactory::loc_id(T::MaxLocLinks::get()), &legal_officer_id, &requester); // New target

		let loc_id: T::LocId = T::LocIdFactory::loc_id(T::MaxLocLinks::get() + 1);
		create_loc::<T>(loc_id, &legal_officer_id, &requester);
		add_many_links::<T>(&loc_id, &requester, 1);

		#[extrinsic_call]
		_(
			RawOrigin::Signed(requester.clone()),
			loc_id,
			loc_link::<T>(T::MaxLocLinks::get() - 1, &requester),
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
	// * Max number of files
	// * Max number of T&C elements
	#[benchmark]
	fn add_collection_item() -> Result<(), BenchmarkError> {
		let legal_officer_id = any_legal_officer::<T>();
		let requester: T::AccountId = account("requester", 1, SEED);
		create_closed_polkadot_identity_loc::<T>(T::LocIdFactory::loc_id(requester_identity_loc::<T>()), &legal_officer_id, &requester);
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
			max_item_files::<T>(),
			None,
			false,
			max_item_tcs::<T>(),
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
		create_closed_polkadot_identity_loc::<T>(T::LocIdFactory::loc_id(requester_identity_loc::<T>()), &legal_officer_id, &requester);
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
	// * LOC with max metadata items.
	// * Acknowledge last item.
	#[benchmark]
	fn acknowledge_metadata() -> Result<(), BenchmarkError> {
		let legal_officer_id = any_legal_officer::<T>();
		let requester: T::AccountId = account("requester", 1, SEED);
		create_closed_polkadot_identity_loc::<T>(T::LocIdFactory::loc_id(requester_identity_loc::<T>()), &legal_officer_id, &requester);
		ensure_enough_funds::<T>(&requester);

		let loc_id: T::LocId = T::LocIdFactory::loc_id(0);
		let items = ItemsParams {
			metadata: max_metadata::<T>(&requester),
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
			T::Hasher::hash(&Vec::from([(T::MaxLocMetadata::get() - 1) as u8])),
		);

		Ok(())
	}

	// Benchmark `acknowledge_file` extrinsic with the worst possible conditions:
	// * LOC with max files.
	// * Acknowledge last item.
	#[benchmark]
	fn acknowledge_file() -> Result<(), BenchmarkError> {
		let legal_officer_id = any_legal_officer::<T>();
		let requester: T::AccountId = account("requester", 1, SEED);
		create_closed_polkadot_identity_loc::<T>(T::LocIdFactory::loc_id(requester_identity_loc::<T>()), &legal_officer_id, &requester);
		ensure_enough_funds::<T>(&requester);

		let loc_id: T::LocId = T::LocIdFactory::loc_id(0);
		let items = ItemsParams {
			metadata: Vec::new(),
			files: max_files::<T>(&requester),
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
			T::Hasher::hash(&Vec::from([(T::MaxLocFiles::get() - 1) as u8])),
		);

		Ok(())
	}

	// Benchmark `acknowledge_link` extrinsic with the worst possible conditions:
	// * LOC with max links.
	// * Acknowledge last item.
	#[benchmark]
	fn acknowledge_link() -> Result<(), BenchmarkError> {
		let legal_officer_id = any_legal_officer::<T>();
		let requester: T::AccountId = account("requester", 1, SEED);
		create_closed_polkadot_identity_loc::<T>(T::LocIdFactory::loc_id(requester_identity_loc::<T>()), &legal_officer_id, &requester);
		ensure_enough_funds::<T>(&requester);
		create_locs_to_link_to::<T>(&requester); // Targets of the many links

		let loc_id: T::LocId = T::LocIdFactory::loc_id(T::MaxLocLinks::get());
		let items = ItemsParams {
			metadata: Vec::new(),
			files: Vec::new(),
			links: max_loc_links::<T>(&requester),
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
			T::LocIdFactory::loc_id(T::MaxLocLinks::get() - 1),
		);

		Ok(())
	}

	// Benchmark `close` extrinsic with the worst possible conditions:
	// * LOC with max items.
	// * With auto-ack.
	#[benchmark]
	fn close() -> Result<(), BenchmarkError> {
		let legal_officer_id = any_legal_officer::<T>();
		let requester: T::AccountId = account("requester", 1, SEED);
		create_closed_polkadot_identity_loc::<T>(T::LocIdFactory::loc_id(requester_identity_loc::<T>()), &legal_officer_id, &requester);
		ensure_enough_funds::<T>(&requester);
		let items = max_items::<T>(&requester);
		let next_loc_id: T::LocId = T::LocIdFactory::loc_id(next_loc_id::<T>());

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
		create_closed_polkadot_identity_loc::<T>(T::LocIdFactory::loc_id(requester_identity_loc::<T>()), &legal_officer_id, &requester);

		ensure_enough_funds::<T>(&requester);

		let invited_contributor: T::AccountId = account("invited_contributor", 1, SEED);
		create_closed_polkadot_identity_loc::<T>(T::LocIdFactory::loc_id(invited_contributor_identity_loc::<T>()), &legal_officer_id, &invited_contributor);

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

	// Benchmark `import_loc` extrinsic with the worst possible conditions:
	// * Requester is Polkadot address -> AccountLocsMap is updated
	// * LOC with max files, metadata and links.
	#[benchmark]
	fn import_loc() -> Result<(), BenchmarkError> {
		let legal_officer_id = any_legal_officer::<T>();
		let requester_account: T::AccountId = account("requester", 1, SEED);
		let requester = Account(requester_account.clone());
		let items = many_items_import::<T>(&requester_account);
		let loc_id: T::LocId = T::LocIdFactory::loc_id(0);

		#[extrinsic_call]
		_(
			RawOrigin::Root,
			loc_id,
			requester,
			legal_officer_id,
			LocType::Transaction,
			items,
			None,
			None,
			false,
			0u32.into(),
			0u32.into(),
			0u32.into(),
			0u32.into(),
			None,
			None,
			None,
			None,
			false,
		);

		assert!(LogionLoc::<T>::loc(loc_id).is_some());

		Ok(())
	}

	// Benchmark `import_collection_item` extrinsic with the worst possible conditions:
	// * Max number of files
	// * Max number of T&C elements
	#[benchmark]
	fn import_collection_item() -> Result<(), BenchmarkError> {
		let loc_id: T::LocId = T::LocIdFactory::loc_id(0);
		let item_id: T::CollectionItemId = T::CollectionItemIdFactory::collection_item_id(0);
		let item_description = T::Hasher::hash(&Vec::from([0u8]));

		#[extrinsic_call]
		_(
			RawOrigin::Root,
			loc_id,
			item_id,
			item_description,
			max_item_files::<T>(),
			None,
			false,
			max_item_tcs::<T>(),
		);

		Ok(())
	}

	// Benchmark `import_tokens_record` extrinsic with the worst possible conditions:
	// * Max files
	#[benchmark]
	fn import_tokens_record() -> Result<(), BenchmarkError> {
		let loc_id: T::LocId = T::LocIdFactory::loc_id(0);
		let record_id: T::TokensRecordId = T::TokensRecordIdFactory::tokens_record_id(0);
		let description = T::Hasher::hash(&Vec::from([0u8]));
		let requester: T::AccountId = account("requester", 1, SEED);

		#[extrinsic_call]
		_(
			RawOrigin::Root,
			loc_id,
			record_id,
			description,
			max_tokens_record_files::<T>(),
			requester,
		);

		Ok(())
	}

	// Benchmark `import_invited_contributor_selection` extrinsic.
	#[benchmark]
	fn import_invited_contributor_selection() -> Result<(), BenchmarkError> {
		let loc_id: T::LocId = T::LocIdFactory::loc_id(0);
		let invited_contributor: T::AccountId = account("invited_contributor", 1, SEED);

		#[extrinsic_call]
		_(
			RawOrigin::Root,
			loc_id,
			invited_contributor.clone(),
		);

		Ok(())
	}

	// Benchmark `import_verified_issuer` extrinsic.
	#[benchmark]
	fn import_verified_issuer() -> Result<(), BenchmarkError> {
		let legal_officer_id = any_legal_officer::<T>();
		let requester: T::AccountId = account("requester", 1, SEED);
		let loc_id: T::LocId = T::LocIdFactory::loc_id(0);

		#[extrinsic_call]
		_(
			RawOrigin::Root,
			legal_officer_id,
			requester.clone(),
			loc_id,
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

fn max_items<T: pallet::Config>(requester: &T::AccountId) -> ItemsParamsOf<T> {
	create_locs_to_link_to::<T>(requester);

	let metadata = max_metadata::<T>(requester);
	let files = max_files::<T>(requester);
	let links = max_loc_links::<T>(requester);
	ItemsParams {
		metadata,
		files,
		links,
	}
}

fn max_metadata<T: pallet::Config>(requester: &T::AccountId) -> Vec<MetadataItemParamsOf<T>> {
	let mut metadata: Vec<MetadataItemParamsOf<T>> = Vec::new();
	for i in 0..T::MaxLocMetadata::get() {
		metadata.push(metadata_item::<T>(i, requester));
	}
	metadata
}

fn max_files<T: pallet::Config>(requester: &T::AccountId) -> Vec<FileParamsOf<T>> {
	let mut files: Vec<FileParamsOf<T>> = Vec::new();
	for i in 0..T::MaxLocFiles::get() {
		files.push(file::<T>(i, requester));
	}
	files
}

fn max_loc_links<T: pallet::Config>(requester: &T::AccountId) -> Vec<LocLinkParamsOf<T>> {
	let mut links: Vec<LocLinkParamsOf<T>> = Vec::new();
	for i in 0..T::MaxLocLinks::get() {
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
	for i in 0..T::MaxLocLinks::get() {
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

fn next_loc_id<T: pallet::Config>() -> u32 {
	T::MaxLocLinks::get()
}

fn requester_identity_loc<T: pallet::Config>() -> u32 {
	next_loc_id::<T>() + 2
}

fn invited_contributor_identity_loc<T: pallet::Config>() -> u32 {
	next_loc_id::<T>() + 3
}

fn setup_empty_loc<T: pallet::Config>() -> (T::LocId, T::AccountId) {
	let legal_officer_id = any_legal_officer::<T>();
	let requester: T::AccountId = account("requester", 1, SEED);
	create_closed_polkadot_identity_loc::<T>(T::LocIdFactory::loc_id(requester_identity_loc::<T>()), &legal_officer_id, &requester);
	ensure_enough_funds::<T>(&requester);
	let loc_id: T::LocId = T::LocIdFactory::loc_id(0);
	create_loc::<T>(loc_id, &legal_officer_id, &requester);
	(loc_id, requester)
}

fn add_many_metadata<T: pallet::Config>(loc_id: &T::LocId, requester: &T::AccountId, reserve: u32) {
	for i in 0..T::MaxLocMetadata::get() - reserve {
		assert_ok!(LogionLoc::<T>::add_metadata(
			<T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(requester.clone())),
			*loc_id,
			metadata_item::<T>(i, requester),
		));
	}
}

fn add_many_files<T: pallet::Config>(loc_id: &T::LocId, requester: &T::AccountId, reserve: u32) {
	for i in 0..T::MaxLocFiles::get() - reserve {
		assert_ok!(LogionLoc::<T>::add_file(
			<T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(requester.clone())),
			*loc_id,
			file::<T>(i, requester),
		));
	}
}

fn add_many_links<T: pallet::Config>(loc_id: &T::LocId, requester: &T::AccountId, reserve: u32) {
	for i in 0..T::MaxLocLinks::get() - reserve {
		assert_ok!(LogionLoc::<T>::add_link(
			<T as frame_system::Config>::RuntimeOrigin::from(RawOrigin::Signed(requester.clone())),
			*loc_id,
			loc_link::<T>(i, requester),
		));
	}
}

fn max_item_files<T: pallet::Config>() -> Vec<CollectionItemFileOf<T>> {
	let mut files = Vec::with_capacity(T::MaxCollectionItemFiles::get().try_into().unwrap());
	for i in 0..files.capacity() {
		files.push(CollectionItemFile {
			name: T::Hasher::hash(&Vec::from([i as u8])),
			content_type: T::Hasher::hash(&Vec::from([i as u8])),
			size: 0,
			hash: T::Hasher::hash(&Vec::from([i as u8])),
		});
	}
	files
}

fn max_item_tcs<T: pallet::Config>() -> Vec<TermsAndConditionsElementOf<T>> {
	let mut tcs = Vec::with_capacity(T::MaxCollectionItemTCs::get().try_into().unwrap());
	let tc_type = T::Hasher::hash(&Vec::from([0]));
	let tc_loc = T::LocIdFactory::loc_id(0u32);
	for i in 0..tcs.capacity() {
		tcs.push(TermsAndConditionsElement {
			tc_type,
			tc_loc,
			details: T::Hasher::hash(&Vec::from([i as u8])),
		});
	}
	tcs
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

fn many_items_import<T: pallet::Config>(requester: &T::AccountId) -> ItemsOf<T> {
	let metadata = many_metadata_import::<T>(requester);
	let files = many_files_import::<T>(requester);
	let links = many_loc_links_import::<T>(requester);
	Items {
		metadata,
		files,
		links,
	}
}

fn many_metadata_import<T: pallet::Config>(requester: &T::AccountId) -> Vec<MetadataItemOf<T>> {
	let mut metadata: Vec<MetadataItemOf<T>> = Vec::new();
	for i in 0..T::MaxLocMetadata::get() {
		metadata.push(metadata_item_import::<T>(i, requester));
	}
	metadata
}

fn metadata_item_import<T: pallet::Config>(i: u32, submitter: &T::AccountId) -> MetadataItemOf<T> {
	MetadataItem {
		name: T::Hasher::hash(&Vec::from([i as u8])),
		value: T::Hasher::hash(&Vec::from([i as u8])),
		submitter: SupportedAccountId::Polkadot(submitter.clone()),
		acknowledged_by_owner: false,
		acknowledged_by_verified_issuer: false,
	}
}

fn many_files_import<T: pallet::Config>(requester: &T::AccountId) -> Vec<FileOf<T>> {
	let mut files: Vec<FileOf<T>> = Vec::new();
	for i in 0..T::MaxLocFiles::get() {
		files.push(file_import::<T>(i, requester));
	}
	files
}

fn file_import<T: pallet::Config>(i: u32, submitter: &T::AccountId) -> FileOf<T> {
	File {
		hash: T::Hasher::hash(&Vec::from([i as u8])),
		nature: T::Hasher::hash(&Vec::from([i as u8])),
		submitter: SupportedAccountId::Polkadot(submitter.clone()),
		size: 0,
		acknowledged_by_owner: false,
		acknowledged_by_verified_issuer: false,
	}
}

fn many_loc_links_import<T: pallet::Config>(requester: &T::AccountId) -> Vec<LocLinkOf<T>> {
	let mut links: Vec<LocLinkOf<T>> = Vec::new();
	for i in 0..T::MaxLocLinks::get() {
		links.push(loc_link_import::<T>(i, requester));
	}
	links
}

fn loc_link_import<T: pallet::Config>(i: u32, submitter: &T::AccountId) -> LocLinkOf<T> {
	LocLink {
		id: T::LocIdFactory::loc_id(i),
		nature: T::Hasher::hash(&Vec::from([i as u8])),
		submitter: SupportedAccountId::Polkadot(submitter.clone()),
		acknowledged_by_owner: false,
		acknowledged_by_verified_issuer: false,
	}
}
