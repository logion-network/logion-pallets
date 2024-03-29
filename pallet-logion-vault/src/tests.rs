use crate::{mock::*, Error};
use frame_support::{assert_err, assert_ok};
use sp_runtime::traits::BlakeTwo256;
use sp_runtime::traits::Hash;
use sp_weights::Weight;

#[test]
fn it_requests_call_if_not_legal_officer() {
    new_test_ext().execute_with(|| {
        let call_hash = BlakeTwo256::hash_of(&"call-bytes".as_bytes().to_vec());
        assert_ok!(Vault::request_call(RuntimeOrigin::signed(USER_ID), vec![LEGAL_OFFICER1, LEGAL_OFFICER2], call_hash.to_fixed_bytes(), Weight::from_parts(10000, 0)));
    });
}

#[test]
fn it_fails_requesting_call_if_legal_officer() {
    new_test_ext().execute_with(|| {
        let call_hash = BlakeTwo256::hash_of(&"call-bytes".as_bytes().to_vec());
        assert_err!(Vault::request_call(RuntimeOrigin::signed(LEGAL_OFFICER1), vec![LEGAL_OFFICER1, LEGAL_OFFICER2], call_hash.to_fixed_bytes(), Weight::from_parts(10000, 0)), Error::<Test>::WrongInitiator);
    });
}

#[test]
fn it_fails_requesting_call_if_not_two_legal_officers() {
    new_test_ext().execute_with(|| {
        let call_hash = BlakeTwo256::hash_of(&"call-bytes".as_bytes().to_vec());
        assert_err!(Vault::request_call(RuntimeOrigin::signed(USER_ID), vec![LEGAL_OFFICER1], call_hash.to_fixed_bytes(), Weight::from_parts(10000, 0)), Error::<Test>::InvalidSignatories);
    });
}

#[test]
fn it_fails_requesting_call_if_not_all_legal_officers() {
    new_test_ext().execute_with(|| {
        let call_hash = BlakeTwo256::hash_of(&"call-bytes".as_bytes().to_vec());
        assert_err!(Vault::request_call(RuntimeOrigin::signed(USER_ID), vec![LEGAL_OFFICER1, ANOTHER_USER_ID], call_hash.to_fixed_bytes(), Weight::from_parts(10000, 0)), Error::<Test>::InvalidSignatories);
    });
}

#[test]
fn it_approves_call_if_two_other_signatories() {
    new_test_ext().execute_with(|| {
        let call = Box::new(RuntimeCall::System(frame_system::Call::remark{ remark : Vec::from([0u8]) }));
        let timepoint = Default::default();
        assert_ok!(Vault::approve_call(RuntimeOrigin::signed(LEGAL_OFFICER1), vec![USER_ID, LEGAL_OFFICER2], call, timepoint, Weight::from_parts(10000, 0)));
    });
}
