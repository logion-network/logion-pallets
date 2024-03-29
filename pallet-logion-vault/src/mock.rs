use crate::{self as pallet_logion_vault};
use logion_shared::{IsLegalOfficer, MultisigApproveAsMultiCallFactory, MultisigAsMultiCallFactory};
use pallet_multisig::Timepoint;
use sp_core::hash::H256;
use frame_support::{derive_impl, parameter_types, traits::EnsureOrigin};
#[cfg(feature = "runtime-benchmarks")]
use frame_support::dispatch::RawOrigin;
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup}, BuildStorage,
};
use frame_system as system;
use sp_std::convert::{TryInto, TryFrom};
use system::{ensure_signed, pallet_prelude::BlockNumberFor};
use sp_weights::Weight;

type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test {
        System: frame_system,
        Vault: pallet_logion_vault,
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = 42;
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
impl MultisigApproveAsMultiCallFactory<<Test as system::Config>::RuntimeOrigin, <Test as system::Config>::AccountId, Timepoint<BlockNumberFor<Test>>> for CreateRecoveryCallFactoryMock {
    type Call = RuntimeCall;

    fn build_approve_as_multi_call(
        _threshold: u16,
        _other_signatories: Vec<<Test as system::Config>::AccountId>,
        _maybe_timepoint: Option<Timepoint<BlockNumberFor<Test>>>,
        _call_hash: [u8; 32],
        _max_weight: Weight
    ) -> Self::Call {
        RuntimeCall::System(frame_system::Call::remark{ remark : Vec::from([0u8]) })
    }
}

pub struct MultisigAsMultiCallFactoryMock;
impl MultisigAsMultiCallFactory<<Test as system::Config>::RuntimeOrigin, <Test as system::Config>::AccountId, Timepoint<BlockNumberFor<Test>>> for MultisigAsMultiCallFactoryMock {
    type Call = RuntimeCall;

    fn build_as_multi_call(
        _threshold: u16,
        _other_signatories: Vec<<Test as system::Config>::AccountId>,
        _maybe_timepoint: Option<Timepoint<BlockNumberFor<Test>>>,
        _call: Box<Self::Call>,
        _max_weight: Weight,
    ) -> Self::Call {
        *_call
    }
}

pub const LEGAL_OFFICER1: u64 = 1;
pub const LEGAL_OFFICER2: u64 = 2;
pub const USER_ID: u64 = 3;
pub const ANOTHER_USER_ID: u64 = 4;

pub struct IsLegalOfficerMock;
impl IsLegalOfficer<<Test as system::Config>::AccountId, RuntimeOrigin> for IsLegalOfficerMock {

    fn legal_officers() -> Vec<<Test as system::Config>::AccountId> {
        vec![LEGAL_OFFICER1, LEGAL_OFFICER2]
    }
}

#[cfg(feature = "runtime-benchmarks")]
pub type OuterOrigin<T> = <T as frame_system::Config>::RuntimeOrigin;

impl EnsureOrigin<RuntimeOrigin> for IsLegalOfficerMock {
    type Success = <Test as system::Config>::AccountId;

    fn try_origin(o: RuntimeOrigin) -> std::result::Result<Self::Success, RuntimeOrigin> {
        let result = ensure_signed(o.clone());
        match result {
            Ok(who) => {
                if <Self as IsLegalOfficer<Self::Success, RuntimeOrigin>>::is_legal_officer(&who) {
                    Ok(who)
                } else {
                    Err(o)
                }
            },
            Err(_) => Err(o)
        }
    }

	#[cfg(feature = "runtime-benchmarks")]
	fn try_successful_origin() -> Result<RuntimeOrigin, ()> {
		Ok(OuterOrigin::<Test>::from(RawOrigin::Signed(LEGAL_OFFICER1)))
	}
}

impl pallet_logion_vault::Config for Test {
    type RuntimeCall = RuntimeCall;
    type MultisigApproveAsMultiCallFactory = CreateRecoveryCallFactoryMock;
    type MultisigAsMultiCallFactory = MultisigAsMultiCallFactoryMock;
    type IsLegalOfficer = IsLegalOfficerMock;
    type RuntimeEvent = RuntimeEvent;
    type WeightInfo = ();
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}
