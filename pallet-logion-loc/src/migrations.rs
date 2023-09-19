use frame_support::codec::{Decode, Encode};
use frame_support::traits::Get;
use frame_support::dispatch::Vec;
use frame_support::weights::Weight;
use frame_support::traits::OnRuntimeUpgrade;
use frame_system::pallet_prelude::BlockNumberFor;

use crate::{Config, PalletStorageVersion, pallet::StorageVersion};
use super::*;

pub mod v21 {
    use super::*;
    use crate::*;

    #[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug, TypeInfo)]
    pub struct LocLinkV20<LocId, Hash> {
        id: LocId,
        nature: Hash,
    }

    #[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug, TypeInfo)]
    pub struct LegalOfficerCaseV20<AccountId, Hash, LocId, BlockNumber, EthereumAddress, SponsorshipId, Balance> {
        owner: AccountId,
        requester: Requester<AccountId, LocId, EthereumAddress>,
        metadata: Vec<MetadataItem<AccountId, EthereumAddress, Hash>>,
        files: Vec<File<Hash, AccountId, EthereumAddress>>,
        closed: bool,
        loc_type: LocType,
        links: Vec<LocLinkV20<LocId, Hash>>,
        void_info: Option<LocVoidInfo<LocId>>,
        replacer_of: Option<LocId>,
        collection_last_block_submission: Option<BlockNumber>,
        collection_max_size: Option<CollectionSize>,
        collection_can_upload: bool,
        seal: Option<Hash>,
        sponsorship_id: Option<SponsorshipId>,
        value_fee: Balance,
        legal_fee: Option<Balance>,
    }

    type LegalOfficerCaseV20Of<T> = LegalOfficerCaseV20<
        <T as frame_system::Config>::AccountId,
        <T as pallet::Config>::Hash,
        <T as pallet::Config>::LocId,
        BlockNumberFor<T>,
        <T as pallet::Config>::EthereumAddress,
        <T as pallet::Config>::SponsorshipId,
        BalanceOf<T>,
    >;

    pub struct EnableRequesterLinks<T>(sp_std::marker::PhantomData<T>);
    impl<T: Config> OnRuntimeUpgrade for EnableRequesterLinks<T> {

        fn on_runtime_upgrade() -> Weight {
            super::do_storage_upgrade::<T, _>(
                StorageVersion::V20AddCustomLegalFee,
                StorageVersion::V21EnableRequesterLinks,
                "EnableRequesterLinks",
                || {
                    LocMap::<T>::translate_values(|loc: LegalOfficerCaseV20Of<T>| {
                        let links = loc.links.iter().map(|link| {
                            LocLink {
                                id: link.id,
                                nature: link.nature,
                                submitter: SupportedAccountId::Polkadot(loc.owner.clone()),
                                acknowledged_by_owner: true,
                                acknowledged_by_verified_issuer: false,
                            }
                        }).collect();
                        Some(LegalOfficerCaseOf::<T> {
                            owner: loc.owner,
                            requester: loc.requester,
                            metadata: loc.metadata,
                            files: loc.files,
                            closed: loc.closed,
                            loc_type: loc.loc_type,
                            links,
                            void_info: loc.void_info,
                            replacer_of: loc.replacer_of,
                            collection_last_block_submission: loc.collection_last_block_submission,
                            collection_max_size: loc.collection_max_size,
                            collection_can_upload: loc.collection_can_upload,
                            seal: loc.seal,
                            sponsorship_id: loc.sponsorship_id,
                            value_fee: loc.value_fee,
                            legal_fee: loc.legal_fee,
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
