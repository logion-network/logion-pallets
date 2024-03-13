use frame_support::dispatch::DispatchResultWithPostInfo;
use crate as pallet_logion_vote;
use frame_benchmarking::account;
use frame_support::{derive_impl, parameter_types};
use frame_support::traits::EnsureOrigin;
use sp_core::hash::H256;
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup}, BuildStorage,
};
use frame_system::{self as system, Config};
use logion_shared::{IsLegalOfficer, LegalOfficerCaseSummary, LegalOfficerCreation, LocQuery, LocValidity};

type Block = frame_system::mocking::MockBlock<Test>;
pub type AccountId = u64;
pub type LocId = u32;

frame_support::construct_runtime!(
    pub enum Test {
        System: frame_system,
        LogionVote: pallet_logion_vote,
    }
);

pub struct LoAuthorityListMock;

#[cfg(feature = "runtime-benchmarks")]
pub type OuterOrigin<T> = <T as frame_system::Config>::RuntimeOrigin;
#[cfg(feature = "runtime-benchmarks")]
use frame_system::RawOrigin;
use scale_info::TypeInfo;

impl EnsureOrigin<RuntimeOrigin> for LoAuthorityListMock {
    type Success = <Test as system::Config>::AccountId;

    fn try_origin(o: <Test as system::Config>::RuntimeOrigin) -> Result<Self::Success, <Test as system::Config>::RuntimeOrigin> {
        <Self as IsLegalOfficer<<Test as system::Config>::AccountId, <Test as system::Config>::RuntimeOrigin>>::try_origin(o)
    }

	#[cfg(feature = "runtime-benchmarks")]
    fn try_successful_origin() -> Result<<Test as frame_system::Config>::RuntimeOrigin, ()> {
        Ok(OuterOrigin::<Test>::from(RawOrigin::Signed(legal_officer_id(1).clone())))
    }
}

pub fn legal_officer_id(index: u32) -> AccountId {
	account("owner", index, 0)
}

pub fn legal_officers() -> Vec<AccountId> {
	[1, 2].map(legal_officer_id).to_vec()
}

pub const LOC_ID: u32 = 1;

impl IsLegalOfficer<<Test as system::Config>::AccountId, RuntimeOrigin> for LoAuthorityListMock {

    fn legal_officers() -> Vec<<Test as Config>::AccountId> {
        legal_officers()
    }
}

pub struct LocValidityMock;

impl LocValidity<<Test as pallet_logion_vote::Config>::LocId, <Test as system::Config>::AccountId> for LocValidityMock {
    fn loc_valid_with_owner(loc_id: &<Test as pallet_logion_vote::Config>::LocId, legal_officer: &<Test as Config>::AccountId) -> bool {
        return *loc_id == LOC_ID && *legal_officer == legal_officer_id(1);
    }
}

pub struct LocQueryMock;

impl LocQuery<<Test as pallet_logion_vote::Config>::LocId, <Test as system::Config>::AccountId> for LocQueryMock {
    fn has_closed_identity_locs(_account: &<Test as Config>::AccountId, _legal_officer: &Vec<<Test as Config>::AccountId>) -> bool {
        false
    }

    fn get_loc(loc_id: &<Test as crate::Config>::LocId) -> Option<LegalOfficerCaseSummary<<Test as Config>::AccountId>> {
        if *loc_id == LOC_ID {
            return Some(LegalOfficerCaseSummary {
                owner: legal_officer_id(1),
                requester: Some(legal_officer_id(3)),
            })
        }
        return None
    }
}

pub struct LegalOfficerCreationMock;

impl LegalOfficerCreation<<Test as system::Config>::AccountId> for LegalOfficerCreationMock {
    fn add_guest_legal_officer(guest_legal_officer_id: <Test as Config>::AccountId, host_legal_officer_id: <Test as Config>::AccountId) -> DispatchResultWithPostInfo {
        if guest_legal_officer_id == legal_officer_id(3) && host_legal_officer_id == legal_officer_id(1) {
            Ok(().into())
        } else {
            panic!()
        }
    }
}

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

#[cfg(feature = "runtime-benchmarks")]
pub struct LocSetupMock;

#[cfg(feature = "runtime-benchmarks")]
use crate::benchmarking::{
	LocSetup,
};
use crate::weights::SubstrateWeight;

#[cfg(feature = "runtime-benchmarks")]
impl LocSetup<LocId, AccountId> for LocSetupMock {

	fn setup_vote_loc() -> (LocId, AccountId) {
		let legal_officer_id = legal_officer_id(1);
		(LOC_ID, legal_officer_id)
	}
}

parameter_types! {
	#[derive(Debug, PartialEq, TypeInfo)]
	pub const MaxBallots: u32 = 2;
}

impl pallet_logion_vote::Config for Test {
    type LocId = LocId;
    type RuntimeEvent = RuntimeEvent;
    type IsLegalOfficer = LoAuthorityListMock;
    type LocValidity = LocValidityMock;
    type LocQuery = LocQueryMock;
    type LegalOfficerCreation = LegalOfficerCreationMock;
    type WeightInfo = SubstrateWeight<Test>;
	type MaxBallots = MaxBallots;
	#[cfg(feature = "runtime-benchmarks")]
	type LocSetup = LocSetupMock;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    let t = system::GenesisConfig::<Test>::default().build_storage().unwrap();
    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| System::set_block_number(1));
    ext
}
