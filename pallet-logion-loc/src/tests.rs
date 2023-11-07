use core::str::FromStr;
use frame_support::{assert_err, assert_ok};
use frame_support::traits::Len;
use sp_core::{H256, H160};
use sp_runtime::traits::BlakeTwo256;
use sp_runtime::traits::Hash;

use logion_shared::{Beneficiary, LocQuery, LocValidity};

use crate::{LocLink, ItemsParams};
use crate::{
    Error, File, LegalOfficerCase, LocType, MetadataItem, CollectionItem, CollectionItemFile,
    CollectionItemToken, mock::*, TermsAndConditionsElement, TokensRecordFile,
    VerifiedIssuer, OtherAccountId, SupportedAccountId, MetadataItemParams, FileParams, Hasher,
    Requester::{Account, OtherAccount}, fees::*, Config, TokensRecordFileOf, LocLinkParams,
};

const LOC_ID: u32 = 0;
const OTHER_LOC_ID: u32 = 1;
const LOGION_CLASSIFICATION_LOC_ID: u32 = 2;
const ADDITIONAL_TC_LOC_ID: u32 = 3;
const ISSUER1_IDENTITY_LOC_ID: u32 = 4;
const ISSUER2_IDENTITY_LOC_ID: u32 = 5;
const FILE_SIZE: u32 = 90;
const ONE_LGNT: Balance = 1_000_000_000_000_000_000;
const INITIAL_BALANCE: Balance = (3 * 2000 * ONE_LGNT) + ONE_LGNT;
const INSUFFICIENT_BALANCE: Balance = 99;
const ACKNOWLEDGED: bool = true;
const NOT_ACKNOWLEDGED: bool = !ACKNOWLEDGED;
const TOKENS_RECORD_FEE: Balance = 2 * ONE_LGNT;

const EXCHANGE_RATE: Balance = 200_000_000_000_000_000; // 1 euro cent = 0.2 LGNT
const ID_LOC_DEFAULT_LEGAL_FEE: Balance = EXCHANGE_RATE * 8_00;
const OTHER_LOC_DEFAULT_LEGAL_FEE: Balance = EXCHANGE_RATE * 100_00;

#[test]
fn it_creates_loc_with_default_legal_fee() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let snapshot = BalancesSnapshot::take(LOC_REQUESTER_ID, LOC_OWNER1);
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        assert_eq!(LogionLoc::loc(LOC_ID), Some(LegalOfficerCase {
            owner: LOC_OWNER1,
            requester: LOC_REQUESTER,
            metadata: vec![],
            files: vec![],
            closed: false,
            loc_type: LocType::Transaction,
            links: vec![],
            void_info: None,
            replacer_of: None,
            collection_last_block_submission: None,
            collection_max_size: None,
            collection_can_upload: false,
            seal: None,
            sponsorship_id: None,
            value_fee: 0,
            legal_fee: OTHER_LOC_DEFAULT_LEGAL_FEE,
            collection_item_fee: 0,
            tokens_record_fee: 0,
        }));

        let fees = Fees::only_legal(2000 * ONE_LGNT, Beneficiary::LegalOfficer(LOC_OWNER1));
        fees.assert_balances_events(snapshot);
    });
}

fn setup_default_balances() {
    set_balance(LOC_REQUESTER_ID, INITIAL_BALANCE);
    set_balance(SPONSOR_ID, INITIAL_BALANCE);
    set_balance(LOC_OWNER1, INITIAL_BALANCE);
    set_balance(LOC_OWNER2, INITIAL_BALANCE);
    set_balance(ISSUER_ID1, INITIAL_BALANCE);
    set_balance(ISSUER_ID2, INITIAL_BALANCE);
    set_balance(LOGION_TREASURY_ACCOUNT_ID, INITIAL_BALANCE);
}

fn set_balance(account_id: AccountId, amount: Balance) {
    assert_ok!(Balances::force_set_balance(RuntimeOrigin::root(), account_id, amount));
}

#[test]
fn it_creates_loc_with_custom_legal_fee() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let snapshot = BalancesSnapshot::take(LOC_REQUESTER_ID, LOC_OWNER1);
        let custom_legal_fee = 1000 * ONE_LGNT;
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, custom_legal_fee, ItemsParams::empty()));
        assert_eq!(LogionLoc::loc(LOC_ID), Some(LegalOfficerCase {
            owner: LOC_OWNER1,
            requester: LOC_REQUESTER,
            metadata: vec![],
            files: vec![],
            closed: false,
            loc_type: LocType::Transaction,
            links: vec![],
            void_info: None,
            replacer_of: None,
            collection_last_block_submission: None,
            collection_max_size: None,
            collection_can_upload: false,
            seal: None,
            sponsorship_id: None,
            value_fee: 0,
            legal_fee: custom_legal_fee,
            collection_item_fee: 0,
            tokens_record_fee: 0,
        }));

        let fees = Fees::only_legal(custom_legal_fee, Beneficiary::LegalOfficer(LOC_OWNER1));
        fees.assert_balances_events(snapshot);
    });
}

#[test]
fn it_makes_existing_loc_void() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        assert_ok!(LogionLoc::make_void(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID));

        let void_info = LogionLoc::loc(LOC_ID).unwrap().void_info;
        assert!(void_info.is_some());
        assert!(!void_info.unwrap().replacer.is_some());
    });
}

#[test]
fn it_makes_existing_loc_void_and_replace_it() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        create_closed_loc();

        const REPLACER_LOC_ID: u32 = OTHER_LOC_ID;
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), REPLACER_LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));

        assert_ok!(LogionLoc::make_void_and_replace(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, REPLACER_LOC_ID));

        let void_info = LogionLoc::loc(LOC_ID).unwrap().void_info;
        assert!(void_info.is_some());
        let replacer: Option<u32> = void_info.unwrap().replacer;
        assert!(replacer.is_some());
        assert_eq!(replacer.unwrap(), REPLACER_LOC_ID);

        let replacer_loc = LogionLoc::loc(REPLACER_LOC_ID).unwrap();
        assert!(replacer_loc.replacer_of.is_some());
        assert_eq!(replacer_loc.replacer_of.unwrap(), LOC_ID)
    });
}

#[test]
fn it_fails_making_existing_loc_void_for_unauthorized_caller() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        assert_err!(LogionLoc::make_void(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID), Error::<Test>::Unauthorized);
        let void_info = LogionLoc::loc(LOC_ID).unwrap().void_info;
        assert!(!void_info.is_some());
    });
}

#[test]
fn it_fails_making_existing_loc_void_for_already_void_loc() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        assert_ok!(LogionLoc::make_void(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID));
        assert_err!(LogionLoc::make_void(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID), Error::<Test>::AlreadyVoid);
    });
}

#[test]
fn it_fails_replacing_with_non_existent_loc() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        assert_err!(LogionLoc::make_void_and_replace(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, OTHER_LOC_ID), Error::<Test>::ReplacerLocNotFound);
    });
}

#[test]
fn it_fails_replacing_with_void_loc() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        const REPLACER_LOC_ID: u32 = OTHER_LOC_ID;
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), OTHER_LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        assert_ok!(LogionLoc::make_void(RuntimeOrigin::signed(LOC_OWNER1), OTHER_LOC_ID));
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        assert_err!(LogionLoc::make_void_and_replace(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, REPLACER_LOC_ID), Error::<Test>::ReplacerLocAlreadyVoid);
    });
}

#[test]
fn it_fails_replacing_with_loc_already_replacing_another_loc() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        const REPLACER_LOC_ID: u32 = 2;
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), OTHER_LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), REPLACER_LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        assert_ok!(LogionLoc::make_void_and_replace(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, REPLACER_LOC_ID));
        assert_err!(LogionLoc::make_void_and_replace(RuntimeOrigin::signed(LOC_OWNER1), OTHER_LOC_ID, REPLACER_LOC_ID), Error::<Test>::ReplacerLocAlreadyReplacing);
    });
}

#[test]
fn it_fails_replacing_with_wrongly_typed_loc() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        const REPLACER_LOC_ID: u32 = 2;
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        assert_ok!(LogionLoc::create_polkadot_identity_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), REPLACER_LOC_ID, LOC_OWNER1, ID_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        assert_err!(LogionLoc::make_void_and_replace(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, REPLACER_LOC_ID), Error::<Test>::ReplacerLocWrongType);
    });
}

#[test]
fn it_adds_metadata_when_caller_and_submitter_is_owner() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        let metadata = MetadataItemParams {
            name: sha256(&vec![1, 2, 3]),
            value: sha256(&vec![4, 5, 6]),
            submitter: SupportedAccountId::Polkadot(LOC_OWNER1),
        };
        assert_ok!(LogionLoc::add_metadata(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, metadata.clone()));
        let loc = LogionLoc::loc(LOC_ID).unwrap();
        assert_eq!(loc.metadata[0], expected_metadata(metadata, ACKNOWLEDGED, NOT_ACKNOWLEDGED));
    });
}

fn sha256(data: &Vec<u8>) -> H256 {
    <SHA256 as Hasher<H256>>::hash(data)
}

fn expected_metadata(metadata: MetadataItemParams<AccountId, EthereumAddress, crate::mock::Hash>, acknowledged_by_owner: bool, acknowledged_by_verified_issuer: bool) -> MetadataItem<AccountId, EthereumAddress, crate::mock::Hash> {
    return MetadataItem {
        name: metadata.name,
        value: metadata.value,
        submitter: metadata.submitter,
        acknowledged_by_owner,
        acknowledged_by_verified_issuer,
    };
}

#[test]
fn it_adds_metadata_when_caller_is_requester_and_submitter_is_requester() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        let metadata = MetadataItemParams {
            name: sha256(&vec![1, 2, 3]),
            value: sha256(&vec![4, 5, 6]),
            submitter: SupportedAccountId::Polkadot(LOC_REQUESTER_ID),
        };
        assert_ok!(LogionLoc::add_metadata(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, metadata.clone()));
        let loc = LogionLoc::loc(LOC_ID).unwrap();
        assert_eq!(loc.metadata[0], expected_metadata(metadata, NOT_ACKNOWLEDGED, NOT_ACKNOWLEDGED));
    });
}

#[test]
fn it_adds_metadata_when_caller_is_requester() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        let metadata = MetadataItemParams {
            name: sha256(&vec![1, 2, 3]),
            value: sha256(&vec![4, 5, 6]),
            submitter: SupportedAccountId::Polkadot(LOC_REQUESTER_ID),
        };
        assert_ok!(LogionLoc::add_metadata(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, metadata.clone()));
        let loc = LogionLoc::loc(LOC_ID).unwrap();
        assert_eq!(loc.metadata[0], expected_metadata(metadata, NOT_ACKNOWLEDGED, NOT_ACKNOWLEDGED));
    });
}

#[test]
fn it_acknowledges_metadata_as_owner() {
    new_test_ext().execute_with(|| {
        let metadata = create_loc_with_metadata_from_requester();
        assert_ok!(LogionLoc::acknowledge_metadata(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, metadata.name.clone()));
        let loc = LogionLoc::loc(LOC_ID).unwrap();
        assert_eq!(loc.metadata[0], expected_metadata(metadata.clone(), ACKNOWLEDGED, NOT_ACKNOWLEDGED));
    });
}

#[test]
fn it_acknowledges_metadata_as_verified_issuer() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        create_collection_and_nominated_issuer();
        assert_ok!(LogionLoc::set_issuer_selection(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, ISSUER_ID1, true));
        let metadata = MetadataItemParams {
            name: sha256(&vec![1, 2, 3]),
            value: sha256(&vec![4, 5, 6]),
            submitter: SupportedAccountId::Polkadot(ISSUER_ID1),
        };
        assert_ok!(LogionLoc::add_metadata(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, metadata.clone()));

        assert_ok!(LogionLoc::acknowledge_metadata(RuntimeOrigin::signed(ISSUER_ID1), LOC_ID, metadata.name.clone()));

        let loc = LogionLoc::loc(LOC_ID).unwrap();
        assert_eq!(loc.metadata[0], expected_metadata(metadata.clone(), NOT_ACKNOWLEDGED, ACKNOWLEDGED));
    });
}

#[test]
fn it_fails_to_close_with_metadata_unacknowledged_by_verified_issuer() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        create_collection_and_nominated_issuer();
        assert_ok!(LogionLoc::set_issuer_selection(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, ISSUER_ID1, true));
        let metadata = MetadataItemParams {
            name: sha256(&vec![1, 2, 3]),
            value: sha256(&vec![4, 5, 6]),
            submitter: SupportedAccountId::Polkadot(ISSUER_ID1),
        };
        assert_ok!(LogionLoc::add_metadata(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, metadata.clone()));

        assert_err!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, None, true), Error::<Test>::CannotCloseUnacknowledgedByVerifiedIssuer);

        let loc = LogionLoc::loc(LOC_ID).unwrap();
        assert!(!loc.closed);
        assert_eq!(loc.metadata[0], expected_metadata(metadata.clone(), NOT_ACKNOWLEDGED, NOT_ACKNOWLEDGED));
    });
}

#[test]
fn it_fails_to_acknowledge_requester_metadata_as_issuer() {
    new_test_ext().execute_with(|| {
        let metadata = create_loc_with_metadata_from_requester();
        assert_err!(LogionLoc::acknowledge_metadata(RuntimeOrigin::signed(ISSUER_ID1), LOC_ID, metadata.name.clone()), Error::<Test>::Unauthorized);
        let loc = LogionLoc::loc(LOC_ID).unwrap();
        assert_eq!(loc.metadata[0], expected_metadata(metadata.clone(), NOT_ACKNOWLEDGED, NOT_ACKNOWLEDGED));
    });
}

#[test]
fn it_fails_to_acknowledge_unknown_metadata() {
    new_test_ext().execute_with(|| {
        create_loc_with_metadata_from_requester();
        let name = sha256(&"unknown_metadata".as_bytes().to_vec());
        assert_err!(LogionLoc::acknowledge_metadata(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, name), Error::<Test>::ItemNotFound);
    });
}

#[test]
fn it_fails_to_acknowledge_already_acknowledged_metadata() {
    new_test_ext().execute_with(|| {
        let metadata = create_loc_with_metadata_from_requester();
        assert_ok!(LogionLoc::acknowledge_metadata(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, metadata.name.clone()));
        assert_err!(LogionLoc::acknowledge_metadata(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, metadata.name.clone()), Error::<Test>::ItemAlreadyAcknowledged);
    });
}

#[test]
fn it_fails_to_acknowledge_metadata_when_unauthorized_caller() {
    new_test_ext().execute_with(|| {
        let metadata = create_loc_with_metadata_from_requester();
        assert_err!(LogionLoc::acknowledge_metadata(RuntimeOrigin::signed(UNAUTHORIZED_CALLER), LOC_ID, metadata.name.clone()), Error::<Test>::Unauthorized);
    });
}

#[test]
fn it_fails_to_close_loc_with_unacknowledged_metadata() {
    new_test_ext().execute_with(|| {
        create_loc_with_metadata_from_requester();
        assert_err!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, None, false), Error::<Test>::CannotCloseUnacknowledgedByVerifiedIssuer);
    });
}

#[test]
fn it_closes_loc_and_acknowledges_metadata() {
    new_test_ext().execute_with(|| {
        let metadata = create_loc_with_metadata_from_requester();
        assert_ok!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, None, true));
        let loc = LogionLoc::loc(LOC_ID).unwrap();
        assert_eq!(loc.metadata[0], expected_metadata(metadata, ACKNOWLEDGED, NOT_ACKNOWLEDGED));
    });
}

#[test]
fn it_fails_to_acknowledge_metadata_when_loc_voided() {
    new_test_ext().execute_with(|| {
        let metadata = create_loc_with_metadata_from_requester();
        assert_ok!(LogionLoc::make_void(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID));
        assert_err!(LogionLoc::acknowledge_metadata(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, metadata.name.clone()), Error::<Test>::CannotMutateVoid);
    });
}

fn create_loc_with_metadata_from_requester() -> MetadataItemParams<AccountId, EthereumAddress, crate::mock::Hash> {
    setup_default_balances();
    assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
    let metadata = MetadataItemParams {
        name: sha256(&vec![1, 2, 3]),
        value: sha256(&vec![4, 5, 6]),
        submitter: SupportedAccountId::Polkadot(LOC_REQUESTER_ID),
    };
    assert_ok!(LogionLoc::add_metadata(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, metadata.clone()));
    let loc = LogionLoc::loc(LOC_ID).unwrap();
    assert_eq!(loc.metadata[0], expected_metadata(metadata.clone(), NOT_ACKNOWLEDGED, NOT_ACKNOWLEDGED));
    metadata
}

#[test]
fn it_fails_adding_metadata_for_unauthorized_caller() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        let metadata = MetadataItemParams {
            name: sha256(&vec![1, 2, 3]),
            value: sha256(&vec![4, 5, 6]),
            submitter: SupportedAccountId::Polkadot(LOC_OWNER1),
        };
        assert_err!(LogionLoc::add_metadata(RuntimeOrigin::signed(UNAUTHORIZED_CALLER), LOC_ID, metadata.clone()), Error::<Test>::Unauthorized);
    });
}

#[test]
fn it_fails_adding_metadata_when_closed() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        create_closed_loc();
        let metadata = MetadataItemParams {
            name: sha256(&vec![1, 2, 3]),
            value: sha256(&vec![4, 5, 6]),
            submitter: SupportedAccountId::Polkadot(LOC_OWNER1),
        };
        assert_err!(LogionLoc::add_metadata(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, metadata.clone()), Error::<Test>::CannotMutate);
    });
}

#[test]
fn it_fails_adding_metadata_when_invalid_submitter() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        let metadata = MetadataItemParams {
            name: sha256(&vec![1, 2, 3]),
            value: sha256(&vec![4, 5, 6]),
            submitter: SupportedAccountId::Polkadot(LOC_OWNER1),
        };
        assert_err!(LogionLoc::add_metadata(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, metadata.clone()), Error::<Test>::CannotSubmit);
    });
}

fn create_closed_loc() {
    assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
    assert_ok!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, None, false));
}

#[test]
fn it_adds_file_when_caller_owner_and_submitter_is_owner() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        let file = FileParams {
            hash: sha256(&"test".as_bytes().to_vec()),
            nature: sha256(&"test-file-nature".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_OWNER1),
            size: FILE_SIZE,
        };
        let snapshot = BalancesSnapshot::take(LOC_REQUESTER_ID, LOC_OWNER1);
        assert_ok!(LogionLoc::add_file(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, file.clone()));
        let loc = LogionLoc::loc(LOC_ID).unwrap();
        assert_eq!(loc.files[0], expected_file(&file, ACKNOWLEDGED, NOT_ACKNOWLEDGED));

        let fees = Fees::only_storage(1, file.size);
        fees.assert_balances_events(snapshot);
    });
}

fn expected_file(file: &FileParams<H256, AccountId, EthereumAddress>, acknowledged_by_owner: bool, acknowledged_by_verified_issuer: bool) -> File<H256, AccountId, EthereumAddress> {
    return File {
        hash: file.hash,
        nature: file.nature.clone(),
        submitter: file.submitter,
        size: file.size,
        acknowledged_by_owner,
        acknowledged_by_verified_issuer,
    }
}

#[test]
fn it_adds_file_when_caller_is_requester_and_submitter_is_requester() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        let file = FileParams {
            hash: sha256(&"test".as_bytes().to_vec()),
            nature: sha256(&"test-file-nature".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_REQUESTER_ID),
            size: FILE_SIZE,
        };
        let snapshot = BalancesSnapshot::take(LOC_REQUESTER_ID, LOC_OWNER1);
        assert_ok!(LogionLoc::add_file(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, file.clone()));
        let loc = LogionLoc::loc(LOC_ID).unwrap();
        assert_eq!(loc.files[0], expected_file(&file, NOT_ACKNOWLEDGED, NOT_ACKNOWLEDGED));
        let fees = Fees::only_storage(1, file.size);
        fees.assert_balances_events(snapshot);
    });
}

#[test]
fn it_adds_file_when_caller_is_requester() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        let file = FileParams {
            hash: sha256(&"test".as_bytes().to_vec()),
            nature: sha256(&"test-file-nature".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_REQUESTER_ID),
            size: FILE_SIZE,
        };
        let snapshot = BalancesSnapshot::take(LOC_REQUESTER_ID, LOC_OWNER1);
        assert_ok!(LogionLoc::add_file(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, file.clone()));
        let loc = LogionLoc::loc(LOC_ID).unwrap();
        assert_eq!(loc.files[0], expected_file(&file, NOT_ACKNOWLEDGED, NOT_ACKNOWLEDGED));
        let fees = Fees::only_storage(1, file.size);
        fees.assert_balances_events(snapshot);
    });
}

#[test]
fn it_acknowledges_file() {
    new_test_ext().execute_with(|| {
        let file = create_loc_with_file_from_requester();
        assert_ok!(LogionLoc::acknowledge_file(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, file.hash.clone()));
        let loc = LogionLoc::loc(LOC_ID).unwrap();
        assert_eq!(loc.files[0], expected_file(&file, ACKNOWLEDGED, NOT_ACKNOWLEDGED));
    });
}

#[test]
fn it_acknowledges_file_as_verified_issuer() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        create_collection_and_nominated_issuer();
        assert_ok!(LogionLoc::set_issuer_selection(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, ISSUER_ID1, true));
        let file = FileParams {
            hash: sha256(&vec![1, 2, 3]),
            nature: sha256(&vec![4, 5, 6]),
            submitter: SupportedAccountId::Polkadot(ISSUER_ID1),
            size: 456,
        };
        assert_ok!(LogionLoc::add_file(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, file.clone()));

        assert_ok!(LogionLoc::acknowledge_file(RuntimeOrigin::signed(ISSUER_ID1), LOC_ID, file.hash.clone()));

        let loc = LogionLoc::loc(LOC_ID).unwrap();
        assert_eq!(loc.files[0], expected_file(&file, NOT_ACKNOWLEDGED, ACKNOWLEDGED));
    });
}

#[test]
fn it_fails_to_close_with_file_unacknowledged_by_verified_issuer() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        create_collection_and_nominated_issuer();
        assert_ok!(LogionLoc::set_issuer_selection(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, ISSUER_ID1, true));
        let file = FileParams {
            hash: sha256(&vec![1, 2, 3]),
            nature: sha256(&vec![4, 5, 6]),
            submitter: SupportedAccountId::Polkadot(ISSUER_ID1),
            size: 456,
        };
        assert_ok!(LogionLoc::add_file(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, file.clone()));

        assert_err!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, None, true), Error::<Test>::CannotCloseUnacknowledgedByVerifiedIssuer);

        let loc = LogionLoc::loc(LOC_ID).unwrap();
        assert!(!loc.closed);
        assert_eq!(loc.files[0], expected_file(&file, NOT_ACKNOWLEDGED, NOT_ACKNOWLEDGED));
    });
}

#[test]
fn it_fails_to_acknowledge_requester_file_as_issuer() {
    new_test_ext().execute_with(|| {
        let file = create_loc_with_file_from_requester();
        assert_err!(LogionLoc::acknowledge_file(RuntimeOrigin::signed(ISSUER_ID1), LOC_ID, file.hash.clone()), Error::<Test>::Unauthorized);
        let loc = LogionLoc::loc(LOC_ID).unwrap();
        assert_eq!(loc.files[0], expected_file(&file, NOT_ACKNOWLEDGED, NOT_ACKNOWLEDGED));
    });
}
#[test]
fn it_fails_to_acknowledge_unknown_file() {
    new_test_ext().execute_with(|| {
        create_loc_with_file_from_requester();
        let hash = BlakeTwo256::hash_of(&"unknown_hash".as_bytes().to_vec());
        assert_err!(LogionLoc::acknowledge_file(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, hash), Error::<Test>::ItemNotFound);
    });
}

#[test]
fn it_fails_to_acknowledge_already_acknowledged_file() {
    new_test_ext().execute_with(|| {
        let file = create_loc_with_file_from_requester();
        assert_ok!(LogionLoc::acknowledge_file(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, file.hash.clone()));
        assert_err!(LogionLoc::acknowledge_file(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, file.hash.clone()), Error::<Test>::ItemAlreadyAcknowledged);
    });
}

#[test]
fn it_fails_to_acknowledge_file_when_unauthorized_caller() {
    new_test_ext().execute_with(|| {
        let file = create_loc_with_file_from_requester();
        assert_err!(LogionLoc::acknowledge_file(RuntimeOrigin::signed(UNAUTHORIZED_CALLER), LOC_ID, file.hash.clone()), Error::<Test>::Unauthorized);
    });
}

#[test]
fn it_fails_to_close_loc_with_unacknowledged_file() {
    new_test_ext().execute_with(|| {
        create_loc_with_file_from_requester();
        assert_err!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, None, false), Error::<Test>::CannotCloseUnacknowledgedByVerifiedIssuer);
    });
}

#[test]
fn it_closes_loc_and_acknowledges_file() {
    new_test_ext().execute_with(|| {
        let file = create_loc_with_file_from_requester();
        assert_ok!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, None, true));
        let loc = LogionLoc::loc(LOC_ID).unwrap();
        assert_eq!(loc.files[0], expected_file(&file, ACKNOWLEDGED, NOT_ACKNOWLEDGED));
    });
}

#[test]
fn it_fails_to_acknowledge_file_when_loc_voided() {
    new_test_ext().execute_with(|| {
        let file = create_loc_with_file_from_requester();
        assert_ok!(LogionLoc::make_void(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID));
        assert_err!(LogionLoc::acknowledge_file(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, file.hash.clone()), Error::<Test>::CannotMutateVoid);
    });
}

fn create_loc_with_file_from_requester() -> FileParams<H256, AccountId, EthereumAddress> {
    setup_default_balances();
    assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
    let file = FileParams {
        hash: sha256(&"test".as_bytes().to_vec()),
        nature: sha256(&"test-file-nature".as_bytes().to_vec()),
        submitter: SupportedAccountId::Polkadot(LOC_REQUESTER_ID),
        size: FILE_SIZE,
    };
    assert_ok!(LogionLoc::add_file(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, file.clone()));
    let loc = LogionLoc::loc(LOC_ID).unwrap();
    assert_eq!(loc.files[0], expected_file(&file, NOT_ACKNOWLEDGED, NOT_ACKNOWLEDGED));
    file
}

#[test]
fn it_fails_adding_file_for_unauthorized_caller() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        let file = FileParams {
            hash: sha256(&"test".as_bytes().to_vec()),
            nature: sha256(&"test-file-nature".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_OWNER1),
            size: FILE_SIZE,
        };
        assert_err!(LogionLoc::add_file(RuntimeOrigin::signed(UNAUTHORIZED_CALLER), LOC_ID, file.clone()), Error::<Test>::Unauthorized);
    });
}

#[test]
fn it_fails_adding_file_when_insufficient_funds() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        set_balance(LOC_REQUESTER_ID, INSUFFICIENT_BALANCE);
        let file = FileParams {
            hash: sha256(&"test".as_bytes().to_vec()),
            nature: sha256(&"test-file-nature".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_REQUESTER_ID),
            size: FILE_SIZE,
        };
        let snapshot = BalancesSnapshot::take(LOC_REQUESTER_ID, LOC_OWNER1);
        assert_err!(LogionLoc::add_file(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, file.clone()), Error::<Test>::InsufficientFunds);
        check_no_fees(snapshot);
    });
}

fn check_no_fees(previous_balances: BalancesSnapshot) {
    let current_balances = BalancesSnapshot::take(previous_balances.payer_account, previous_balances.legal_officer_account);
    let balances_delta = current_balances.delta_since(&previous_balances);

    assert_eq!(balances_delta.total_credited(), 0);
    assert_eq!(balances_delta.total_debited(), 0);
}

#[test]
fn it_fails_adding_file_when_closed() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        create_closed_loc();
        let file = FileParams {
            hash: sha256(&"test".as_bytes().to_vec()),
            nature: sha256(&"test-file-nature".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_OWNER1),
            size: FILE_SIZE,
        };
        assert_err!(LogionLoc::add_file(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, file.clone()), Error::<Test>::CannotMutate);
    });
}

#[test]
fn it_adds_link_with_owner() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), OTHER_LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        let link = LocLinkParams {
            id: OTHER_LOC_ID,
            nature: sha256(&"test-link-nature".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_OWNER1),
        };
        assert_ok!(LogionLoc::add_link(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, link.clone()));
        let loc = LogionLoc::loc(LOC_ID).unwrap();
        assert_eq!(loc.links[0].id, link.id);
        assert_eq!(loc.links[0].nature, link.nature);
        assert_eq!(loc.links[0].submitter, link.submitter);
        assert_eq!(loc.links[0].acknowledged_by_owner, true);
        assert_eq!(loc.links[0].acknowledged_by_verified_issuer, false);
    });
}

#[test]
fn it_adds_link_with_requester() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), OTHER_LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        let link = LocLinkParams {
            id: OTHER_LOC_ID,
            nature: sha256(&"test-link-nature".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_REQUESTER_ID),
        };
        assert_ok!(LogionLoc::add_link(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, link.clone()));
    });
}

#[test]
fn it_fails_adding_link_for_unauthorized_caller() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), OTHER_LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        let link = LocLinkParams {
            id: OTHER_LOC_ID,
            nature: sha256(&"test-link-nature".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(UNAUTHORIZED_CALLER),
        };
        assert_err!(LogionLoc::add_link(RuntimeOrigin::signed(UNAUTHORIZED_CALLER), LOC_ID, link.clone()), Error::<Test>::Unauthorized);
    });
}

#[test]
fn it_fails_adding_link_when_closed() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        create_closed_loc();
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), OTHER_LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        let link = LocLinkParams {
            id: OTHER_LOC_ID,
            nature: sha256(&"test-link-nature".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_OWNER1),
        };
        assert_err!(LogionLoc::add_link(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, link.clone()), Error::<Test>::CannotMutate);
    });
}

#[test]
fn it_fails_adding_wrong_link() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        let link = LocLinkParams {
            id: OTHER_LOC_ID,
            nature: sha256(&"test-link-nature".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_OWNER1),
        };
        assert_err!(LogionLoc::add_link(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, link.clone()), Error::<Test>::LinkedLocNotFound);
    });
}

#[test]
fn it_acknowledges_link() {
    new_test_ext().execute_with(|| {
        let link = create_loc_with_link_from_requester();
        assert_ok!(LogionLoc::acknowledge_link(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, link.id.clone()));
        let loc = LogionLoc::loc(LOC_ID).unwrap();
        assert_eq!(loc.links[0], expected_link(&link, ACKNOWLEDGED, NOT_ACKNOWLEDGED));
    });
}

fn expected_link(link: &LocLinkParams<LocId, crate::mock::Hash, AccountId, EthereumAddress>, acknowledged_by_owner: bool, acknowledged_by_verified_issuer: bool) -> LocLink<LocId, crate::mock::Hash, AccountId, EthereumAddress> {
    return LocLink {
        id: link.id,
        nature: link.nature.clone(),
        submitter: link.submitter,
        acknowledged_by_owner,
        acknowledged_by_verified_issuer,
    }
}

fn create_loc_with_link_from_requester() -> LocLinkParams<LocId, crate::mock::Hash, AccountId, EthereumAddress> {
    setup_default_balances();
    assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
    assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), OTHER_LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
    let link = LocLinkParams {
        id: OTHER_LOC_ID,
        nature: sha256(&"test-file-nature".as_bytes().to_vec()),
        submitter: SupportedAccountId::Polkadot(LOC_REQUESTER_ID),
    };
    assert_ok!(LogionLoc::add_link(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, link.clone()));
    let loc = LogionLoc::loc(LOC_ID).unwrap();
    assert_eq!(loc.links[0], expected_link(&link, NOT_ACKNOWLEDGED, NOT_ACKNOWLEDGED));
    link
}

#[test]
fn it_acknowledges_link_as_verified_issuer() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        create_collection_and_nominated_issuer();
        assert_ok!(LogionLoc::set_issuer_selection(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, ISSUER_ID1, true));
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), OTHER_LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        let link = LocLinkParams {
            id: OTHER_LOC_ID,
            nature: sha256(&"test-file-nature".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(ISSUER_ID1),
        };
        assert_ok!(LogionLoc::add_link(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, link.clone()));

        assert_ok!(LogionLoc::acknowledge_link(RuntimeOrigin::signed(ISSUER_ID1), LOC_ID, link.id.clone()));

        let loc = LogionLoc::loc(LOC_ID).unwrap();
        assert_eq!(loc.links[0], expected_link(&link, NOT_ACKNOWLEDGED, ACKNOWLEDGED));
    });
}

#[test]
fn it_fails_to_close_with_link_unacknowledged_by_verified_issuer() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        create_collection_and_nominated_issuer();
        assert_ok!(LogionLoc::set_issuer_selection(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, ISSUER_ID1, true));
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), OTHER_LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        let link = LocLinkParams {
            id: OTHER_LOC_ID,
            nature: sha256(&"test-file-nature".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(ISSUER_ID1),
        };
        assert_ok!(LogionLoc::add_link(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, link.clone()));

        assert_err!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, None, true), Error::<Test>::CannotCloseUnacknowledgedByVerifiedIssuer);

        let loc = LogionLoc::loc(LOC_ID).unwrap();
        assert!(!loc.closed);
        assert_eq!(loc.links[0], expected_link(&link, NOT_ACKNOWLEDGED, NOT_ACKNOWLEDGED));
    });
}

#[test]
fn it_fails_to_acknowledge_requester_link_as_issuer() {
    new_test_ext().execute_with(|| {
        let link = create_loc_with_link_from_requester();
        assert_err!(LogionLoc::acknowledge_link(RuntimeOrigin::signed(ISSUER_ID1), LOC_ID, link.id.clone()), Error::<Test>::Unauthorized);
        let loc = LogionLoc::loc(LOC_ID).unwrap();
        assert_eq!(loc.links[0], expected_link(&link, NOT_ACKNOWLEDGED, NOT_ACKNOWLEDGED));
    });
}
#[test]
fn it_fails_to_acknowledge_unknown_link() {
    new_test_ext().execute_with(|| {
        create_loc_with_link_from_requester();
        let id = 42;
        assert_err!(LogionLoc::acknowledge_link(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, id), Error::<Test>::ItemNotFound);
    });
}

#[test]
fn it_fails_to_acknowledge_already_acknowledged_link() {
    new_test_ext().execute_with(|| {
        let link = create_loc_with_link_from_requester();
        assert_ok!(LogionLoc::acknowledge_link(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, link.id.clone()));
        assert_err!(LogionLoc::acknowledge_link(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, link.id.clone()), Error::<Test>::ItemAlreadyAcknowledged);
    });
}

#[test]
fn it_fails_to_acknowledge_link_when_unauthorized_caller() {
    new_test_ext().execute_with(|| {
        let link = create_loc_with_link_from_requester();
        assert_err!(LogionLoc::acknowledge_link(RuntimeOrigin::signed(UNAUTHORIZED_CALLER), LOC_ID, link.id.clone()), Error::<Test>::Unauthorized);
    });
}

#[test]
fn it_fails_to_close_loc_with_unacknowledged_link() {
    new_test_ext().execute_with(|| {
        create_loc_with_link_from_requester();
        assert_err!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, None, false), Error::<Test>::CannotCloseUnacknowledgedByVerifiedIssuer);
    });
}

#[test]
fn it_closes_loc_and_acknowledges_link() {
    new_test_ext().execute_with(|| {
        let link = create_loc_with_link_from_requester();
        assert_ok!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, None, true));
        let loc = LogionLoc::loc(LOC_ID).unwrap();
        assert_eq!(loc.links[0], expected_link(&link, ACKNOWLEDGED, NOT_ACKNOWLEDGED));
    });
}

#[test]
fn it_fails_to_acknowledge_link_when_loc_voided() {
    new_test_ext().execute_with(|| {
        let link = create_loc_with_link_from_requester();
        assert_ok!(LogionLoc::make_void(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID));
        assert_err!(LogionLoc::acknowledge_link(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, link.id.clone()), Error::<Test>::CannotMutateVoid);
    });
}

#[test]
fn it_closes_loc() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        assert_ok!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, None, false));
        let loc = LogionLoc::loc(LOC_ID).unwrap();
        assert!(loc.closed);
        assert!(loc.seal.is_none());
    });
}

#[test]
fn it_closes_loc_and_auto_acknowledges() {
    new_test_ext().execute_with(|| {
        setup_default_balances();

        // links
        let link = create_loc_with_link_from_requester();

        // metadata
        let metadata0 = add_metadata(0, LOC_REQUESTER_ID);
        let metadata1 = add_metadata(1, LOC_REQUESTER_ID);
        let metadata2 = add_metadata(2, LOC_OWNER1);

        // files
        let file0 = add_file(&"0", LOC_REQUESTER_ID);
        let file1 = add_file(&"1", LOC_REQUESTER_ID);
        let file2 = add_file(&"2", LOC_OWNER1);

        assert_ok!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, None, true));
        let loc = LogionLoc::loc(LOC_ID).unwrap();
        assert!(loc.closed);
        assert!(loc.seal.is_none());

        assert_eq!(loc.links[0], expected_link(&link, ACKNOWLEDGED, NOT_ACKNOWLEDGED));

        assert_eq!(loc.metadata[0], expected_metadata(metadata0.clone(), ACKNOWLEDGED, NOT_ACKNOWLEDGED));
        assert_eq!(loc.metadata[1], expected_metadata(metadata1.clone(), ACKNOWLEDGED, NOT_ACKNOWLEDGED));
        assert_eq!(loc.metadata[2], expected_metadata(metadata2.clone(), ACKNOWLEDGED, NOT_ACKNOWLEDGED));

        assert_eq!(loc.files[0], expected_file(&file0, ACKNOWLEDGED, NOT_ACKNOWLEDGED));
        assert_eq!(loc.files[1], expected_file(&file1, ACKNOWLEDGED, NOT_ACKNOWLEDGED));
        assert_eq!(loc.files[2], expected_file(&file2, ACKNOWLEDGED, NOT_ACKNOWLEDGED));
    });
}

fn add_metadata(name: u8, submitter: u64) -> MetadataItemParams<AccountId, EthereumAddress, H256> {
    let metadata = MetadataItemParams {
        name: sha256(&vec![name]),
        value: sha256(&vec![4, 5, 6]),
        submitter: SupportedAccountId::Polkadot(submitter),
    };
    assert_ok!(LogionLoc::add_metadata(RuntimeOrigin::signed(submitter), LOC_ID, metadata.clone()));
    metadata
}
fn add_file(content: &str, submitter: u64) -> FileParams<H256, AccountId, EthereumAddress> {
    let file = FileParams {
        hash: sha256(&content.as_bytes().to_vec()),
        nature: sha256(&"test-file-nature".as_bytes().to_vec()),
        submitter: SupportedAccountId::Polkadot(submitter),
        size: FILE_SIZE,
    };
    assert_ok!(LogionLoc::add_file(RuntimeOrigin::signed(submitter), LOC_ID, file.clone()));
    file
}

#[test]
fn it_fails_closing_loc_for_unauthorized_caller() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        assert_err!(LogionLoc::close(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, None, false), Error::<Test>::Unauthorized);
    });
}

#[test]
fn it_fails_closing_loc_for_already_closed() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        create_closed_loc();
        assert_err!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, None, false), Error::<Test>::AlreadyClosed);
    });
}

#[test]
fn it_links_locs_to_account() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        assert_ok!(LogionLoc::create_polkadot_identity_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), OTHER_LOC_ID, LOC_OWNER1, ID_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        assert!(LogionLoc::account_locs(LOC_REQUESTER_ID).is_some());
        assert!(LogionLoc::account_locs(LOC_REQUESTER_ID).unwrap().len() == 2);
        assert_eq!(LogionLoc::account_locs(LOC_REQUESTER_ID).unwrap()[0], LOC_ID);
        assert_eq!(LogionLoc::account_locs(LOC_REQUESTER_ID).unwrap()[1], OTHER_LOC_ID);
    });
}

#[test]
fn it_fails_creating_loc_with_non_legal_officer() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_err!(LogionLoc::create_polkadot_identity_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_REQUESTER_ID, ID_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()), Error::<Test>::Unauthorized);
        assert_err!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_REQUESTER_ID, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()), Error::<Test>::Unauthorized);
        assert_err!(LogionLoc::create_collection_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_REQUESTER_ID, None, Some(10), false, 0, OTHER_LOC_DEFAULT_LEGAL_FEE, 0, 0, ItemsParams::empty()), Error::<Test>::Unauthorized);
    });
}

#[test]
fn it_detects_existing_identity_loc() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_polkadot_identity_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, ID_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        assert_ok!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, None, false));

        assert_ok!(LogionLoc::create_polkadot_identity_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), OTHER_LOC_ID, LOC_OWNER2, ID_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        assert_ok!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER2), OTHER_LOC_ID, None, false));

        let legal_officers = Vec::from([LOC_OWNER1, LOC_OWNER2]);
        assert!(LogionLoc::has_closed_identity_locs(&LOC_REQUESTER_ID, &legal_officers));
    });
}

#[test]
fn it_detects_valid_loc_with_owner() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_polkadot_identity_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, ID_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        assert_ok!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, None, false));
        assert_eq!(LogionLoc::loc_valid_with_owner(&LOC_ID, &LOC_OWNER1), true);
    });
}

#[test]
fn it_detects_non_existing_loc_as_invalid() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_eq!(LogionLoc::loc_valid_with_owner(&LOC_ID, &LOC_OWNER1), false);
    });
}

#[test]
fn it_detects_open_loc_as_invalid() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_polkadot_identity_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, ID_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        assert_eq!(LogionLoc::loc_valid_with_owner(&LOC_ID, &LOC_OWNER1), false);
    });
}

#[test]
fn it_detects_void_loc_as_invalid() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_polkadot_identity_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, ID_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        assert_ok!(LogionLoc::make_void(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID));
        assert_eq!(LogionLoc::loc_valid_with_owner(&LOC_ID, &LOC_OWNER1), false);
    });
}

#[test]
fn it_detects_loc_with_wrong_owner_as_invalid() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_polkadot_identity_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, ID_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        assert_ok!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, None, false));
        assert_eq!(LogionLoc::loc_valid_with_owner(&LOC_ID, &LOC_OWNER2), false);
    });
}

#[test]
fn it_creates_logion_identity_loc() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let snapshot = BalancesSnapshot::take(LOC_OWNER1, LOC_OWNER1);
        assert_ok!(LogionLoc::create_logion_identity_loc(RuntimeOrigin::signed(LOC_OWNER1), LOGION_IDENTITY_LOC_ID));

        assert!(LogionLoc::loc(LOGION_IDENTITY_LOC_ID).is_some());
        assert!(LogionLoc::identity_loc_locs(LOGION_IDENTITY_LOC_ID).is_none());

        check_no_fees(snapshot);
    });
}

#[test]
fn it_creates_and_links_logion_locs_to_identity_loc() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let snapshot = BalancesSnapshot::take(LOC_OWNER1, LOC_OWNER1);
        assert_ok!(LogionLoc::create_logion_identity_loc(RuntimeOrigin::signed(LOC_OWNER1), LOGION_IDENTITY_LOC_ID));
        assert_ok!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOGION_IDENTITY_LOC_ID, None, false));

        assert_ok!(LogionLoc::create_logion_transaction_loc(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, LOGION_IDENTITY_LOC_ID));
        assert_ok!(LogionLoc::create_logion_transaction_loc(RuntimeOrigin::signed(LOC_OWNER1), OTHER_LOC_ID, LOGION_IDENTITY_LOC_ID));

        assert!(LogionLoc::loc(LOC_ID).is_some());
        assert!(LogionLoc::loc(OTHER_LOC_ID).is_some());
        assert!(LogionLoc::identity_loc_locs(LOGION_IDENTITY_LOC_ID).is_some());
        assert!(LogionLoc::identity_loc_locs(LOGION_IDENTITY_LOC_ID).unwrap().len() == 2);
        assert_eq!(LogionLoc::identity_loc_locs(LOGION_IDENTITY_LOC_ID).unwrap()[0], LOC_ID);
        assert_eq!(LogionLoc::identity_loc_locs(LOGION_IDENTITY_LOC_ID).unwrap()[1], OTHER_LOC_ID);

        check_no_fees(snapshot);
    });
}

#[test]
fn it_fails_creating_logion_loc_with_polkadot_identity_loc() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_polkadot_identity_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), OTHER_LOC_ID, LOC_OWNER1, ID_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));

        assert_err!(LogionLoc::create_logion_transaction_loc(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, OTHER_LOC_ID), Error::<Test>::UnexpectedRequester);
    });
}

#[test]
fn it_fails_creating_logion_loc_with_polkadot_transaction_loc() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_polkadot_identity_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), OTHER_LOC_ID, LOC_OWNER1, ID_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));

        assert_err!(LogionLoc::create_logion_transaction_loc(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, OTHER_LOC_ID), Error::<Test>::UnexpectedRequester);
    });
}

#[test]
fn it_fails_creating_logion_loc_with_logion_transaction_loc() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_logion_identity_loc(RuntimeOrigin::signed(LOC_OWNER1), LOGION_IDENTITY_LOC_ID));
        assert_ok!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOGION_IDENTITY_LOC_ID, None, false));
        assert_ok!(LogionLoc::create_logion_transaction_loc(RuntimeOrigin::signed(LOC_OWNER1), OTHER_LOC_ID, LOGION_IDENTITY_LOC_ID));

        assert_err!(LogionLoc::create_logion_transaction_loc(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, OTHER_LOC_ID), Error::<Test>::UnexpectedRequester);
    });
}

#[test]
fn it_fails_creating_logion_loc_with_open_logion_identity_loc() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_logion_identity_loc(RuntimeOrigin::signed(LOC_OWNER1), LOGION_IDENTITY_LOC_ID));

        assert_err!(LogionLoc::create_logion_transaction_loc(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, LOGION_IDENTITY_LOC_ID), Error::<Test>::UnexpectedRequester);
    });
}

#[test]
fn it_fails_creating_logion_loc_with_closed_void_logion_identity_loc() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_logion_identity_loc(RuntimeOrigin::signed(LOC_OWNER1), LOGION_IDENTITY_LOC_ID));
        assert_ok!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOGION_IDENTITY_LOC_ID, None, false));
        assert_ok!(LogionLoc::make_void(RuntimeOrigin::signed(LOC_OWNER1), LOGION_IDENTITY_LOC_ID));

        assert_err!(LogionLoc::create_logion_transaction_loc(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, LOGION_IDENTITY_LOC_ID), Error::<Test>::UnexpectedRequester);
    });
}

#[test]
fn it_creates_collection_loc() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let snapshot = BalancesSnapshot::take(LOC_REQUESTER_ID, LOC_OWNER1);
        assert_ok!(LogionLoc::create_collection_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, None, Some(10), false, 0, OTHER_LOC_DEFAULT_LEGAL_FEE, 0, 0, ItemsParams::empty()));
        assert_eq!(LogionLoc::loc(LOC_ID), Some(LegalOfficerCase {
            owner: LOC_OWNER1,
            requester: LOC_REQUESTER,
            metadata: vec![],
            files: vec![],
            closed: false,
            loc_type: LocType::Collection,
            links: vec![],
            void_info: None,
            replacer_of: None,
            collection_last_block_submission: None,
            collection_max_size: Some(10),
            collection_can_upload: false,
            seal: None,
            sponsorship_id: None,
            value_fee: 0,
            legal_fee: OTHER_LOC_DEFAULT_LEGAL_FEE,
            collection_item_fee: 0,
            tokens_record_fee: 0,
        }));

        let fees = Fees::only_legal(2000 * ONE_LGNT, Beneficiary::LegalOfficer(LOC_OWNER1));
        fees.assert_balances_events(snapshot);
    });
}

#[test]
fn it_fails_creating_collection_loc_without_limit() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_err!(LogionLoc::create_collection_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, None, None, false, 0, OTHER_LOC_DEFAULT_LEGAL_FEE, 0, 0, ItemsParams::empty()), Error::<Test>::CollectionHasNoLimit);
    });
}

#[test]
fn it_fails_adding_item_to_open_collection_loc() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_collection_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, None, Some(10), false, 0, OTHER_LOC_DEFAULT_LEGAL_FEE, 0, 0, ItemsParams::empty()));
        let collection_item_id = BlakeTwo256::hash_of(&"item-id".as_bytes().to_vec());
        let collection_item_description = sha256(&"item-description".as_bytes().to_vec());
        assert_err!(LogionLoc::add_collection_item(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, collection_item_id, collection_item_description, vec![], None, false, Vec::new()), Error::<Test>::WrongCollectionLoc);
    });
}

#[test]
fn it_adds_item_to_closed_collection_loc() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_collection_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, None, Some(10), false, 0, OTHER_LOC_DEFAULT_LEGAL_FEE, 0, 0, ItemsParams::empty()));
        assert_ok!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, None, false));

        let collection_item_id = BlakeTwo256::hash_of(&"item-id".as_bytes().to_vec());
        let collection_item_description = sha256(&"item-description".as_bytes().to_vec());
        assert_ok!(LogionLoc::add_collection_item(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, collection_item_id, collection_item_description.clone(), vec![], None, false, Vec::new()));
        assert_eq!(LogionLoc::collection_items(LOC_ID, collection_item_id), Some(CollectionItem {
            description: collection_item_description,
            files: vec![],
            token: None,
            restricted_delivery: false,
            terms_and_conditions: vec![],
        }));
        assert_eq!(LogionLoc::collection_size(LOC_ID), Some(1));
    });
}

#[test]
fn it_fails_to_item_with_terms_and_conditions_when_non_existent_tc_loc() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_collection_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, None, Some(10), false, 0, OTHER_LOC_DEFAULT_LEGAL_FEE, 0, 0, ItemsParams::empty()));
        assert_ok!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, None, false));

        let collection_item_id = BlakeTwo256::hash_of(&"item-id".as_bytes().to_vec());
        let collection_item_description = sha256(&"item-description".as_bytes().to_vec());
        let terms_and_conditions_details = "ITEM-A, ITEM-B".as_bytes().to_vec();
        let terms_and_conditions = vec![TermsAndConditionsElement {
            tc_type: sha256(&"Logion".as_bytes().to_vec()),
            tc_loc: LOGION_CLASSIFICATION_LOC_ID,
            details: sha256(&terms_and_conditions_details),
        }];
        assert_err!(LogionLoc::add_collection_item(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, collection_item_id, collection_item_description.clone(), vec![], None, false, terms_and_conditions), Error::<Test>::TermsAndConditionsLocNotFound);
    });
}

#[test]
fn it_fails_to_item_with_terms_and_conditions_when_open_tc_loc() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_collection_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, None, Some(10), false, 0, OTHER_LOC_DEFAULT_LEGAL_FEE, 0, 0, ItemsParams::empty()));
        assert_ok!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, None, false));
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOGION_CLASSIFICATION_LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));

        let collection_item_id = BlakeTwo256::hash_of(&"item-id".as_bytes().to_vec());
        let collection_item_description = sha256(&"item-description".as_bytes().to_vec());
        let terms_and_conditions_details = sha256(&"ITEM-A, ITEM-B".as_bytes().to_vec());
        let terms_and_conditions = vec![TermsAndConditionsElement {
            tc_type: sha256(&"Logion".as_bytes().to_vec()),
            tc_loc: LOGION_CLASSIFICATION_LOC_ID,
            details: terms_and_conditions_details.clone(),
        }];
        assert_err!(LogionLoc::add_collection_item(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, collection_item_id, collection_item_description.clone(), vec![], None, false, terms_and_conditions), Error::<Test>::TermsAndConditionsLocNotClosed);
    });
}

#[test]
fn it_fails_to_item_with_terms_and_conditions_when_void_tc_loc() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_collection_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, None, Some(10), false, 0, OTHER_LOC_DEFAULT_LEGAL_FEE, 0, 0, ItemsParams::empty()));
        assert_ok!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, None, false));
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOGION_CLASSIFICATION_LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        assert_ok!(LogionLoc::make_void(RuntimeOrigin::signed(LOC_OWNER1), LOGION_CLASSIFICATION_LOC_ID));

        let collection_item_id = BlakeTwo256::hash_of(&"item-id".as_bytes().to_vec());
        let collection_item_description = sha256(&"item-description".as_bytes().to_vec());
        let terms_and_conditions_details = sha256(&"ITEM-A, ITEM-B".as_bytes().to_vec());
        let terms_and_conditions = vec![TermsAndConditionsElement {
            tc_type: sha256(&"Logion".as_bytes().to_vec()),
            tc_loc: LOGION_CLASSIFICATION_LOC_ID,
            details: terms_and_conditions_details,
        }];
        assert_err!(LogionLoc::add_collection_item(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, collection_item_id, collection_item_description.clone(), vec![], None, false, terms_and_conditions), Error::<Test>::TermsAndConditionsLocVoid);
    });
}

#[test]
fn it_adds_item_with_terms_and_conditions_to_closed_collection_loc() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_collection_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, None, Some(10), false, 0, OTHER_LOC_DEFAULT_LEGAL_FEE, 0, 0, ItemsParams::empty()));
        assert_ok!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, None, false));
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOGION_CLASSIFICATION_LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        assert_ok!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOGION_CLASSIFICATION_LOC_ID, None, false));
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), ADDITIONAL_TC_LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        assert_ok!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), ADDITIONAL_TC_LOC_ID, None, false));

        let collection_item_id = BlakeTwo256::hash_of(&"item-id".as_bytes().to_vec());
        let collection_item_description = sha256(&"item-description".as_bytes().to_vec());
        let tc1 = TermsAndConditionsElement {
            tc_type: sha256(&"Logion".as_bytes().to_vec()),
            tc_loc: LOGION_CLASSIFICATION_LOC_ID,
            details: sha256(&"ITEM-A, ITEM-B".as_bytes().to_vec()),
        };
        let tc2 = TermsAndConditionsElement {
            tc_type: sha256(&"Specific".as_bytes().to_vec()),
            tc_loc: ADDITIONAL_TC_LOC_ID,
            details: sha256(&"Some more details".as_bytes().to_vec()),
        };
        let terms_and_conditions = vec![tc1, tc2];
        assert_ok!(LogionLoc::add_collection_item(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, collection_item_id, collection_item_description.clone(), vec![], None, false, terms_and_conditions.clone()));
        assert_eq!(LogionLoc::collection_items(LOC_ID, collection_item_id), Some(CollectionItem {
            description: collection_item_description,
            files: vec![],
            token: None,
            restricted_delivery: false,
            terms_and_conditions: terms_and_conditions.clone(),
        }));
        assert_eq!(LogionLoc::collection_size(LOC_ID), Some(1));
    });
}

#[test]
fn it_fails_adding_item_to_collection_loc_if_not_requester() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_collection_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, None, Some(10), false, 0, OTHER_LOC_DEFAULT_LEGAL_FEE, 0, 0, ItemsParams::empty()));
        assert_ok!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, None, false));

        let collection_item_id = BlakeTwo256::hash_of(&"item-id".as_bytes().to_vec());
        let collection_item_description = sha256(&"item-description".as_bytes().to_vec());
        assert_err!(LogionLoc::add_collection_item(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, collection_item_id, collection_item_description, vec![], None, false, Vec::new()), Error::<Test>::WrongCollectionLoc);
    });
}

#[test]
fn it_fails_adding_item_if_duplicate_key() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_collection_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, None, Some(10), false, 0, OTHER_LOC_DEFAULT_LEGAL_FEE, 0, 0, ItemsParams::empty()));
        assert_ok!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, None, false));

        let collection_item_id = BlakeTwo256::hash_of(&"item-id".as_bytes().to_vec());
        let collection_item_description = sha256(&"item-description".as_bytes().to_vec());
        assert_ok!(LogionLoc::add_collection_item(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, collection_item_id.clone(), collection_item_description.clone(), vec![], None, false, Vec::new()));
        assert_err!(LogionLoc::add_collection_item(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, collection_item_id, collection_item_description, vec![], None, false, Vec::new()), Error::<Test>::CollectionItemAlreadyExists);
    });
}

#[test]
fn it_fails_adding_item_if_size_limit_reached() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_collection_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, None, Some(1), false, 0, OTHER_LOC_DEFAULT_LEGAL_FEE, 0, 0, ItemsParams::empty()));
        assert_ok!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, None, false));

        let collection_item_id = BlakeTwo256::hash_of(&"item-id".as_bytes().to_vec());
        let collection_item_description = sha256(&"item-description".as_bytes().to_vec());
        assert_ok!(LogionLoc::add_collection_item(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, collection_item_id.clone(), collection_item_description.clone(), vec![], None, false, Vec::new()));
        let collection_item_id2 = BlakeTwo256::hash_of(&"item-id2".as_bytes().to_vec());
        assert_err!(LogionLoc::add_collection_item(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, collection_item_id2, collection_item_description, vec![], None, false, Vec::new()), Error::<Test>::CollectionLimitsReached);
    });
}

#[test]
fn it_fails_adding_item_if_block_limit_reached() {
    let current_block: u64 = 10;
    new_test_ext_at_block(current_block).execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_collection_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, Some(current_block - 1), None, false, 0, OTHER_LOC_DEFAULT_LEGAL_FEE, 0, 0, ItemsParams::empty()));
        assert_ok!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, None, false));

        let collection_item_id = BlakeTwo256::hash_of(&"item-id".as_bytes().to_vec());
        let collection_item_description = sha256(&"item-description".as_bytes().to_vec());
        assert_err!(LogionLoc::add_collection_item(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, collection_item_id, collection_item_description, vec![], None, false, Vec::new()), Error::<Test>::CollectionLimitsReached);
    });
}

#[test]
fn it_fails_adding_item_if_collection_void() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_collection_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, None, Some(1), false, 0, OTHER_LOC_DEFAULT_LEGAL_FEE, 0, 0, ItemsParams::empty()));
        assert_ok!(LogionLoc::make_void(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID));

        let collection_item_id = BlakeTwo256::hash_of(&"item-id".as_bytes().to_vec());
        let collection_item_description = sha256(&"item-description".as_bytes().to_vec());
        assert_err!(LogionLoc::add_collection_item(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, collection_item_id, collection_item_description, vec![], None, false, Vec::new()), Error::<Test>::WrongCollectionLoc);
    });
}

#[test]
fn it_fails_adding_item_if_files_attached_but_upload_not_enabled() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_collection_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, None, Some(1), false, 0, OTHER_LOC_DEFAULT_LEGAL_FEE, 0, 0, ItemsParams::empty()));
        assert_ok!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, None, false));

        let collection_item_id = BlakeTwo256::hash_of(&"item-id".as_bytes().to_vec());
        let collection_item_description = sha256(&"item-description".as_bytes().to_vec());
        let collection_item_files = vec![CollectionItemFile {
            name: sha256(&"picture.png".as_bytes().to_vec()),
            content_type: sha256(&"image/png".as_bytes().to_vec()),
            hash: BlakeTwo256::hash_of(&"file content".as_bytes().to_vec()),
            size: 123456,
        }];
        assert_err!(LogionLoc::add_collection_item(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, collection_item_id, collection_item_description, collection_item_files, None, false, Vec::new()), Error::<Test>::CannotUpload);
    });
}

#[test]
fn it_adds_item_if_no_files_attached_and_upload_enabled() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_collection_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, None, Some(1), true, 0, OTHER_LOC_DEFAULT_LEGAL_FEE, 0, 0, ItemsParams::empty()));
        assert_ok!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, None, false));

        let collection_item_id = BlakeTwo256::hash_of(&"item-id".as_bytes().to_vec());
        let collection_item_description = sha256(&"item-description".as_bytes().to_vec());
        assert_ok!(LogionLoc::add_collection_item(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, collection_item_id, collection_item_description, vec![], None, false, Vec::new()));
    });
}

#[test]
fn it_adds_item_with_one_file_attached() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_collection_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, None, Some(1), true, 0, OTHER_LOC_DEFAULT_LEGAL_FEE, 0, 0, ItemsParams::empty()));
        assert_ok!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, None, false));

        let collection_item_id = BlakeTwo256::hash_of(&"item-id".as_bytes().to_vec());
        let collection_item_description = sha256(&"item-description".as_bytes().to_vec());
        let collection_item_files = vec![CollectionItemFile {
            name: sha256(&"picture.png".as_bytes().to_vec()),
            content_type: sha256(&"image/png".as_bytes().to_vec()),
            hash: BlakeTwo256::hash_of(&"file content".as_bytes().to_vec()),
            size: FILE_SIZE,
        }];
        let snapshot = BalancesSnapshot::take(LOC_REQUESTER_ID, LOC_OWNER1);
        assert_ok!(LogionLoc::add_collection_item(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, collection_item_id, collection_item_description, collection_item_files, None, false, Vec::new()));
        let fees = Fees::only_storage(1, FILE_SIZE);
        fees.assert_balances_events(snapshot);
    });
}

#[test]
fn it_fails_adding_item_with_insufficient_balance() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_collection_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, None, Some(1), true, 0, OTHER_LOC_DEFAULT_LEGAL_FEE, 0, 0, ItemsParams::empty()));
        assert_ok!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, None, false));
        set_balance(LOC_REQUESTER_ID, INSUFFICIENT_BALANCE);

        let collection_item_id = BlakeTwo256::hash_of(&"item-id".as_bytes().to_vec());
        let collection_item_description = sha256(&"item-description".as_bytes().to_vec());
        let collection_item_files = vec![CollectionItemFile {
            name: sha256(&"picture.png".as_bytes().to_vec()),
            content_type: sha256(&"image/png".as_bytes().to_vec()),
            hash: BlakeTwo256::hash_of(&"file content".as_bytes().to_vec()),
            size: FILE_SIZE,
        }];
        let snapshot = BalancesSnapshot::take(LOC_REQUESTER_ID, LOC_OWNER1);
        assert_err!(LogionLoc::add_collection_item(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, collection_item_id, collection_item_description, collection_item_files, None, false, Vec::new()), Error::<Test>::InsufficientFunds);
        check_no_fees(snapshot);
    });
}

#[test]
fn it_adds_item_with_token() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_collection_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, None, Some(1), true, 0, OTHER_LOC_DEFAULT_LEGAL_FEE, 0, 0, ItemsParams::empty()));
        assert_ok!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, None, false));

        let collection_item_id = BlakeTwo256::hash_of(&"item-id".as_bytes().to_vec());
        let collection_item_description = sha256(&"item-description".as_bytes().to_vec());
        let collection_item_files = vec![CollectionItemFile {
            name: sha256(&"picture.png".as_bytes().to_vec()),
            content_type: sha256(&"image/png".as_bytes().to_vec()),
            hash: BlakeTwo256::hash_of(&"file content".as_bytes().to_vec()),
            size: 123456,
        }];
        let collection_item_token = CollectionItemToken {
            token_type: sha256(&"ethereum_erc721".as_bytes().to_vec()),
            token_id: sha256(&"{\"contract\":\"0x765df6da33c1ec1f83be42db171d7ee334a46df5\",\"token\":\"4391\"}".as_bytes().to_vec()),
            token_issuance: 2,
        };
        let snapshot = BalancesSnapshot::take(LOC_REQUESTER_ID, LOC_OWNER1);
        assert_ok!(LogionLoc::add_collection_item(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, collection_item_id, collection_item_description, collection_item_files.clone(), Some(collection_item_token), true, Vec::new()));
        let fees = Fees {
            storage_fees: Fees::storage_fees(1, collection_item_files[0].size),
            legal_fees: 0,
            fee_beneficiary: None,
            certificate_fees: 8_000_000_000_000_000,
            value_fee: 0,
            collection_item_fee: 0,
            tokens_record_fee: 0,
        };
        fees.assert_balances_events(snapshot);
    });
}

#[test]
fn it_fails_adding_item_with_missing_token() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_collection_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, None, Some(1), true, 0, OTHER_LOC_DEFAULT_LEGAL_FEE, 0, 0, ItemsParams::empty()));
        assert_ok!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, None, false));

        let collection_item_id = BlakeTwo256::hash_of(&"item-id".as_bytes().to_vec());
        let collection_item_description = sha256(&"item-description".as_bytes().to_vec());
        let collection_item_files = vec![CollectionItemFile {
            name: sha256(&"picture.png".as_bytes().to_vec()),
            content_type: sha256(&"image/png".as_bytes().to_vec()),
            hash: BlakeTwo256::hash_of(&"file content".as_bytes().to_vec()),
            size: 123456,
        }];
        assert_err!(LogionLoc::add_collection_item(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, collection_item_id, collection_item_description, collection_item_files, None, true, Vec::new()), Error::<Test>::MissingToken);
    });
}

#[test]
fn it_fails_adding_item_with_missing_files() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_collection_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, None, Some(1), true, 0, OTHER_LOC_DEFAULT_LEGAL_FEE, 0, 0, ItemsParams::empty()));
        assert_ok!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, None, false));

        let collection_item_id = BlakeTwo256::hash_of(&"item-id".as_bytes().to_vec());
        let collection_item_description = sha256(&"item-description".as_bytes().to_vec());
        let collection_item_files = vec![];
        let collection_item_token = CollectionItemToken {
            token_type: sha256(&"ethereum_erc721".as_bytes().to_vec()),
            token_id: sha256(&"{\"contract\":\"0x765df6da33c1ec1f83be42db171d7ee334a46df5\",\"token\":\"4391\"}".as_bytes().to_vec()),
            token_issuance: 1,
        };
        assert_err!(LogionLoc::add_collection_item(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, collection_item_id, collection_item_description, collection_item_files, Some(collection_item_token), true, Vec::new()), Error::<Test>::MissingFiles);
    });
}

#[test]
fn it_adds_item_with_two_files_attached() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_collection_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, None, Some(1), true, 0, OTHER_LOC_DEFAULT_LEGAL_FEE, 0, 0, ItemsParams::empty()));
        assert_ok!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, None, false));

        let collection_item_id = BlakeTwo256::hash_of(&"item-id".as_bytes().to_vec());
        let collection_item_description = sha256(&"item-description".as_bytes().to_vec());
        let collection_item_files = vec![
            CollectionItemFile {
                name: sha256(&"picture.png".as_bytes().to_vec()),
                content_type: sha256(&"image/png".as_bytes().to_vec()),
                hash: BlakeTwo256::hash_of(&"file content".as_bytes().to_vec()),
                size: 123456,
            },
            CollectionItemFile {
                name: sha256(&"doc.pdf".as_bytes().to_vec()),
                content_type: sha256(&"application/pdf".as_bytes().to_vec()),
                hash: BlakeTwo256::hash_of(&"some other content".as_bytes().to_vec()),
                size: 789,
            },
        ];
        let snapshot = BalancesSnapshot::take(LOC_REQUESTER_ID, LOC_OWNER1);
        assert_ok!(LogionLoc::add_collection_item(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, collection_item_id, collection_item_description, collection_item_files, None, false, Vec::new()));

        let fees = Fees::only_storage(2, 123456 + 789);
        fees.assert_balances_events(snapshot);
    });
}

#[test]
fn it_fails_to_add_item_with_duplicate_hash() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_collection_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, None, Some(1), true, 0, OTHER_LOC_DEFAULT_LEGAL_FEE, 0, 0, ItemsParams::empty()));
        assert_ok!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, None, false));

        let collection_item_id = BlakeTwo256::hash_of(&"item-id".as_bytes().to_vec());
        let collection_item_description = sha256(&"item-description".as_bytes().to_vec());
        let same_hash = BlakeTwo256::hash_of(&"file content".as_bytes().to_vec());
        let collection_item_files = vec![
            CollectionItemFile {
                name: sha256(&"picture.png".as_bytes().to_vec()),
                content_type: sha256(&"image/png".as_bytes().to_vec()),
                hash: same_hash,
                size: 123456,
            },
            CollectionItemFile {
                name: sha256(&"doc.pdf".as_bytes().to_vec()),
                content_type: sha256(&"application/pdf".as_bytes().to_vec()),
                hash: same_hash,
                size: 789,
            },
        ];
        assert_err!(LogionLoc::add_collection_item(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, collection_item_id, collection_item_description, collection_item_files, None, false, Vec::new()), Error::<Test>::DuplicateFile);
    });
}

#[test]
fn it_closes_and_seals_loc() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        let seal = BlakeTwo256::hash_of(&"some external private data".as_bytes().to_vec());
        assert_ok!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, Some(seal), false));
        let loc = LogionLoc::loc(LOC_ID).unwrap();
        assert!(loc.closed);
        assert!(loc.seal.is_some());
        assert_eq!(loc.seal.unwrap(), seal);
    });
}

#[test]
fn it_fails_adding_file_with_same_hash() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        let file1 = FileParams {
            hash: sha256(&"test".as_bytes().to_vec()),
            nature: sha256(&"test-file-nature".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_OWNER1),
            size: FILE_SIZE,
        };
        let snapshot = BalancesSnapshot::take(LOC_REQUESTER_ID, LOC_OWNER1);
        assert_ok!(LogionLoc::add_file(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, file1.clone()));
        let file2 = FileParams {
            hash: sha256(&"test".as_bytes().to_vec()),
            nature: sha256(&"test-file2-nature".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_OWNER1),
            size: FILE_SIZE,
        };
        assert_err!(LogionLoc::add_file(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, file2.clone()), Error::<Test>::DuplicateLocFile);
        let fees = Fees::only_storage(1, FILE_SIZE);
        fees.assert_balances_events(snapshot);
    });
}

#[test]
fn it_fails_adding_metadata_with_same_name() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        let metadata1 = MetadataItemParams {
            name: sha256(&"name".as_bytes().to_vec()),
            value: sha256(&"value1".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_OWNER1),
        };
        assert_ok!(LogionLoc::add_metadata(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, metadata1.clone()));
        let metadata2 = MetadataItemParams {
            name: sha256(&"name".as_bytes().to_vec()),
            value: sha256(&"value2".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_OWNER1),
        };
        assert_err!(LogionLoc::add_metadata(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, metadata2.clone()), Error::<Test>::DuplicateLocMetadata);
    });
}

#[test]
fn it_fails_adding_link_with_same_target() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), OTHER_LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        let link1 = LocLinkParams {
            id: OTHER_LOC_ID,
            nature: sha256(&"test-link1-nature".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_OWNER1),
        };
        assert_ok!(LogionLoc::add_link(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, link1.clone()));
        let link2 = LocLinkParams {
            id: OTHER_LOC_ID,
            nature: sha256(&"test-link2-nature".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_OWNER1),
        };
        assert_err!(LogionLoc::add_link(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, link2.clone()), Error::<Test>::DuplicateLocLink);
    });
}

#[test]
fn it_adds_several_metadata() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        let metadata1 = MetadataItemParams {
            name: sha256(&vec![1, 2, 3]),
            value: sha256(&vec![4, 5, 6]),
            submitter: SupportedAccountId::Polkadot(LOC_OWNER1),
        };
        assert_ok!(LogionLoc::add_metadata(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, metadata1.clone()));
        let metadata2 = MetadataItemParams {
            name: sha256(&vec![1, 2, 4]),
            value: sha256(&vec![4, 5, 6]),
            submitter: SupportedAccountId::Polkadot(LOC_OWNER1),
        };
        assert_ok!(LogionLoc::add_metadata(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, metadata2.clone()));
        let loc = LogionLoc::loc(LOC_ID).unwrap();
        assert_eq!(loc.metadata[0], expected_metadata(metadata1, ACKNOWLEDGED, NOT_ACKNOWLEDGED));
        assert_eq!(loc.metadata[1], expected_metadata(metadata2, ACKNOWLEDGED, NOT_ACKNOWLEDGED));
    });
}

#[test]
fn it_nominates_an_issuer() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        nominate_issuer(ISSUER_ID1, ISSUER1_IDENTITY_LOC_ID);

        assert_eq!(LogionLoc::verified_issuers(LOC_OWNER1, ISSUER_ID1), Some(VerifiedIssuer { identity_loc: ISSUER1_IDENTITY_LOC_ID }));
    });
}

#[test]
fn it_dismisses_an_issuer() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        nominate_issuer(ISSUER_ID1, ISSUER1_IDENTITY_LOC_ID);

        assert_ok!(LogionLoc::dismiss_issuer(RuntimeOrigin::signed(LOC_OWNER1), ISSUER_ID1));

        assert_eq!(LogionLoc::verified_issuers(LOC_OWNER1, ISSUER_ID1), None);
    });
}

fn nominate_issuer(issuer: u64, identity_loc: u32) {
    assert_ok!(LogionLoc::create_polkadot_identity_loc(RuntimeOrigin::signed(issuer), identity_loc, LOC_OWNER1, ID_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
    assert_ok!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), identity_loc, None, false));
    assert_ok!(LogionLoc::nominate_issuer(RuntimeOrigin::signed(LOC_OWNER1), issuer, identity_loc));
}

#[test]
fn it_selects_an_issuer() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        create_collection_and_nominated_issuer();

        assert_ok!(LogionLoc::set_issuer_selection(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, ISSUER_ID1, true));

        assert_eq!(LogionLoc::selected_verified_issuers(LOC_ID, ISSUER_ID1), Some(()));
        assert_eq!(LogionLoc::locs_by_verified_issuer((ISSUER_ID1, LOC_OWNER1, LOC_ID)), Some(()));
    });
}

fn create_collection_and_nominated_issuer() {
    assert_ok!(LogionLoc::create_collection_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, None, Some(10), true, 0, OTHER_LOC_DEFAULT_LEGAL_FEE, 0, TOKENS_RECORD_FEE, ItemsParams::empty()));
    nominate_issuer(ISSUER_ID1, ISSUER1_IDENTITY_LOC_ID);
}

#[test]
fn it_fails_selecting_an_issuer_loc_not_found() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_err!(LogionLoc::set_issuer_selection(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, ISSUER_ID1, true), Error::<Test>::NotFound);
    });
}

#[test]
fn it_fails_selecting_an_issuer_not_nominated() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_collection_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, None, Some(10), true, 0, OTHER_LOC_DEFAULT_LEGAL_FEE, 0, 0, ItemsParams::empty()));

        assert_err!(LogionLoc::set_issuer_selection(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, ISSUER_ID1, true), Error::<Test>::NotNominated);
    });
}

#[test]
fn it_fails_selecting_an_issuer_unauthorized() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        create_collection_and_nominated_issuer();

        assert_err!(LogionLoc::set_issuer_selection(RuntimeOrigin::signed(LOC_OWNER2), LOC_ID, ISSUER_ID1, true), Error::<Test>::Unauthorized);
    });
}

#[test]
fn it_selects_an_issuer_closed() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        create_collection_and_nominated_issuer();
        assert_ok!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, None, false));

        assert_ok!(LogionLoc::set_issuer_selection(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, ISSUER_ID1, true));
    });
}

#[test]
fn it_fails_selecting_an_issuer_void() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        create_collection_and_nominated_issuer();
        assert_ok!(LogionLoc::make_void(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID));

        assert_err!(LogionLoc::set_issuer_selection(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, ISSUER_ID1, true), Error::<Test>::CannotMutateVoid);
    });
}

#[test]
fn it_selects_an_issuer_not_collection() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        nominate_issuer(ISSUER_ID1, ISSUER1_IDENTITY_LOC_ID);
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));

        assert_ok!(LogionLoc::set_issuer_selection(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, ISSUER_ID1, true));
    });
}

#[test]
fn it_unselects_an_issuer() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        create_collection_with_selected_issuer();

        assert_ok!(LogionLoc::set_issuer_selection(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, ISSUER_ID1, false));

        assert_eq!(LogionLoc::selected_verified_issuers(LOC_ID, ISSUER_ID1), None);
        assert_eq!(LogionLoc::locs_by_verified_issuer((ISSUER_ID1, LOC_OWNER1, LOC_ID)), None);
    });
}

fn create_collection_with_selected_issuer() {
    create_collection_and_nominated_issuer();
    assert_ok!(LogionLoc::set_issuer_selection(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, ISSUER_ID1, true));
}

#[test]
fn it_fails_unselecting_an_issuer_loc_not_found() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_err!(LogionLoc::set_issuer_selection(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, ISSUER_ID1, false), Error::<Test>::NotFound);
    });
}

#[test]
fn it_fails_unselecting_an_issuer_unauthorized() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        create_collection_with_selected_issuer();

        assert_err!(LogionLoc::set_issuer_selection(RuntimeOrigin::signed(LOC_OWNER2), LOC_ID, ISSUER_ID1, false), Error::<Test>::Unauthorized);
    });
}

#[test]
fn it_unselects_an_issuer_closed() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        create_collection_with_selected_issuer();
        assert_ok!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, None, false));

        assert_ok!(LogionLoc::set_issuer_selection(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, ISSUER_ID1, false));
    });
}

#[test]
fn it_fails_unselecting_an_issuer_void() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        create_collection_with_selected_issuer();
        assert_ok!(LogionLoc::make_void(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID));

        assert_err!(LogionLoc::set_issuer_selection(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, ISSUER_ID1, false), Error::<Test>::CannotMutateVoid);
    });
}

#[test]
fn it_unselects_issuer_on_dismiss() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        create_closed_collection_with_selected_issuer();
        nominate_issuer(ISSUER_ID2, ISSUER2_IDENTITY_LOC_ID);
        assert_ok!(LogionLoc::set_issuer_selection(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, ISSUER_ID2, true));
        assert!(LogionLoc::selected_verified_issuers(LOC_ID, ISSUER_ID1).is_some());
        assert!(LogionLoc::selected_verified_issuers(LOC_ID, ISSUER_ID2).is_some());
        assert!(LogionLoc::locs_by_verified_issuer((ISSUER_ID1, LOC_OWNER1, LOC_ID)).is_some());
        assert!(LogionLoc::locs_by_verified_issuer((ISSUER_ID2, LOC_OWNER1, LOC_ID)).is_some());

        assert_ok!(LogionLoc::dismiss_issuer(RuntimeOrigin::signed(LOC_OWNER1), ISSUER_ID1));

        assert!(LogionLoc::selected_verified_issuers(LOC_ID, ISSUER_ID1).is_none());
        assert!(LogionLoc::selected_verified_issuers(LOC_ID, ISSUER_ID2).is_some());
        assert!(LogionLoc::locs_by_verified_issuer((ISSUER_ID1, LOC_OWNER1, LOC_ID)).is_none());
        assert!(LogionLoc::locs_by_verified_issuer((ISSUER_ID2, LOC_OWNER1, LOC_ID)).is_some());
    });
}

#[test]
fn it_adds_tokens_record_issuer() {
    it_adds_tokens_record(ISSUER_ID1);
}

fn it_adds_tokens_record(submitter: AccountId) {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        create_closed_collection_with_selected_issuer();
        let record_id = build_record_id();
        let record_description = build_record_description();
        let record_files = build_record_files(1);

        let snapshot = BalancesSnapshot::take(LOC_REQUESTER_ID, LOC_OWNER1);
        assert_ok!(LogionLoc::add_tokens_record(RuntimeOrigin::signed(submitter), LOC_ID, record_id, record_description.clone(), record_files.clone()));

        let record = LogionLoc::tokens_records(LOC_ID, record_id).unwrap();
        assert_eq!(record.description, record_description);
        assert_eq!(record.submitter, submitter);
        assert_eq!(record.files.len(), 1);
        assert_eq!(record.files[0].name, record_files[0].name);
        assert_eq!(record.files[0].content_type, record_files[0].content_type);
        assert_eq!(record.files[0].size, record_files[0].size);
        assert_eq!(record.files[0].hash, record_files[0].hash);

        let fees = Fees::only_storage_and_tokens_record(1, record_files[0].size, TOKENS_RECORD_FEE, Beneficiary::LegalOfficer(LOC_OWNER1));
        fees.assert_balances_events(snapshot);
    });
}

fn create_closed_collection_with_selected_issuer() {
    create_collection_with_selected_issuer();
    assert_ok!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, None, false));
}

fn build_record_id() -> H256 {
    BlakeTwo256::hash_of(&"Record ID".as_bytes().to_vec())
}

fn build_record_description() -> H256 {
    sha256(&"Some description".as_bytes().to_vec())
}

fn build_record_files(files: usize) -> Vec<TokensRecordFileOf<Test>> {
    let mut record_files = Vec::with_capacity(files);
    for i in 0..files {
        let file = TokensRecordFile {
            name: sha256(&"File name".as_bytes().to_vec()),
            content_type: sha256(&"text/plain".as_bytes().to_vec()),
            size: i as u32 % 10,
            hash: BlakeTwo256::hash_of(&i.to_string().as_bytes().to_vec()),
        };
        record_files.push(file);
    }
    record_files
}

#[test]
fn it_adds_tokens_record_requester() {
    it_adds_tokens_record(LOC_REQUESTER_ID);
}

#[test]
fn it_adds_tokens_record_owner() {
    it_adds_tokens_record(LOC_OWNER1);
}

#[test]
fn it_fails_adding_tokens_record_already_exists() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        create_closed_collection_with_selected_issuer();
        let record_id = build_record_id();
        let record_description = build_record_description();
        let record_files = build_record_files(1);

        let snapshot = BalancesSnapshot::take(LOC_REQUESTER_ID, LOC_OWNER1);
        assert_ok!(LogionLoc::add_tokens_record(RuntimeOrigin::signed(ISSUER_ID1), LOC_ID, record_id, record_description.clone(), record_files.clone()));
        assert_err!(LogionLoc::add_tokens_record(RuntimeOrigin::signed(ISSUER_ID1), LOC_ID, record_id, record_description, record_files.clone()), Error::<Test>::TokensRecordAlreadyExists);
        let file = record_files.get(0).unwrap();

        let fees = Fees::only_storage_and_tokens_record(1, file.size, TOKENS_RECORD_FEE, Beneficiary::LegalOfficer(LOC_OWNER1));
        fees.assert_balances_events(snapshot);
    });
}

#[test]
fn it_fails_adding_tokens_record_not_contributor() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        create_closed_collection_with_selected_issuer();
        let record_id = build_record_id();
        let record_description = build_record_description();
        let record_files = build_record_files(1);

        assert_err!(LogionLoc::add_tokens_record(RuntimeOrigin::signed(ISSUER_ID2), LOC_ID, record_id, record_description, record_files), Error::<Test>::CannotAddRecord);
    });
}

#[test]
fn it_fails_adding_tokens_record_collection_open() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        create_collection_with_selected_issuer();
        let record_id = build_record_id();
        let record_description = build_record_description();
        let record_files = build_record_files(1);

        assert_err!(LogionLoc::add_tokens_record(RuntimeOrigin::signed(ISSUER_ID1), LOC_ID, record_id, record_description, record_files), Error::<Test>::CannotAddRecord);
    });
}

#[test]
fn it_fails_adding_tokens_record_collection_void() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        create_collection_with_selected_issuer();
        assert_ok!(LogionLoc::make_void(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID));
        let record_id = build_record_id();
        let record_description = build_record_description();
        let record_files = build_record_files(1);

        assert_err!(LogionLoc::add_tokens_record(RuntimeOrigin::signed(ISSUER_ID1), LOC_ID, record_id, record_description, record_files), Error::<Test>::CannotAddRecord);
    });
}

#[test]
fn it_fails_adding_tokens_record_not_collection() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        let record_id = build_record_id();
        let record_description = build_record_description();
        let record_files = build_record_files(1);

        assert_err!(LogionLoc::add_tokens_record(RuntimeOrigin::signed(ISSUER_ID1), LOC_ID, record_id, record_description, record_files), Error::<Test>::CannotAddRecord);
    });
}

#[test]
fn it_fails_adding_tokens_record_no_files() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        create_closed_collection_with_selected_issuer();
        let record_id = build_record_id();
        let record_description = build_record_description();
        let record_files = vec![];

        assert_err!(LogionLoc::add_tokens_record(RuntimeOrigin::signed(ISSUER_ID1), LOC_ID, record_id, record_description, record_files), Error::<Test>::MustUpload);
    });
}

#[test]
fn it_fails_adding_tokens_record_duplicate_file() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        create_closed_collection_with_selected_issuer();
        let record_id = build_record_id();
        let record_description = build_record_description();
        let file1 = TokensRecordFile {
            name: sha256(&"File name".as_bytes().to_vec()),
            content_type: sha256(&"text/plain".as_bytes().to_vec()),
            size: 4,
            hash: BlakeTwo256::hash_of(&"test".as_bytes().to_vec()),
        };
        let file2 = TokensRecordFile {
            name: sha256(&"File name 2".as_bytes().to_vec()),
            content_type: sha256(&"text/plain".as_bytes().to_vec()),
            size: 4,
            hash: BlakeTwo256::hash_of(&"test".as_bytes().to_vec()),
        };
        let record_files = vec![file1, file2];

        assert_err!(LogionLoc::add_tokens_record(RuntimeOrigin::signed(ISSUER_ID1), LOC_ID, record_id, record_description, record_files), Error::<Test>::DuplicateFile);
    });
}

#[test]
fn it_fails_adding_tokens_record_too_many_files() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        create_closed_collection_with_selected_issuer();
        let record_id = build_record_id();
        let record_description = build_record_description();
        let record_files = build_record_files(256);

        assert_err!(LogionLoc::add_tokens_record(RuntimeOrigin::signed(ISSUER_ID1), LOC_ID, record_id, record_description, record_files), Error::<Test>::TokensRecordTooMuchData);
    });
}

#[test]
fn it_fails_adding_tokens_record_when_insufficient_funds() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        create_closed_collection_with_selected_issuer();
        set_balance(LOC_REQUESTER_ID, INSUFFICIENT_BALANCE);
        let record_id = build_record_id();
        let record_description = build_record_description();
        let record_files = build_record_files(1);

        let snapshot = BalancesSnapshot::take(LOC_REQUESTER_ID, LOC_OWNER1);
        assert_err!(LogionLoc::add_tokens_record(RuntimeOrigin::signed(ISSUER_ID1), LOC_ID, record_id, record_description, record_files), Error::<Test>::InsufficientFunds);
        check_no_fees(snapshot);
    });
}

fn nominated_and_select_issuer(loc_id: u32) {
    nominate_issuer(ISSUER_ID1, ISSUER1_IDENTITY_LOC_ID);
    assert_ok!(LogionLoc::set_issuer_selection(RuntimeOrigin::signed(LOC_OWNER1), loc_id, ISSUER_ID1, true));
}

#[test]
fn it_adds_file_on_polkadot_transaction_loc_when_caller_is_requester_and_submitter_is_issuer() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        nominated_and_select_issuer(LOC_ID);
        let file = FileParams {
            hash: sha256(&"test".as_bytes().to_vec()),
            nature: sha256(&"test-file-nature".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(ISSUER_ID1),
            size: FILE_SIZE,
        };
        let snapshot = BalancesSnapshot::take(LOC_REQUESTER_ID, LOC_OWNER1);
        assert_ok!(LogionLoc::add_file(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, file.clone()));
        let loc = LogionLoc::loc(LOC_ID).unwrap();
        assert_eq!(loc.files[0], expected_file(&file, NOT_ACKNOWLEDGED, NOT_ACKNOWLEDGED));

        let fees = Fees::only_storage(1, FILE_SIZE);
        fees.assert_balances_events(snapshot);
    });
}

#[test]
fn it_fails_adding_file_on_polkadot_transaction_loc_cannot_submit() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        let file = FileParams {
            hash: sha256(&"test".as_bytes().to_vec()),
            nature: sha256(&"test-file-nature".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_OWNER2),
            size: FILE_SIZE,
        };
        assert_err!(LogionLoc::add_file(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, file.clone()), Error::<Test>::CannotSubmit);
    });
}

#[test]
fn it_fails_adding_metadata_on_logion_identity_loc_cannot_submit() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_logion_identity_loc(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID));
        let metadata = MetadataItemParams {
            name: sha256(&vec![1, 2, 3]),
            value: sha256(&vec![4, 5, 6]),
            submitter: SupportedAccountId::Polkadot(LOC_OWNER2),
        };
        assert_err!(LogionLoc::add_metadata(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, metadata.clone()), Error::<Test>::CannotSubmit);
    });
}

#[test]
fn it_adds_metadata_on_polkadot_transaction_loc_when_submitter_is_issuer() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        nominated_and_select_issuer(LOC_ID);
        let metadata = MetadataItemParams {
            name: sha256(&vec![1, 2, 3]),
            value: sha256(&vec![4, 5, 6]),
            submitter: SupportedAccountId::Polkadot(ISSUER_ID1),
        };
        assert_ok!(LogionLoc::add_metadata(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, metadata.clone()));
    });
}

#[test]
fn it_fails_adding_metadata_on_polkadot_transaction_loc_cannot_submit() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        let metadata = MetadataItemParams {
            name: sha256(&vec![1, 2, 3]),
            value: sha256(&vec![4, 5, 6]),
            submitter: SupportedAccountId::Polkadot(LOC_OWNER2),
        };
        assert_err!(LogionLoc::add_metadata(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, metadata.clone()), Error::<Test>::CannotSubmit);
    });
}

#[test]
fn it_creates_sponsorship() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let sponsorship_id = 1;
        let beneficiary = H160::from_str("0x900edc98db53508e6742723988b872dd08cd09c2").unwrap();
        let sponsored_account = SupportedAccountId::Other(OtherAccountId::Ethereum(beneficiary));

        assert_ok!(LogionLoc::sponsor(RuntimeOrigin::signed(SPONSOR_ID), sponsorship_id, sponsored_account, LOC_OWNER1));

        let sponsorship = LogionLoc::sponsorship(sponsorship_id).unwrap();
        assert_eq!(sponsorship.legal_officer, LOC_OWNER1);
        assert_eq!(sponsorship.sponsor, SPONSOR_ID);
        assert_eq!(sponsorship.sponsored_account, sponsored_account);
        assert_eq!(sponsorship.loc_id, None);
    });
}

#[test]
fn it_fails_creating_sponsorship_with_duplicate_id() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let sponsorship_id = 1;
        let beneficiary = H160::from_str("0x900edc98db53508e6742723988b872dd08cd09c2").unwrap();
        let sponsored_account = SupportedAccountId::Other(OtherAccountId::Ethereum(beneficiary));

        assert_ok!(LogionLoc::sponsor(RuntimeOrigin::signed(SPONSOR_ID), sponsorship_id, sponsored_account, LOC_OWNER1));
        assert_err!(LogionLoc::sponsor(RuntimeOrigin::signed(SPONSOR_ID), sponsorship_id, sponsored_account, LOC_OWNER1), Error::<Test>::AlreadyExists);
    });
}

#[test]
fn it_withdraws_unused_sponsorship() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let sponsorship_id = 1;
        let beneficiary = H160::from_str("0x900edc98db53508e6742723988b872dd08cd09c2").unwrap();
        let sponsored_account = SupportedAccountId::Other(OtherAccountId::Ethereum(beneficiary));
        assert_ok!(LogionLoc::sponsor(RuntimeOrigin::signed(SPONSOR_ID), sponsorship_id, sponsored_account, LOC_OWNER1));
        assert!(LogionLoc::sponsorship(sponsorship_id).is_some());

        assert_ok!(LogionLoc::withdraw_sponsorship(RuntimeOrigin::signed(SPONSOR_ID), sponsorship_id));

        assert!(LogionLoc::sponsorship(sponsorship_id).is_none());
    });
}

#[test]
fn it_creates_ethereum_identity_loc() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let ethereum_address = H160::from_str("0x590E9c11b1c2f20210b9b84dc2417B4A7955d4e6").unwrap();
        let requester_account_id = OtherAccountId::Ethereum(ethereum_address);
        let sponsorship_id = 1;
        let sponsored_account = SupportedAccountId::Other(OtherAccountId::Ethereum(ethereum_address));
        assert_ok!(LogionLoc::sponsor(RuntimeOrigin::signed(SPONSOR_ID), sponsorship_id, sponsored_account, LOC_OWNER1));
        let snapshot = BalancesSnapshot::take(SPONSOR_ID, LOC_OWNER1);
        assert_ok!(LogionLoc::create_other_identity_loc(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, requester_account_id.clone(), sponsorship_id, ID_LOC_DEFAULT_LEGAL_FEE));
        assert_eq!(LogionLoc::loc(LOC_ID), Some(LegalOfficerCase {
            owner: LOC_OWNER1,
            requester: OtherAccount(requester_account_id.clone()),
            metadata: vec![],
            files: vec![],
            closed: false,
            loc_type: LocType::Identity,
            links: vec![],
            void_info: None,
            replacer_of: None,
            collection_last_block_submission: None,
            collection_max_size: None,
            collection_can_upload: false,
            seal: None,
            sponsorship_id: Some(sponsorship_id),
            value_fee: 0,
            legal_fee: ID_LOC_DEFAULT_LEGAL_FEE,
            collection_item_fee: 0,
            tokens_record_fee: 0,
        }));
        assert_eq!(LogionLoc::other_account_locs(requester_account_id), Some(vec![LOC_ID]));
        assert_eq!(LogionLoc::sponsorship(sponsorship_id).unwrap().loc_id, Some(LOC_ID));
        System::assert_has_event(RuntimeEvent::LogionLoc(crate::Event::LocCreated { 0: LOC_ID }));

        let fees = Fees::only_legal(160 * ONE_LGNT, Beneficiary::Other);
        fees.assert_balances_events(snapshot);
    });
}

#[test]
fn it_creates_polkadot_identity_loc() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let snapshot = BalancesSnapshot::take(LOC_REQUESTER_ID, LOC_OWNER1);
        assert_ok!(LogionLoc::create_polkadot_identity_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, ID_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        assert_eq!(LogionLoc::loc(LOC_ID), Some(LegalOfficerCase {
            owner: LOC_OWNER1,
            requester: Account(LOC_REQUESTER_ID),
            metadata: vec![],
            files: vec![],
            closed: false,
            loc_type: LocType::Identity,
            links: vec![],
            void_info: None,
            replacer_of: None,
            collection_last_block_submission: None,
            collection_max_size: None,
            collection_can_upload: false,
            seal: None,
            sponsorship_id: None,
            value_fee: 0,
            legal_fee: ID_LOC_DEFAULT_LEGAL_FEE,
            collection_item_fee: 0,
            tokens_record_fee: 0,
        }));
        assert_eq!(LogionLoc::account_locs(LOC_REQUESTER_ID), Some(vec![LOC_ID]));
        System::assert_has_event(RuntimeEvent::LogionLoc(crate::Event::LocCreated { 0: LOC_ID }));

        let fees = Fees::only_legal(160 * ONE_LGNT, Beneficiary::Other);
        fees.assert_balances_events(snapshot);
    });
}

#[test]
fn it_fails_creating_ethereum_identity_loc_if_duplicate_loc_id() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let ethereum_address = H160::from_str("0x590E9c11b1c2f20210b9b84dc2417B4A7955d4e6").unwrap();
        let requester_address = OtherAccountId::Ethereum(ethereum_address);
        let sponsorship_id = 1;
        let sponsored_account: SupportedAccountId<AccountId, H160> = SupportedAccountId::Other(requester_address);
        assert_ok!(LogionLoc::sponsor(RuntimeOrigin::signed(SPONSOR_ID), sponsorship_id, sponsored_account, LOC_OWNER1));
        assert_ok!(LogionLoc::create_other_identity_loc(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, requester_address.clone(), sponsorship_id, ID_LOC_DEFAULT_LEGAL_FEE));
        assert_err!(LogionLoc::create_other_identity_loc(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, requester_address.clone(), sponsorship_id, ID_LOC_DEFAULT_LEGAL_FEE), Error::<Test>::AlreadyExists);
    });
}

#[test]
fn it_fails_creating_several_ethereum_identity_loc_with_single_sponsorship() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let ethereum_address = H160::from_str("0x590E9c11b1c2f20210b9b84dc2417B4A7955d4e6").unwrap();
        let requester_address = OtherAccountId::Ethereum(ethereum_address);
        let sponsorship_id = 1;
        let sponsored_account: SupportedAccountId<AccountId, H160> = SupportedAccountId::Other(requester_address);
        assert_ok!(LogionLoc::sponsor(RuntimeOrigin::signed(SPONSOR_ID), sponsorship_id, sponsored_account, LOC_OWNER1));
        assert_ok!(LogionLoc::create_other_identity_loc(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, requester_address.clone(), sponsorship_id, ID_LOC_DEFAULT_LEGAL_FEE));
        assert_err!(LogionLoc::create_other_identity_loc(RuntimeOrigin::signed(LOC_OWNER1), OTHER_LOC_ID, requester_address.clone(), sponsorship_id, ID_LOC_DEFAULT_LEGAL_FEE), Error::<Test>::CannotLinkToSponsorship);
    });
}

#[test]
fn it_fails_withdrawing_used_sponsorship() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let sponsorship_id = 1;
        let beneficiary = H160::from_str("0x900edc98db53508e6742723988b872dd08cd09c2").unwrap();
        let requester_address = OtherAccountId::Ethereum(beneficiary);
        let sponsored_account = SupportedAccountId::Other(OtherAccountId::Ethereum(beneficiary));
        assert_ok!(LogionLoc::sponsor(RuntimeOrigin::signed(SPONSOR_ID), sponsorship_id, sponsored_account, LOC_OWNER1));
        assert_ok!(LogionLoc::create_other_identity_loc(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, requester_address, sponsorship_id, ID_LOC_DEFAULT_LEGAL_FEE));

        assert_err!(LogionLoc::withdraw_sponsorship(RuntimeOrigin::signed(SPONSOR_ID), sponsorship_id), Error::<Test>::AlreadyUsed);
    });
}

#[test]
fn it_adds_metadata_when_submitter_is_ethereum_requester() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let ethereum_address = H160::from_str("0x590E9c11b1c2f20210b9b84dc2417B4A7955d4e6").unwrap();
        let requester_address = OtherAccountId::Ethereum(ethereum_address);
        let sponsorship_id = 1;
        let sponsored_account = SupportedAccountId::Other(requester_address);
        assert_ok!(LogionLoc::sponsor(RuntimeOrigin::signed(SPONSOR_ID), sponsorship_id, sponsored_account, LOC_OWNER1));
        assert_ok!(LogionLoc::create_other_identity_loc(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, requester_address, sponsorship_id, ID_LOC_DEFAULT_LEGAL_FEE));
        let metadata = MetadataItemParams {
            name: sha256(&vec![1, 2, 3]),
            value: sha256(&vec![4, 5, 6]),
            submitter: sponsored_account,
        };
        assert_ok!(LogionLoc::add_metadata(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, metadata.clone()));
        let loc = LogionLoc::loc(LOC_ID).unwrap();
        assert_eq!(loc.metadata[0], expected_metadata(metadata, ACKNOWLEDGED, NOT_ACKNOWLEDGED));
    });
}

#[test]
fn it_adds_file_when_submitter_is_ethereum_requester() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let requester = H160::from_str("0x900edc98db53508e6742723988b872dd08cd09c2").unwrap();
        let sponsorship_id = 1;
        let sponsored_account = SupportedAccountId::Other(OtherAccountId::Ethereum(requester));
        assert_ok!(LogionLoc::sponsor(RuntimeOrigin::signed(SPONSOR_ID), sponsorship_id, sponsored_account, LOC_OWNER1));
        assert_ok!(LogionLoc::create_other_identity_loc(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, OtherAccountId::Ethereum(requester), sponsorship_id, ID_LOC_DEFAULT_LEGAL_FEE));
        let file = FileParams {
            hash: sha256(&"test".as_bytes().to_vec()),
            nature: sha256(&"test-file-nature".as_bytes().to_vec()),
            submitter: SupportedAccountId::Other(OtherAccountId::Ethereum(requester)),
            size: FILE_SIZE,
        };
        let snapshot = BalancesSnapshot::take(SPONSOR_ID, LOC_OWNER1);
        assert_ok!(LogionLoc::add_file(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, file.clone()));
        let loc = LogionLoc::loc(LOC_ID).unwrap();
        assert_eq!(loc.files[0], expected_file(&file, ACKNOWLEDGED, NOT_ACKNOWLEDGED));

        let fees = Fees::only_storage(1, file.size);
        fees.assert_balances_events(snapshot);
    });
}

#[test]
fn it_fails_adding_item_with_token_with_zero_issuance() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_collection_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, None, Some(1), true, 0, OTHER_LOC_DEFAULT_LEGAL_FEE, 0, 0, ItemsParams::empty()));
        assert_ok!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, None, false));

        let collection_item_id = BlakeTwo256::hash_of(&"item-id".as_bytes().to_vec());
        let collection_item_description = sha256(&"item-description".as_bytes().to_vec());
        let collection_item_files = vec![CollectionItemFile {
            name: sha256(&"picture.png".as_bytes().to_vec()),
            content_type: sha256(&"image/png".as_bytes().to_vec()),
            hash: BlakeTwo256::hash_of(&"file content".as_bytes().to_vec()),
            size: 123456,
        }];
        let collection_item_token = CollectionItemToken {
            token_type: sha256(&"ethereum_erc721".as_bytes().to_vec()),
            token_id: sha256(&"{\"contract\":\"0x765df6da33c1ec1f83be42db171d7ee334a46df5\",\"token\":\"4391\"}".as_bytes().to_vec()),
            token_issuance: 0,
        };
        assert_err!(LogionLoc::add_collection_item(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, collection_item_id, collection_item_description, collection_item_files.clone(), Some(collection_item_token), true, Vec::new()), Error::<Test>::BadTokenIssuance);
    });
}

#[test]
fn it_reserves_value_fees() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let legal_fee = 2000 * ONE_LGNT;
        let value_fee = 100;
        assert_ok!(LogionLoc::create_collection_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, None, Some(10), false, value_fee, OTHER_LOC_DEFAULT_LEGAL_FEE, 0, 0, ItemsParams::empty()));
        assert_eq!(LogionLoc::loc(LOC_ID), Some(LegalOfficerCase {
            owner: LOC_OWNER1,
            requester: LOC_REQUESTER,
            metadata: vec![],
            files: vec![],
            closed: false,
            loc_type: LocType::Collection,
            links: vec![],
            void_info: None,
            replacer_of: None,
            collection_last_block_submission: None,
            collection_max_size: Some(10),
            collection_can_upload: false,
            seal: None,
            sponsorship_id: None,
            value_fee,
            legal_fee: OTHER_LOC_DEFAULT_LEGAL_FEE,
            collection_item_fee: 0,
            tokens_record_fee: 0,
        }));

        let expected_free_balance = INITIAL_BALANCE.saturating_sub(legal_fee).saturating_sub(value_fee);
        assert_eq!(<Test as Config>::Currency::free_balance(LOC_REQUESTER_ID), expected_free_balance);
        assert_eq!(<Test as Config>::Currency::reserved_balance(LOC_REQUESTER_ID), value_fee);
    });
}

#[test]
fn it_captures_value_fees() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let value_fee = 100;
        assert_ok!(LogionLoc::create_collection_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, None, Some(10), false, value_fee, OTHER_LOC_DEFAULT_LEGAL_FEE, 0, 0, ItemsParams::empty()));
        let previous_balances = BalancesSnapshot::take(LOC_REQUESTER_ID, LOC_OWNER1);
        assert_ok!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, None, false));

        let fees = Fees {
            certificate_fees: 0,
            fee_beneficiary: None,
            legal_fees: 0,
            storage_fees: 0,
            value_fee,
            collection_item_fee: 0,
            tokens_record_fee: 0,
        };
        fees.assert_balances_events(previous_balances);
    });
}

#[test]
fn it_frees_value_fees_community_treasury_on_void() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let value_fee = 100;
        assert_ok!(LogionLoc::create_collection_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, None, Some(10), false, value_fee, OTHER_LOC_DEFAULT_LEGAL_FEE, 0, 0, ItemsParams::empty()));
        assert_ok!(LogionLoc::make_void(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID));

        let legal_fee = 2000 * ONE_LGNT;
        assert_eq!(<Test as Config>::Currency::free_balance(LOC_REQUESTER_ID), INITIAL_BALANCE.saturating_sub(legal_fee));
    });
}

#[test]
fn it_does_not_free_value_fees_community_treasury_on_void_closed() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let value_fee = 100;
        assert_ok!(LogionLoc::create_collection_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, None, Some(10), false, value_fee, OTHER_LOC_DEFAULT_LEGAL_FEE, 0, 0, ItemsParams::empty()));
        assert_ok!(LogionLoc::create_collection_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), OTHER_LOC_ID, LOC_OWNER1, None, Some(10), false, value_fee, OTHER_LOC_DEFAULT_LEGAL_FEE, 0, 0, ItemsParams::empty()));
        assert_ok!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), OTHER_LOC_ID, None, false));
        assert_ok!(LogionLoc::make_void(RuntimeOrigin::signed(LOC_OWNER1), OTHER_LOC_ID));

        let legal_fee = 2000 * ONE_LGNT;
        let expected_free_balance = INITIAL_BALANCE.saturating_sub(2 * legal_fee).saturating_sub(2 * value_fee);
        assert_eq!(<Test as Config>::Currency::free_balance(LOC_REQUESTER_ID), expected_free_balance);
        assert_eq!(<Test as Config>::Currency::reserved_balance(LOC_REQUESTER_ID), value_fee);
    });
}

#[test]
fn it_creates_transaction_loc_with_initial_metadata() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let metadata = MetadataItemParams {
            name: sha256(&vec![1, 2, 3]),
            value: sha256(&vec![4, 5, 6]),
            submitter: SupportedAccountId::Polkadot(LOC_REQUESTER_ID),
        };
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::only_metadata(Vec::from([ metadata.clone() ]))));
        let loc = LogionLoc::loc(LOC_ID).unwrap();
        assert_eq!(loc.metadata[0], expected_metadata(metadata, NOT_ACKNOWLEDGED, NOT_ACKNOWLEDGED));
    });
}

#[test]
fn it_fails_creating_transaction_loc_with_duplicate_initial_metadata() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let metadata1 = MetadataItemParams {
            name: sha256(&"name".as_bytes().to_vec()),
            value: sha256(&"value1".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_REQUESTER_ID),
        };
        let metadata2 = MetadataItemParams {
            name: sha256(&"name".as_bytes().to_vec()),
            value: sha256(&"value2".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_REQUESTER_ID),
        };
        assert_err!(
            LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::only_metadata(Vec::from([ metadata1.clone(), metadata2.clone() ]))),
            Error::<Test>::DuplicateLocMetadata
        );
    });
}

#[test]
fn it_fails_creating_transaction_loc_with_invalid_metadata_submitter() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let metadata = MetadataItemParams {
            name: sha256(&vec![1, 2, 3]),
            value: sha256(&vec![4, 5, 6]),
            submitter: SupportedAccountId::Polkadot(LOC_OWNER1),
        };
        assert_err!(
            LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::only_metadata(Vec::from([ metadata.clone() ]))),
            Error::<Test>::CannotSubmit
        );
    });
}

#[test]
fn it_creates_transaction_loc_with_initial_files() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let file = FileParams {
            hash: sha256(&"test".as_bytes().to_vec()),
            nature: sha256(&"Test".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_REQUESTER_ID),
            size: 4,
        };
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::only_files(Vec::from([ file.clone() ]))));
        let loc = LogionLoc::loc(LOC_ID).unwrap();
        assert_eq!(loc.files[0], expected_file(&file, NOT_ACKNOWLEDGED, NOT_ACKNOWLEDGED));
    });
}

#[test]
fn it_fails_creating_transaction_loc_with_duplicate_initial_files() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let file1 = FileParams {
            hash: sha256(&"test".as_bytes().to_vec()),
            nature: sha256(&"Test 1".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_REQUESTER_ID),
            size: 4,
        };
        let file2 = FileParams {
            hash: sha256(&"test".as_bytes().to_vec()),
            nature: sha256(&"Test 2".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_REQUESTER_ID),
            size: 4,
        };
        assert_err!(
            LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::only_files(Vec::from([ file1, file2 ]))),
            Error::<Test>::DuplicateLocFile
        );
    });
}

#[test]
fn it_fails_creating_transaction_loc_with_invalid_file_submitter() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let file = FileParams {
            hash: sha256(&"test".as_bytes().to_vec()),
            nature: sha256(&"Test".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_OWNER1),
            size: 4,
        };
        assert_err!(
            LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::only_files(Vec::from([ file ]))),
            Error::<Test>::CannotSubmit
        );
    });
}

#[test]
fn it_creates_transaction_loc_with_initial_links() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), OTHER_LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        let link = LocLinkParams {
            id: OTHER_LOC_ID,
            nature: sha256(&"test-link-nature".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_REQUESTER_ID),
        };
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::only_links(Vec::from([ link.clone() ]))));
        let loc = LogionLoc::loc(LOC_ID).unwrap();
        assert_eq!(loc.links[0], expected_link(&link, NOT_ACKNOWLEDGED, NOT_ACKNOWLEDGED));
    });
}

#[test]
fn it_fails_creating_transaction_loc_with_initial_links_if_not_found() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let link = LocLinkParams {
            id: OTHER_LOC_ID,
            nature: sha256(&"test-link-nature".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_REQUESTER_ID),
        };
        assert_err!(
            LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::only_links(Vec::from([ link.clone() ]))),
            Error::<Test>::LinkedLocNotFound
        );
    });
}

#[test]
fn it_fails_creating_transaction_loc_with_duplicate_initial_links() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let link1 = LocLinkParams {
            id: OTHER_LOC_ID,
            nature: sha256(&"test-link1-nature".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_REQUESTER_ID),
        };
        let link2 = LocLinkParams {
            id: OTHER_LOC_ID,
            nature: sha256(&"test-link2-nature".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_REQUESTER_ID),
        };
        assert_err!(
            LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::only_links(Vec::from([ link1, link2 ]))),
            Error::<Test>::DuplicateLocLink
        );
    });
}

#[test]
fn it_fails_creating_transaction_loc_with_invalid_link_submitter() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let link = LocLinkParams {
            id: OTHER_LOC_ID,
            nature: sha256(&"test-link-nature".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_OWNER1),
        };
        assert_err!(
            LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, OTHER_LOC_DEFAULT_LEGAL_FEE, ItemsParams::only_links(Vec::from([ link ]))),
            Error::<Test>::CannotSubmit
        );
    });
}

#[test]
fn it_creates_identity_loc_with_initial_metadata() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let metadata = MetadataItemParams {
            name: sha256(&vec![1, 2, 3]),
            value: sha256(&vec![4, 5, 6]),
            submitter: SupportedAccountId::Polkadot(LOC_REQUESTER_ID),
        };
        assert_ok!(LogionLoc::create_polkadot_identity_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, ID_LOC_DEFAULT_LEGAL_FEE, ItemsParams::only_metadata(Vec::from([ metadata.clone() ]))));
        let loc = LogionLoc::loc(LOC_ID).unwrap();
        assert_eq!(loc.metadata[0], expected_metadata(metadata, NOT_ACKNOWLEDGED, NOT_ACKNOWLEDGED));
    });
}

#[test]
fn it_fails_creating_identity_loc_with_duplicate_initial_metadata() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let metadata1 = MetadataItemParams {
            name: sha256(&"name".as_bytes().to_vec()),
            value: sha256(&"value1".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_REQUESTER_ID),
        };
        let metadata2 = MetadataItemParams {
            name: sha256(&"name".as_bytes().to_vec()),
            value: sha256(&"value2".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_REQUESTER_ID),
        };
        assert_err!(
            LogionLoc::create_polkadot_identity_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, ID_LOC_DEFAULT_LEGAL_FEE, ItemsParams::only_metadata(Vec::from([ metadata1.clone(), metadata2.clone() ]))),
            Error::<Test>::DuplicateLocMetadata
        );
    });
}

#[test]
fn it_fails_creating_identity_loc_with_invalid_metadata_submitter() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let metadata = MetadataItemParams {
            name: sha256(&vec![1, 2, 3]),
            value: sha256(&vec![4, 5, 6]),
            submitter: SupportedAccountId::Polkadot(LOC_OWNER1),
        };
        assert_err!(
            LogionLoc::create_polkadot_identity_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, ID_LOC_DEFAULT_LEGAL_FEE, ItemsParams::only_metadata(Vec::from([ metadata.clone() ]))),
            Error::<Test>::CannotSubmit
        );
    });
}

#[test]
fn it_creates_identity_loc_with_initial_files() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let file = FileParams {
            hash: sha256(&"test".as_bytes().to_vec()),
            nature: sha256(&"Test".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_REQUESTER_ID),
            size: 4,
        };
        assert_ok!(LogionLoc::create_polkadot_identity_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, ID_LOC_DEFAULT_LEGAL_FEE, ItemsParams::only_files(Vec::from([ file.clone() ]))));
        let loc = LogionLoc::loc(LOC_ID).unwrap();
        assert_eq!(loc.files[0], expected_file(&file, NOT_ACKNOWLEDGED, NOT_ACKNOWLEDGED));
    });
}

#[test]
fn it_fails_creating_identity_loc_with_duplicate_initial_files() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let file1 = FileParams {
            hash: sha256(&"test".as_bytes().to_vec()),
            nature: sha256(&"Test 1".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_REQUESTER_ID),
            size: 4,
        };
        let file2 = FileParams {
            hash: sha256(&"test".as_bytes().to_vec()),
            nature: sha256(&"Test 2".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_REQUESTER_ID),
            size: 4,
        };
        assert_err!(
            LogionLoc::create_polkadot_identity_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, ID_LOC_DEFAULT_LEGAL_FEE, ItemsParams::only_files(Vec::from([ file1, file2 ]))),
            Error::<Test>::DuplicateLocFile
        );
    });
}

#[test]
fn it_fails_creating_identity_loc_with_invalid_file_submitter() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let file = FileParams {
            hash: sha256(&"test".as_bytes().to_vec()),
            nature: sha256(&"Test".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_OWNER1),
            size: 4,
        };
        assert_err!(
            LogionLoc::create_polkadot_identity_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, ID_LOC_DEFAULT_LEGAL_FEE, ItemsParams::only_files(Vec::from([ file ]))),
            Error::<Test>::CannotSubmit
        );
    });
}

#[test]
fn it_creates_identity_loc_with_initial_links() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_polkadot_identity_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), OTHER_LOC_ID, LOC_OWNER1, ID_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        let link = LocLinkParams {
            id: OTHER_LOC_ID,
            nature: sha256(&"test-link-nature".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_REQUESTER_ID),
        };
        assert_ok!(LogionLoc::create_polkadot_identity_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, ID_LOC_DEFAULT_LEGAL_FEE, ItemsParams::only_links(Vec::from([ link.clone() ]))));
        let loc = LogionLoc::loc(LOC_ID).unwrap();
        assert_eq!(loc.links[0], expected_link(&link, NOT_ACKNOWLEDGED, NOT_ACKNOWLEDGED));
    });
}

#[test]
fn it_fails_creating_identity_loc_with_initial_links_if_not_found() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let link = LocLinkParams {
            id: OTHER_LOC_ID,
            nature: sha256(&"test-link-nature".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_REQUESTER_ID),
        };
        assert_err!(
            LogionLoc::create_polkadot_identity_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, ID_LOC_DEFAULT_LEGAL_FEE, ItemsParams::only_links(Vec::from([ link.clone() ]))),
            Error::<Test>::LinkedLocNotFound
        );
    });
}

#[test]
fn it_fails_creating_identity_loc_with_duplicate_initial_links() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let link1 = LocLinkParams {
            id: OTHER_LOC_ID,
            nature: sha256(&"test-link1-nature".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_REQUESTER_ID),
        };
        let link2 = LocLinkParams {
            id: OTHER_LOC_ID,
            nature: sha256(&"test-link2-nature".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_REQUESTER_ID),
        };
        assert_err!(
            LogionLoc::create_polkadot_identity_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, ID_LOC_DEFAULT_LEGAL_FEE, ItemsParams::only_links(Vec::from([ link1, link2 ]))),
            Error::<Test>::DuplicateLocLink
        );
    });
}

#[test]
fn it_fails_creating_identity_loc_with_invalid_link_submitter() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let link = LocLinkParams {
            id: OTHER_LOC_ID,
            nature: sha256(&"test-link-nature".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_OWNER1),
        };
        assert_err!(
            LogionLoc::create_polkadot_identity_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, ID_LOC_DEFAULT_LEGAL_FEE, ItemsParams::only_links(Vec::from([ link ]))),
            Error::<Test>::CannotSubmit
        );
    });
}

#[test]
fn it_creates_collection_loc_with_initial_metadata() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let metadata = MetadataItemParams {
            name: sha256(&vec![1, 2, 3]),
            value: sha256(&vec![4, 5, 6]),
            submitter: SupportedAccountId::Polkadot(LOC_REQUESTER_ID),
        };
        assert_ok!(LogionLoc::create_collection_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, None, Some(10), true, 0, OTHER_LOC_DEFAULT_LEGAL_FEE, 0, 0, ItemsParams::only_metadata(Vec::from([ metadata.clone() ]))));
        let loc = LogionLoc::loc(LOC_ID).unwrap();
        assert_eq!(loc.metadata[0], expected_metadata(metadata, NOT_ACKNOWLEDGED, NOT_ACKNOWLEDGED));
    });
}

#[test]
fn it_fails_creating_collection_loc_with_duplicate_initial_metadata() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let metadata1 = MetadataItemParams {
            name: sha256(&"name".as_bytes().to_vec()),
            value: sha256(&"value1".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_REQUESTER_ID),
        };
        let metadata2 = MetadataItemParams {
            name: sha256(&"name".as_bytes().to_vec()),
            value: sha256(&"value2".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_REQUESTER_ID),
        };
        assert_err!(
            LogionLoc::create_collection_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, None, Some(10), true, 0, OTHER_LOC_DEFAULT_LEGAL_FEE, 0, 0, ItemsParams::only_metadata(Vec::from([ metadata1.clone(), metadata2.clone() ]))),
            Error::<Test>::DuplicateLocMetadata
        );
    });
}

#[test]
fn it_fails_creating_collection_loc_with_invalid_metadata_submitter() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let metadata = MetadataItemParams {
            name: sha256(&vec![1, 2, 3]),
            value: sha256(&vec![4, 5, 6]),
            submitter: SupportedAccountId::Polkadot(LOC_OWNER1),
        };
        assert_err!(
            LogionLoc::create_collection_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, None, Some(10), true, 0, OTHER_LOC_DEFAULT_LEGAL_FEE, 0, 0, ItemsParams::only_metadata(Vec::from([ metadata.clone() ]))),
            Error::<Test>::CannotSubmit
        );
    });
}

#[test]
fn it_creates_collection_loc_with_initial_files() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let file = FileParams {
            hash: sha256(&"test".as_bytes().to_vec()),
            nature: sha256(&"Test".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_REQUESTER_ID),
            size: 4,
        };
        assert_ok!(LogionLoc::create_collection_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, None, Some(10), true, 0, OTHER_LOC_DEFAULT_LEGAL_FEE, 0, 0, ItemsParams::only_files(Vec::from([ file.clone() ]))));
        let loc = LogionLoc::loc(LOC_ID).unwrap();
        assert_eq!(loc.files[0], expected_file(&file, NOT_ACKNOWLEDGED, NOT_ACKNOWLEDGED));
    });
}

#[test]
fn it_fails_creating_collection_loc_with_duplicate_initial_files() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let file1 = FileParams {
            hash: sha256(&"test".as_bytes().to_vec()),
            nature: sha256(&"Test 1".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_REQUESTER_ID),
            size: 4,
        };
        let file2 = FileParams {
            hash: sha256(&"test".as_bytes().to_vec()),
            nature: sha256(&"Test 2".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_REQUESTER_ID),
            size: 4,
        };
        assert_err!(
            LogionLoc::create_collection_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, None, Some(10), true, 0, OTHER_LOC_DEFAULT_LEGAL_FEE, 0, 0, ItemsParams::only_files(Vec::from([ file1, file2 ]))),
            Error::<Test>::DuplicateLocFile
        );
    });
}

#[test]
fn it_fails_creating_collection_loc_with_invalid_file_submitter() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let file = FileParams {
            hash: sha256(&"test".as_bytes().to_vec()),
            nature: sha256(&"Test".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_OWNER1),
            size: 4,
        };
        assert_err!(
            LogionLoc::create_collection_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, None, Some(10), true, 0, OTHER_LOC_DEFAULT_LEGAL_FEE, 0, 0, ItemsParams::only_files(Vec::from([ file ]))),
            Error::<Test>::CannotSubmit
        );
    });
}

#[test]
fn it_creates_collection_loc_with_initial_links() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        assert_ok!(LogionLoc::create_polkadot_identity_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), OTHER_LOC_ID, LOC_OWNER1, ID_LOC_DEFAULT_LEGAL_FEE, ItemsParams::empty()));
        let link = LocLinkParams {
            id: OTHER_LOC_ID,
            nature: sha256(&"test-link-nature".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_REQUESTER_ID),
        };
        assert_ok!(LogionLoc::create_collection_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, None, Some(10), true, 0, OTHER_LOC_DEFAULT_LEGAL_FEE, 0, 0, ItemsParams::only_links(Vec::from([ link.clone() ]))));
        let loc = LogionLoc::loc(LOC_ID).unwrap();
        assert_eq!(loc.links[0], expected_link(&link, NOT_ACKNOWLEDGED, NOT_ACKNOWLEDGED));
    });
}

#[test]
fn it_fails_creating_collection_loc_with_initial_links_if_not_found() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let link = LocLinkParams {
            id: OTHER_LOC_ID,
            nature: sha256(&"test-link-nature".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_REQUESTER_ID),
        };
        assert_err!(
            LogionLoc::create_collection_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, None, Some(10), true, 0, OTHER_LOC_DEFAULT_LEGAL_FEE, 0, 0, ItemsParams::only_links(Vec::from([ link.clone() ]))),
            Error::<Test>::LinkedLocNotFound
        );
    });
}

#[test]
fn it_fails_creating_collection_loc_with_duplicate_initial_links() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let link1 = LocLinkParams {
            id: OTHER_LOC_ID,
            nature: sha256(&"test-link1-nature".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_REQUESTER_ID),
        };
        let link2 = LocLinkParams {
            id: OTHER_LOC_ID,
            nature: sha256(&"test-link2-nature".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_REQUESTER_ID),
        };
        assert_err!(
            LogionLoc::create_collection_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, None, Some(10), true, 0, OTHER_LOC_DEFAULT_LEGAL_FEE, 0, 0, ItemsParams::only_links(Vec::from([ link1, link2 ]))),
            Error::<Test>::DuplicateLocLink
        );
    });
}

#[test]
fn it_fails_creating_collection_loc_with_invalid_link_submitter() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let link = LocLinkParams {
            id: OTHER_LOC_ID,
            nature: sha256(&"test-link-nature".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_OWNER1),
        };
        assert_err!(
            LogionLoc::create_collection_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, None, Some(10), true, 0, OTHER_LOC_DEFAULT_LEGAL_FEE, 0, 0, ItemsParams::only_links(Vec::from([ link ]))),
            Error::<Test>::CannotSubmit
        );
    });
}

#[test]
fn it_applies_storage_fees_on_transaction_creation() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let snapshot = BalancesSnapshot::take(LOC_REQUESTER_ID, LOC_OWNER1);
        let file = FileParams {
            hash: sha256(&"test".as_bytes().to_vec()),
            nature: sha256(&"Test".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_REQUESTER_ID),
            size: 4,
        };
        assert_ok!(LogionLoc::create_polkadot_transaction_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, 0u128, ItemsParams::only_files(Vec::from([ file.clone() ]))));
        let fees = Fees::only_storage(1, 4);
        fees.assert_balances_events(snapshot);
    });
}

#[test]
fn it_applies_storage_fees_on_identity_creation() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let snapshot = BalancesSnapshot::take(LOC_REQUESTER_ID, LOC_OWNER1);
        let file = FileParams {
            hash: sha256(&"test".as_bytes().to_vec()),
            nature: sha256(&"Test".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_REQUESTER_ID),
            size: 4,
        };
        assert_ok!(LogionLoc::create_polkadot_identity_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, 0u128, ItemsParams::only_files(Vec::from([ file.clone() ]))));
        let fees = Fees::only_storage(1, 4);
        fees.assert_balances_events(snapshot);
    });
}

#[test]
fn it_applies_storage_fees_on_collection_creation() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let snapshot = BalancesSnapshot::take(LOC_REQUESTER_ID, LOC_OWNER1);
        let file = FileParams {
            hash: sha256(&"test".as_bytes().to_vec()),
            nature: sha256(&"Test".as_bytes().to_vec()),
            submitter: SupportedAccountId::Polkadot(LOC_REQUESTER_ID),
            size: 4,
        };
        assert_ok!(LogionLoc::create_collection_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, None, Some(10), true, 0, 0u128, 0, 0, ItemsParams::only_files(Vec::from([ file ]))));
        let fees = Fees::only_storage(1, 4);
        fees.assert_balances_events(snapshot);
    });
}

#[test]
fn it_applies_collection_item_fee() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let collection_item_fee: Balance = 5 * ONE_LGNT;
        assert_ok!(LogionLoc::create_collection_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, None, Some(10), false, 0, OTHER_LOC_DEFAULT_LEGAL_FEE, collection_item_fee, 0, ItemsParams::empty()));
        assert_ok!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, None, false));

        let snapshot = BalancesSnapshot::take(LOC_REQUESTER_ID, LOC_OWNER1);
        let collection_item_id = BlakeTwo256::hash_of(&"item-id".as_bytes().to_vec());
        let collection_item_description = sha256(&"item-description".as_bytes().to_vec());
        assert_ok!(LogionLoc::add_collection_item(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, collection_item_id, collection_item_description.clone(), vec![], None, false, Vec::new()));
        let fees = Fees::only_collection_item(collection_item_fee, Beneficiary::LegalOfficer(LOC_OWNER1));
        fees.assert_balances_events(snapshot);
    });
}

#[test]
fn it_fails_to_add_collection_item_when_insufficient_funds() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        let collection_item_fee: Balance = INITIAL_BALANCE;
        assert_ok!(LogionLoc::create_collection_loc(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, LOC_OWNER1, None, Some(10), false, 0, OTHER_LOC_DEFAULT_LEGAL_FEE, collection_item_fee, 0, ItemsParams::empty()));
        assert_ok!(LogionLoc::close(RuntimeOrigin::signed(LOC_OWNER1), LOC_ID, None, false));

        let collection_item_id = BlakeTwo256::hash_of(&"item-id".as_bytes().to_vec());
        let collection_item_description = sha256(&"item-description".as_bytes().to_vec());
        assert_err!(LogionLoc::add_collection_item(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, collection_item_id, collection_item_description.clone(), vec![], None, false, Vec::new()), Error::<Test>::InsufficientFunds);
    });
}

#[test]
fn it_fails_to_add_tokens_record_when_insufficient_funds() {
    new_test_ext().execute_with(|| {
        setup_default_balances();
        create_closed_collection_with_selected_issuer();
        let record_id = build_record_id();
        let record_description = build_record_description();
        let record_files = build_record_files(1);

        let storage_fees = Fees::storage_fees(1, record_files[0].size);
        set_balance(LOC_REQUESTER_ID, 2 * storage_fees); // Requester can pay storage but not tokens record

        assert_err!(LogionLoc::add_tokens_record(RuntimeOrigin::signed(LOC_REQUESTER_ID), LOC_ID, record_id, record_description.clone(), record_files.clone()), Error::<Test>::InsufficientFunds);
    });
}
