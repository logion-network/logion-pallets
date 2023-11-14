use frame_support::sp_runtime::Percent;
use crate::{DistributionKey, RewardDistributor};

#[test]
fn distribution_key_with_only_community_treasury_is_valid() {
    let key = DistributionKey {
        community_treasury_percent: Percent::from_percent(100),
        collators_percent: Percent::from_percent(0),
        logion_treasury_percent: Percent::from_percent(0),
        loc_owner_percent: Percent::from_percent(0),
    };
    assert!(key.is_valid());
    assert!(key.is_valid_without_loc_owner());
}

#[test]
fn distribution_key_with_only_collators_is_valid() {
    let key = DistributionKey {
        community_treasury_percent: Percent::from_percent(0),
        collators_percent: Percent::from_percent(100),
        logion_treasury_percent: Percent::from_percent(0),
        loc_owner_percent: Percent::from_percent(0),
    };
    assert!(key.is_valid());
}

#[test]
fn distribution_key_is_valid_only_with_loc_owner() {
    let key = DistributionKey {
        community_treasury_percent: Percent::from_percent(40),
        collators_percent: Percent::from_percent(20),
        logion_treasury_percent: Percent::from_percent(30),
        loc_owner_percent: Percent::from_percent(10),
    };
    assert!(key.is_valid());
    assert!(!key.is_valid_without_loc_owner());
}

#[test]
fn distribution_key_invalid_lower_than_hundred() {
    let key = DistributionKey {
        community_treasury_percent: Percent::from_percent(49),
        collators_percent: Percent::from_percent(20),
        logion_treasury_percent: Percent::from_percent(0),
        loc_owner_percent: Percent::from_percent(0),
    };
    assert!(!key.is_valid());
    assert!(!key.is_valid_without_loc_owner());
}

#[test]
fn distribution_key_invalid_greater_than_hundred() {
    let key = DistributionKey {
        community_treasury_percent: Percent::from_percent(51),
        collators_percent: Percent::from_percent(20),
        logion_treasury_percent: Percent::from_percent(0),
        loc_owner_percent: Percent::from_percent(0),
    };
    assert!(!key.is_valid());
    assert!(!key.is_valid_without_loc_owner());
}

