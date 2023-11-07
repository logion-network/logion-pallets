use crate::{self as pallet_loc, LocType, NegativeImbalanceOf, RequesterOf, Hasher};
use logion_shared::{Beneficiary, DistributionKey, EuroCent, IsLegalOfficer, LegalFee};
use sp_core::hash::H256;
use frame_support::{construct_runtime, parameter_types, traits::{EnsureOrigin, Currency}};
use sp_io::hashing::sha2_256;
use sp_runtime::{traits::{BlakeTwo256, IdentityLookup}, testing::Header, Percent, generic, BuildStorage};
use frame_system as system;
use sp_core::H160;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;

pub type AccountId = u64;
pub type Balance = u128;
pub type TokenIssuance = u64;
pub type EthereumAddress = H160;
pub type SponsorshipId = u32;
pub type Hash = H256;
pub type LocId = u32;

construct_runtime!(
    pub struct Test {
        System: frame_system,
        Balances: pallet_balances,
        LogionLoc: pallet_loc,
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
    type Block = generic::Block<Header, UncheckedExtrinsic>;
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type Nonce = u64;
    type RuntimeCall = RuntimeCall;
    type Hash = Hash;
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
    type SS58Prefix = SS58Prefix;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
    pub const MaxLocks: u32 = 4;
    pub const MaxReserves: u32 = 2;
    pub const ExistentialDeposit: Balance = 2;
    pub const MaxFreezes: u32 = 2;
    pub const MaxHolds: u32 = 2;
}

impl pallet_balances::Config for Test {
    type MaxLocks = MaxLocks;
    type MaxReserves = ();
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

pub const LOC_OWNER1: u64 = 1;
pub const LOC_OWNER2: u64 = 2;
pub const LOC_REQUESTER_ID: u64 = 3;
pub const LOC_REQUESTER: RequesterOf<Test> = RequesterOf::<Test>::Account(LOC_REQUESTER_ID);
pub const LOGION_IDENTITY_LOC_ID: u32 = 4;
pub const ISSUER_ID1: u64 = 5;
pub const ISSUER_ID2: u64 = 6;
pub const SPONSOR_ID: u64 = 7;
pub const LOGION_TREASURY_ACCOUNT_ID: u64 = 8;
pub const UNAUTHORIZED_CALLER: u64 = 9;

pub struct LoAuthorityListMock;
impl EnsureOrigin<RuntimeOrigin> for LoAuthorityListMock {
    type Success = <Test as system::Config>::AccountId;

    fn try_origin(o: <Test as system::Config>::RuntimeOrigin) -> Result<Self::Success, <Test as system::Config>::RuntimeOrigin> {
        <Self as IsLegalOfficer<<Test as system::Config>::AccountId, <Test as system::Config>::RuntimeOrigin>>::try_origin(o)
    }
}

impl IsLegalOfficer<<Test as system::Config>::AccountId, RuntimeOrigin> for LoAuthorityListMock {

    fn legal_officers() -> Vec<<Test as system::Config>::AccountId> {
        vec![ LOC_OWNER1, LOC_OWNER2 ]
    }
}

parameter_types! {
    pub const MaxCollectionItemDescriptionSize: usize = 4096;
    pub const MaxCollectionItemTokenIdSize: usize = 255;
    pub const MaxCollectionItemTokenTypeSize: usize = 255;
    pub const MaxTokensRecordDescriptionSize: u32 = 255;
    pub const MaxFileNameSize: u32 = 255;
    pub const MaxFileContentTypeSize: u32 = 255;
    pub const MaxIssuers: u32 = 2;
    pub const MaxTokensRecordFiles: u32 = 10;
}

// Fake accounts used to simulate reward beneficiaries balances
pub const COMMUNITY_TREASURY_ACCOUNT: AccountId = 20;
pub const COLLATORS_ACCOUNT: AccountId = 21;

// Type used as beneficiary payout handle
pub struct RewardDistributor;
impl logion_shared::RewardDistributor<NegativeImbalanceOf<Test>, Balance, AccountId>
for RewardDistributor
{
    fn payout_community_treasury(reward: NegativeImbalanceOf<Test>) {
        Balances::resolve_creating(&COMMUNITY_TREASURY_ACCOUNT, reward);
    }

    fn payout_collators(reward: NegativeImbalanceOf<Test>) {
        Balances::resolve_creating(&COLLATORS_ACCOUNT, reward);
    }

    fn payout_logion_treasury(reward: NegativeImbalanceOf<Test>) {
        Balances::resolve_creating(&LOGION_TREASURY_ACCOUNT_ID, reward);
    }

    fn payout_to(reward: NegativeImbalanceOf<Test>, account: &AccountId) {
        Balances::resolve_creating(account, reward);
    }
}

parameter_types! {
    pub const FileStorageByteFee: u32 = 10u32;
    pub const FileStorageEntryFee: u32 = 100u32;
    pub const FileStorageFeeDistributionKey: DistributionKey = DistributionKey {
        collators_percent: Percent::from_percent(80),
        community_treasury_percent: Percent::from_percent(20),
        logion_treasury_percent: Percent::from_percent(0),
        loc_owner_percent: Percent::from_percent(0),
    };
    pub const ExchangeRate: Balance = 200_000_000_000_000_000; // 1 euro cent = 0.2 LGNT;
    pub const LogionTreasuryAccountId: u64 = LOGION_TREASURY_ACCOUNT_ID;
    pub const CertificateFee: u64 = 4_000_000_000_000_000; // 0.004 LGNT
    pub const CertificateFeeDistributionKey: DistributionKey = DistributionKey {
        collators_percent: Percent::from_percent(20),
        community_treasury_percent: Percent::from_percent(80),
        logion_treasury_percent: Percent::from_percent(0),
        loc_owner_percent: Percent::from_percent(0),
    };
    pub const ValueFeeDistributionKey: DistributionKey = DistributionKey {
        collators_percent: Percent::from_percent(0),
        community_treasury_percent: Percent::from_percent(0),
        logion_treasury_percent: Percent::from_percent(100),
        loc_owner_percent: Percent::from_percent(0),
    };
    pub const RecurentFeeDistributionKey: DistributionKey = DistributionKey {
        collators_percent: Percent::from_percent(0),
        community_treasury_percent: Percent::from_percent(0),
        logion_treasury_percent: Percent::from_percent(95),
        loc_owner_percent: Percent::from_percent(5),
    };
}

pub struct LegalFeeImpl;
impl LegalFee<NegativeImbalanceOf<Test>, Balance, LocType, AccountId> for LegalFeeImpl {
    fn get_default_legal_fee(loc_type: LocType) -> EuroCent {
        match loc_type {
            LocType::Identity => 8_00, // 8.00 euros
            _ => 100_00, // 100.00 euros
        }
    }

    fn distribute(amount: NegativeImbalanceOf<Test>, loc_type: LocType, loc_owner: AccountId) -> Beneficiary<AccountId> {

        let (beneficiary, target) = match loc_type {
            LocType::Identity => (Beneficiary::Other, LOGION_TREASURY_ACCOUNT_ID),
            _ => (Beneficiary::LegalOfficer(loc_owner), loc_owner),
        };
        Balances::resolve_creating(&target, amount);
        beneficiary
    }
}

pub struct SHA256;
impl Hasher<H256> for SHA256 {

    fn hash(data: &Vec<u8>) -> H256 {
        let bytes = sha2_256(data);
        H256(bytes)
    }
}

impl pallet_loc::Config for Test {
    type LocId = LocId;
    type RuntimeEvent = RuntimeEvent;
    type Hash = H256;
    type Hasher = SHA256;
    type IsLegalOfficer = LoAuthorityListMock;
    type CollectionItemId = H256;
    type MaxCollectionItemDescriptionSize = MaxCollectionItemDescriptionSize;
    type MaxCollectionItemTokenIdSize = MaxCollectionItemTokenIdSize;
    type MaxCollectionItemTokenTypeSize = MaxCollectionItemTokenTypeSize;
    type TokensRecordId = H256;
    type MaxTokensRecordDescriptionSize = MaxTokensRecordDescriptionSize;
    type MaxFileNameSize = MaxFileNameSize;
    type MaxFileContentTypeSize = MaxFileContentTypeSize;
    type MaxTokensRecordFiles = MaxTokensRecordFiles;
    type WeightInfo = ();
    type Currency = Balances;
    type FileStorageByteFee = FileStorageByteFee;
    type FileStorageEntryFee = FileStorageEntryFee;
    type RewardDistributor = RewardDistributor;
    type FileStorageFeeDistributionKey = FileStorageFeeDistributionKey;
    type EthereumAddress = EthereumAddress;
    type SponsorshipId = SponsorshipId;
    type LegalFee = LegalFeeImpl;
    type ExchangeRate = ExchangeRate;
    type CertificateFee = CertificateFee;
    type CertificateFeeDistributionKey = CertificateFeeDistributionKey;
    type TokenIssuance = TokenIssuance;
    type ValueFeeDistributionKey = ValueFeeDistributionKey;
    type CollectionItemFeeDistributionKey = RecurentFeeDistributionKey;
    type TokensRecordFeeDistributionKey = RecurentFeeDistributionKey;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    new_test_ext_at_block(1)
}

pub fn new_test_ext_at_block(block_number: u64) -> sp_io::TestExternalities {
    let t = system::GenesisConfig::<Test>::default().build_storage().unwrap();
    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| System::set_block_number(block_number));
    ext
}
