use crate::{mock::*, Error};
use frame_support::{assert_err, assert_ok};

#[test]
fn it_creates_recovery_config_if_both_closed() {
    new_test_ext().execute_with(|| {
        assert_ok!(VerifiedRecovery::create_recovery(RuntimeOrigin::signed(requester()), legal_officers_closed()));
    });
}

#[test]
fn it_fails_creating_recovery_config_if_both_open_or_pending() {
    new_test_ext().execute_with(|| {
        assert_err!(VerifiedRecovery::create_recovery(RuntimeOrigin::signed(requester()), legal_officers_not_closed()), Error::<Test>::MissingIdentityLoc);
    });
}

#[test]
fn it_fails_creating_recovery_config_if_first_open_or_pending() {
    new_test_ext().execute_with(|| {
        assert_err!(VerifiedRecovery::create_recovery(RuntimeOrigin::signed(requester()), vec![legal_officer(3), legal_officer(2)]), Error::<Test>::MissingIdentityLoc);
    });
}

#[test]
fn it_fails_creating_recovery_config_if_second_open_or_pending() {
    new_test_ext().execute_with(|| {
        assert_err!(VerifiedRecovery::create_recovery(RuntimeOrigin::signed(requester()), vec![legal_officer(1), legal_officer(4)]), Error::<Test>::MissingIdentityLoc);
    });
}
