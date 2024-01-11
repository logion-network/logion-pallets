use crate::{self as pallet_verified_recovery};
use logion_shared::{LocQuery, CreateRecoveryCallFactory, LegalOfficerCaseSummary};
use sp_core::hash::H256;
use frame_benchmarking::account;
use frame_support::parameter_types;
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup}, testing::Header, generic, BuildStorage,
};
use frame_system as system;
use system::pallet_prelude::BlockNumberFor;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type AccountId = u64;

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
    type AccountId = AccountId;
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

pub fn legal_officer(index: u32) -> AccountId {
	account("legal_officer", index, 0)
}

pub fn requester() -> AccountId {
	account("requester", 1, 0)
}

pub fn legal_officers_closed() -> Vec<AccountId> {
	Vec::from([legal_officer(1), legal_officer(2)])
}

pub fn legal_officers_not_closed() -> Vec<AccountId> {
	Vec::from([legal_officer(3), legal_officer(4)])
}

pub struct LocQueryMock;
impl LocQuery<<Test as pallet_verified_recovery::Config>::LocId, <Test as system::Config>::AccountId> for LocQueryMock {
    fn has_closed_identity_locs(
        account: &<Test as system::Config>::AccountId,
        legal_officers: &Vec<<Test as system::Config>::AccountId>
    ) -> bool {
        return *account == requester() && legal_officers[0] == legal_officer(1) && legal_officers[1] == legal_officer(2);
    }

    fn get_loc(_loc_id: &<Test as pallet_verified_recovery::Config>::LocId) -> Option<LegalOfficerCaseSummary<<Test as system::Config>::AccountId>> {
        return None;
    }
}

#[cfg(feature = "runtime-benchmarks")]
pub struct SetupBenchmarkMock;
#[cfg(feature = "runtime-benchmarks")]
pub use crate::benchmarking::SetupBenchmark;
use crate::weights::SubstrateWeight;

#[cfg(feature = "runtime-benchmarks")]
impl SetupBenchmark<AccountId> for SetupBenchmarkMock {

	fn setup() -> (AccountId, Vec<AccountId>) {
		(
			requester(),
			vec!(legal_officer(1), legal_officer(2))
		)
	}
}

impl pallet_verified_recovery::Config for Test {
    type LocId = u32;
    type CreateRecoveryCallFactory = CreateRecoveryCallFactoryMock;
    type LocQuery = LocQueryMock;
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = SubstrateWeight<Test>;
	#[cfg(feature = "runtime-benchmarks")]
	type SetupBenchmark = SetupBenchmarkMock;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}
