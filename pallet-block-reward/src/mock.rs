use crate::{self as pallet_block_reward, NegativeImbalanceOf};

use frame_support::{
    construct_runtime, parameter_types, traits::Currency,
};

use sp_core::H256;
use sp_runtime::{
    generic,
    testing::Header,
    traits::{BlakeTwo256, IdentityLookup},
    Percent,
    BuildStorage,
};
use logion_shared::{DistributionKey, RewardDistributor};

pub type AccountId = u64;
pub type Balance = u128;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;

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

impl frame_system::Config for Test {
    type Block = generic::Block<Header, UncheckedExtrinsic>;
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type RuntimeOrigin = RuntimeOrigin;
    type Nonce = u64;
    type RuntimeCall = RuntimeCall;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = BlockHashCount;
    type DbWeight = ();
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
    type FreezeIdentifier = [u8; 8];
    type MaxFreezes = MaxFreezes;
    type MaxHolds = MaxHolds;
    type RuntimeHoldReason = [u8; 8];
    type WeightInfo = ();
}

// Fake accounts used to simulate reward beneficiaries balances
pub const COMMUNITY_TREASURY_ACCOUNT: AccountId = 2;
pub const COLLATORS_ACCOUNT_1: AccountId = 3;
pub const COLLATORS_ACCOUNT_2: AccountId = 4;
pub const LOGION_TREASURY_ACCOUNT: AccountId = 5;
pub const COLLATORS_ACCOUNT_3: AccountId = 6;
pub const COLLATORS_ACCOUNT_4: AccountId = 7;
pub const COLLATORS_ACCOUNT_5: AccountId = 8;

// Type used as beneficiary payout handle
pub struct RewardDistributorImpl();
impl RewardDistributor<NegativeImbalanceOf<Test>, Balance, AccountId>
for RewardDistributorImpl
{
    fn payout_community_treasury(reward: NegativeImbalanceOf<Test>) {
        Balances::resolve_creating(&COMMUNITY_TREASURY_ACCOUNT, reward);
    }

    fn get_collators() -> Vec<AccountId> {
        vec![COLLATORS_ACCOUNT_1, COLLATORS_ACCOUNT_2, COLLATORS_ACCOUNT_3, COLLATORS_ACCOUNT_4, COLLATORS_ACCOUNT_5]
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
        collators_percent: Percent::from_percent(35),
        community_treasury_percent: Percent::from_percent(30),
        logion_treasury_percent: Percent::from_percent(35),
        loc_owner_percent: Percent::from_percent(0),
    };
}

impl pallet_block_reward::Config for Test {
    type Currency = Balances;
    type RewardAmount = RewardAmount;
    type RewardDistributor = RewardDistributorImpl;
    type DistributionKey = RewardDistributionKey;
}

pub fn new_test_ext() -> sp_io::TestExternalities {
    let t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();
    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| System::set_block_number(1));
    ext
}
