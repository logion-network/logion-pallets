#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
    dispatch::{GetDispatchInfo},
    Parameter,
    traits::{EnsureOrigin, UnfilteredDispatchable, Imbalance},
};
use frame_support::dispatch::DispatchResultWithPostInfo;
use frame_support::sp_runtime::Percent;
use frame_support::traits::tokens::Balance;
use frame_system::{ensure_signed, RawOrigin};
use scale_info::TypeInfo;
use sp_std::{boxed::Box, vec::Vec};
use sp_weights::Weight;

#[cfg(test)]
mod tests;

pub trait CreateRecoveryCallFactory<Origin, AccountId, BlockNumber> {
    type Call: Parameter + UnfilteredDispatchable<RuntimeOrigin = Origin> + GetDispatchInfo;

    fn build_create_recovery_call(legal_officers: Vec<AccountId>, threshold: u16, delay_period: BlockNumber) -> Self::Call;
}

pub struct LegalOfficerCaseSummary<AccountId> {
    pub owner: AccountId,
    pub requester: Option<AccountId>,
}

pub trait LocQuery<LocId, AccountId> {
    fn has_closed_identity_locs(account: &AccountId, legal_officer: &Vec<AccountId>) -> bool;
    fn get_loc(loc_id: &LocId) -> Option<LegalOfficerCaseSummary<AccountId>>;
}

pub trait LocValidity<LocId, AccountId> {
    fn loc_valid_with_owner(loc_id: &LocId, legal_officer: &AccountId) -> bool;
}

pub trait MultisigApproveAsMultiCallFactory<Origin, AccountId, Timepoint> {
    type Call: Parameter + UnfilteredDispatchable<RuntimeOrigin = Origin> + GetDispatchInfo;

    fn build_approve_as_multi_call(
        threshold: u16,
        other_signatories: Vec<AccountId>,
        maybe_timepoint: Option<Timepoint>,
        call_hash: [u8; 32],
        max_weight: Weight,
    ) -> Self::Call;
}

pub trait MultisigAsMultiCallFactory<Origin, AccountId, Timepoint> {
    type Call: Parameter + UnfilteredDispatchable<RuntimeOrigin = Origin> + GetDispatchInfo;

    fn build_as_multi_call(
        threshold: u16,
        other_signatories: Vec<AccountId>,
        maybe_timepoint: Option<Timepoint>,
        call: Box<Self::Call>,
        max_weight: Weight,
    ) -> Self::Call;
}

pub trait IsLegalOfficer<AccountId: PartialEq, Origin: Clone + Into<Result<RawOrigin<AccountId>, Origin>>>: EnsureOrigin<Origin, Success = AccountId> {
    fn is_legal_officer(account: &AccountId) -> bool {
        Self::legal_officers().contains(account)
    }

    fn try_origin(o: Origin) -> Result<AccountId, Origin> {
        let result = ensure_signed(o.clone());
        match result {
            Ok(who) => {
                if Self::is_legal_officer(&who) {
                    Ok(who)
                } else {
                    Err(o)
                }
            },
            Err(_) => Err(o)
        }
    }

    fn legal_officers() -> Vec<AccountId>;
}

pub trait LegalOfficerCreation<AccountId> {
    fn add_guest_legal_officer(
        guest_legal_officer_id: AccountId,
        host_legal_officer_id: AccountId,
    ) -> DispatchResultWithPostInfo;
}

#[derive(Debug, PartialEq)]
pub struct DistributionKey {
    pub community_treasury_percent: Percent,
    pub collators_percent: Percent,
    pub logion_treasury_percent: Percent,
    pub loc_owner_percent: Percent,
}

impl DistributionKey {

    pub fn is_valid(&self) -> bool {
        let mut should_become_zero = Self::into_signed(Percent::one());

        should_become_zero = should_become_zero - Self::into_signed(self.community_treasury_percent);
        should_become_zero = should_become_zero - Self::into_signed(self.collators_percent);
        should_become_zero = should_become_zero - Self::into_signed(self.logion_treasury_percent);
        should_become_zero = should_become_zero - Self::into_signed(self.loc_owner_percent);

        should_become_zero == 0
    }

    pub fn is_valid_without_loc_owner(&self) -> bool {
        self.loc_owner_percent == Percent::zero() && self.is_valid()
    }

    fn into_signed(percent: Percent) -> i16 {
        <u8 as Into<i16>>::into(percent.deconstruct())
    }
}

pub trait RewardDistributor<I: Imbalance<B>, B: Balance, AccountId: Clone> {

    fn payout_collators(reward: I);

    fn payout_community_treasury(reward: I);

    fn payout_logion_treasury(reward: I);

    fn payout_to(reward: I, account: &AccountId);

    fn distribute_with_loc_owner(amount: I, distribution_key: DistributionKey, loc_owner: &AccountId) -> (Beneficiary<AccountId>, B) {
        Self::_distribute(amount, distribution_key, Some(loc_owner))
    }

    fn distribute(amount: I, distribution_key: DistributionKey) {
        Self::_distribute(amount, distribution_key, None);
    }

    fn _distribute(amount: I, distribution_key: DistributionKey, loc_owner: Option<&AccountId>) -> (Beneficiary<AccountId>, B)  {
        let amount_balance = amount.peek();

        let collators_part = distribution_key.collators_percent * amount_balance;
        let logion_treasury_part = distribution_key.logion_treasury_percent * amount_balance;
        let loc_owner_part = distribution_key.loc_owner_percent * amount_balance;

        let (collators_imbalance, remainder1) = amount.split(collators_part);
        let (loc_owner_imbalance, remainder2) = remainder1.split(loc_owner_part);
        let (logion_treasury_imbalance, community_treasury_imbalance) = remainder2.split(logion_treasury_part);

        Self::payout_community_treasury(community_treasury_imbalance);
        Self::payout_collators(collators_imbalance);
        Self::payout_logion_treasury(logion_treasury_imbalance);
        match loc_owner {
            Some(account) => {
                if distribution_key.loc_owner_percent != Percent::zero() {
                    let received = loc_owner_imbalance.peek();
                    Self::payout_to(loc_owner_imbalance, account);
                    (Beneficiary::LegalOfficer(account.clone()), received)
                } else {
                    (Beneficiary::Other, B::zero())
                }
            }
            None => {
                (Beneficiary::Other, B::zero())
            }
        }
    }

}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, Copy)]
pub enum Beneficiary<AccountId> {
    Other,
    LegalOfficer(AccountId),
}
