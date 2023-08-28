use frame_support::codec::{Decode, Encode};
use frame_support::traits::Get;
use frame_support::dispatch::Vec;
use frame_support::weights::Weight;
use frame_support::traits::OnRuntimeUpgrade;

use crate::{Config, PalletStorageVersion, pallet::StorageVersion};
use super::*;

pub mod v19 {
    use super::*;
    use crate::*;

    #[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug, TypeInfo)]
    pub struct MetadataItemV18<AccountId, EthereumAddress, Hash> {
        name: Hash,
        value: Hash,
        submitter: SupportedAccountId<AccountId, EthereumAddress>,
        acknowledged: bool,
    }

    type MetadataItemV18Of<T> = MetadataItemV18<
        <T as frame_system::Config>::AccountId,
        <T as pallet::Config>::EthereumAddress,
        <T as pallet::Config>::Hash,
    >;

    #[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug, TypeInfo)]
    struct FileV18<Hash, AccountId, EthereumAddress> {
        hash: Hash,
        nature: Hash,
        submitter: SupportedAccountId<AccountId, EthereumAddress>,
        size: u32,
        acknowledged: bool,
    }

    type FileV18Of<T> = FileV18<<T as pallet::Config>::Hash, <T as frame_system::Config>::AccountId, <T as pallet::Config>::EthereumAddress>;

    #[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug, TypeInfo)]
    pub struct LegalOfficerCaseV18<AccountId, Hash, LocId, BlockNumber, EthereumAddress, SponsorshipId, Balance> {
        owner: AccountId,
        requester: Requester<AccountId, LocId, EthereumAddress>,
        metadata: Vec<MetadataItemV18<AccountId, EthereumAddress, Hash>>,
        files: Vec<FileV18<Hash, AccountId, EthereumAddress>>,
        closed: bool,
        loc_type: LocType,
        links: Vec<LocLink<LocId, Hash>>,
        void_info: Option<LocVoidInfo<LocId>>,
        replacer_of: Option<LocId>,
        collection_last_block_submission: Option<BlockNumber>,
        collection_max_size: Option<CollectionSize>,
        collection_can_upload: bool,
        seal: Option<Hash>,
        sponsorship_id: Option<SponsorshipId>,
        value_fee: Balance,
    }

    type LegalOfficerCaseV18Of<T> = LegalOfficerCaseV18<
        <T as frame_system::Config>::AccountId,
        <T as pallet::Config>::Hash,
        <T as pallet::Config>::LocId,
        <T as frame_system::Config>::BlockNumber,
        <T as pallet::Config>::EthereumAddress,
        <T as pallet::Config>::SponsorshipId,
        BalanceOf<T>,
    >;

    pub struct AcknowledgeItemsByIssuer<T>(sp_std::marker::PhantomData<T>);

    impl<T: Config> AcknowledgeItemsByIssuer<T> {

        fn acknowledged_by_verified_issuer(loc: &LegalOfficerCaseV18Of<T>, submitter: &SupportedAccountId<T::AccountId, T::EthereumAddress>) -> bool {
            // Any polkadot submitter different from owner or requester, will be assumed to be verified issuer.
            match submitter {
                SupportedAccountId::Polkadot(polkadot_submitter) => {
                    if *polkadot_submitter == loc.owner {
                        false
                    } else {
                        let submitted_by_requester = match &loc.requester {
                            Account(polkadot_requester) => polkadot_requester == polkadot_submitter,
                            _ => false
                        };
                        !submitted_by_requester
                    }
                },
                _ => false
            }
        }
    }
    impl<T: Config> OnRuntimeUpgrade for AcknowledgeItemsByIssuer<T> {

        fn on_runtime_upgrade() -> Weight {
            super::do_storage_upgrade::<T, _>(
                StorageVersion::V18AddValueFee,
                StorageVersion::V19AcknowledgeItemsByIssuer,
                "AcknowledgeItemsByIssuer",
                || {
                    LocMap::<T>::translate_values(|loc: LegalOfficerCaseV18Of<T>| {
                        let files: Vec<File<<T as pallet::Config>::Hash, <T as frame_system::Config>::AccountId, T::EthereumAddress>> = loc.files
                            .iter()
                            .map(|file: &FileV18Of<T>| File {
                                hash: file.hash,
                                nature: file.nature,
                                submitter: file.submitter.clone(),
                                size: file.size,
                                acknowledged_by_owner: file.acknowledged,
                                acknowledged_by_verified_issuer: Self::acknowledged_by_verified_issuer(&loc, &file.submitter),
                            })
                            .collect();
                        let metadata: Vec<MetadataItem<<T as frame_system::Config>::AccountId, T::EthereumAddress, <T as pallet::Config>::Hash>> = loc.metadata
                            .iter()
                            .map(|item: &MetadataItemV18Of<T>| MetadataItem {
                                name: item.name,
                                value: item.value,
                                submitter: item.submitter.clone(),
                                acknowledged_by_owner: item.acknowledged,
                                acknowledged_by_verified_issuer: Self::acknowledged_by_verified_issuer(&loc, &item.submitter),
                            })
                            .collect();
                        Some(LegalOfficerCaseOf::<T> {
                            owner: loc.owner,
                            requester: loc.requester,
                            metadata,
                            files,
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
