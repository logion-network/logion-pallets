use frame_support::traits::Get;
use frame_support::weights::Weight;
use frame_support::traits::OnRuntimeUpgrade;
use sp_io::{hashing::twox_128, storage::clear_prefix, KillStorageResult};

use crate::{Config, PalletStorageVersion, pallet::StorageVersion};
use super::*;

pub mod v23 {
    use super::*;
    use crate::*;

    pub struct RemoveUselessMaps<P: Get<&'static str>, T>(sp_std::marker::PhantomData<(P, T)>);

    impl<P: Get<&'static str>, T: Config> OnRuntimeUpgrade for RemoveUselessMaps<P, T>
        where <<T as pallet::Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance: From<u128> {

        fn on_runtime_upgrade() -> Weight {
            super::do_storage_upgrade::<T, _>(
                StorageVersion::V22AddRecurrentFees,
                StorageVersion::V23RemoveUselessMaps,
                "RemoveUselessMaps",
                || {
					super::clear_storage::<T>(P::get(), "IdentityLocLocsMap")
						.saturating_add(super::clear_storage::<T>(P::get(), "OtherAccountLocsMap"))
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
