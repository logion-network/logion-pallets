use frame_support::sp_runtime::Percent;
use crate::DistributionKey;

#[test]
fn distribution_key_with_only_reserve_is_valid() {
    let key = DistributionKey {
        reserve_percent: Percent::from_percent(100),
        stakers_percent: Percent::from_percent(0),
        collators_percent: Percent::from_percent(0),
        treasury_percent: Percent::from_percent(0),
        loc_owner_percent: Percent::from_percent(0),
    };
    assert!(key.is_valid());
    assert!(key.is_valid_without_loc_owner());
}

#[test]
fn distribution_key_with_only_stakers_is_valid() {
    let key = DistributionKey {
        reserve_percent: Percent::from_percent(0),
        stakers_percent: Percent::from_percent(100),
        collators_percent: Percent::from_percent(0),
        treasury_percent: Percent::from_percent(0),
        loc_owner_percent: Percent::from_percent(0),
    };
    assert!(key.is_valid());
    assert!(key.is_valid_without_loc_owner());
}

#[test]
fn distribution_key_with_only_collators_is_valid() {
    let key = DistributionKey {
        reserve_percent: Percent::from_percent(0),
        stakers_percent: Percent::from_percent(0),
        collators_percent: Percent::from_percent(100),
        treasury_percent: Percent::from_percent(0),
        loc_owner_percent: Percent::from_percent(0),
    };
    assert!(key.is_valid());
}

#[test]
fn distribution_key_is_valid_only_with_loc_owner() {
    let key = DistributionKey {
        reserve_percent: Percent::from_percent(40),
        stakers_percent: Percent::from_percent(30),
        collators_percent: Percent::from_percent(20),
        treasury_percent: Percent::from_percent(6),
        loc_owner_percent: Percent::from_percent(4),
    };
    assert!(key.is_valid());
    assert!(!key.is_valid_without_loc_owner());
}

#[test]
fn distribution_key_invalid_lower_than_hundred() {
    let key = DistributionKey {
        reserve_percent: Percent::from_percent(49),
        stakers_percent: Percent::from_percent(30),
        collators_percent: Percent::from_percent(20),
        treasury_percent: Percent::from_percent(0),
        loc_owner_percent: Percent::from_percent(0),
    };
    assert!(!key.is_valid());
    assert!(!key.is_valid_without_loc_owner());
}

#[test]
fn distribution_key_invalid_greater_than_hundred() {
    let key = DistributionKey {
        reserve_percent: Percent::from_percent(51),
        stakers_percent: Percent::from_percent(30),
        collators_percent: Percent::from_percent(20),
        treasury_percent: Percent::from_percent(0),
        loc_owner_percent: Percent::from_percent(0),
    };
    assert!(!key.is_valid());
    assert!(!key.is_valid_without_loc_owner());
}

