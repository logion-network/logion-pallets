#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub mod migrations;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use frame_support::{
    BoundedVec,
    codec::{Decode, Encode},
    dispatch::Vec,
};
use frame_support::traits::Currency;
use scale_info::TypeInfo;
use logion_shared::LegalOfficerCaseSummary;
use crate::Requester::Account;
use frame_support::sp_runtime::Saturating;

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo)]
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

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug, TypeInfo)]
pub struct MetadataItem<AccountId> {
    name: Vec<u8>,
    value: Vec<u8>,
    submitter: AccountId,
}

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug, TypeInfo)]
pub struct LocLink<LocId> {
    id: LocId,
    nature: Vec<u8>,
}

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug, TypeInfo)]
pub struct File<Hash, AccountId> {
    hash: Hash,
    nature: Vec<u8>,
    submitter: AccountId,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo)]
pub struct LocVoidInfo<LocId> {
    replacer: Option<LocId>,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo)]
pub enum Requester<AccountId, LocId> {
    None,
    Account(AccountId),
    Loc(LocId)
}

pub type RequesterOf<T> = Requester<<T as frame_system::Config>::AccountId, <T as Config>::LocId>;

impl<AccountId, LocId> Default for Requester<AccountId, LocId> {

    fn default() -> Requester<AccountId, LocId> {
        Requester::None
    }
}

pub type CollectionSize = u32;

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug, TypeInfo)]
pub struct LegalOfficerCase<AccountId, Hash, LocId, BlockNumber> {
    owner: AccountId,
    requester: Requester<AccountId, LocId>,
    metadata: Vec<MetadataItem<AccountId>>,
    files: Vec<File<Hash, AccountId>>,
    closed: bool,
    loc_type: LocType,
    links: Vec<LocLink<LocId>>,
    void_info: Option<LocVoidInfo<LocId>>,
    replacer_of: Option<LocId>,
    collection_last_block_submission: Option<BlockNumber>,
    collection_max_size: Option<CollectionSize>,
    collection_can_upload: bool,
    seal: Option<Hash>,
}

pub type LegalOfficerCaseOf<T> = LegalOfficerCase<
    <T as frame_system::Config>::AccountId,
    <T as pallet::Config>::Hash,
    <T as pallet::Config>::LocId,
    <T as frame_system::Config>::BlockNumber,
>;

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug, TypeInfo)]
pub struct TermsAndConditionsElement<LocId> {
    tc_type: Vec<u8>,
    tc_loc: LocId,
    details: Vec<u8>,
}

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug, TypeInfo)]
pub struct CollectionItem<Hash, LocId> {
    description: Vec<u8>,
    files: Vec<CollectionItemFile<Hash>>,
    token: Option<CollectionItemToken>,
    restricted_delivery: bool,
    terms_and_conditions: Vec<TermsAndConditionsElement<LocId>>,
}

pub type CollectionItemOf<T> = CollectionItem<<T as pallet::Config>::Hash, <T as pallet::Config>::LocId>;

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug, TypeInfo)]
pub struct CollectionItemFile<Hash> {
    name: Vec<u8>,
    content_type: Vec<u8>,
    size: u32,
    hash: Hash,
}

pub type CollectionItemFileOf<T> = CollectionItemFile<<T as pallet::Config>::Hash>;

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug, TypeInfo)]
pub struct CollectionItemToken {
    token_type: Vec<u8>,
    token_id: Vec<u8>,
}

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug, TypeInfo)]
pub struct VerifiedIssuer<LocId> {
    identity_loc: LocId,
}

pub type VerifiedIssuerOf<T> = VerifiedIssuer<
    <T as pallet::Config>::LocId,
>;

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug, TypeInfo)]
pub struct TokensRecord<BoundedDescription, BoundedTokensRecordFilesList, AccountId> {
    description: BoundedDescription,
    files: BoundedTokensRecordFilesList,
    submitter: AccountId,
}

pub type TokensRecordOf<T> = TokensRecord<
    BoundedVec<u8, <T as pallet::Config>::MaxTokensRecordDescriptionSize>,
    BoundedVec<TokensRecordFileOf<T>, <T as pallet::Config>::MaxTokensRecordFiles>,
    <T as frame_system::Config>::AccountId,
>;

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug, TypeInfo)]
pub struct TokensRecordFile<Hash, BoundedName, BoundedContentType> {
    name: BoundedName,
    content_type: BoundedContentType,
    size: u32,
    hash: Hash,
}

pub type TokensRecordFileOf<T> = TokensRecordFile<
    <T as pallet::Config>::Hash,
    BoundedVec<u8, <T as pallet::Config>::MaxFileNameSize>,
    BoundedVec<u8, <T as pallet::Config>::MaxFileContentTypeSize>,
>;

pub type UnboundedTokensRecordFileOf<T> = TokensRecordFile<
    <T as pallet::Config>::Hash,
    Vec<u8>,
    Vec<u8>,
>;

pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub type NegativeImbalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId, >>::NegativeImbalance;


pub mod weights;

#[frame_support::pallet]
pub mod pallet {
    use sp_std::collections::btree_set::BTreeSet;
    use frame_system::pallet_prelude::*;
    use frame_support::{
        dispatch::DispatchResultWithPostInfo,
        pallet_prelude::*,
    };
    use codec::HasCompact;
    use frame_support::traits::{Currency};
    use logion_shared::{LocQuery, LocValidity, IsLegalOfficer, RewardDistributor, DistributionKey};
    use super::*;
    pub use crate::weights::WeightInfo;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// LOC identifier
        type LocId: Member + Parameter + Default + Copy + HasCompact;

        /// Type for hashes stored in LOCs
        type Hash: Member + Parameter + Default + Copy + Ord;

        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;

        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Collection item identifier
        type CollectionItemId: Member + Parameter + Default + Copy;

        /// The maximum size of a LOC metadata name
        type MaxMetadataItemNameSize: Get<usize>;

        /// The maximum size of a LOC metadata value
        type MaxMetadataItemValueSize: Get<usize>;

        /// The maximum size of a LOC file nature
        type MaxFileNatureSize: Get<usize>;

        /// The maximum size of a LOC link nature
        type MaxLinkNatureSize: Get<usize>;

        /// The maximum size of a Collection Item description
        type MaxCollectionItemDescriptionSize: Get<usize>;

        /// The maximum size of a Collection Item Token Type
        type MaxCollectionItemTokenTypeSize: Get<usize>;

        /// The maximum size of a Collection Item Token ID
        type MaxCollectionItemTokenIdSize: Get<usize>;

        /// Query for checking that a signer is a legal officer
        type IsLegalOfficer: IsLegalOfficer<Self::AccountId, Self::RuntimeOrigin>;

        /// Token Record identifier
        type TokensRecordId: Member + Parameter + Default + Copy;

        /// The maximum size of a Token Record description
        type MaxTokensRecordDescriptionSize: Get<u32>;

        /// The maximum size of a file name
        type MaxFileNameSize: Get<u32>;

        /// The maximum size of a file's content type
        type MaxFileContentTypeSize: Get<u32>;

        /// The maximum number of files per token record
        type MaxTokensRecordFiles: Get<u32>;

        /// The currency trait.
        type Currency: Currency<Self::AccountId>;

        /// The variable part of the Fee to pay to store a file (per byte)
        type FileStorageByteFee: Get<u32>;

        /// The constant part of the Fee to pay to store a file.
        type FileStorageEntryFee: Get<u32>;

        /// Used to payout file storage fees
        type FileStorageFeeDistributor: RewardDistributor<NegativeImbalanceOf<Self>, BalanceOf<Self>>;

        /// Used to payout rewards
        type FileStorageFeeDistributionKey: Get<DistributionKey>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    /// All LOCs indexed by ID.
    #[pallet::storage]
    #[pallet::getter(fn loc)]
    pub type LocMap<T> = StorageMap<_, Blake2_128Concat, <T as Config>::LocId, LegalOfficerCaseOf<T>>;

    /// Requested LOCs by account ID.
    #[pallet::storage]
    #[pallet::getter(fn account_locs)]
    pub type AccountLocsMap<T> = StorageMap<_, Blake2_128Concat, <T as frame_system::Config>::AccountId, Vec<<T as Config>::LocId>>;

    /// Requested LOCs by logion Identity LOC.
    #[pallet::storage]
    #[pallet::getter(fn identity_loc_locs)]
    pub type IdentityLocLocsMap<T> = StorageMap<_, Blake2_128Concat, <T as Config>::LocId, Vec<<T as Config>::LocId>>;

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
    pub type TokensRecordsMap<T> = StorageDoubleMap<_, Blake2_128Concat, <T as Config>::LocId, Blake2_128Concat, <T as Config>::TokensRecordId, TokensRecordOf<T>>;

    /// Verified Issuers by guardian
    #[pallet::storage]
    #[pallet::getter(fn verified_issuers)]
    pub type VerifiedIssuersMap<T> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        <T as frame_system::Config>::AccountId, // guardian
        Blake2_128Concat,
        <T as frame_system::Config>::AccountId, // issuer
        VerifiedIssuerOf<T>,
    >;

    /// Verified Issuers by LOC
    #[pallet::storage]
    #[pallet::getter(fn verified_issuers_by_loc)]
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
            NMapKey<Blake2_128Concat, <T as frame_system::Config>::AccountId>, // guardian
            NMapKey<Blake2_128Concat, <T as Config>::LocId>,
        ),
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
        /// Submitter must be either LOC owner, either LOC requester (only when requester is a Polkadot account)
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
        /// Issuer has already been nominated by the guardian
        AlreadyNominated,
        /// Issuer is not nominated by the guardian
        NotNominated,
        /// The submitter of added item cannot contribute to this LOC
        CannotSubmit,
        /// The requester has not enough funds to import file
        InsufficientFunds,
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn integrity_test() {
            assert!(T::FileStorageFeeDistributionKey::get().is_valid());
        }
    }

    #[derive(Encode, Decode, Eq, PartialEq, Debug, TypeInfo)]
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
    }

    impl Default for StorageVersion {
        fn default() -> StorageVersion {
            return StorageVersion::V8AddSeal;
        }
    }

    /// Storage version
    #[pallet::storage]
    #[pallet::getter(fn pallet_storage_version)]
    pub type PalletStorageVersion<T> = StorageValue<_, StorageVersion, ValueQuery>;

    #[pallet::call]
    impl<T:Config> Pallet<T> {

        /// Creates a new Polkadot Identity LOC i.e. a LOC linking a real identity to an AccountId.
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::create_polkadot_identity_loc())]
        pub fn create_polkadot_identity_loc(
            origin: OriginFor<T>,
            #[pallet::compact] loc_id: T::LocId,
            requester_account_id: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            let who = T::IsLegalOfficer::ensure_origin(origin.clone())?;

            if <LocMap<T>>::contains_key(&loc_id) {
                Err(Error::<T>::AlreadyExists)?
            } else {
                let requester = RequesterOf::<T>::Account(requester_account_id.clone());
                let loc = Self::build_open_loc(&who, &requester, LocType::Identity);

                <LocMap<T>>::insert(loc_id, loc);
                Self::link_with_account(&requester_account_id, &loc_id);

                Self::deposit_event(Event::LocCreated(loc_id));
                Ok(().into())
            }
        }

        /// Creates a new logion Identity LOC i.e. a LOC describing a real identity not yet linked to an AccountId
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
                let loc = Self::build_open_loc(&who, &requester, LocType::Identity);
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
            requester_account_id: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            let who = T::IsLegalOfficer::ensure_origin(origin.clone())?;

            if <LocMap<T>>::contains_key(&loc_id) {
                Err(Error::<T>::AlreadyExists)?
            } else {
                let requester = RequesterOf::<T>::Account(requester_account_id.clone());
                let loc = Self::build_open_loc(&who, &requester, LocType::Transaction);

                <LocMap<T>>::insert(loc_id, loc);
                Self::link_with_account(&requester_account_id, &loc_id);

                Self::deposit_event(Event::LocCreated(loc_id));
                Ok(().into())
            }
        }

        /// Creates a new logion Transaction LOC i.e. a LOC requested with a logion Identity LOC
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
                            let new_loc = Self::build_open_loc(&who, &requester, LocType::Transaction);
                            <LocMap<T>>::insert(loc_id, new_loc);
                            Self::link_with_identity_loc(&requester_loc_id, &loc_id);
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
            requester_account_id: T::AccountId,
            collection_last_block_submission: Option<T::BlockNumber>,
            collection_max_size: Option<u32>,
            collection_can_upload: bool,
        ) -> DispatchResultWithPostInfo {
            let who = T::IsLegalOfficer::ensure_origin(origin.clone())?;

            if collection_last_block_submission.is_none() && collection_max_size.is_none() {
                Err(Error::<T>::CollectionHasNoLimit)?
            }

            if <LocMap<T>>::contains_key(&loc_id) {
                Err(Error::<T>::AlreadyExists)?
            } else {
                let requester = RequesterOf::<T>::Account(requester_account_id.clone());
                let loc = Self::build_open_collection_loc(
                    &who,
                    &requester,
                    collection_last_block_submission,
                    collection_max_size,
                    collection_can_upload,
                );

                <LocMap<T>>::insert(loc_id, loc);
                Self::link_with_account(&requester_account_id, &loc_id);

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
            item: MetadataItem<T::AccountId>
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            if item.name.len() > T::MaxMetadataItemNameSize::get() {
                Err(Error::<T>::MetadataItemInvalid)?
            }
            if item.value.len() > T::MaxMetadataItemValueSize::get() {
                Err(Error::<T>::MetadataItemInvalid)?
            }

            if !<LocMap<T>>::contains_key(&loc_id) {
                Err(Error::<T>::NotFound)?
            } else {
                let loc = <LocMap<T>>::get(&loc_id).unwrap();
                if loc.owner != who {
                    Err(Error::<T>::Unauthorized)?
                } else if loc.closed {
                    Err(Error::<T>::CannotMutate)?
                } else if loc.void_info.is_some() {
                    Err(Error::<T>::CannotMutateVoid)?
                } else if !Self::can_submit(&loc_id, &loc, &item.submitter) {
                    Err(Error::<T>::CannotSubmit)?
                } else {
                    if loc.metadata.iter().find(|metadata_item| metadata_item.name == item.name).is_some() {
                        Err(Error::<T>::DuplicateLocMetadata)?
                    }
                    <LocMap<T>>::mutate(loc_id, |loc| {
                        let mutable_loc = loc.as_mut().unwrap();
                        mutable_loc.metadata.push(item);
                    });
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
            file: File<<T as pallet::Config>::Hash, T::AccountId>
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            if file.nature.len() > T::MaxFileNatureSize::get() {
                Err(Error::<T>::FileInvalid)?
            }

            if !<LocMap<T>>::contains_key(&loc_id) {
                Err(Error::<T>::NotFound)?
            } else {
                let loc = <LocMap<T>>::get(&loc_id).unwrap();
                if loc.owner != who {
                    Err(Error::<T>::Unauthorized)?
                } else if loc.closed {
                    Err(Error::<T>::CannotMutate)?
                } else if loc.void_info.is_some() {
                    Err(Error::<T>::CannotMutateVoid)?
                } else if !Self::can_submit(&loc_id, &loc, &file.submitter) {
                    Err(Error::<T>::CannotSubmit)?
                } else {
                    if loc.files.iter().find(|item| item.hash == file.hash).is_some() {
                        Err(Error::<T>::DuplicateLocFile)?
                    }
                    <LocMap<T>>::mutate(loc_id, |loc| {
                        let mutable_loc = loc.as_mut().unwrap();
                        mutable_loc.files.push(file);
                    });
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
            link: LocLink<T::LocId>
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            if link.nature.len() > T::MaxLinkNatureSize::get() {
                Err(Error::<T>::LocLinkInvalid)?
            }

            if !<LocMap<T>>::contains_key(&loc_id) {
                Err(Error::<T>::NotFound)?
            } else {
                let loc = <LocMap<T>>::get(&loc_id).unwrap();
                if loc.owner != who {
                    Err(Error::<T>::Unauthorized)?
                } else if loc.closed {
                    Err(Error::<T>::CannotMutate)?
                } else if loc.void_info.is_some() {
                    Err(Error::<T>::CannotMutateVoid)?
                } else if !<LocMap<T>>::contains_key(&link.id) {
                    Err(Error::<T>::LinkedLocNotFound)?
                } else {
                    if loc.links.iter().find(|item| item.id == link.id).is_some() {
                        Err(Error::<T>::DuplicateLocLink)?
                    }
                    <LocMap<T>>::mutate(loc_id, |loc| {
                        let mutable_loc = loc.as_mut().unwrap();
                        mutable_loc.links.push(link);
                    });
                    Ok(().into())
                }
            }
        }

        /// Close LOC.
        #[pallet::call_index(8)]
        #[pallet::weight(T::WeightInfo::close())]
        pub fn close(
            origin: OriginFor<T>,
            #[pallet::compact] loc_id: T::LocId,
        ) -> DispatchResultWithPostInfo {
            Self::do_close(origin, loc_id, None)
        }

        /// Close and seal LOC.
        #[pallet::call_index(9)]
        #[pallet::weight(T::WeightInfo::close())]
        pub fn close_and_seal(
            origin: OriginFor<T>,
            #[pallet::compact] loc_id: T::LocId,
            seal: <T as Config>::Hash,
        ) -> DispatchResultWithPostInfo {
            Self::do_close(origin, loc_id, Some(seal))
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
            item_description: Vec<u8>,
            item_files: Vec<CollectionItemFileOf<T>>,
            item_token: Option<CollectionItemToken>,
            restricted_delivery: bool,
        ) -> DispatchResultWithPostInfo { Self::do_add_collection_item(origin, collection_loc_id, item_id, item_description, item_files, item_token, restricted_delivery, Vec::new()) }

        /// Adds an item with terms and conditions to a collection
        #[pallet::call_index(13)]
        #[pallet::weight(T::WeightInfo::add_collection_item())]
        pub fn add_collection_item_with_terms_and_conditions(
            origin: OriginFor<T>,
            #[pallet::compact] collection_loc_id: T::LocId,
            item_id: T::CollectionItemId,
            item_description: Vec<u8>,
            item_files: Vec<CollectionItemFileOf<T>>,
            item_token: Option<CollectionItemToken>,
            restricted_delivery: bool,
            terms_and_conditions: Vec<TermsAndConditionsElement<<T as pallet::Config>::LocId>>,
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
                    identity_loc: identity_loc_id
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
                    let already_issuer = Self::verified_issuers_by_loc(loc_id, &issuer);
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
            description: Vec<u8>,
            files: Vec<UnboundedTokensRecordFileOf<T>>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            let collection_loc_option = <LocMap<T>>::get(&collection_loc_id);
            match collection_loc_option {
                None => Err(Error::<T>::WrongCollectionLoc)?,
                Some(collection_loc) => {
                    if <TokensRecordsMap<T>>::contains_key(&collection_loc_id, &record_id) {
                        Err(Error::<T>::TokensRecordAlreadyExists)?
                    }
                    if ! Self::can_add_record(&who, &collection_loc_id, &collection_loc) {
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

                    let bounded_description: BoundedVec<u8, T::MaxTokensRecordDescriptionSize> =
                        description.clone().try_into().map_err(|_| Error::<T>::TokensRecordTooMuchData)?;
                    let mut bounded_files: BoundedVec<TokensRecordFileOf<T>, T::MaxTokensRecordFiles> = BoundedVec::with_bounded_capacity(files.len());
                    for unbounded_file in files.iter() {
                        let bounded_name =
                            unbounded_file.name.clone().try_into().map_err(|_| Error::<T>::TokensRecordTooMuchData)?;
                        let bounded_content_type =
                            unbounded_file.content_type.clone().try_into().map_err(|_| Error::<T>::TokensRecordTooMuchData)?;
                        bounded_files.try_push(TokensRecordFile {
                            name: bounded_name,
                            content_type: bounded_content_type,
                            hash: unbounded_file.hash,
                            size: unbounded_file.size,
                        }).map_err(|_| Error::<T>::TokensRecordTooMuchData)?;
                    }
                    let fee_payer = match collection_loc.requester {
                        Account(requester_account) => requester_account,
                        _ => collection_loc.owner
                    };

                    let tot_size = files.iter()
                        .map(|file| file.size)
                        .fold(0, |tot, current| tot + current);
                    Self::apply_file_storage_fee(fee_payer, files.len(), tot_size)?;
                    let record = TokensRecord {
                        description: bounded_description,
                        files: bounded_files,
                        submitter: who,
                    };
                    <TokensRecordsMap<T>>::insert(collection_loc_id, record_id, record);
                },
            }

            Ok(().into())
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
            Self::deposit_event(Event::LocVoid(loc_id));
            Ok(().into())
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
        ) {
            if <AccountLocsMap<T>>::contains_key(account_id) {
                <AccountLocsMap<T>>::mutate(account_id, |locs| {
                    let list = locs.as_mut().unwrap();
                    list.push(loc_id.clone());
                });
            } else {
                <AccountLocsMap<T>>::insert(account_id, Vec::from([loc_id.clone()]));
            }
        }

        fn link_with_identity_loc(
            requester_loc_id: &<T as Config>::LocId,
            loc_id: &<T as Config>::LocId,
        ) {
            if <IdentityLocLocsMap<T>>::contains_key(requester_loc_id) {
                <IdentityLocLocsMap<T>>::mutate(requester_loc_id, |locs| {
                    let list = locs.as_mut().unwrap();
                    list.push(loc_id.clone());
                });
            } else {
                <IdentityLocLocsMap<T>>::insert(requester_loc_id, Vec::from([loc_id.clone()]));
            }
        }

        fn is_valid_logion_id(loc: &LegalOfficerCaseOf<T>) -> bool {
            loc.loc_type != LocType::Identity
                || match loc.requester { RequesterOf::<T>::None => false, _ => true }
                || !loc.closed
                || loc.void_info.is_some()
        }

        fn build_open_loc(
            who: &T::AccountId,
            requester: &RequesterOf<T>,
            loc_type: LocType,
        ) -> LegalOfficerCaseOf<T> {
            LegalOfficerCaseOf::<T> {
                owner: who.clone(),
                requester: requester.clone(),
                metadata: Vec::new(),
                files: Vec::new(),
                closed: false,
                loc_type: loc_type.clone(),
                links: Vec::new(),
                void_info: None,
                replacer_of: None,
                collection_last_block_submission: Option::None,
                collection_max_size: Option::None,
                collection_can_upload: false,
                seal: Option::None,
            }
        }

        fn build_open_collection_loc(
            who: &T::AccountId,
            requester: &RequesterOf<T>,
            collection_last_block_submission: Option<T::BlockNumber>,
            collection_max_size: Option<CollectionSize>,
            collection_can_upload: bool,
        ) -> LegalOfficerCaseOf<T> {
            LegalOfficerCaseOf::<T> {
                owner: who.clone(),
                requester: requester.clone(),
                metadata: Vec::new(),
                files: Vec::new(),
                closed: false,
                loc_type: LocType::Collection,
                links: Vec::new(),
                void_info: None,
                replacer_of: None,
                collection_last_block_submission: collection_last_block_submission.clone(),
                collection_max_size: collection_max_size.clone(),
                collection_can_upload,
                seal: Option::None,
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

        fn do_close(
            origin: OriginFor<T>,
            loc_id: T::LocId,
            seal: Option<<T as Config>::Hash>,
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
                } else {
                    <LocMap<T>>::mutate(loc_id, |loc| {
                        let mutable_loc = loc.as_mut().unwrap();
                        mutable_loc.closed = true;
                        mutable_loc.seal = seal;
                    });

                    Self::deposit_event(Event::LocClosed(loc_id));
                    Ok(().into())
                }
            }
        }

        fn do_add_collection_item(
            origin: OriginFor<T>,
            collection_loc_id: T::LocId,
            item_id: T::CollectionItemId,
            item_description: Vec<u8>,
            item_files: Vec<CollectionItemFileOf<T>>,
            item_token: Option<CollectionItemToken>,
            restricted_delivery: bool,
            terms_and_conditions: Vec<TermsAndConditionsElement<<T as pallet::Config>::LocId>>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed(origin)?;

            if item_description.len() > T::MaxCollectionItemDescriptionSize::get() {
                Err(Error::<T>::CollectionItemTooMuchData)?
            }

            if item_token.is_some() && item_token.as_ref().unwrap().token_type.len() > T::MaxCollectionItemTokenTypeSize::get() {
                Err(Error::<T>::CollectionItemTooMuchData)?
            }

            if item_token.is_some() && item_token.as_ref().unwrap().token_id.len() > T::MaxCollectionItemTokenIdSize::get() {
                Err(Error::<T>::CollectionItemTooMuchData)?
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
                    Self::apply_file_storage_fee(who, item_files.len(), tot_size)?;
                    let item = CollectionItem {
                        description: item_description.clone(),
                        files: item_files.clone(),
                        token: item_token.clone(),
                        restricted_delivery,
                        terms_and_conditions,
                    };
                    <CollectionItemsMap<T>>::insert(collection_loc_id, item_id, item);
                    let collection_size = <CollectionSizeMap<T>>::get(&collection_loc_id).unwrap_or(0);
                    <CollectionSizeMap<T>>::insert(&collection_loc_id, collection_size + 1);
                },
            }

            Self::deposit_event(Event::ItemAdded(collection_loc_id, item_id));
            Ok(().into())
        }

        fn can_add_record(adder: &T::AccountId, loc_id: &T::LocId, collection_loc: &LegalOfficerCaseOf<T>) -> bool {
            collection_loc.loc_type == LocType::Collection
                && (
                    match &collection_loc.requester { Account(requester) => requester == adder, _ => false }
                    || *adder == collection_loc.owner
                    || Self::verified_issuers_by_loc(loc_id, adder).is_some()
                )
                && collection_loc.closed
                && collection_loc.void_info.is_none()
        }

        fn can_submit(loc_id: &T::LocId, loc: &LegalOfficerCaseOf<T>, submitter: &T::AccountId) -> bool {
            *submitter == loc.owner
                || match &loc.requester { Account(requester_account) => *submitter == *requester_account, _ => false }
                || Self::verified_issuers_by_loc(loc_id, submitter).is_some()
        }

        fn apply_file_storage_fee(fee_payer: T::AccountId, num_of_entries: usize, tot_size: u32) -> DispatchResult {
            let byte_fee: BalanceOf<T> = T::FileStorageByteFee::get().into();
            let entry_fee: BalanceOf<T> = T::FileStorageEntryFee::get().into();
            let fee =
                byte_fee.saturating_mul(tot_size.into())
                    .saturating_add(entry_fee.saturating_mul((num_of_entries as u32).into()));
            ensure!(T::Currency::can_slash(&fee_payer, fee), Error::<T>::InsufficientFunds);
            let (credit, _) = T::Currency::slash(&fee_payer, fee);
            T::FileStorageFeeDistributor::distribute(credit, T::FileStorageFeeDistributionKey::get());
            Ok(())
        }
    }
}