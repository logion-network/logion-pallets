use codec::{Decode, Encode};
use frame_support::traits::Get;
use frame_support::weights::Weight;
use frame_support::traits::OnRuntimeUpgrade;
use frame_system::pallet_prelude::BlockNumberFor;
use sp_std::vec::Vec;

use crate::{Config, PalletStorageVersion, pallet::StorageVersion};
use super::*;

pub mod v22 {
    use super::*;
    use crate::*;

    #[derive(Encode, Decode, Default, Clone, PartialEq, Eq, Debug, TypeInfo)]
    pub struct LegalOfficerCaseV21<AccountId, Hash, LocId, BlockNumber, EthereumAddress, SponsorshipId, Balance> {
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
        legal_fee: Option<Balance>,
    }

    type LegalOfficerCaseV21Of<T> = LegalOfficerCaseV21<
        <T as frame_system::Config>::AccountId,
        <T as pallet::Config>::Hash,
        <T as pallet::Config>::LocId,
        BlockNumberFor<T>,
        <T as pallet::Config>::EthereumAddress,
        <T as pallet::Config>::SponsorshipId,
        BalanceOf<T>,
    >;

    pub struct AddRecurrentFees<T>(sp_std::marker::PhantomData<T>);

    impl<T: Config> AddRecurrentFees<T>
        where <<T as pallet::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance: From<u128> {

        fn calculate_default_legal_fee(loc: &LegalOfficerCaseV21Of<T>) -> BalanceOf<T> {
            match loc.requester {
                Requester::None => BalanceOf::<T>::zero(),   // logion identity has no legal fee
                Requester::Loc(_) => BalanceOf::<T>::zero(), // logion transaction has no legal fee
                _ => {
                    let exchange_rate: BalanceOf<T> = 200_000_000_000_000_000u128.into(); // 1 euro cent = 0.2 LGNT
                    let fee_in_euro_cent: u32 = match loc.loc_type {
                        LocType::Identity => 8_00, // 8.00 euros
                        _ => 100_00, // 100.00 euros
                    };
                    exchange_rate.saturating_mul(fee_in_euro_cent.into())
                }
            }
        }
    }

    impl<T: Config> OnRuntimeUpgrade for AddRecurrentFees<T>
        where <<T as pallet::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance: From<u128> {

        fn on_runtime_upgrade() -> Weight {
            super::do_storage_upgrade::<T, _>(
                StorageVersion::V21EnableRequesterLinks,
                StorageVersion::V22AddRecurrentFees,
                "AddRecurrentFees",
                || {
                    LocMap::<T>::translate_values(|loc: LegalOfficerCaseV21Of<T>| {
                        let legal_fee = match loc.legal_fee {
                            Some(value) => value,
                            None => Self::calculate_default_legal_fee(&loc)
                        };
                        Some(LegalOfficerCaseOf::<T> {
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
                            legal_fee,
                            collection_item_fee: 0u32.into(),
                            tokens_record_fee: 0u32.into(),
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
