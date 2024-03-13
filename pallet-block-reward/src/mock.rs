use crate::{self as pallet_block_reward, NegativeImbalanceOf};

use frame_support::{
    derive_impl, construct_runtime, parameter_types, traits::Currency,
};
#[cfg(feature = "runtime-benchmarks")]
use frame_support::dispatch::RawOrigin;
use frame_support::traits::EnsureOrigin;
use frame_system as system;

use sp_core::H256;
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup},
    Percent,
    BuildStorage,
};
use logion_shared::{DistributionKey, IsLegalOfficer, RewardDistributor};

pub type AccountId = u64;
pub type Balance = u128;

type Block = frame_system::mocking::MockBlock<Test>;

construct_runtime!(
    pub struct Test {
        System: frame_system,
        Balances: pallet_balances,
        BlockReward: pallet_block_reward,
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
}

#[derive_impl(frame_system::config_preludes::TestDefaultConfig as frame_system::DefaultConfig)]
impl system::Config for Test {
    type Block = Block;
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
    pub const MaxLocks: u32 = 2;
    pub const MaxReserves: u32 = 2;
    pub const ExistentialDeposit: Balance = 2;
    pub const MaxFreezes: u32 = 2;
    pub const MaxHolds: u32 = 2;
}

impl pallet_balances::Config for Test {
    type MaxLocks = MaxLocks;
    type MaxReserves = MaxReserves;
    type ReserveIdentifier = [u8; 8];
    type Balance = Balance;
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type FreezeIdentifier = ();
    type MaxFreezes = MaxFreezes;
    type RuntimeHoldReason = ();
    type RuntimeFreezeReason = ();
    type WeightInfo = ();
}

// Fake accounts used to simulate reward beneficiaries balances
pub const COMMUNITY_TREASURY_ACCOUNT: AccountId = 2;
pub const LEGAL_OFFICER_ACCOUNT_1: AccountId = 3;
pub const LEGAL_OFFICER_ACCOUNT_2: AccountId = 4;
pub const LOGION_TREASURY_ACCOUNT: AccountId = 5;
pub const LEGAL_OFFICER_ACCOUNT_3: AccountId = 6;
pub const LEGAL_OFFICER_ACCOUNT_4: AccountId = 7;
pub const LEGAL_OFFICER_ACCOUNT_5: AccountId = 8;

// Type used as beneficiary payout handle
pub struct RewardDistributorImpl();
impl RewardDistributor<NegativeImbalanceOf<Test>, Balance, AccountId, RuntimeOrigin, LoAuthorityListMock>
for RewardDistributorImpl
{
    fn payout_community_treasury(reward: NegativeImbalanceOf<Test>) {
        Balances::resolve_creating(&COMMUNITY_TREASURY_ACCOUNT, reward);
    }

    fn payout_logion_treasury(reward: NegativeImbalanceOf<Test>) {
        Balances::resolve_creating(&LOGION_TREASURY_ACCOUNT, reward);
    }

    fn payout_to(reward: NegativeImbalanceOf<Test>, account: &AccountId) {
        Balances::resolve_creating(account, reward);
    }
}

pub const BLOCK_REWARD: Balance = 10_000_000_000_000_000_000; // 10 LGNT

parameter_types! {
    pub const RewardAmount: Balance = BLOCK_REWARD;
    pub const RewardDistributionKey: DistributionKey = DistributionKey {
        legal_officers_percent: Percent::from_percent(35),
        community_treasury_percent: Percent::from_percent(30),
        logion_treasury_percent: Percent::from_percent(35),
        loc_owner_percent: Percent::from_percent(0),
    };
}

#[cfg(feature = "runtime-benchmarks")]
pub type OuterOrigin<T> = <T as frame_system::Config>::RuntimeOrigin;

pub struct LoAuthorityListMock;
impl EnsureOrigin<RuntimeOrigin> for LoAuthorityListMock {
    type Success = <Test as system::Config>::AccountId;

    fn try_origin(o: <Test as system::Config>::RuntimeOrigin) -> Result<Self::Success, <Test as system::Config>::RuntimeOrigin> {
        <Self as IsLegalOfficer<<Test as system::Config>::AccountId, <Test as system::Config>::RuntimeOrigin>>::try_origin(o)
    }

	#[cfg(feature = "runtime-benchmarks")]
	fn try_successful_origin() -> Result<RuntimeOrigin, ()> {
		Ok(OuterOrigin::<Test>::from(RawOrigin::Signed(LEGAL_OFFICER_ACCOUNT_1)))
	}
}

impl IsLegalOfficer<<Test as system::Config>::AccountId, RuntimeOrigin> for LoAuthorityListMock {

    fn legal_officers() -> Vec<<Test as system::Config>::AccountId> {
        vec![LEGAL_OFFICER_ACCOUNT_1, LEGAL_OFFICER_ACCOUNT_2, LEGAL_OFFICER_ACCOUNT_3, LEGAL_OFFICER_ACCOUNT_4, LEGAL_OFFICER_ACCOUNT_5]
    }
}

impl pallet_block_reward::Config for Test {
    type Currency = Balances;
    type RewardAmount = RewardAmount;
    type RewardDistributor = RewardDistributorImpl;
    type DistributionKey = RewardDistributionKey;
    type IsLegalOfficer = LoAuthorityListMock;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
    let t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();
    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| System::set_block_number(1));
    ext
}
