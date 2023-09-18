use crate::{self as pallet_verified_recovery};
use logion_shared::{LocQuery, CreateRecoveryCallFactory, LegalOfficerCaseSummary};
use sp_core::hash::H256;
use frame_support::parameter_types;
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup}, testing::Header, generic, BuildStorage,
};
use frame_system as system;
use system::pallet_prelude::BlockNumberFor;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;

frame_support::construct_runtime!(
    pub enum Test {
        System: frame_system,
        VerifiedRecovery: pallet_verified_recovery,
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
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = ();
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

pub struct CreateRecoveryCallFactoryMock;
impl CreateRecoveryCallFactory<<Test as system::Config>::RuntimeOrigin, <Test as system::Config>::AccountId, BlockNumberFor<Test>> for CreateRecoveryCallFactoryMock {
    type Call = RuntimeCall;

    fn build_create_recovery_call(_legal_officers: Vec<<Test as system::Config>::AccountId>, _threshold: u16, _delay_period: BlockNumberFor<Test>) -> Self::Call {
        RuntimeCall::System(frame_system::Call::remark{ remark : Vec::from([0u8]) })
    }
}

pub const LEGAL_OFFICER_CLOSED_ID1: u64 = 1;
pub const LEGAL_OFFICER_CLOSED_ID2: u64 = 2;
pub const LEGAL_OFFICER_PENDING_OR_OPEN_ID1: u64 = 3;
pub const LEGAL_OFFICER_PENDING_OR_OPEN_ID2: u64 = 4;
pub const USER_ID: u64 = 5;

pub struct LocQueryMock;
impl LocQuery<<Test as pallet_verified_recovery::Config>::LocId, <Test as system::Config>::AccountId> for LocQueryMock {
    fn has_closed_identity_locs(
        account: &<Test as system::Config>::AccountId,
        legal_officers: &Vec<<Test as system::Config>::AccountId>
    ) -> bool {
        return *account == USER_ID && legal_officers[0] == LEGAL_OFFICER_CLOSED_ID1 && legal_officers[1] == LEGAL_OFFICER_CLOSED_ID2;
    }

    fn get_loc(_loc_id: &<Test as pallet_verified_recovery::Config>::LocId) -> Option<LegalOfficerCaseSummary<<Test as system::Config>::AccountId>> {
        return None;
    }
}

impl pallet_verified_recovery::Config for Test {
    type LocId = u32;
    type CreateRecoveryCallFactory = CreateRecoveryCallFactoryMock;
    type LocQuery = LocQueryMock;
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}
