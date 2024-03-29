use frame_support::traits::Get;
use frame_support::weights::Weight;
use frame_support::traits::OnRuntimeUpgrade;
use sp_io::{hashing::twox_128, storage::clear_prefix, KillStorageResult};

use crate::{Config, PalletStorageVersion, pallet::StorageVersion};
use super::*;

pub mod v23 {
    use super::*;
    use crate::*;

    pub struct RemoveUselessMapsAddImported<P: Get<&'static str>, T>(sp_std::marker::PhantomData<(P, T)>);

    impl<P: Get<&'static str>, T: Config> OnRuntimeUpgrade for RemoveUselessMapsAddImported<P, T>
        where <<T as pallet::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance: From<u128> {

        fn on_runtime_upgrade() -> Weight {
            super::do_storage_upgrade::<T, _>(
                StorageVersion::V22AddRecurrentFees,
                StorageVersion::V23RemoveUselessMapsAddImported,
                "RemoveUselessMapsAddImported",
                || {
					super::clear_storage::<T>(P::get(), "IdentityLocLocsMap")
						.saturating_add(super::clear_storage::<T>(P::get(), "OtherAccountLocsMap"))
                        .saturating_add(add_imported_flag::<T>())
                }
            )
        }
    }
}

fn do_storage_upgrade<T: Config, F>(expected_version: StorageVersion, target_version: StorageVersion, migration_name: &str, migration: F) -> Weight
    where F: FnOnce() -> Weight {
    let storage_version = PalletStorageVersion::<T>::get();
    if storage_version == expected_version {
        let weight = migration();

        PalletStorageVersion::<T>::set(target_version);
        log::info!("✅ {:?} migration successfully executed", migration_name);
        weight
    } else {
        if storage_version != target_version {
            log::warn!("❗ {:?} cannot run migration with storage version {:?} (expected {:?})", migration_name, storage_version, expected_version);
        } else {
            log::info!("❎ {:?} execution skipped, already at target version {:?}", migration_name, target_version);
        }
        T::DbWeight::get().reads(1)
    }
}

fn clear_storage<T: Config>(pallet_name: &str, storage_name: &str) -> Weight {
    let pallet_name_hash = twox_128(pallet_name.as_bytes());
	let storage_name_hash = twox_128(storage_name.as_bytes());
	let hashed_prefix = [pallet_name_hash, storage_name_hash].concat();
	let keys_removed = match clear_prefix(&hashed_prefix, None) {
		KillStorageResult::AllRemoved(value) => value,
		KillStorageResult::SomeRemaining(value) => {
			log::error!(
				"`clear_prefix` failed to remove all keys for {}.{}. THIS SHOULD NEVER HAPPEN! 🚨",
				pallet_name,
				storage_name,
			);
			value
		},
	} as u64;

	log::info!("Removed {} {}.{} keys 🧹", keys_removed, pallet_name, storage_name);

	T::DbWeight::get().reads_writes(keys_removed + 1, keys_removed)
}

fn add_imported_flag<T: Config>() -> Weight {
    let mut number_translated = 0;

    LocMap::<T>::translate_values(|loc: LegalOfficerCaseV22Of<T>| {
        let translated = LegalOfficerCase {
            owner: loc.owner,
            requester: loc.requester,
            metadata: loc.metadata,
            files: loc.files,
            closed: loc.closed,
            loc_type: loc.loc_type,
            links: loc.links,
            void_info: loc.void_info,
            replacer_of: loc.replacer_of,
            collection_last_block_submission: loc.collection_last_block_submission,
            collection_max_size: loc.collection_max_size,
            collection_can_upload: loc.collection_can_upload,
            seal: loc.seal,
            sponsorship_id: loc.sponsorship_id,
            value_fee: loc.value_fee,
            legal_fee: loc.value_fee,
            collection_item_fee: loc.collection_item_fee,
            tokens_record_fee: loc.tokens_record_fee,
            imported: false,
        };
        number_translated += 1;
        Some(translated)
    });

    CollectionItemsMap::<T>::translate_values(|loc: CollectionItemV22Of<T>| {
        let translated = CollectionItem {
            description: loc.description,
            files: loc.files,
            token: loc.token,
            restricted_delivery: loc.restricted_delivery,
            terms_and_conditions: loc.terms_and_conditions,
            imported: false,
        };
        number_translated += 1;
        Some(translated)
    });

    TokensRecordsMap::<T>::translate_values(|record: TokensRecordV22Of<T>| {
        let translated = TokensRecord {
            description: record.description,
            files: record.files,
            submitter: record.submitter,
            imported: false,
        };
        number_translated += 1;
        Some(translated)
    });

    VerifiedIssuersMap::<T>::translate_values(|issuer: VerifiedIssuerV22Of<T>| {
        let translated = VerifiedIssuer {
            identity_loc: issuer.identity_loc,
            imported: false,
        };
        number_translated += 1;
        Some(translated)
    });

    SponsorshipMap::<T>::translate_values(|sponsorship: SponsorshipV22Of<T>| {
        let translated = Sponsorship {
            sponsor: sponsorship.sponsor,
            sponsored_account: sponsorship.sponsored_account,
            legal_officer: sponsorship.legal_officer,
            loc_id: sponsorship.loc_id,
            imported: false,
        };
        number_translated += 1;
        Some(translated)
    });

    T::DbWeight::get().reads_writes(number_translated, number_translated)
}

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
pub struct LegalOfficerCaseV22<AccountId, Hash, LocId, BlockNumber, EthereumAddress, SponsorshipId, Balance,
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
}

pub type LegalOfficerCaseV22Of<T> = LegalOfficerCaseV22<
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
pub struct CollectionItemV22<Hash, TokenIssuance, BoundedCollectionItemFilesList, BoundedCollectionItemTCList> {
    description: Hash,
    files: BoundedCollectionItemFilesList,
    token: Option<CollectionItemToken<TokenIssuance, Hash>>,
    restricted_delivery: bool,
    terms_and_conditions: BoundedCollectionItemTCList,
}

pub type CollectionItemV22Of<T> = CollectionItemV22<
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
pub struct TokensRecordV22<Hash, BoundedTokensRecordFilesList, AccountId> {
    description: Hash,
    files: BoundedTokensRecordFilesList,
    submitter: AccountId,
}

pub type TokensRecordV22Of<T> = TokensRecordV22<
    <T as pallet::Config>::Hash,
    BoundedVec<
        TokensRecordFileOf<T>,
        <T as pallet::Config>::MaxTokensRecordFiles
    >,
    <T as frame_system::Config>::AccountId,
>;

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
pub struct VerifiedIssuerV22<LocId> {
    identity_loc: LocId,
}

pub type VerifiedIssuerV22Of<T> = VerifiedIssuerV22<
    <T as pallet::Config>::LocId,
>;

#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
pub struct SponsorshipV22<AccountId, EthereumAddress, LocId> {
    sponsor: AccountId,
    sponsored_account: SupportedAccountId<AccountId, EthereumAddress>,
    legal_officer: AccountId,
    loc_id: Option<LocId>,
}

pub type SponsorshipV22Of<T> = SponsorshipV22<
    <T as frame_system::Config>::AccountId,
    <T as Config>::EthereumAddress,
    <T as Config>::LocId,
>;
