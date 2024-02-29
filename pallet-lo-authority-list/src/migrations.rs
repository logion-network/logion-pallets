use frame_support::traits::Get;
use frame_support::weights::Weight;

use crate::{Config, PalletStorageVersion, pallet::StorageVersion};

pub mod v5 {
    use frame_support::traits::OnRuntimeUpgrade;
    use super::*;
    use crate::*;

    #[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
    pub struct HostDataV4<Region, MaxPeerIdLength: Get<u32>, MaxBaseUrlLen: Get<u32>> {
        pub node_id: Option<BoundedPeerId<MaxPeerIdLength>>,
        pub base_url: Option<BoundedVec<u8, MaxBaseUrlLen>>,
        pub region: Region,
    }

    pub type HostDataV4Of<T> = HostDataV4<
        <T as Config>::Region,
        <T as Config>::MaxPeerIdLength,
        <T as Config>::MaxBaseUrlLen,
    >;

    #[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
    pub enum LegalOfficerDataV4<AccountId, Region, MaxPeerIdLength: Get<u32>, MaxBaseUrlLen: Get<u32>> {
        Host(HostDataV4<Region, MaxPeerIdLength, MaxBaseUrlLen>),
        Guest(AccountId),
    }

    pub type LegalOfficerDataV4Of<T> = LegalOfficerDataV4<
        <T as frame_system::Config>::AccountId,
        <T as Config>::Region,
        <T as Config>::MaxPeerIdLength,
        <T as Config>::MaxBaseUrlLen,
    >;

    pub struct AddImported<T>(sp_std::marker::PhantomData<T>);

    impl<T: Config> OnRuntimeUpgrade for AddImported<T> {

        fn on_runtime_upgrade() -> Weight {
            super::do_storage_upgrade::<T, _>(
                StorageVersion::V4Region,
                StorageVersion::V5Imported,
                "AddImported",
                || {
                    let mut number_translated = 0;
                    LegalOfficerSet::<T>::translate_values(|legal_officer: LegalOfficerDataV4Of<T>| {
                        let translated = match legal_officer {
                            LegalOfficerDataV4::Host(host_data) => LegalOfficerData::Host(HostData {
                                node_id: host_data.node_id,
                                base_url: host_data.base_url,
                                region: host_data.region,
                                imported: false,
                            }),
                            LegalOfficerDataV4::Guest(account_id) => LegalOfficerData::Guest(GuestData {
                                host_id: account_id,
                                imported: false,
                            }),
                        };
                        number_translated += 1;
                        Some(translated)
                    });
                    T::DbWeight::get().reads_writes(number_translated, number_translated)
                }
            )
        }
    }
}

#[allow(dead_code)]
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
