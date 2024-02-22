use codec::{Decode, Encode};
use frame_support::traits::Get;
use frame_support::weights::Weight;
use frame_support::traits::OnRuntimeUpgrade;
use frame_system::pallet_prelude::BlockNumberFor;
use sp_std::vec::Vec;

use crate::{Config, PalletStorageVersion, pallet::StorageVersion};
use super::*;

pub mod v23 {
    use super::*;
    use crate::*;

    #[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug, TypeInfo)]
    pub struct LegalOfficerCaseV22<AccountId, Hash, LocId, BlockNumber, EthereumAddress, SponsorshipId, Balance> {
		owner: AccountId,
		requester: Requester<AccountId, LocId, EthereumAddress>,
        metadata: Vec<MetadataItem<AccountId, EthereumAddress, Hash>>,
		files: Vec<File<Hash, AccountId, EthereumAddress>>,
		closed: bool,
		loc_type: LocType,
		links: Vec<LocLink<LocId, Hash, AccountId, EthereumAddress>>,
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

    type LegalOfficerCaseV22Of<T> = LegalOfficerCaseV22<
        <T as frame_system::Config>::AccountId,
        <T as pallet::Config>::Hash,
        <T as pallet::Config>::LocId,
        BlockNumberFor<T>,
        <T as pallet::Config>::EthereumAddress,
        <T as pallet::Config>::SponsorshipId,
        BalanceOf<T>,
    >;

	#[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug, TypeInfo)]
	pub struct CollectionItemV22<Hash, LocId, TokenIssuance> {
		description: Hash,
		files: Vec<CollectionItemFile<Hash>>,
		token: Option<CollectionItemToken<TokenIssuance, Hash>>,
		restricted_delivery: bool,
		terms_and_conditions: Vec<TermsAndConditionsElement<LocId, Hash>>,
	}

	pub type CollectionItemV22Of<T> = CollectionItemV22<
		<T as pallet::Config>::Hash,
		<T as pallet::Config>::LocId,
		<T as pallet::Config>::TokenIssuance,
	>;

    pub struct BoundedLocItems<T>(sp_std::marker::PhantomData<T>);

    impl<T: Config> OnRuntimeUpgrade for BoundedLocItems<T>
        where <<T as pallet::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance: From<u128> {

        fn on_runtime_upgrade() -> Weight {
            super::do_storage_upgrade::<T, _>(
                StorageVersion::V22AddRecurrentFees,
                StorageVersion::V23BoundedLocItems,
                "BoundedLocItems",
                || {
                    LocMap::<T>::translate_values(|loc: LegalOfficerCaseV22Of<T>| {
                        Some(LegalOfficerCaseOf::<T> {
                            owner: loc.owner,
                            requester: loc.requester,
                            metadata: BoundedVec::try_from(loc.metadata).expect("Failed to migrate metadata"),
                            files: BoundedVec::try_from(loc.files).expect("Failed to migrate files"),
                            closed: loc.closed,
                            loc_type: loc.loc_type,
                            links: BoundedVec::try_from(loc.links).expect("Failed to migrate links"),
                            void_info: loc.void_info,
                            replacer_of: loc.replacer_of,
                            collection_last_block_submission: loc.collection_last_block_submission,
                            collection_max_size: loc.collection_max_size,
                            collection_can_upload: loc.collection_can_upload,
                            seal: loc.seal,
                            sponsorship_id: loc.sponsorship_id,
                            value_fee: loc.value_fee,
                            legal_fee: loc.legal_fee,
                            collection_item_fee: loc.collection_item_fee,
                            tokens_record_fee: loc.tokens_record_fee,
                        })
                    });

					CollectionItemsMap::<T>::translate_values(| collection_item: CollectionItemV22Of<T> | {
						Some(CollectionItemOf::<T>{
							description: collection_item.description,
							files: BoundedVec::try_from(collection_item.files).expect("Failed to migrate collection item files"),
							token: collection_item.token,
							restricted_delivery: collection_item.restricted_delivery,
							terms_and_conditions: BoundedVec::try_from(collection_item.terms_and_conditions).expect("Failed to migrate collection item T&C"),

						})
					})
                }
            )
        }
    }
}

fn do_storage_upgrade<T: Config, F>(expected_version: StorageVersion, target_version: StorageVersion, migration_name: &str, migration: F) -> Weight
    where F: FnOnce() -> () {
    let storage_version = PalletStorageVersion::<T>::get();
    if storage_version == expected_version {
        migration();

        PalletStorageVersion::<T>::set(target_version);
        log::info!("✅ {:?} migration successfully executed", migration_name);
        T::BlockWeights::get().max_block
    } else {
        if storage_version != target_version {
            log::warn!("❗ {:?} cannot run migration with storage version {:?} (expected {:?})", migration_name, storage_version, expected_version);
        } else {
            log::info!("❎ {:?} execution skipped, already at target version {:?}", migration_name, target_version);
        }
        T::DbWeight::get().reads(1)
    }
}
