use frame_support::traits::Get;
use frame_support::weights::Weight;
use frame_support::traits::OnRuntimeUpgrade;

use crate::{Config, PalletStorageVersion, pallet::StorageVersion};

pub mod v5 {
    use super::*;
    use crate::*;

    #[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo)]
    pub enum LegalOfficerDataV4<AccountId, Region> {
        Host(HostDataV4<Region>),
        Guest(AccountId),
    }

    pub type LegalOfficerDataV4Of<T> = LegalOfficerDataV4<
        <T as frame_system::Config>::AccountId,
        <T as pallet::Config>::Region,
    >;

    #[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo)]
    pub struct HostDataV4<Region> {
		pub node_id: Option<PeerId>,
		pub base_url: Option<Vec<u8>>,
		pub region: Region,
    }

    pub struct BoundedBaseUrl<T>(sp_std::marker::PhantomData<T>);
    impl<T: Config> OnRuntimeUpgrade for BoundedBaseUrl<T> {

        fn on_runtime_upgrade() -> Weight {
            super::do_storage_upgrade::<T, _>(
                StorageVersion::V4Region,
                StorageVersion::V4Region,
                "BoundedBaseUrl",
                || {
                    LegalOfficerSet::<T>::translate_values(|legal_officer_data: LegalOfficerDataV4Of<T>| {
                        match legal_officer_data {
                            LegalOfficerDataV4::Guest(guest_data) => Some(LegalOfficerData::Guest(guest_data)),
                            LegalOfficerDataV4::Host(host_data) => Some(LegalOfficerData::Host(HostData {
                                node_id: host_data.node_id.clone(),
                                base_url: match host_data.base_url {
									None => None,
									Some(url) => Some(BoundedVec::try_from(url).expect("Failed to migrate base_url")),
								},
                                region: host_data.region.clone(),
                            })),
                        }
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
