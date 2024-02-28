use crate::{self as pallet_lo_authority_list, HostDataParam, HostDataParamOf};
use codec::{Encode, Decode, MaxEncodedLen};
use frame_support::parameter_types;
use frame_system::{self as system, EnsureRoot};
use scale_info::TypeInfo;
use sp_core::hash::H256;
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup}, testing::Header, generic,
    BuildStorage,
};
use crate::weights::SubstrateWeight;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;

frame_support::construct_runtime!(
    pub enum Test {
        System: frame_system,
        LoAuthorityList: pallet_lo_authority_list,
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

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, Copy, MaxEncodedLen)]
pub enum Region {
    Europe,
    Other,
}

impl core::str::FromStr for Region {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Europe" => Ok(Region::Europe),
            "Other" => Ok(Region::Other),
            _ => Err(()),
        }
    }
}

impl Default for Region {

    fn default() -> Self {
        Self::Europe
    }
}

impl Default for HostDataParamOf<Test> {

    fn default() -> Self {
        return HostDataParam {
            node_id: None,
            base_url: None,
            region: Region::Europe,
        }
    }
}

parameter_types! {
	#[derive(Debug, Eq, Clone, PartialEq, TypeInfo)]
	pub const MaxBaseUrlLen: u32 = 30;
	pub const MaxNodes: u32 = 3;
	#[derive(Debug, Eq, Clone, PartialEq, TypeInfo, PartialOrd, Ord)]
	pub const MaxPeerIdLength: u32 = 48;
}

impl pallet_lo_authority_list::Config for Test {
    type AddOrigin = EnsureRoot<u64>;
    type RemoveOrigin = EnsureRoot<u64>;
    type UpdateOrigin = EnsureRoot<u64>;
    type Region = Region;
    type RuntimeEvent = RuntimeEvent;
	type WeightInfo = SubstrateWeight<Test>;
	type MaxBaseUrlLen = MaxBaseUrlLen;
	type MaxNodes = MaxNodes;
	type MaxPeerIdLength = MaxPeerIdLength;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
    system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}
