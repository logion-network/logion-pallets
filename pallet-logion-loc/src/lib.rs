#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub mod migrations;
pub mod runtime_api;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod fees;

#[cfg(test)]
mod tests;

mod benchmarking;

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::{
    BoundedVec,
    sp_runtime::Saturating,
    traits::{Currency, ReservableCurrency},
};
use scale_info::TypeInfo;
use logion_shared::LegalOfficerCaseSummary;
use crate::Requester::Account;
use frame_system::pallet_prelude::BlockNumberFor;
use sp_core::Get;
#[cfg(feature = "runtime-benchmarks")]
use sp_core::H160;
use sp_std::{
    collections::btree_set::BTreeSet,
    vec::Vec,
};
use sp_runtime::traits::Zero;
#[cfg(feature = "runtime-benchmarks")]
use benchmarking::{
	LocIdFactory,
	CollectionItemIdFactory,
	TokensRecordIdFactory,
	EthereumAddressFactory,
	SponsorshipIdFactory,
};

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen, Copy)]
pub enum LocType {
    Transaction,
    Identity,
    Collection,
}

impl Default for LocType {
    fn default() -> LocType {
        return LocType::Transaction;
    }
}

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
pub struct MetadataItem<AccountId, EthereumAddress, Hash> {
    name: Hash,
    value: Hash,
    submitter: SupportedAccountId<AccountId, EthereumAddress>,
    acknowledged_by_owner: bool,
    acknowledged_by_verified_issuer: bool,
}

pub type MetadataItemOf<T> = MetadataItem<
    <T as frame_system::Config>::AccountId,
    <T as pallet::Config>::EthereumAddress,
    <T as pallet::Config>::Hash,
>;

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
pub struct MetadataItemParams<AccountId, EthereumAddress, Hash> {
    name: Hash,
    value: Hash,
    submitter: SupportedAccountId<AccountId, EthereumAddress>,
}

pub type MetadataItemParamsOf<T> = MetadataItemParams<
	<T as frame_system::Config>::AccountId,
	<T as pallet::Config>::EthereumAddress,
	<T as pallet::Config>::Hash,
>;

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
pub struct LocLink<LocId, Hash, AccountId, EthereumAddress> {
    id: LocId,
    nature: Hash,
    submitter: SupportedAccountId<AccountId, EthereumAddress>,
    acknowledged_by_owner: bool,
    acknowledged_by_verified_issuer: bool,
}

pub type LocLinkOf<T> = LocLink<
    <T as pallet::Config>::LocId,
    <T as pallet::Config>::Hash,
    <T as frame_system::Config>::AccountId,
    <T as pallet::Config>::EthereumAddress,
>;

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
pub struct LocLinkParams<LocId, Hash, AccountId, EthereumAddress> {
    id: LocId,
    nature: Hash,
    submitter: SupportedAccountId<AccountId, EthereumAddress>,
}

pub type LocLinkParamsOf<T> = LocLinkParams<
	<T as pallet::Config>::LocId,
	<T as pallet::Config>::Hash,
	<T as frame_system::Config>::AccountId,
	<T as pallet::Config>::EthereumAddress,
>;

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
pub struct File<Hash, AccountId, EthereumAddress> {
    hash: Hash,
    nature: Hash,
    submitter: SupportedAccountId<AccountId, EthereumAddress>,
    size: u32,
    acknowledged_by_owner: bool,
    acknowledged_by_verified_issuer: bool,
}

pub type FileOf<T> = File<
    <T as pallet::Config>::Hash,
    <T as frame_system::Config>::AccountId,
    <T as pallet::Config>::EthereumAddress,
>;

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
pub struct FileParams<Hash, AccountId, EthereumAddress> {
    hash: Hash,
    nature: Hash,
    submitter: SupportedAccountId<AccountId, EthereumAddress>,
    size: u32,
}

pub type FileParamsOf<T> = FileParams<
	<T as pallet::Config>::Hash,
	<T as frame_system::Config>::AccountId,
	<T as pallet::Config>::EthereumAddress,
>;

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
pub struct ItemsParams<LocId, AccountId, EthereumAddress, Hash> {
    metadata: Vec<MetadataItemParams<AccountId, EthereumAddress, Hash>>,
    files: Vec<FileParams<Hash, AccountId, EthereumAddress>>,
    links: Vec<LocLinkParams<LocId, Hash, AccountId, EthereumAddress>>,
}

impl<LocId, AccountId, EthereumAddress, Hash> ItemsParams<LocId, AccountId, EthereumAddress, Hash> {

    pub fn empty() -> ItemsParams<LocId, AccountId, EthereumAddress, Hash> {
        ItemsParams {
            metadata: Vec::new(),
            files: Vec::new(),
            links: Vec::new(),
        }
    }

    pub fn only_metadata(metadata: Vec<MetadataItemParams<AccountId, EthereumAddress, Hash>>) -> ItemsParams<LocId, AccountId, EthereumAddress, Hash> {
        ItemsParams {
            metadata,
            files: Vec::new(),
            links: Vec::new(),
        }
    }

    pub fn only_files(files: Vec<FileParams<Hash, AccountId, EthereumAddress>>) -> ItemsParams<LocId, AccountId, EthereumAddress, Hash> {
        ItemsParams {
            metadata: Vec::new(),
            files,
            links: Vec::new(),
        }
    }

    pub fn only_links(links: Vec<LocLinkParams<LocId, Hash, AccountId, EthereumAddress>>) -> ItemsParams<LocId, AccountId, EthereumAddress, Hash> {
        ItemsParams {
            metadata: Vec::new(),
            files: Vec::new(),
            links,
        }
    }
}

pub type ItemsParamsOf<T> = ItemsParams<
    <T as pallet::Config>::LocId,
    <T as frame_system::Config>::AccountId,
    <T as pallet::Config>::EthereumAddress,
    <T as pallet::Config>::Hash,
>;

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
pub struct Items<LocId, AccountId, EthereumAddress, Hash> {
    metadata: Vec<MetadataItem<AccountId, EthereumAddress, Hash>>,
    files: Vec<File<Hash, AccountId, EthereumAddress>>,
    links: Vec<LocLink<LocId, Hash, AccountId, EthereumAddress>>,
}

pub type ItemsOf<T> = Items<
    <T as pallet::Config>::LocId,
    <T as frame_system::Config>::AccountId,
    <T as pallet::Config>::EthereumAddress,
    <T as pallet::Config>::Hash,
>;

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
pub struct LocVoidInfo<LocId> {
    replacer: Option<LocId>,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen, Copy)]
pub enum OtherAccountId<EthereumAddress> {
    Ethereum(EthereumAddress)
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
pub enum Requester<AccountId, LocId, EthereumAddress> {
    None,
    Account(AccountId),
    Loc(LocId),
    OtherAccount(OtherAccountId<EthereumAddress>),
}

pub type RequesterOf<T> = Requester<<T as frame_system::Config>::AccountId, <T as Config>::LocId, <T as Config>::EthereumAddress>;

impl<AccountId, LocId, EthereumAddress> Default for Requester<AccountId, LocId, EthereumAddress> {

    fn default() -> Requester<AccountId, LocId, EthereumAddress> {
        Requester::None
    }
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen, Copy)]
pub enum SupportedAccountId<AccountId, EthereumAddress> {
    None, // Enables "null" account ID
    Polkadot(AccountId),
    Other(OtherAccountId<EthereumAddress>),
}

impl<AccountId, EthereumAddress> Default for SupportedAccountId<AccountId, EthereumAddress> {

    fn default() -> SupportedAccountId<AccountId, EthereumAddress> {
        SupportedAccountId::None
    }
}

pub type CollectionSize = u32;

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
pub struct LegalOfficerCase<AccountId, Hash, LocId, BlockNumber, EthereumAddress, SponsorshipId, Balance,
	MaxLocMetadata: Get<u32>, MaxLocFiles: Get<u32>, MaxLocLinks: Get<u32>> {
    owner: AccountId,
    requester: Requester<AccountId, LocId, EthereumAddress>,
    metadata: BoundedVec<MetadataItem<AccountId, EthereumAddress, Hash>, MaxLocMetadata>,
    files: BoundedVec<File<Hash, AccountId, EthereumAddress>, MaxLocFiles>,
    closed: bool,
    loc_type: LocType,
    links: BoundedVec<LocLink<LocId, Hash, AccountId, EthereumAddress>, MaxLocLinks>,
    void_info: Option<LocVoidInfo<LocId>>,
    replacer_of: Option<LocId>,
    collection_last_block_submission: Option<BlockNumber>,
    collection_max_size: Option<CollectionSize>,
    collection_can_upload: bool,
    seal: Option<Hash>,
    sponsorship_id: Option<SponsorshipId>,
    value_fee: Balance,
    legal_fee: Balance,
    collection_item_fee: Balance,
    tokens_record_fee: Balance,
    imported: bool,
}

impl<AccountId, Hash, LocId, BlockNumber, EthereumAddress, SponsorshipId, Balance, MaxLocMetadata, MaxLocFiles, MaxLocLinks>
LegalOfficerCase<AccountId, Hash, LocId, BlockNumber, EthereumAddress, SponsorshipId, Balance, MaxLocMetadata, MaxLocFiles, MaxLocLinks>
where
    AccountId: PartialEq + Clone,
    Hash: PartialEq + Copy + Ord,
    LocId: PartialEq + Copy + Ord,
    EthereumAddress: PartialEq + Clone,
	MaxLocMetadata: Get<u32>,
	MaxLocFiles: Get<u32>,
	MaxLocLinks: Get<u32>,
{

    pub fn ensure_can_add<T: pallet::Config>(&self, items: &ItemsParams<LocId, AccountId, EthereumAddress, Hash>) -> Result<(), sp_runtime::DispatchError> {
        self.ensure_requester_submits::<T>(&items)?;
        self.ensure_can_add_metadata::<T>(&items.metadata.iter().map(|item| item.name).collect())?;
        self.ensure_can_add_files::<T>(&items.files.iter().map(|item| item.hash).collect())?;
        self.ensure_can_add_links::<T>(&items.links.iter().map(|item| item.id).collect())?;
        Ok(())
    }

    pub fn ensure_requester_submits<T: pallet::Config>(&self, items: &ItemsParams<LocId, AccountId, EthereumAddress, Hash>) -> Result<(), sp_runtime::DispatchError> {
        if items.metadata.iter().find(|item| !self.is_requester(&item.submitter)).is_some()
            || items.files.iter().find(|item| !self.is_requester(&item.submitter)).is_some()
            || items.links.iter().find(|item| !self.is_requester(&item.submitter)).is_some() {
            Err(Error::<T>::CannotSubmit)?
        }
        Ok(())
    }

    pub fn is_requester(&self, submitter: &SupportedAccountId<AccountId, EthereumAddress>) -> bool {
        match &self.requester {
            Requester::Account(account_id) => match submitter {
                SupportedAccountId::Polkadot(polkadot_address) => *account_id == *polkadot_address,
                _ => false,
            },
            Requester::OtherAccount(other) => match other {
                OtherAccountId::Ethereum(ethereum_account) => match submitter {
                    SupportedAccountId::Other(other_account) => match other_account {
                        OtherAccountId::Ethereum(ethereum_address) => *ethereum_account == *ethereum_address,
                    }
                    _ => false,
                },
            },
            _ => false,
        }
    }

    pub fn ensure_can_add_metadata<T: pallet::Config>(&self, metadata_names: &Vec<Hash>) -> Result<(), sp_runtime::DispatchError> {
        let mut keys = BTreeSet::new();
        metadata_names.iter().for_each(|name| { keys.insert(*name); });
        self.metadata.iter().for_each(|item| { keys.insert(item.name); });

        if keys.len() < self.metadata.len() + metadata_names.len() {
            Err(Error::<T>::DuplicateLocMetadata)?
        }
        Ok(())
    }

    pub fn ensure_can_add_files<T: pallet::Config>(&self, file_hashes: &Vec<Hash>) -> Result<(), sp_runtime::DispatchError> {
        let mut keys = BTreeSet::new();
        file_hashes.iter().for_each(|hash| { keys.insert(*hash); });
        self.files.iter().for_each(|item| { keys.insert(item.hash); });

        if keys.len() < self.files.len() + file_hashes.len() {
            Err(Error::<T>::DuplicateLocFile)?
        }
        Ok(())
    }

    pub fn ensure_can_add_links<T: pallet::Config>(&self, link_ids: &Vec<LocId>) -> Result<(), sp_runtime::DispatchError> {
        let mut keys = BTreeSet::new();
        link_ids.iter().for_each(|id| { keys.insert(*id); });
        self.links.iter().for_each(|item| { keys.insert(item.id); });

        if keys.len() < self.links.len() + link_ids.len() {
            Err(Error::<T>::DuplicateLocLink)?
        }
        Ok(())
    }

    pub fn add_items<T: pallet::Config>(&mut self, origin: &AccountId, items: &ItemsParams<LocId, AccountId, EthereumAddress, Hash>) -> Result<(), sp_runtime::DispatchError> {
		for item in items.metadata.iter() {
			self.add_metadata::<T>(origin, item)?;
		}
		for item in items.files.iter() {
			self.add_file::<T>(origin, item)?;
		}
		for item in items.links.iter() {
			self.add_link::<T>(origin, item)?;
		}
		Ok(())
    }

    pub fn add_metadata<T: pallet::Config>(&mut self, origin: &AccountId, item: &MetadataItemParams<AccountId, EthereumAddress, Hash>) -> Result<(), sp_runtime::DispatchError> {
        self.metadata.try_push(MetadataItem {
            name: item.name,
            value: item.value,
            submitter: item.submitter.clone(),
            acknowledged_by_owner: self.is_owner(origin),
            acknowledged_by_verified_issuer: false,
        }).map_err(|_| Error::<T>::LocMetadataTooMuchData)?;
		Ok(())
    }

    pub fn is_owner(&self, origin: &AccountId) -> bool {
        self.owner == *origin
    }

    pub fn add_file<T: pallet::Config>(&mut self, origin: &AccountId, file: &FileParams<Hash, AccountId, EthereumAddress>) -> Result<(), sp_runtime::DispatchError> {
        self.files.try_push(File {
            hash: file.hash,
            nature: file.nature,
            submitter: file.submitter.clone(),
            size: file.size,
            acknowledged_by_owner: self.is_owner(origin),
            acknowledged_by_verified_issuer: false,
		}).map_err(|_| Error::<T>::LocFilesTooMuchData)?;
		Ok(())
    }

    pub fn add_link<T: pallet::Config>(&mut self, origin: &AccountId, link: &LocLinkParams<LocId, Hash, AccountId, EthereumAddress>) -> Result<(), sp_runtime::DispatchError> {
        self.links.try_push(LocLink {
            id: link.id,
            nature: link.nature,
            submitter: link.submitter.clone(),
            acknowledged_by_owner: self.is_owner(origin),
            acknowledged_by_verified_issuer: false,
        }).map_err(|_| Error::<T>::LocLinksTooMuchData)?;
		Ok(())
    }

    pub fn has_items_unacknowledged_by_owner(&self) -> bool {
        self.files.iter().find(|file| { !file.acknowledged_by_owner }).is_some()
            || self.metadata.iter().find(|item| { !item.acknowledged_by_owner }).is_some()
            || self.links.iter().find(|link| { !link.acknowledged_by_owner }).is_some()
    }

    pub fn has_items_unacknowledged_by_verified_issuer(&self) -> bool {
        self.files.iter().find(|file| { self.is_submitted_by_verified_issuer(&file.submitter) && !file.acknowledged_by_verified_issuer }).is_some()
            || self.metadata.iter().find(|item| { self.is_submitted_by_verified_issuer(&item.submitter) && !item.acknowledged_by_verified_issuer }).is_some()
            || self.links.iter().find(|link| { self.is_submitted_by_verified_issuer( &link.submitter) && !link.acknowledged_by_verified_issuer }).is_some()
    }

    fn is_submitted_by_verified_issuer(&self, submitter: &SupportedAccountId<AccountId, EthereumAddress>) -> bool {
        match submitter {
            SupportedAccountId::Polkadot(polkadot_submitter) => !self.is_owner(polkadot_submitter) && !self.is_requester(submitter),
            _ => false
        }
    }

    pub fn ensure_can_import<T: pallet::Config>(&self, items: &Items<LocId, AccountId, EthereumAddress, Hash>) -> Result<(), sp_runtime::DispatchError> {
        self.ensure_can_add_metadata::<T>(&items.metadata.iter().map(|item| item.name).collect())?;
        Ok(())
    }

    pub fn import_items<T: pallet::Config>(&mut self, items: &Items<LocId, AccountId, EthereumAddress, Hash>) -> Result<(), sp_runtime::DispatchError> {
        for item in items.metadata.iter() {
            self.metadata.try_push(item.clone()).map_err(|_| Error::<T>::LocMetadataTooMuchData)?;
        }
        for item in items.files.iter() {
            self.files.try_push(item.clone()).map_err(|_| Error::<T>::LocFilesTooMuchData)?;
        }
        for item in items.links.iter() {
            self.links.try_push(item.clone()).map_err(|_| Error::<T>::LocLinksTooMuchData)?;
        }
        Ok(())
    }
}

pub type LegalOfficerCaseOf<T> = LegalOfficerCase<
    <T as frame_system::Config>::AccountId,
    <T as pallet::Config>::Hash,
    <T as pallet::Config>::LocId,
    BlockNumberFor<T>,
    <T as pallet::Config>::EthereumAddress,
    <T as pallet::Config>::SponsorshipId,
    BalanceOf<T>,
	<T as pallet::Config>::MaxLocMetadata,
	<T as pallet::Config>::MaxLocFiles,
	<T as pallet::Config>::MaxLocLinks,
>;

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
pub struct TermsAndConditionsElement<LocId, Hash> {
    tc_type: Hash,
    tc_loc: LocId,
    details: Hash,
}

pub type TermsAndConditionsElementOf<T> = TermsAndConditionsElement<
	<T as pallet::Config>::LocId,
	<T as pallet::Config>::Hash,
>;

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
pub struct CollectionItem<Hash, TokenIssuance, BoundedCollectionItemFilesList, BoundedCollectionItemTCList> {
    description: Hash,
    files: BoundedCollectionItemFilesList,
    token: Option<CollectionItemToken<TokenIssuance, Hash>>,
    restricted_delivery: bool,
    terms_and_conditions: BoundedCollectionItemTCList,
    imported: bool,
}

pub type CollectionItemOf<T> = CollectionItem<
    <T as pallet::Config>::Hash,
    <T as pallet::Config>::TokenIssuance,
	BoundedVec<
		CollectionItemFileOf<T>,
		<T as pallet::Config>::MaxCollectionItemFiles
	>,
	BoundedVec<
		TermsAndConditionsElementOf<T>,
		<T as pallet::Config>::MaxCollectionItemTCs
	>,
>;

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
pub struct CollectionItemFile<Hash> {
    name: Hash,
    content_type: Hash,
    size: u32,
    hash: Hash,
}

pub type CollectionItemFileOf<T> = CollectionItemFile<<T as pallet::Config>::Hash>;

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
pub struct CollectionItemToken<TokenIssuance, Hash> {
    token_type: Hash,
    token_id: Hash,
    token_issuance: TokenIssuance,
}

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
pub struct VerifiedIssuer<LocId> {
    identity_loc: LocId,
    imported: bool,
}

pub type VerifiedIssuerOf<T> = VerifiedIssuer<
    <T as pallet::Config>::LocId,
>;

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
pub struct TokensRecord<Hash, BoundedTokensRecordFilesList, AccountId> {
    description: Hash,
    files: BoundedTokensRecordFilesList,
    submitter: AccountId,
    imported: bool,
}

pub type TokensRecordOf<T> = TokensRecord<
    <T as pallet::Config>::Hash,
    BoundedVec<
        TokensRecordFileOf<T>,
        <T as pallet::Config>::MaxTokensRecordFiles
    >,
    <T as frame_system::Config>::AccountId,
>;

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
pub struct TokensRecordFile<Hash> {
    name: Hash,
    content_type: Hash,
    size: u32,
    hash: Hash,
}

pub type TokensRecordFileOf<T> = TokensRecordFile<
    <T as pallet::Config>::Hash,
>;

pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId, >>::NegativeImbalance;

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
pub struct Sponsorship<AccountId, EthereumAddress, LocId> {
    sponsor: AccountId,
    sponsored_account: SupportedAccountId<AccountId, EthereumAddress>,
    legal_officer: AccountId,
    loc_id: Option<LocId>,
    imported: bool,
}

pub type SponsorshipOf<T> = Sponsorship<
    <T as frame_system::Config>::AccountId,
    <T as Config>::EthereumAddress,
    <T as Config>::LocId,
>;

pub mod weights;

pub trait Hasher<Hash> {

    fn hash(data: &Vec<u8>) -> Hash;
}

#[frame_support::pallet]
pub mod pallet {
	use sp_std::collections::btree_set::BTreeSet;
    use frame_system::pallet_prelude::*;
    use frame_support::{
        dispatch::DispatchResultWithPostInfo,
        pallet_prelude::*, traits::tokens::Balance,
    };
    use codec::HasCompact;
    use frame_support::traits::Currency;
    use logion_shared::{
        LocQuery, LocValidity, IsLegalOfficer, RewardDistributor,
        DistributionKey, Beneficiary,
    };
    use crate::SupportedAccountId::Polkadot;
    use super::*;
    pub use crate::weights::WeightInfo;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// LOC identifier
        type LocId: Member + Parameter + Default + Copy + HasCompact + Ord + MaxEncodedLen;

        /// Type for hashes stored in LOCs
        type Hash: Member + Parameter + Default + Copy + Ord + MaxEncodedLen;

        /// Type for hasher
        type Hasher: Hasher<<Self as pallet::Config>::Hash>;

        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;

        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Collection item identifier
        type CollectionItemId: Member + Parameter + Default + Copy + MaxEncodedLen;

        /// Query for checking that a signer is a legal officer
        type IsLegalOfficer: IsLegalOfficer<Self::AccountId, Self::RuntimeOrigin>;

        /// Token Record identifier
        type TokensRecordId: Member + Parameter + Default + Copy + MaxEncodedLen;

		/// The maximum number of LOCs per account
		type MaxAccountLocs: Get<u32>;

        /// The maximum number of metadata items per LOC
        type MaxLocMetadata: Get<u32> + TypeInfo;

        /// The maximum number of files per LOC
        type MaxLocFiles: Get<u32> + TypeInfo;

        /// The maximum number of links per LOC
        type MaxLocLinks: Get<u32> + TypeInfo;

        /// The maximum number of files per collection item
        type MaxCollectionItemFiles: Get<u32>;

        /// The maximum number of files per collection item
        type MaxCollectionItemTCs: Get<u32>;

        /// The maximum number of files per token record
        type MaxTokensRecordFiles: Get<u32>;

        /// The currency trait.
        type Currency: ReservableCurrency<Self::AccountId>;

        /// The variable part of the Fee to pay to store a file (per byte)
        type FileStorageByteFee: Get<BalanceOf<Self>>;

        /// The constant part of the Fee to pay to store a file.
        type FileStorageEntryFee: Get<BalanceOf<Self>>;

        /// Used to payout fees
        type RewardDistributor: RewardDistributor<NegativeImbalanceOf<Self>, BalanceOf<Self>, Self::AccountId, Self::RuntimeOrigin, Self::IsLegalOfficer>;

        /// Used to compute storage fees rewards
        type FileStorageFeeDistributionKey: Get<DistributionKey>;

        /// Ethereum Address type
        type EthereumAddress: Member + Parameter + Default + Copy + MaxEncodedLen;

        /// The identifier of a sponsorship
        type SponsorshipId: Member + Parameter + Default + Copy + HasCompact + MaxEncodedLen;

        /// The certificate fee per issued token
        type CertificateFee: Get<BalanceOf<Self>>;

        /// Used to compute certificate fees rewards
        type CertificateFeeDistributionKey: Get<DistributionKey>;

        /// The collection item's token issuance type
        type TokenIssuance: Balance + Into<BalanceOf<Self>>;

        /// Used to compute value fees rewards
        type ValueFeeDistributionKey: Get<DistributionKey>;

        /// Used to compute collection item fees rewards
        type CollectionItemFeeDistributionKey: Get<DistributionKey>;

        /// Used to compute token record fees rewards
        type TokensRecordFeeDistributionKey: Get<DistributionKey>;

        /// Used to payout legal fees of an Identity LOC
        type IdentityLocLegalFeeDistributionKey: Get<DistributionKey>;

        /// Used to payout legal fees of a Transaction LOC
        type TransactionLocLegalFeeDistributionKey: Get<DistributionKey>;

        /// Used to payout legal fees of a Collection LOC
        type CollectionLocLegalFeeDistributionKey: Get<DistributionKey>;

		/// Loc ID factory for benchmark
		#[cfg(feature = "runtime-benchmarks")]
		type LocIdFactory: LocIdFactory<Self::LocId>;

		/// Collection Item ID factory for benchmark
		#[cfg(feature = "runtime-benchmarks")]
		type CollectionItemIdFactory: CollectionItemIdFactory<Self::CollectionItemId>;

		/// Tokens Record ID factory for benchmark
		#[cfg(feature = "runtime-benchmarks")]
		type TokensRecordIdFactory: TokensRecordIdFactory<Self::TokensRecordId>;

		/// Ethereum address factory for benchmark
		#[cfg(feature = "runtime-benchmarks")]
		type EthereumAddressFactory: EthereumAddressFactory<Self::EthereumAddress>;

		/// Sponsorship ID factory for benchmark
		#[cfg(feature = "runtime-benchmarks")]
		type SponsorshipIdFactory: SponsorshipIdFactory<Self::SponsorshipId>;
    }

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// All LOCs indexed by ID.
    #[pallet::storage]
    #[pallet::getter(fn loc)]
    pub type LocMap<T> = StorageMap<_, Blake2_128Concat, <T as Config>::LocId, LegalOfficerCaseOf<T>>;

    /// Requested LOCs by account ID.
    #[pallet::storage]
    #[pallet::getter(fn account_locs)]
    pub type AccountLocsMap<T> = StorageMap<_, Blake2_128Concat, <T as frame_system::Config>::AccountId, BoundedVec<<T as Config>::LocId, <T as Config>::MaxAccountLocs>>;

	/// Collection items by LOC ID.
    #[pallet::storage]
    #[pallet::getter(fn collection_items)]
    pub type CollectionItemsMap<T> = StorageDoubleMap<_, Blake2_128Concat, <T as Config>::LocId, Blake2_128Concat, <T as Config>::CollectionItemId, CollectionItemOf<T>>;

    /// Collection size by LOC ID.
    #[pallet::storage]
    #[pallet::getter(fn collection_size)]
    pub type CollectionSizeMap<T> = StorageMap<_, Blake2_128Concat, <T as Config>::LocId, CollectionSize>;

    /// Collection tokens records by LOC ID and record ID.
    #[pallet::storage]
    #[pallet::getter(fn tokens_records)]
    pub type TokensRecordsMap<T> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        <T as Config>::LocId, Blake2_128Concat,
        <T as Config>::TokensRecordId,
        TokensRecordOf<T>
    >;

    /// Verified Issuers by owner
    #[pallet::storage]
    #[pallet::getter(fn verified_issuers)]
    pub type VerifiedIssuersMap<T> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        <T as frame_system::Config>::AccountId, // owner
        Blake2_128Concat,
        <T as frame_system::Config>::AccountId, // issuer
        VerifiedIssuerOf<T>,
    >;

    /// Verified Issuers by LOC
    #[pallet::storage]
    #[pallet::getter(fn selected_verified_issuers)]
    pub type VerifiedIssuersByLocMap<T> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        <T as Config>::LocId, // LOC
        Blake2_128Concat,
        <T as frame_system::Config>::AccountId, // issuer
        ()
    >;

    /// LOCs by Verified Issuer
    #[pallet::storage]
    #[pallet::getter(fn locs_by_verified_issuer)]
    pub type LocsByVerifiedIssuerMap<T> = StorageNMap<
        _,
        (
            NMapKey<Blake2_128Concat, <T as frame_system::Config>::AccountId>, // issuer
            NMapKey<Blake2_128Concat, <T as frame_system::Config>::AccountId>, // owner
            NMapKey<Blake2_128Concat, <T as Config>::LocId>,
        ),
        ()
    >;

    /// Sponsorships indexed by ID
    #[pallet::storage]
    #[pallet::getter(fn sponsorship)]
    pub type SponsorshipMap<T> = StorageMap<_, Blake2_128Concat, <T as Config>::SponsorshipId, SponsorshipOf<T>>;

	/// Invited Contributors by LOC
	#[pallet::storage]
	#[pallet::getter(fn selected_invited_contributors)]
	pub type InvitedContributorsByLocMap<T> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		<T as Config>::LocId,
		Blake2_128Concat,
		<T as frame_system::Config>::AccountId, // invited contributor
        ()
	>;

	#[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Issued upon LOC creation. [locId]
        LocCreated(T::LocId),
        /// Issued when LOC is closed. [locId]
        LocClosed(T::LocId),
        /// Issued when LOC is voided. [locId]
        LocVoid(T::LocId),
        /// Issued when an item was added to a collection. [locId, collectionItemId]
        ItemAdded(T::LocId, T::CollectionItemId),
        /// Issued when File Storage Fee is withdrawn. [payerAccountId, storageFee]
        StorageFeeWithdrawn(T::AccountId, BalanceOf<T>),
        /// Issued when a sponsorship was successfully created [sponsorship_id, sponsor, sponsored_account]
        SponsorshipCreated(T::SponsorshipId, T::AccountId, SupportedAccountId<T::AccountId, T::EthereumAddress>),
        /// Issued when a sponsorship was successfully withdrawn [sponsorship_id, sponsor, sponsored_account]
        SponsorshipWithdrawn(T::SponsorshipId, T::AccountId, SupportedAccountId<T::AccountId, T::EthereumAddress>),
        /// Issued when Legal Fee is withdrawn. [payerAccountId, beneficiary, legalFee]
        LegalFeeWithdrawn(T::AccountId, Beneficiary<T::AccountId>, BalanceOf<T>),
        /// Issued when Certificate Fee is withdrawn. [payerAccountId, fee]
        CertificateFeeWithdrawn(T::AccountId, BalanceOf<T>),
        /// Issued when Value Fee is withdrawn. [payerAccountId, storageFee]
        ValueFeeWithdrawn(T::AccountId, BalanceOf<T>),
        /// Issued when Collection Item Fee is withdrawn. [payerAccountId, fee, beneficiary, amountReceived]
        CollectionItemFeeWithdrawn(T::AccountId, BalanceOf<T>, Beneficiary<T::AccountId>, BalanceOf<T>),
        /// Issued when Token Record Fee is withdrawn. [payerAccountId, fee, beneficiary, amountReceived]
        TokensRecordFeeWithdrawn(T::AccountId, BalanceOf<T>, Beneficiary<T::AccountId>, BalanceOf<T>),
        /// Issued upon LOC import. [locId]
        LocImported(T::LocId),
        /// Issued upon collection item import. [locId, collectionItemId]
        ItemImported(T::LocId, T::CollectionItemId),
        /// Issued upon tokens record import. [locId, recordId]
        TokensRecordImported(T::LocId, T::TokensRecordId),
        /// Issued upon sponsorship import. [sponsorshipId]
        SponsorshipImported(T::SponsorshipId)
    }

    #[pallet::error]
    pub enum Error<T> {
        /// The LOC ID has already been used.
        AlreadyExists,
        /// Target LOC does not exist
        NotFound,
        /// Unauthorized LOC operation
        Unauthorized,
        /// Occurs when trying to mutate a closed LOC
        CannotMutate,
        /// Occurs when trying to close an already closed LOC
        AlreadyClosed,
        /// Occurs when trying to link to a non-existent LOC
        LinkedLocNotFound,
        /// Occurs when trying to replace void LOC with a non-existent LOC
        ReplacerLocNotFound,
        /// Occurs when trying to void a LOC already void
        AlreadyVoid,
        /// Occurs when trying to void a LOC by replacing it with an already void LOC
        ReplacerLocAlreadyVoid,
        /// Occurs when trying to void a LOC by replacing it with a LOC already replacing another LOC
        ReplacerLocAlreadyReplacing,
        /// Occurs when trying to mutate a void LOC
        CannotMutateVoid,
        /// Unexpected requester given LOC type
        UnexpectedRequester,
        /// Occurs when trying to void a LOC by replacing it with a LOC of a different type
        ReplacerLocWrongType,
        /// Submitter is not consistent with caller
        InvalidSubmitter,
        /// A collection LOC must be limited in time and/or quantity of items
        CollectionHasNoLimit,
        /// Item cannot be added to given collection, it may be missing or limits are reached
        WrongCollectionLoc,
        /// An item with same identifier already exists in the collection
        CollectionItemAlreadyExists,
        /// Collection Item cannot be added to given collection because some fields contain too many bytes
        CollectionItemTooMuchData,
        /// The collection limits have been reached
        CollectionLimitsReached,
        /// Metadata Item cannot be added to given LOC because submitted data are invalid
        MetadataItemInvalid,
        /// File cannot be added to given LOC because submitted data are invalid
        FileInvalid,
        /// Link cannot be added to given LOC because submitted data are invalid
        LocLinkInvalid,
        /// Cannot attach files to this item because the Collection LOC does not allow it
        CannotUpload,
        /// Must attach at least one file
        MustUpload,
        /// Cannot attach same file multiple times
        DuplicateFile,
        /// Collection items with restricted delivery require an underlying token to be defined
        MissingToken,
        /// Collection items with restricted delivery require at least one associated file
        MissingFiles,
        /// TermsAndConditions LOC does not exist
        TermsAndConditionsLocNotFound,
        /// TermsAndConditions LOC not closed
        TermsAndConditionsLocNotClosed,
        /// TermsAndConditions LOC is void
        TermsAndConditionsLocVoid,
        /// Cannot add several files with same hash to LOC
        DuplicateLocFile,
        /// Cannot add several metadata items with same name to LOC
        DuplicateLocMetadata,
        /// Cannot add several links with same target to LOC
        DuplicateLocLink,
        /// Token Record cannot be added because some fields contain too many bytes
        TokensRecordTooMuchData,
        /// A token record with the same identifier already exists
        TokensRecordAlreadyExists,
        /// The token record cannot be added because either the collection is in a wrong state
        /// or the submitter is not an issuer or the requester
        CannotAddRecord,
        /// Given identity LOC does not exist or is invalid
        InvalidIdentityLoc,
        /// Issuer has already been nominated by the owner
        AlreadyNominated,
        /// Issuer is not nominated by the owner
        NotNominated,
        /// The submitter of added item cannot contribute to this LOC
        CannotSubmit,
        /// The requester has not enough funds to import file
        InsufficientFunds,
        /// The sponsorship to be withdrawn has already been used
        AlreadyUsed,
        /// The sponsorship cannot be used for creating the new LOC
        CannotLinkToSponsorship,
        /// Target Item (Metadata or File) could not be found in LOC
        ItemNotFound,
        /// Target Item (Metadata or File) is already acknowledged
        ItemAlreadyAcknowledged,
        /// There is still at least one Item (Metadata, Link or File) unacknowledged by LOC owner
        CannotCloseUnacknowledgedByOwner,
        /// Invalid token issuance
        BadTokenIssuance,
        /// There is still at least one Item (Metadata, Link or File) unacknowledged by verified issuer
        CannotCloseUnacknowledgedByVerifiedIssuer,
		/// The provided Polkadot account has no closed, non-void identity LOC
		AccountNotIdentified,
		/// There are too much metadata in the LOC
		LocMetadataTooMuchData,
		/// There are too much files in the LOC
		LocFilesTooMuchData,
		/// There are too much links in the LOC
		LocLinksTooMuchData,
		/// There are too much files in the Collection Item
		CollectionItemFilesTooMuchData,
		/// There are too much terms and conditions in the Collection Item
		CollectionItemTCsTooMuchData,
		/// There are too much LOCs linked to account
		AccountLocsTooMuchData,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {

        fn integrity_test() {
            assert!(T::FileStorageFeeDistributionKey::get().is_valid());
            assert!(T::CertificateFeeDistributionKey::get().is_valid());
            assert!(T::ValueFeeDistributionKey::get().is_valid());
            assert!(T::CollectionItemFeeDistributionKey::get().is_valid());
            assert!(T::TokensRecordFeeDistributionKey::get().is_valid());
        }

        #[cfg(feature = "try-runtime")]
        fn post_upgrade(_state: Vec<u8>) -> Result<(), sp_runtime::DispatchError> {
            assert_eq!(PalletStorageVersion::<T>::get(), StorageVersion::default());
            for loc in LocMap::<T>::iter_values() {
                assert!(!loc.imported);
            }
            for loc in CollectionItemsMap::<T>::iter_values() {
                assert!(!loc.imported);
            }
            for loc in TokensRecordsMap::<T>::iter_values() {
                assert!(!loc.imported);
            }
            for loc in VerifiedIssuersMap::<T>::iter_values() {
                assert!(!loc.imported);
            }
            for loc in SponsorshipMap::<T>::iter_values() {
                assert!(!loc.imported);
            }
            Ok(())
        }
    }

    #[derive(Encode, Decode, Eq, PartialEq, Debug, TypeInfo, MaxEncodedLen)]
    pub enum StorageVersion {
        V1,
        V2MakeLocVoid,
        V3RequesterEnum,
        V4ItemSubmitter,
        V5Collection,
        V6ItemUpload,
        V7ItemToken,
        V8AddSeal,
        V9TermsAndConditions,
        V10AddLocFileSize,
        V11EnableEthereumSubmitter,
        V12Sponsorship,
        V13AcknowledgeItems,
        V14HashLocPublicData,
        V15AddTokenIssuance,
        V16MoveTokenIssuance,
        V17HashItemRecordPublicData,
        V18AddValueFee,
        V19AcknowledgeItemsByIssuer,
        V20AddCustomLegalFee,
        V21EnableRequesterLinks,
        V22AddRecurrentFees,
        V23RemoveUselessMapsAddImported,
    }

    impl Default for StorageVersion {
        fn default() -> StorageVersion {
            return StorageVersion::V23RemoveUselessMapsAddImported;
        }
    }

    /// Storage version
    #[pallet::storage]
    #[pallet::getter(fn pallet_storage_version)]
    pub type PalletStorageVersion<T> = StorageValue<_, StorageVersion, ValueQuery>;

	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config>(PhantomData<T>);

	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self(PhantomData::<T>)
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {

		fn build(&self) {
			PalletStorageVersion::<T>::put(StorageVersion::default());
		}
	}

    #[pallet::call]
    impl<T: Config> Pallet<T> {

        /// Creates a new Polkadot Identity LOC i.e. a LOC linking a real identity to an AccountId.
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::create_polkadot_identity_loc())]
        pub fn create_polkadot_identity_loc(
            origin: OriginFor<T>,
            #[pallet::compact] loc_id: T::LocId,
            legal_officer: T::AccountId,
            legal_fee: BalanceOf<T>,
            items: ItemsParamsOf<T>,
        ) -> DispatchResultWithPostInfo {
            let requester_account_id = ensure_signed(origin)?;

            if !T::IsLegalOfficer::is_legal_officer(&legal_officer) {
                Err(Error::<T>::Unauthorized)?
            } else if <LocMap<T>>::contains_key(&loc_id) {
                Err(Error::<T>::AlreadyExists)?
            } else {
                let requester = RequesterOf::<T>::Account(requester_account_id.clone());
                let mut loc = Self::build_open_loc(&legal_officer, &requester, LocType::Identity, None, legal_fee);
                loc.ensure_can_add::<T>(&items)?;
                Self::ensure_valid_links(&items.links)?;
                let tot_size = items.files.iter()
                    .map(|file| file.size)
                    .fold(0, |tot, current| tot + current);
                Self::apply_file_storage_fee(&requester_account_id, items.files.len(), tot_size)?;
                loc.add_items::<T>(&requester_account_id, &items)?;

                Self::apply_legal_fee(&loc)?;
                <LocMap<T>>::insert(loc_id, loc);
                Self::link_with_account(&requester_account_id, &loc_id)?;

                Self::deposit_event(Event::LocCreated(loc_id));
                Ok(().into())
            }
        }

        /// Creates a new logion Identity LOC i.e. a LOC describing a real identity not yet linked to an AccountId;
        /// No Legal Fee is applied.
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::create_logion_identity_loc())]
        pub fn create_logion_identity_loc(
            origin: OriginFor<T>,
            #[pallet::compact] loc_id: T::LocId,
        ) -> DispatchResultWithPostInfo {
            let who = T::IsLegalOfficer::ensure_origin(origin.clone())?;

            if <LocMap<T>>::contains_key(&loc_id) {
                Err(Error::<T>::AlreadyExists)?
            } else {
                let requester = RequesterOf::<T>::None;
                let loc = Self::build_open_loc(&who, &requester, LocType::Identity, None, BalanceOf::<T>::zero());
                <LocMap<T>>::insert(loc_id, loc);

                Self::deposit_event(Event::LocCreated(loc_id));
                Ok(().into())
            }
        }

        /// Creates a new Polkadot Transaction LOC i.e. a LOC requested with an AccountId
        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::create_polkadot_transaction_loc())]
        pub fn create_polkadot_transaction_loc(
            origin: OriginFor<T>,
            #[pallet::compact] loc_id: T::LocId,
            legal_officer: T::AccountId,
            legal_fee: BalanceOf<T>,
            items: ItemsParamsOf<T>,
        ) -> DispatchResultWithPostInfo {
            let requester_account_id = ensure_signed(origin)?;

            if !T::IsLegalOfficer::is_legal_officer(&legal_officer) {
                Err(Error::<T>::Unauthorized)?
            } else if <LocMap<T>>::contains_key(&loc_id) {
                Err(Error::<T>::AlreadyExists)?
			} else if !Self::has_closed_identity_loc(&requester_account_id, &legal_officer) {
				Err(Error::<T>::AccountNotIdentified)?
            } else {
                let requester = RequesterOf::<T>::Account(requester_account_id.clone());
                let mut loc = Self::build_open_loc(&legal_officer, &requester, LocType::Transaction, None, legal_fee);
                loc.ensure_can_add::<T>(&items)?;
                Self::ensure_valid_links(&items.links)?;
                let tot_size = items.files.iter()
                    .map(|file| file.size)
                    .fold(0, |tot, current| tot + current);
                Self::apply_file_storage_fee(&requester_account_id, items.files.len(), tot_size)?;
                loc.add_items::<T>(&requester_account_id, &items)?;

                Self::apply_legal_fee(&loc)?;
                <LocMap<T>>::insert(loc_id, loc);
                Self::link_with_account(&requester_account_id, &loc_id)?;

                Self::deposit_event(Event::LocCreated(loc_id));
                Ok(().into())
            }
        }

        /// Creates a new logion Transaction LOC i.e. a LOC requested with a logion Identity LOC;
        /// No Legal Fee is applied.
        #[pallet::call_index(3)]
        #[pallet::weight(T::WeightInfo::create_logion_transaction_loc())]
        pub fn create_logion_transaction_loc(
            origin: OriginFor<T>,
            #[pallet::compact] loc_id: T::LocId,
            requester_loc_id: T::LocId,
        ) -> DispatchResultWithPostInfo {
            let who = T::IsLegalOfficer::ensure_origin(origin.clone())?;

            if <LocMap<T>>::contains_key(&loc_id) {
                Err(Error::<T>::AlreadyExists)?
            } else {
                let requester_loc = <LocMap<T>>::get(&requester_loc_id);
                match requester_loc {
                    None => Err(Error::<T>::UnexpectedRequester)?,
                    Some(loc) =>
                        if Self::is_valid_logion_id(&loc) {
                            Err(Error::<T>::UnexpectedRequester)?
                        } else {
                            let requester = RequesterOf::<T>::Loc(requester_loc_id.clone());
                            let new_loc = Self::build_open_loc(&who, &requester, LocType::Transaction, None, BalanceOf::<T>::zero());
                            <LocMap<T>>::insert(loc_id, new_loc);
                        },
                }

                Self::deposit_event(Event::LocCreated(loc_id));
                Ok(().into())
            }
        }

        /// Creates a new Collection LOC
        #[pallet::call_index(4)]
        #[pallet::weight(T::WeightInfo::create_collection_loc())]
        pub fn create_collection_loc(
            origin: OriginFor<T>,
            #[pallet::compact] loc_id: T::LocId,
            legal_officer: T::AccountId,
            collection_last_block_submission: Option<BlockNumberFor<T>>,
            collection_max_size: Option<u32>,
            collection_can_upload: bool,
            value_fee: BalanceOf<T>,
            legal_fee: BalanceOf<T>,
            collection_item_fee: BalanceOf<T>,
            tokens_record_fee: BalanceOf<T>,
            items: ItemsParamsOf<T>,
        ) -> DispatchResultWithPostInfo {
            let requester_account_id = ensure_signed(origin)?;

            if !T::IsLegalOfficer::is_legal_officer(&legal_officer) {
                Err(Error::<T>::Unauthorized)?
            } else if collection_last_block_submission.is_none() && collection_max_size.is_none() {
                Err(Error::<T>::CollectionHasNoLimit)?
            } else if !Self::has_closed_identity_loc(&requester_account_id, &legal_officer) {
				Err(Error::<T>::AccountNotIdentified)?
			}

            if <LocMap<T>>::contains_key(&loc_id) {
                Err(Error::<T>::AlreadyExists)?
            } else {
                let requester = RequesterOf::<T>::Account(requester_account_id.clone());
                let mut loc = Self::build_open_collection_loc(
                    &legal_officer,
                    &requester,
                    collection_last_block_submission,
                    collection_max_size,
                    collection_can_upload,
                    value_fee,
                    legal_fee,
                    collection_item_fee,
                    tokens_record_fee,
                );
                loc.ensure_can_add::<T>(&items)?;
                Self::ensure_valid_links(&items.links)?;
                let tot_size = items.files.iter()
                    .map(|file| file.size)
                    .fold(0, |tot, current| tot + current);
                Self::apply_file_storage_fee(&requester_account_id, items.files.len(), tot_size)?;
                loc.add_items::<T>(&requester_account_id, &items)?;

                Self::apply_legal_fee(&loc)?;
                if value_fee > 0_u32.into() {
                    ensure!(T::Currency::can_reserve(&requester_account_id, value_fee), Error::<T>::InsufficientFunds);
                    T::Currency::reserve(&requester_account_id, value_fee)?
                }
                <LocMap<T>>::insert(loc_id, loc);
                Self::link_with_account(&requester_account_id, &loc_id)?;

                Self::deposit_event(Event::LocCreated(loc_id));
                Ok(().into())
            }
        }

        /// Add LOC metadata
        #[pallet::call_index(5)]
        #[pallet::weight(T::WeightInfo::add_metadata())]
        pub fn add_metadata(
            origin: OriginFor<T>,
            #[pallet::compact] loc_id: T::LocId,
            item: MetadataItemParamsOf<T>
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            if !<LocMap<T>>::contains_key(&loc_id) {
                Err(Error::<T>::NotFound)?
            } else {
                let loc = <LocMap<T>>::get(&loc_id).unwrap();
                let published_by_owner: bool = Self::is_published_by_owner(&loc, &who)?;
                if !Self::is_valid_submitter(&loc_id, &loc, &item.submitter, published_by_owner) {
                    Err(Error::<T>::CannotSubmit)?
                } else if loc.closed {
                    Err(Error::<T>::CannotMutate)?
                } else if loc.void_info.is_some() {
                    Err(Error::<T>::CannotMutateVoid)?
                } else {
                    loc.ensure_can_add_metadata::<T>(&Vec::from([item.name]))?;
                    <LocMap<T>>::try_mutate(loc_id, |loc| {
                        let mutable_loc = loc.as_mut().unwrap();
                        mutable_loc.add_metadata::<T>(&who, &item)
                    })?;
                    Ok(().into())
                }
            }
        }

        /// Add file to LOC
        #[pallet::call_index(6)]
        #[pallet::weight(T::WeightInfo::add_file())]
        pub fn add_file(
            origin: OriginFor<T>,
            #[pallet::compact] loc_id: T::LocId,
            file: FileParamsOf<T>
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            if !<LocMap<T>>::contains_key(&loc_id) {
                Err(Error::<T>::NotFound)?
            } else {
                let loc = <LocMap<T>>::get(&loc_id).unwrap();
                let published_by_owner: bool = Self::is_published_by_owner(&loc, &who)?;
                if !Self::is_valid_submitter(&loc_id, &loc, &file.submitter, published_by_owner) {
                    Err(Error::<T>::CannotSubmit)?
                } else if loc.closed {
                    Err(Error::<T>::CannotMutate)?
                } else if loc.void_info.is_some() {
                    Err(Error::<T>::CannotMutateVoid)?
                } else {
                    loc.ensure_can_add_files::<T>(&Vec::from([file.hash]))?;
                    let fee_payer;
                    if loc.sponsorship_id.is_some() {
                        let sponsorship = <SponsorshipMap<T>>::get(loc.sponsorship_id.unwrap()).unwrap();
                        fee_payer = sponsorship.sponsor;
                    } else {
                        fee_payer = match loc.requester {
                            Account(requester_account) => requester_account,
                            _ => loc.owner
                        };
                    }
                    Self::apply_file_storage_fee(&fee_payer, 1, file.size)?;
                    <LocMap<T>>::try_mutate(loc_id, |loc| {
                        let mutable_loc = loc.as_mut().unwrap();
                        mutable_loc.add_file::<T>(&who, &file)
                    })?;
                    Ok(().into())
                }
            }
        }

        /// Add a link to LOC
        #[pallet::call_index(7)]
        #[pallet::weight(T::WeightInfo::add_link())]
        pub fn add_link(
            origin: OriginFor<T>,
            #[pallet::compact] loc_id: T::LocId,
            link: LocLinkParamsOf<T>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            if !<LocMap<T>>::contains_key(&loc_id) {
                Err(Error::<T>::NotFound)?
            } else {
                let loc = <LocMap<T>>::get(&loc_id).unwrap();
                let published_by_owner: bool = Self::is_published_by_owner(&loc, &who)?;
                if !Self::is_valid_submitter(&loc_id, &loc, &SupportedAccountId::Polkadot(who.clone()), published_by_owner) {
                    Err(Error::<T>::CannotSubmit)?
                } else if loc.closed {
                    Err(Error::<T>::CannotMutate)?
                } else if loc.void_info.is_some() {
                    Err(Error::<T>::CannotMutateVoid)?
                } else if !<LocMap<T>>::contains_key(&link.id) {
                    Err(Error::<T>::LinkedLocNotFound)?
                } else {
                    loc.ensure_can_add_links::<T>(&Vec::from([link.id]))?;
                    <LocMap<T>>::try_mutate(loc_id, |loc| {
                        let mutable_loc = loc.as_mut().unwrap();
                        mutable_loc.add_link::<T>(&who, &link)
                    })?;
                    Ok(().into())
                }
            }
        }

        /// Make a LOC void.
        #[pallet::call_index(10)]
        #[pallet::weight(T::WeightInfo::make_void())]
        pub fn make_void(
            origin: OriginFor<T>,
            #[pallet::compact] loc_id: T::LocId,
        ) -> DispatchResultWithPostInfo {
            Self::do_make_void(origin, loc_id, None)
        }

        /// Make a LOC void and provide a replacer.
        #[pallet::call_index(11)]
        #[pallet::weight(T::WeightInfo::make_void_and_replace())]
        pub fn make_void_and_replace(
            origin: OriginFor<T>,
            #[pallet::compact] loc_id: T::LocId,
            #[pallet::compact] replacer_loc_id: T::LocId,
        ) -> DispatchResultWithPostInfo {
            Self::do_make_void(origin, loc_id, Some(replacer_loc_id))
        }

        /// Adds an item to a collection
        #[pallet::call_index(12)]
        #[pallet::weight(T::WeightInfo::add_collection_item())]
        pub fn add_collection_item(
            origin: OriginFor<T>,
            #[pallet::compact] collection_loc_id: T::LocId,
            item_id: T::CollectionItemId,
            item_description: <T as Config>::Hash,
            item_files: Vec<CollectionItemFileOf<T>>,
            item_token: Option<CollectionItemToken<T::TokenIssuance, <T as Config>::Hash>>,
            restricted_delivery: bool,
            terms_and_conditions: Vec<TermsAndConditionsElement<T::LocId, <T as Config>::Hash>>,
        ) -> DispatchResultWithPostInfo { Self::do_add_collection_item(origin, collection_loc_id, item_id, item_description, item_files, item_token, restricted_delivery, terms_and_conditions) }

        /// Nominate an issuer
        #[pallet::call_index(14)]
        #[pallet::weight(T::WeightInfo::nominate_issuer())]
        pub fn nominate_issuer(
            origin: OriginFor<T>,
            issuer: T::AccountId,
            #[pallet::compact] identity_loc_id: T::LocId,
        ) -> DispatchResultWithPostInfo {
            let who = T::IsLegalOfficer::ensure_origin(origin.clone())?;

            let maybe_identity_loc = Self::loc(identity_loc_id);
            if maybe_identity_loc.is_none() {
                Err(Error::<T>::InvalidIdentityLoc)?
            }
            let identity_loc = maybe_identity_loc.unwrap();
            if !identity_loc.closed
                || identity_loc.void_info.is_some()
                || match identity_loc.requester { Account(requester_account) => requester_account != issuer, _ => true } {
                Err(Error::<T>::InvalidIdentityLoc)?
            } else {
                let existing_issuer = Self::verified_issuers(&who, &issuer);
                if existing_issuer.is_some() {
                    Err(Error::<T>::AlreadyNominated)?
                }
                <VerifiedIssuersMap<T>>::insert(&who, &issuer, VerifiedIssuer {
                    identity_loc: identity_loc_id,
                    imported: false,
                });
                Ok(().into())
            }
        }

        /// Dismiss an issuer
        #[pallet::call_index(15)]
        #[pallet::weight(T::WeightInfo::dismiss_issuer())]
        pub fn dismiss_issuer(
            origin: OriginFor<T>,
            issuer: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            let who = T::IsLegalOfficer::ensure_origin(origin.clone())?;

            let existing_issuer = Self::verified_issuers(&who, &issuer);
            if existing_issuer.is_none() {
                Err(Error::<T>::NotNominated)?
            }
            <VerifiedIssuersMap<T>>::remove(&who, &issuer);

            let issuer_locs: Vec<T::LocId> = <LocsByVerifiedIssuerMap<T>>::drain_prefix((&issuer, &who))
                .map(|entry| entry.0)
                .collect();
            issuer_locs.iter().for_each(|loc_id| {
                <VerifiedIssuersByLocMap<T>>::remove(loc_id, &issuer);
            });

            Ok(().into())
        }

        /// Select/unselect an issuer on a given LOC
        #[pallet::call_index(16)]
        #[pallet::weight(T::WeightInfo::set_issuer_selection())]
        pub fn set_issuer_selection(
            origin: OriginFor<T>,
            #[pallet::compact] loc_id: T::LocId,
            issuer: T::AccountId,
            selected: bool,
        ) -> DispatchResultWithPostInfo {
            let who = T::IsLegalOfficer::ensure_origin(origin.clone())?;

            if !<LocMap<T>>::contains_key(&loc_id) {
                Err(Error::<T>::NotFound)?
            } else {
                let loc = <LocMap<T>>::get(&loc_id).unwrap();
                if loc.owner != who {
                    Err(Error::<T>::Unauthorized)?
                } else if loc.void_info.is_some() {
                    Err(Error::<T>::CannotMutateVoid)?
                } else if Self::verified_issuers(&who, &issuer).is_none() {
                    Err(Error::<T>::NotNominated)?
                } else {
                    let already_issuer = Self::selected_verified_issuers(loc_id, &issuer);
                    if already_issuer.is_some() && !selected {
                        <VerifiedIssuersByLocMap<T>>::remove(loc_id, &issuer);
                        <LocsByVerifiedIssuerMap<T>>::remove((&issuer, loc.owner, loc_id));
                    } else if already_issuer.is_none() && selected {
                        <VerifiedIssuersByLocMap<T>>::insert(loc_id, &issuer, ());
                        <LocsByVerifiedIssuerMap<T>>::insert((&issuer, loc.owner, loc_id), ());
                    }
                    Ok(().into())
                }
            }
        }

        /// Add token record
        #[pallet::call_index(17)]
        #[pallet::weight(T::WeightInfo::add_tokens_record())]
        pub fn add_tokens_record(
            origin: OriginFor<T>,
            #[pallet::compact] collection_loc_id: T::LocId,
            record_id: T::TokensRecordId,
            description: <T as Config>::Hash,
            files: Vec<TokensRecordFileOf<T>>,
            charge_submitter: bool,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            let collection_loc_option = <LocMap<T>>::get(&collection_loc_id);
            match collection_loc_option {
                None => Err(Error::<T>::WrongCollectionLoc)?,
                Some(collection_loc) => {
                    if <TokensRecordsMap<T>>::contains_key(&collection_loc_id, &record_id) {
                        Err(Error::<T>::TokensRecordAlreadyExists)?
                    }
                    if !Self::can_add_record(&who, &collection_loc_id, &collection_loc) {
                        Err(Error::<T>::CannotAddRecord)?
                    }
                    if files.len() == 0 {
                        Err(Error::<T>::MustUpload)?
                    } else {
                        let files_hashes: Vec<<T as Config>::Hash> = files.iter()
                            .map(|file| file.hash)
                            .collect();
                        if !Self::has_unique_elements(&files_hashes) {
                            Err(Error::<T>::DuplicateFile)?
                        }
                    }

                    let mut bounded_files: BoundedVec<TokensRecordFileOf<T>, T::MaxTokensRecordFiles> = BoundedVec::with_bounded_capacity(files.len());
                    for file in files.iter() {
                        bounded_files.try_push(file.clone()).map_err(|_| Error::<T>::TokensRecordTooMuchData)?;
                    }
                    let fee_payer = if charge_submitter { who.clone() } else {
                        match collection_loc.requester {
                            Account(requester_account) => requester_account,
                            _ => panic!("Requester cannot pay the fees")
                        }
                    };

                    let tot_size = files.iter()
                        .map(|file| file.size)
                        .fold(0, |tot, current| tot + current);
                    Self::apply_file_storage_fee(&fee_payer, files.len(), tot_size)?;

                    let fee = collection_loc.tokens_record_fee;
                    if fee > 0_u32.into() {
                        let (beneficiary, amount) = Self::slash_and_distribute(&fee_payer, fee, &|credit| {
                            T::RewardDistributor::distribute_with_loc_owner(credit, T::TokensRecordFeeDistributionKey::get(), &collection_loc.owner)
                        })?;
                        Self::deposit_event(Event::TokensRecordFeeWithdrawn(fee_payer, fee, beneficiary, amount));
                    }

                    let record = TokensRecord {
                        description,
                        files: bounded_files,
                        submitter: who.clone(),
                        imported: false,
                    };
                    <TokensRecordsMap<T>>::insert(collection_loc_id, record_id, record);
                },
            }

            Ok(().into())
        }

        /// Creates a new Identity LOC whose requester is another address (Currently only Ethereum address is supported).
        #[pallet::call_index(18)]
        #[pallet::weight(T::WeightInfo::create_other_identity_loc())]
        pub fn create_other_identity_loc(
            origin: OriginFor<T>,
            #[pallet::compact] loc_id: T::LocId,
            requester_account_id: OtherAccountId<T::EthereumAddress>,
            #[pallet::compact] sponsorship_id: T::SponsorshipId,
            legal_fee: BalanceOf<T>,
        ) -> DispatchResultWithPostInfo {
            let who = T::IsLegalOfficer::ensure_origin(origin.clone())?;

            if <LocMap<T>>::contains_key(&loc_id) {
                Err(Error::<T>::AlreadyExists)?
            } else if !Self::can_link_to_sponsorship(&sponsorship_id, &who, &SupportedAccountId::Other(requester_account_id)) {
                Err(Error::<T>::CannotLinkToSponsorship)?
            } else {
                let requester = RequesterOf::<T>::OtherAccount(requester_account_id.clone());
                let loc = Self::build_open_loc(&who, &requester, LocType::Identity, Some(sponsorship_id), legal_fee);

                Self::apply_legal_fee(&loc)?;
                <LocMap<T>>::insert(loc_id, loc);
                Self::link_sponsorship_to_loc(&sponsorship_id, &loc_id);

                Self::deposit_event(Event::LocCreated(loc_id));
                Ok(().into())
            }
        }

        /// Creates a sponsorship.
        #[pallet::call_index(19)]
        #[pallet::weight(T::WeightInfo::sponsor())]
        pub fn sponsor(
            origin: OriginFor<T>,
            #[pallet::compact] sponsorship_id: T::SponsorshipId,
            sponsored_account: SupportedAccountId<T::AccountId, T::EthereumAddress>,
            legal_officer: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            let sponsor = ensure_signed(origin)?;

            if !T::IsLegalOfficer::is_legal_officer(&legal_officer) {
                Err(Error::<T>::Unauthorized)?
            } else if <SponsorshipMap<T>>::contains_key(&sponsorship_id) {
                Err(Error::<T>::AlreadyExists)?
            } else {
                let sponsorship = Sponsorship {
                    sponsor: sponsor.clone(),
                    sponsored_account: sponsored_account.clone(),
                    legal_officer,
                    loc_id: None,
                    imported: false,
                };
                <SponsorshipMap<T>>::insert(sponsorship_id, sponsorship);

                Self::deposit_event(Event::SponsorshipCreated(sponsorship_id, sponsor, sponsored_account));
                Ok(().into())
            }
        }

        /// Withdraws an unused sponsorship.
        #[pallet::call_index(20)]
        #[pallet::weight(T::WeightInfo::withdraw_sponsorship())]
        pub fn withdraw_sponsorship(
            origin: OriginFor<T>,
            #[pallet::compact] sponsorship_id: T::SponsorshipId,
        ) -> DispatchResultWithPostInfo {
            let sponsor = ensure_signed(origin)?;

            let maybe_sponsorship = <SponsorshipMap<T>>::get(&sponsorship_id);
            if maybe_sponsorship.is_none() {
                Err(Error::<T>::NotFound)?
            } else {
                let sponsorship = maybe_sponsorship.unwrap();
                if sponsorship.loc_id.is_some() {
                    Err(Error::<T>::AlreadyUsed)?
                } else {
                    let sponsored_account = sponsorship.sponsored_account;
                    <SponsorshipMap<T>>::remove(&sponsorship_id);

                    Self::deposit_event(Event::SponsorshipWithdrawn(sponsorship_id, sponsor, sponsored_account));
                    Ok(().into())
                }
            }
        }

        /// Acknowledge a metadata item.
        #[pallet::call_index(21)]
        #[pallet::weight(T::WeightInfo::acknowledge_metadata())]
        pub fn acknowledge_metadata(
            origin: OriginFor<T>,
            #[pallet::compact] loc_id: T::LocId,
            name: <T as pallet::Config>::Hash,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            if !<LocMap<T>>::contains_key(&loc_id) {
                Err(Error::<T>::NotFound)?
            } else {
                let loc = <LocMap<T>>::get(&loc_id).unwrap();
                let ack_by_owner = loc.owner == who;
                let ack_by_verified_issuer = Self::selected_verified_issuers(loc_id, &who).is_some();
                if !ack_by_owner && !ack_by_verified_issuer {
                    Err(Error::<T>::Unauthorized)?
                } else if loc.closed {
                    Err(Error::<T>::CannotMutate)?
                } else if loc.void_info.is_some() {
                    Err(Error::<T>::CannotMutateVoid)?
                }
                let option_item_index = loc.metadata.iter().position(|item| item.name == name);
                if option_item_index.is_none() {
                    Err(Error::<T>::ItemNotFound)?
                } else {
                    let item_index = option_item_index.unwrap();
                    if ack_by_owner && loc.metadata[item_index].acknowledged_by_owner {
                        Err(Error::<T>::ItemAlreadyAcknowledged)?
                    }
                    if ack_by_verified_issuer {
                        if loc.metadata[item_index].acknowledged_by_verified_issuer {
                            Err(Error::<T>::ItemAlreadyAcknowledged)?
                        }
                        match &loc.metadata[item_index].submitter {
                            Polkadot(polkadot_submitter) => {
                                if *polkadot_submitter != who {
                                    Err(Error::<T>::Unauthorized)?
                                }
                            },
                            _ => Err(Error::<T>::Unauthorized)?
                        }
                    }
                    <LocMap<T>>::mutate(loc_id, |loc| {
                        let mutable_loc = loc.as_mut().unwrap();
                        if ack_by_owner {
                            mutable_loc.metadata[item_index].acknowledged_by_owner = true;
                        } else {
                            mutable_loc.metadata[item_index].acknowledged_by_verified_issuer = true;
                        }
                    });
                    Ok(().into())
                }
            }
        }

        /// Acknowledge a file.
        #[pallet::call_index(22)]
        #[pallet::weight(T::WeightInfo::acknowledge_file())]
        pub fn acknowledge_file(
            origin: OriginFor<T>,
            #[pallet::compact] loc_id: T::LocId,
            hash: <T as pallet::Config>::Hash,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            if !<LocMap<T>>::contains_key(&loc_id) {
                Err(Error::<T>::NotFound)?
            } else {
                let loc = <LocMap<T>>::get(&loc_id).unwrap();
                let ack_by_owner = loc.owner == who;
                let ack_by_verified_issuer = Self::selected_verified_issuers(loc_id, &who).is_some();
                if !ack_by_owner && !ack_by_verified_issuer {
                    Err(Error::<T>::Unauthorized)?
                } else if loc.closed {
                    Err(Error::<T>::CannotMutate)?
                } else if loc.void_info.is_some() {
                    Err(Error::<T>::CannotMutateVoid)?
                }
                let option_item_index = loc.files.iter().position(|item| item.hash == hash);
                if option_item_index.is_none() {
                    Err(Error::<T>::ItemNotFound)?
                } else {
                    let item_index = option_item_index.unwrap();
                    if ack_by_owner && loc.files[item_index].acknowledged_by_owner {
                        Err(Error::<T>::ItemAlreadyAcknowledged)?
                    }
                    if ack_by_verified_issuer {
                        if loc.files[item_index].acknowledged_by_verified_issuer {
                            Err(Error::<T>::ItemAlreadyAcknowledged)?
                        }
                        match &loc.files[item_index].submitter {
                            Polkadot(polkadot_submitter) => {
                                if *polkadot_submitter != who {
                                    Err(Error::<T>::Unauthorized)?
                                }
                            },
                            _ => Err(Error::<T>::Unauthorized)?
                        }
                    }
                    <LocMap<T>>::mutate(loc_id, |loc| {
                        let mutable_loc = loc.as_mut().unwrap();
                        if ack_by_owner {
                            mutable_loc.files[item_index].acknowledged_by_owner = true;
                        } else {
                            mutable_loc.files[item_index].acknowledged_by_verified_issuer = true;
                        }
                    });
                    Ok(().into())
                }
            }
        }

        /// Acknowledge a link.
        #[pallet::call_index(23)]
        #[pallet::weight(T::WeightInfo::acknowledge_link())]
        pub fn acknowledge_link(
            origin: OriginFor<T>,
            #[pallet::compact] loc_id: T::LocId,
            #[pallet::compact] target: T::LocId,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            if !<LocMap<T>>::contains_key(&loc_id) {
                Err(Error::<T>::NotFound)?
            } else {
                let loc = <LocMap<T>>::get(&loc_id).unwrap();
                let ack_by_owner = loc.owner == who;
                let ack_by_verified_issuer = Self::selected_verified_issuers(loc_id, &who).is_some();
                if !ack_by_owner && !ack_by_verified_issuer {
                    Err(Error::<T>::Unauthorized)?
                } else if loc.closed {
                    Err(Error::<T>::CannotMutate)?
                } else if loc.void_info.is_some() {
                    Err(Error::<T>::CannotMutateVoid)?
                }
                let option_item_index = loc.links.iter().position(|item| item.id == target);
                if option_item_index.is_none() {
                    Err(Error::<T>::ItemNotFound)?
                } else {
                    let item_index = option_item_index.unwrap();
                    if ack_by_owner && loc.links[item_index].acknowledged_by_owner {
                        Err(Error::<T>::ItemAlreadyAcknowledged)?
                    }
                    if ack_by_verified_issuer {
                        if loc.links[item_index].acknowledged_by_verified_issuer {
                            Err(Error::<T>::ItemAlreadyAcknowledged)?
                        }
                        match &loc.links[item_index].submitter {
                            Polkadot(polkadot_submitter) => {
                                if *polkadot_submitter != who {
                                    Err(Error::<T>::Unauthorized)?
                                }
                            },
                            _ => Err(Error::<T>::Unauthorized)?
                        }
                    }
                    <LocMap<T>>::mutate(loc_id, |loc| {
                        let mutable_loc = loc.as_mut().unwrap();
                        if ack_by_owner {
                            mutable_loc.links[item_index].acknowledged_by_owner = true;
                        } else {
                            mutable_loc.links[item_index].acknowledged_by_verified_issuer = true;
                        }
                    });
                    Ok(().into())
                }
            }
        }

        /// Close LOC.
        #[pallet::call_index(24)]
        #[pallet::weight(T::WeightInfo::close())]
        pub fn close(
            origin: OriginFor<T>,
            #[pallet::compact] loc_id: T::LocId,
            seal: Option<<T as Config>::Hash>,
            auto_ack: bool,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            if ! <LocMap<T>>::contains_key(&loc_id) {
                Err(Error::<T>::NotFound)?
            } else {
                let loc = <LocMap<T>>::get(&loc_id).unwrap();
                if loc.owner != who {
                    Err(Error::<T>::Unauthorized)?
                } else if loc.void_info.is_some() {
                    Err(Error::<T>::CannotMutateVoid)?
                } else if loc.closed {
                    Err(Error::<T>::AlreadyClosed)?
                } else if !auto_ack && loc.has_items_unacknowledged_by_owner() {
                    Err(Error::<T>::CannotCloseUnacknowledgedByVerifiedIssuer)?
                } else if loc.has_items_unacknowledged_by_verified_issuer() {
                    Err(Error::<T>::CannotCloseUnacknowledgedByVerifiedIssuer)?
                } else {
                    <LocMap<T>>::mutate(loc_id, |loc| {
                        let mutable_loc = loc.as_mut().unwrap();
                        if auto_ack {
                            mutable_loc.metadata.iter_mut()
                                .filter(|item| !item.acknowledged_by_owner)
                                .for_each(|item| item.acknowledged_by_owner = true);
                            mutable_loc.files.iter_mut()
                                .filter(|item| !item.acknowledged_by_owner)
                                .for_each(|item| item.acknowledged_by_owner = true);
                            mutable_loc.links.iter_mut()
                                .filter(|item| !item.acknowledged_by_owner)
                                .for_each(|item| item.acknowledged_by_owner = true);
                        }
                        mutable_loc.closed = true;
                        mutable_loc.seal = seal;
                    });

                    if loc.loc_type == LocType::Collection && loc.value_fee > 0_u32.into() {
                        match loc.requester {
                            Account(requester_account) => {
                                let (credit, _) = T::Currency::slash_reserved(&requester_account, loc.value_fee);
                                T::RewardDistributor::distribute_with_loc_owner(credit, T::ValueFeeDistributionKey::get(), &loc.owner);
                                Self::deposit_event(Event::ValueFeeWithdrawn(requester_account, loc.value_fee));
                            },
                            _ => {},
                        }
                    }

                    Self::deposit_event(Event::LocClosed(loc_id));
                    Ok(().into())
                }
            }
        }

		/// Select/unselect an invited contributor on a given LOC
		#[pallet::call_index(25)]
		#[pallet::weight(T::WeightInfo::set_invited_contributor_selection())]
		pub fn set_invited_contributor_selection(
			origin: OriginFor<T>,
			#[pallet::compact] loc_id: T::LocId,
			invited_contributor: T::AccountId,
			selected: bool,
		) -> DispatchResultWithPostInfo {
			let who = ensure_signed(origin)?;
			if !<LocMap<T>>::contains_key(&loc_id) {
				Err(Error::<T>::NotFound)?
			} else {
				let loc = <LocMap<T>>::get(&loc_id).unwrap();
				match &loc.requester {
					Account(requester_account) =>
						if requester_account.clone() != who {
							Err(Error::<T>::Unauthorized)?
						},
					_ => Err(Error::<T>::Unauthorized)?
				};
				if loc.void_info.is_some() {
					Err(Error::<T>::CannotMutateVoid)?
				} else if !Self::has_closed_identity_loc(&invited_contributor, &loc.owner) {
					Err(Error::<T>::AccountNotIdentified)?
				} else {
					let already_invited_contributor = Self::selected_invited_contributors(loc_id, &invited_contributor);
					if already_invited_contributor.is_some() && !selected {
						<InvitedContributorsByLocMap<T>>::remove(loc_id, &invited_contributor);
					} else if already_invited_contributor.is_none() && selected {
						<InvitedContributorsByLocMap<T>>::insert(loc_id, &invited_contributor, ());
					}
					Ok(().into())
				}
			}
		}

		/// Import LOC data.
        #[pallet::call_index(26)]
        #[pallet::weight(T::WeightInfo::import_loc())]
        pub fn import_loc(
            origin: OriginFor<T>,
            #[pallet::compact] loc_id: T::LocId,
            requester: RequesterOf<T>,
            legal_officer: T::AccountId,
            loc_type: LocType,
            items: ItemsOf<T>,
            collection_last_block_submission: Option<BlockNumberFor<T>>,
            collection_max_size: Option<u32>,
            collection_can_upload: bool,
            value_fee: BalanceOf<T>,
            legal_fee: BalanceOf<T>,
            collection_item_fee: BalanceOf<T>,
            tokens_record_fee: BalanceOf<T>,
            sponsorship_id: Option<T::SponsorshipId>,
            seal: Option<<T as crate::pallet::Config>::Hash>,
            void_info: Option<LocVoidInfo<T::LocId>>,
            replacer_of: Option<T::LocId>,
            closed: bool,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            if <crate::pallet::LocMap<T>>::contains_key(&loc_id) {
                Err(Error::<T>::AlreadyExists)?
            } else {
                let mut loc;
                if loc_type == LocType::Identity || loc_type == LocType::Transaction {
                    loc = Self::build_open_loc(&legal_officer, &requester, loc_type, sponsorship_id, legal_fee);
                } else {
                    loc = Self::build_open_collection_loc(
                        &legal_officer,
                        &requester,
                        collection_last_block_submission,
                        collection_max_size,
                        collection_can_upload,
                        value_fee,
                        legal_fee,
                        collection_item_fee,
                        tokens_record_fee,
                    );
                }
                loc.ensure_can_import::<T>(&items)?;
                loc.import_items::<T>(&items)?;
                loc.closed = closed;
                loc.seal = seal;
                loc.void_info = void_info;
                loc.replacer_of = replacer_of;
                loc.imported = true;

                <LocMap<T>>::insert(loc_id, loc);
                match requester {
                    Requester::Account(requester_account_id) => Self::link_with_account(&requester_account_id, &loc_id)?,
                    _ => {},
                };

                Self::deposit_event(Event::LocImported(loc_id));
                Ok(().into())
            }
		}

        /// Imports a collection item
        #[pallet::call_index(27)]
        #[pallet::weight(T::WeightInfo::import_collection_item())]
        pub fn import_collection_item(
            origin: OriginFor<T>,
            #[pallet::compact] collection_loc_id: T::LocId,
            item_id: T::CollectionItemId,
            item_description: <T as Config>::Hash,
            item_files: Vec<CollectionItemFileOf<T>>,
            item_token: Option<CollectionItemToken<T::TokenIssuance, <T as Config>::Hash>>,
            restricted_delivery: bool,
            terms_and_conditions: Vec<TermsAndConditionsElement<T::LocId, <T as Config>::Hash>>,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            if restricted_delivery && item_token.is_none() {
                Err(Error::<T>::MissingToken)?
            }

            if restricted_delivery && item_files.len() == 0 {
                Err(Error::<T>::MissingFiles)?
            }

            let files_hashes: Vec<<T as Config>::Hash> = item_files.iter()
                .map(|file| file.hash)
                .collect();
            if !Self::has_unique_elements(&files_hashes) {
                Err(Error::<T>::DuplicateFile)?
            }

            let bounded_files: BoundedVec<CollectionItemFileOf<T>, T::MaxCollectionItemFiles> = BoundedVec::try_from(item_files)
                .map_err(|_| Error::<T>::CollectionItemFilesTooMuchData)?;
            let bounded_tcs: BoundedVec<TermsAndConditionsElementOf<T>, T::MaxCollectionItemTCs> = BoundedVec::try_from(terms_and_conditions)
                .map_err(|_| Error::<T>::CollectionItemTCsTooMuchData)?;
            if <CollectionItemsMap<T>>::contains_key(&collection_loc_id, &item_id) {
                Err(Error::<T>::CollectionItemAlreadyExists)?
            }
            let item = CollectionItem {
                description: item_description,
                files: bounded_files,
                token: item_token.clone(),
                restricted_delivery,
                terms_and_conditions: bounded_tcs,
                imported: true,
            };
            <CollectionItemsMap<T>>::insert(collection_loc_id, item_id, item);
            let collection_size = <CollectionSizeMap<T>>::get(&collection_loc_id).unwrap_or(0);
            <CollectionSizeMap<T>>::insert(&collection_loc_id, collection_size + 1);

            Self::deposit_event(Event::ItemImported(collection_loc_id, item_id));
            Ok(().into())
        }

        /// Imports a tokens record
        #[pallet::call_index(28)]
        #[pallet::weight(T::WeightInfo::import_tokens_record())]
        pub fn import_tokens_record(
            origin: OriginFor<T>,
            #[pallet::compact] collection_loc_id: T::LocId,
            record_id: T::TokensRecordId,
            description: <T as Config>::Hash,
            files: Vec<crate::TokensRecordFileOf<T>>,
            submitter: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            if <crate::pallet::TokensRecordsMap<T>>::contains_key(&collection_loc_id, &record_id) {
                Err(crate::pallet::Error::<T>::TokensRecordAlreadyExists)?
            }
            if files.len() == 0 {
                Err(crate::pallet::Error::<T>::MustUpload)?
            } else {
                let files_hashes: Vec<<T as Config>::Hash> = files.iter()
                    .map(|file| file.hash)
                    .collect();
                if !Self::has_unique_elements(&files_hashes) {
                    Err(crate::pallet::Error::<T>::DuplicateFile)?
                }
            }

            let mut bounded_files: BoundedVec<crate::TokensRecordFileOf<T>, T::MaxTokensRecordFiles> = BoundedVec::with_bounded_capacity(files.len());
            for file in files.iter() {
                bounded_files.try_push(file.clone()).map_err(|_| crate::pallet::Error::<T>::TokensRecordTooMuchData)?;
            }

            let record = crate::TokensRecord {
                description,
                files: bounded_files,
                submitter,
                imported: true,
            };
            <TokensRecordsMap<T>>::insert(collection_loc_id, record_id, record);

            Self::deposit_event(Event::TokensRecordImported(collection_loc_id, record_id));
            Ok(().into())
        }

        /// Imports an invited contributor selection
        #[pallet::call_index(29)]
        #[pallet::weight(T::WeightInfo::import_invited_contributor_selection())]
        pub fn import_invited_contributor_selection(
            origin: OriginFor<T>,
            #[pallet::compact] loc_id: T::LocId,
            invited_contributor: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            let already_invited_contributor = Self::selected_invited_contributors(loc_id, &invited_contributor);
            if already_invited_contributor.is_some() {
                Err(Error::<T>::AlreadyExists)?
            } else {
                <InvitedContributorsByLocMap<T>>::insert(loc_id, &invited_contributor, ());
            }
            Ok(().into())
        }

        /// Import a verified issuer
        #[pallet::call_index(30)]
        #[pallet::weight(T::WeightInfo::import_verified_issuer())]
        pub fn import_verified_issuer(
            origin: OriginFor<T>,
            legal_officer: T::AccountId,
            issuer: T::AccountId,
            #[pallet::compact] identity_loc_id: T::LocId,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            let existing_issuer = Self::verified_issuers(&legal_officer, &issuer);
            if existing_issuer.is_some() {
                Err(Error::<T>::AlreadyExists)?
            } else {
                <VerifiedIssuersMap<T>>::insert(&legal_officer, &issuer, VerifiedIssuer {
                    identity_loc: identity_loc_id,
                    imported: true,
                });
            }
            Ok(().into())
        }

        /// Import a verified issuer selection
        #[pallet::call_index(31)]
        #[pallet::weight(T::WeightInfo::import_verified_issuer_selection())]
        pub fn import_verified_issuer_selection(
            origin: OriginFor<T>,
            #[pallet::compact] loc_id: T::LocId,
            issuer: T::AccountId,
            loc_owner: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            let already_issuer = Self::selected_verified_issuers(loc_id, &issuer);
            if already_issuer.is_some() {
                Err(Error::<T>::AlreadyExists)?
            } else {
                <VerifiedIssuersByLocMap<T>>::insert(loc_id, &issuer, ());
                <LocsByVerifiedIssuerMap<T>>::insert((&issuer, loc_owner, loc_id), ());
            }
            Ok(().into())
        }

        /// Import a sponsorship.
        #[pallet::call_index(32)]
        #[pallet::weight(T::WeightInfo::import_sponsorship())]
        pub fn import_sponsorship(
            origin: OriginFor<T>,
            #[pallet::compact] sponsorship_id: T::SponsorshipId,
            sponsor: T::AccountId,
            sponsored_account: SupportedAccountId<T::AccountId, T::EthereumAddress>,
            legal_officer: T::AccountId,
            loc_id: Option<T::LocId>,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            if <SponsorshipMap<T>>::contains_key(&sponsorship_id) {
                Err(Error::<T>::AlreadyExists)?
            } else {
                let sponsorship = Sponsorship {
                    sponsor: sponsor.clone(),
                    sponsored_account: sponsored_account.clone(),
                    legal_officer,
                    loc_id,
                    imported: true,
                };
                <SponsorshipMap<T>>::insert(sponsorship_id, sponsorship);

                Self::deposit_event(Event::SponsorshipImported(sponsorship_id));
                Ok(().into())
            }
        }
    }

    impl<T: Config> LocQuery<T::LocId, <T as frame_system::Config>::AccountId> for Pallet<T> {
        fn has_closed_identity_locs(
            account: &<T as frame_system::Config>::AccountId,
            legal_officers: &Vec<<T as frame_system::Config>::AccountId>
        ) -> bool {
            Self::has_closed_identity_loc(account, &legal_officers[0]) && Self::has_closed_identity_loc(account, &legal_officers[1])
        }

        fn get_loc(loc_id: &T::LocId) -> Option<LegalOfficerCaseSummary<T::AccountId>> {
            let option_loc = <LocMap<T>>::get(&loc_id);

            match option_loc {
                Some(loc) => Some(LegalOfficerCaseSummary {
                    owner: loc.owner,
                    requester: match loc.requester {
                        Account(account) => Some(account),
                        _ => None
                    }
                }),
                _ => None
            }
        }
    }

    impl<T: Config> LocValidity<T::LocId, <T as frame_system::Config>::AccountId> for Pallet<T> {
        fn loc_valid_with_owner(
            loc_id: &<T as pallet::Config>::LocId,
            legal_officer: &<T as frame_system::Config>::AccountId,
        ) -> bool {
            Self::loc_valid_with_owner(&loc_id, &legal_officer)
        }
    }

    impl<T: Config> Pallet<T> {

        fn do_make_void(
            origin: OriginFor<T>,
            loc_id: T::LocId,
            replacer_loc_id: Option<T::LocId>
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            if !<LocMap<T>>::contains_key(&loc_id) {
                Err(Error::<T>::NotFound)?
            } else {
                let loc = <LocMap<T>>::get(&loc_id).unwrap();
                if loc.owner != who {
                    Err(Error::<T>::Unauthorized)?
                }
                if loc.void_info.is_some() {
                    Err(Error::<T>::AlreadyVoid)?
                }

                if replacer_loc_id.is_some() {
                    let replacer = replacer_loc_id.unwrap();
                    if !<LocMap<T>>::contains_key(&replacer) {
                        Err(Error::<T>::ReplacerLocNotFound)?
                    } else {
                        let replacer_loc = <LocMap<T>>::get(&replacer).unwrap();
                        if replacer_loc.void_info.is_some() {
                            Err(Error::<T>::ReplacerLocAlreadyVoid)?
                        }
                        if replacer_loc.replacer_of.is_some() {
                            Err(Error::<T>::ReplacerLocAlreadyReplacing)?
                        }
                        if !replacer_loc.loc_type.eq(&loc.loc_type) {
                            Err(Error::<T>::ReplacerLocWrongType)?
                        }
                    }
                }

                let loc_void_info = LocVoidInfo {
                    replacer:replacer_loc_id
                };
                <LocMap<T>>::mutate(loc_id, |loc| {
                    let mutable_loc = loc.as_mut().unwrap();
                    mutable_loc.void_info = Some(loc_void_info);
                });
                if replacer_loc_id.is_some() {
                    <LocMap<T>>::mutate(replacer_loc_id.unwrap(), |replacer_loc| {
                        let mutable_replacer_loc = replacer_loc.as_mut().unwrap();
                        mutable_replacer_loc.replacer_of = Some(loc_id);
                    });
                }

                if loc.loc_type == LocType::Collection && !loc.closed && loc.value_fee > 0_u32.into() {
                    match loc.requester {
                        Account(requester_account) => {
                            T::Currency::unreserve(&requester_account, loc.value_fee);
                        },
                        _ => {},
                    }
                }

                Self::deposit_event(Event::LocVoid(loc_id));
                Ok(().into())
            }
        }

        fn has_closed_identity_loc(
            account: &<T as frame_system::Config>::AccountId,
            legal_officer: &<T as frame_system::Config>::AccountId
        ) -> bool {
            let value = <AccountLocsMap<T>>::get(account);
            match value {
                Some(loc_ids) => {
                    return loc_ids.iter().map(|id| <LocMap<T>>::get(id))
                        .filter(|option| option.is_some())
                        .map(|some| some.unwrap())
                        .find(|loc| loc.owner == *legal_officer && loc.loc_type == LocType::Identity && loc.closed)
                        .is_some();
                }
                None => false
            }
        }

        fn loc_valid_with_owner(
            loc_id: &<T as Config>::LocId,
            legal_officer: &<T as frame_system::Config>::AccountId
        ) -> bool {
            let loc = <LocMap<T>>::get(loc_id);
            match loc {
                Some(loc) => {
                    return loc.closed && loc.void_info.is_none() && loc.owner == *legal_officer;
                }
                None => false
            }
        }

        fn link_with_account(
            account_id: &<T as frame_system::Config>::AccountId,
            loc_id: &<T as Config>::LocId,
        ) -> Result<(), sp_runtime::DispatchError> {
            if <AccountLocsMap<T>>::contains_key(account_id) {
                <AccountLocsMap<T>>::mutate(account_id, |locs| {
                    let list = locs.as_mut().unwrap();
                    list.try_push(loc_id.clone())
                }).map_err(|_| Error::<T>::AccountLocsTooMuchData)?;
				Ok(())
            } else {
				let mut list: BoundedVec<<T as Config>::LocId, <T as Config>::MaxAccountLocs> = BoundedVec::new();
				list.try_push(loc_id.clone())
					.map_err(|_| Error::<T>::AccountLocsTooMuchData)?;
				<AccountLocsMap<T>>::insert(account_id, list);
				Ok(())
            }
        }

        fn is_valid_logion_id(loc: &LegalOfficerCaseOf<T>) -> bool {
            loc.loc_type != LocType::Identity
                || match loc.requester { RequesterOf::<T>::None => false, _ => true }
                || !loc.closed
                || loc.void_info.is_some()
        }

        fn build_open_loc(
            legal_officer: &T::AccountId,
            requester: &RequesterOf<T>,
            loc_type: LocType,
            sponsorship_id: Option<T::SponsorshipId>,
            legal_fee: BalanceOf<T>,
        ) -> LegalOfficerCaseOf<T> {
            LegalOfficerCaseOf::<T> {
                owner: legal_officer.clone(),
                requester: requester.clone(),
                metadata: BoundedVec::new(),
                files: BoundedVec::new(),
                closed: false,
                loc_type: loc_type.clone(),
                links: BoundedVec::new(),
                void_info: None,
                replacer_of: None,
                collection_last_block_submission: None,
                collection_max_size: None,
                collection_can_upload: false,
                seal: None,
                sponsorship_id: sponsorship_id.clone(),
                value_fee: 0u32.into(),
                legal_fee: legal_fee.clone(),
                collection_item_fee: 0u32.into(),
                tokens_record_fee: 0u32.into(),
                imported: false,
            }
        }

        fn build_open_collection_loc(
            who: &T::AccountId,
            requester: &RequesterOf<T>,
            collection_last_block_submission: Option<BlockNumberFor<T>>,
            collection_max_size: Option<CollectionSize>,
            collection_can_upload: bool,
            value_fee: BalanceOf<T>,
            legal_fee: BalanceOf<T>,
            collection_item_fee: BalanceOf<T>,
            tokens_record_fee: BalanceOf<T>,
        ) -> LegalOfficerCaseOf<T> {
            LegalOfficerCaseOf::<T> {
                owner: who.clone(),
                requester: requester.clone(),
                metadata: BoundedVec::new(),
                files: BoundedVec::new(),
                closed: false,
                loc_type: LocType::Collection,
                links: BoundedVec::new(),
                void_info: None,
                replacer_of: None,
                collection_last_block_submission: collection_last_block_submission.clone(),
                collection_max_size: collection_max_size.clone(),
                collection_can_upload,
                seal: None,
                sponsorship_id: None,
                value_fee,
                legal_fee,
                collection_item_fee,
                tokens_record_fee,
                imported: false,
            }
        }

        fn can_add_item(who: &T::AccountId, collection_loc: &LegalOfficerCaseOf<T>) -> bool {
            collection_loc.loc_type == LocType::Collection
                && match &collection_loc.requester { Requester::Account(requester) => requester == who, _ => false }
                && collection_loc.closed
                && collection_loc.void_info.is_none()
        }

        fn collection_limits_reached(collection_loc_id: &T::LocId, collection_loc: &LegalOfficerCaseOf<T>) -> bool {
            let collection_size = <CollectionSizeMap<T>>::get(collection_loc_id).unwrap_or(0);
            let current_block_number = <frame_system::Pallet<T>>::block_number();
            return match collection_loc.collection_max_size { None => false, Some(limit) => collection_size >= limit }
                || match collection_loc.collection_last_block_submission { None => false, Some(last_block) => current_block_number >= last_block };
        }

        fn has_unique_elements<I>(iter: I) -> bool
            where
                I: IntoIterator,
                I::Item: Ord,
        {
            let mut uniq = BTreeSet::new();
            iter.into_iter().all(move |x| uniq.insert(x))
        }

        fn do_add_collection_item(
            origin: OriginFor<T>,
            collection_loc_id: T::LocId,
            item_id: T::CollectionItemId,
            item_description: <T as Config>::Hash,
            item_files: Vec<CollectionItemFileOf<T>>,
            item_token: Option<CollectionItemToken<T::TokenIssuance, <T as Config>::Hash>>,
            restricted_delivery: bool,
            terms_and_conditions: Vec<TermsAndConditionsElement<T::LocId, <T as Config>::Hash>>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            if item_token.is_some() && item_token.as_ref().unwrap().token_issuance < 1_u32.into() {
                Err(Error::<T>::BadTokenIssuance)?
            }

            if restricted_delivery && item_token.is_none() {
                Err(Error::<T>::MissingToken)?
            }

            if restricted_delivery && item_files.len() == 0 {
                Err(Error::<T>::MissingFiles)?
            }

            let collection_loc_option = <LocMap<T>>::get(&collection_loc_id);
            match collection_loc_option {
                None => Err(Error::<T>::WrongCollectionLoc)?,
                Some(collection_loc) => {
                    if <CollectionItemsMap<T>>::contains_key(&collection_loc_id, &item_id) {
                        Err(Error::<T>::CollectionItemAlreadyExists)?
                    }
                    if ! Self::can_add_item(&who, &collection_loc) {
                        Err(Error::<T>::WrongCollectionLoc)?
                    }
                    if Self::collection_limits_reached(&collection_loc_id, &collection_loc) {
                        Err(Error::<T>::CollectionLimitsReached)?
                    }
                    if !collection_loc.collection_can_upload && item_files.len() > 0 {
                        Err(Error::<T>::CannotUpload)?
                    }
                    if collection_loc.collection_can_upload {
                        let files_hashes: Vec<<T as Config>::Hash> = item_files.iter()
                            .map(|file| file.hash)
                            .collect();
                        if !Self::has_unique_elements(&files_hashes) {
                            Err(Error::<T>::DuplicateFile)?
                        }
                    }

                    for terms_and_conditions_element in &terms_and_conditions {
                        if !<LocMap<T>>::contains_key(&terms_and_conditions_element.tc_loc) {
                            Err(Error::<T>::TermsAndConditionsLocNotFound)?
                        } else {
                            let tc_loc = <LocMap<T>>::get(terms_and_conditions_element.tc_loc).unwrap();
                            if tc_loc.void_info.is_some() {
                                Err(Error::<T>::TermsAndConditionsLocVoid)?
                            } else if !tc_loc.closed {
                                Err(Error::<T>::TermsAndConditionsLocNotClosed)?
                            }
                        }
                    }
                    let tot_size = item_files.iter()
                        .map(|file| file.size)
                        .fold(0, |tot, current| tot + current);
                    Self::apply_file_storage_fee(&who, item_files.len(), tot_size)?;
					let bounded_files: BoundedVec<CollectionItemFileOf<T>, T::MaxCollectionItemFiles> = BoundedVec::try_from(item_files)
						.map_err(|_| Error::<T>::CollectionItemFilesTooMuchData)?;
					let bounded_tcs: BoundedVec<TermsAndConditionsElementOf<T>, T::MaxCollectionItemTCs> = BoundedVec::try_from(terms_and_conditions)
						.map_err(|_| Error::<T>::CollectionItemTCsTooMuchData)?;
					let item = CollectionItem {
                        description: item_description,
                        files: bounded_files,
                        token: item_token.clone(),
                        restricted_delivery,
                        terms_and_conditions: bounded_tcs,
                        imported: false,
                    };
                    <CollectionItemsMap<T>>::insert(collection_loc_id, item_id, item);
                    let collection_size = <CollectionSizeMap<T>>::get(&collection_loc_id).unwrap_or(0);
                    <CollectionSizeMap<T>>::insert(&collection_loc_id, collection_size + 1);

                    match item_token {
                        Some(token) => {
                            let fee = Self::calculate_certificate_fee(token.token_issuance);
                            Self::slash_and_distribute(&who, fee, &|credit| {
                                T::RewardDistributor::distribute_with_loc_owner(credit, T::CertificateFeeDistributionKey::get(), &collection_loc.owner)
                            })?;
                            Self::deposit_event(Event::CertificateFeeWithdrawn(who.clone(), fee));
                        }
                        _ => {}
                    };

                    let fee = collection_loc.collection_item_fee;
                    if fee > 0_u32.into() {
                        let (beneficiary, amount) = Self::slash_and_distribute(&who, fee, &|credit| {
                            T::RewardDistributor::distribute_with_loc_owner(credit, T::CollectionItemFeeDistributionKey::get(), &collection_loc.owner)
                        })?;
                        Self::deposit_event(Event::CollectionItemFeeWithdrawn(who.clone(), fee, beneficiary, amount));
                    }
                },
            }

            Self::deposit_event(Event::ItemAdded(collection_loc_id, item_id));
            Ok(().into())
        }

        pub fn calculate_certificate_fee(token_issuance: T::TokenIssuance) -> BalanceOf<T> {
            T::CertificateFee::get().saturating_mul(token_issuance.into())
        }

        fn can_add_record(adder: &T::AccountId, loc_id: &T::LocId, collection_loc: &LegalOfficerCaseOf<T>) -> bool {
            collection_loc.loc_type == LocType::Collection
                && (
                    match &collection_loc.requester { Account(requester) => requester == adder, _ => false }
                    || *adder == collection_loc.owner
                    || Self::selected_verified_issuers(loc_id, adder).is_some()
                    || Self::selected_invited_contributors(loc_id, adder).is_some()
                )
                && collection_loc.closed
                && collection_loc.void_info.is_none()
        }

        fn is_valid_submitter(loc_id: &T::LocId, loc: &LegalOfficerCaseOf<T>, submitter: &SupportedAccountId<T::AccountId, T::EthereumAddress>, published_by_owner: bool) -> bool {

            let polkadot_requester: Option<&T::AccountId> = match &loc.requester {
                Account(requester_account) => Some(requester_account),
                _ => None
            };
            if published_by_owner {
                match &submitter {
                    SupportedAccountId::Polkadot(polkadot_submitter) => *polkadot_submitter == loc.owner,
                    SupportedAccountId::Other(other_submitter) => match &other_submitter {
                        OtherAccountId::Ethereum(ethereum_submitter) => match &loc.requester {
                            Requester::OtherAccount(other_requester) => match &other_requester {
                                OtherAccountId::Ethereum(ethereum_requester) => *ethereum_submitter == *ethereum_requester,
                            },
                            _ => false,
                        },
                    },
                    _ => false,
                }
            } else { // published_by_requester
                match &submitter {
                    SupportedAccountId::Polkadot(polkadot_submitter) =>
                        *polkadot_submitter == polkadot_requester.unwrap().clone() || Self::selected_verified_issuers(loc_id, polkadot_submitter).is_some(),
                    _ => false
                }
            }
        }

        fn is_published_by_owner(loc: &LegalOfficerCaseOf<T>, who: &T::AccountId) -> Result<bool, sp_runtime::DispatchError> {
            let published_by_owner: bool = loc.owner == who.clone();
            if published_by_owner {
                return Ok(true);
            }
            let published_by_requester: bool =
                match &loc.requester {
                    Account(requester_account) => *requester_account == *who,
                    _ => false
                };
            if !published_by_requester {
                Err(Error::<T>::Unauthorized.into())
            } else {
                Ok(published_by_owner)
            }
        }

        fn apply_file_storage_fee(fee_payer: &T::AccountId, num_of_entries: usize, tot_size: u32) -> DispatchResult {
            let fee = Self::calculate_fee(num_of_entries as u32, tot_size);
            Self::slash_and_distribute(&fee_payer, fee, &|credit| {
                T::RewardDistributor::distribute(credit, T::FileStorageFeeDistributionKey::get())
            })?;
            Self::deposit_event(Event::StorageFeeWithdrawn(fee_payer.clone(), fee));
            Ok(())
        }

        pub fn calculate_fee(num_of_entries: u32, tot_size: u32) -> BalanceOf<T> {
            let byte_fee: BalanceOf<T> = T::FileStorageByteFee::get();
            let entry_fee: BalanceOf<T> = T::FileStorageEntryFee::get();
            byte_fee.saturating_mul(tot_size.into())
                .saturating_add(entry_fee.saturating_mul(num_of_entries.into()))
        }

        fn apply_legal_fee(loc: &LegalOfficerCaseOf<T>) -> DispatchResult {
            let fee_payer: Option<T::AccountId> = match loc.sponsorship_id {
                Some(sponsorship_id) => {
                    let sponsorship = <SponsorshipMap<T>>::get(sponsorship_id).unwrap();
                    Some(sponsorship.sponsor)
                }
                _ => {
                    match loc.requester.clone() {
                        Account(requester_account) => Some(requester_account),
                        _ => None
                    }
                }
            };
            if fee_payer.is_some() {
                let fee = loc.legal_fee;
                let (beneficiary, _) = Self::slash_and_distribute(&fee_payer.as_ref().unwrap(), fee, &|credit| {
                    let distribution_key = match loc.loc_type {
                        LocType::Identity => T::IdentityLocLegalFeeDistributionKey::get(),
                        LocType::Transaction => T::TransactionLocLegalFeeDistributionKey::get(),
                        LocType::Collection => T::CollectionItemFeeDistributionKey::get(),
                    };
                    T::RewardDistributor::distribute_with_loc_owner(credit, distribution_key, &loc.owner.clone())
                })?;
                Self::deposit_event(Event::LegalFeeWithdrawn(fee_payer.unwrap(), beneficiary, fee));
            }
            Ok(())
        }

        fn can_link_to_sponsorship(
            sponsorship_id: &T::SponsorshipId,
            expected_owner: &T::AccountId,
            expected_sponsored_account: &SupportedAccountId<T::AccountId, T::EthereumAddress>
        ) -> bool {
            let maybe_sponsorship = Self::sponsorship(sponsorship_id);
            if maybe_sponsorship.is_some() {
                let sponsorship = maybe_sponsorship.unwrap();
                sponsorship.legal_officer == *expected_owner
                    && sponsorship.sponsored_account == *expected_sponsored_account
                    && sponsorship.loc_id.is_none()
            } else {
                false
            }
        }

        fn link_sponsorship_to_loc(sponsorship_id: &T::SponsorshipId, loc_id: &T::LocId) -> () {
            <SponsorshipMap<T>>::mutate(sponsorship_id, |maybe_sponsorship| {
                let sponsorship = maybe_sponsorship.as_mut().unwrap();
                sponsorship.loc_id = Some(*loc_id);
            });
        }

        fn ensure_valid_links(links: &Vec<LocLinkParams<T::LocId, <T as pallet::Config>::Hash, T::AccountId, T::EthereumAddress>>) -> Result<(), sp_runtime::DispatchError> {
            for link in links.iter() {
                if Self::loc(link.id).is_none() {
                    Err(Error::<T>::LinkedLocNotFound)?;
                }
            }
            Ok(())
        }

        fn slash_and_distribute<F, R>(fee_payer: &T::AccountId, fee: BalanceOf<T>, distributor: &F) -> Result<R, sp_runtime::DispatchError>
            where F: Fn(NegativeImbalanceOf<T>) -> R {
            ensure!(T::Currency::can_slash(&fee_payer, fee), Error::<T>::InsufficientFunds);
            let (credit, _) = T::Currency::slash(&fee_payer, fee);
            Ok(distributor(credit))
        }
    }
}
