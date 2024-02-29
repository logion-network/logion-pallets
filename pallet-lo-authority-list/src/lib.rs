#![cfg_attr(not(feature = "std"), no_std)]

pub mod migrations;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use codec::{Decode, Encode, MaxEncodedLen};

use frame_support::dispatch::DispatchResultWithPostInfo;
use sp_runtime::traits::BadOrigin;
use frame_support::{BoundedVec, sp_runtime, traits::EnsureOrigin};

use logion_shared::{IsLegalOfficer, LegalOfficerCreation};
use scale_info::{TypeInfo, prelude::string::String};
use serde::{Deserialize, Serialize};

use sp_core::{Get, OpaquePeerId as PeerId};
use sp_std::{
    fmt::Debug,
    str::FromStr,
    vec::Vec
};

pub use pallet::*;

#[cfg(feature = "runtime-benchmarks")]
use frame_system::RawOrigin;
use sp_core::bounded::BoundedBTreeSet;

pub trait GenesisRegion<Region>: Into<Region> {}

#[derive(Encode, Decode, Clone, PartialEq, Eq, TypeInfo, Serialize, Deserialize)]
pub struct GenesisHostData {
    pub node_id: Option<PeerId>,
    pub base_url: Option<Vec<u8>>,
    pub region: String,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
pub enum LegalOfficerData<AccountId, Region, MaxPeerIdLength: Get<u32>, MaxBaseUrlLen: Get<u32>> {
    Host(HostData<Region, MaxPeerIdLength, MaxBaseUrlLen>),
    Guest(GuestData<AccountId>),
}

pub type LegalOfficerDataOf<T> = LegalOfficerData<
    <T as frame_system::Config>::AccountId,
    <T as Config>::Region,
    <T as Config>::MaxPeerIdLength,
    <T as Config>::MaxBaseUrlLen,
>;

#[derive(Encode, Decode, Clone, Eq, Ord, PartialEq, PartialOrd, Debug, TypeInfo, MaxEncodedLen)]
pub struct BoundedPeerId<MaxPeerIdLength: Get<u32>>(pub BoundedVec<u8, MaxPeerIdLength>);

impl<MaxPeerIdLength: Get<u32>> BoundedPeerId<MaxPeerIdLength> {
	pub fn new(vec: BoundedVec<u8, MaxPeerIdLength>) -> Self {
		BoundedPeerId(vec)
	}
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
pub struct HostData<Region, MaxPeerIdLength: Get<u32>, MaxBaseUrlLen: Get<u32>> {
    pub node_id: Option<BoundedPeerId<MaxPeerIdLength>>,
	pub base_url: Option<BoundedVec<u8, MaxBaseUrlLen>>,
    pub region: Region,
    pub imported: bool,
}

pub type HostDataOf<T> = HostData<
	<T as Config>::Region,
	<T as Config>::MaxPeerIdLength,
	<T as Config>::MaxBaseUrlLen,
>;

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo, MaxEncodedLen)]
pub struct GuestData<AccountId> {
    pub host_id: AccountId,
    pub imported: bool,
}

pub type GuestDataOf<T> = GuestData<
    <T as frame_system::Config>::AccountId,
>;

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo)]
pub enum LegalOfficerDataParam<AccountId, Region> {
	Host(HostDataParam<Region>),
	Guest(AccountId),
}

pub type LegalOfficerDataParamOf<T> = LegalOfficerDataParam<
	<T as frame_system::Config>::AccountId,
	<T as Config>::Region,
>;

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo)]
pub struct HostDataParam<Region> {
	pub node_id: Option<PeerId>,
	pub base_url: Option<Vec<u8>>,
	pub region: Region,
}

pub type HostDataParamOf<T> = HostDataParam<<T as Config>::Region>;

pub mod weights;

#[frame_support::pallet]
pub mod pallet {
    use frame_system::pallet_prelude::*;
    use frame_support::{
        dispatch::DispatchResultWithPostInfo,
        pallet_prelude::*,
    };
    use super::*;
    pub use crate::weights::WeightInfo;

    #[pallet::config]
    pub trait Config: frame_system::Config {

        /// The origin which can add a Logion Legal Officer.
        type AddOrigin: EnsureOrigin<Self::RuntimeOrigin>;

        /// The origin which can remove a Logion Legal Officer.
        type RemoveOrigin: EnsureOrigin<Self::RuntimeOrigin>;

        /// The origin which can update a Logion Legal Officer's data (in addition to himself).
        type UpdateOrigin: EnsureOrigin<Self::RuntimeOrigin>;

        /// The Legal Officers region
        type Region: frame_support::pallet_prelude::Member + frame_support::pallet_prelude::Parameter + Copy + FromStr + Default + MaxEncodedLen;

        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;

		/// The Maximum size of base_url
		type MaxBaseUrlLen: Get<u32> + Member + TypeInfo;

		/// The Maximum number of nodes
		type MaxNodes: Get<u32>;

		/// The maximum length in bytes of PeerId
		type MaxPeerIdLength: Get<u32> + Member + TypeInfo + Ord;
	}

    #[pallet::pallet]
    pub struct Pallet<T>(_);

    /// All LOs indexed by their account ID.
    #[pallet::storage]
    #[pallet::getter(fn legal_officer_set)]
    pub type LegalOfficerSet<T> = StorageMap<
        _,
        Blake2_128Concat,
        <T as frame_system::Config>::AccountId,
        LegalOfficerDataOf<T>
    >;

    /// The set of LO nodes.
    #[pallet::storage]
    #[pallet::getter(fn legal_officer_nodes)]
    pub type LegalOfficerNodes<T> = StorageValue<_, BoundedBTreeSet<BoundedPeerId<<T as pallet::Config>::MaxPeerIdLength>, <T as pallet::Config>::MaxNodes>, ValueQuery>;

    #[derive(Encode, Decode, Eq, PartialEq, Debug, TypeInfo, MaxEncodedLen)]
    pub enum StorageVersion {
        V1,
        V2AddOnchainSettings,
        V3GuestLegalOfficers,
        V4Region,
        V5Imported,
    }

    impl Default for StorageVersion {
        fn default() -> StorageVersion {
            return StorageVersion::V5Imported;
        }
    }

    /// Storage version
    #[pallet::storage]
    #[pallet::getter(fn pallet_storage_version)]
    pub type PalletStorageVersion<T> = StorageValue<_, StorageVersion, ValueQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub legal_officers: Vec<(T::AccountId, GenesisHostData)>,
    }

    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self { legal_officers: Vec::new() }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> BuildGenesisConfig for GenesisConfig<T>
    where <T::Region as FromStr>::Err: Debug
    {
        fn build(&self) {
            PalletStorageVersion::<T>::put(StorageVersion::default());
            Pallet::<T>::initialize_legal_officers(&self.legal_officers);
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {

        #[cfg(feature = "try-runtime")]
        fn post_upgrade(_state: Vec<u8>) -> Result<(), sp_runtime::DispatchError> {
            assert_eq!(PalletStorageVersion::<T>::get(), StorageVersion::default());
            for legal_officer in LegalOfficerSet::<T>::iter_values() {
                match legal_officer {
                    LegalOfficerData::Host(host_data) => assert!(!host_data.imported),
                    LegalOfficerData::Guest(guest_data) => assert!(!guest_data.imported),
                }
            }
            Ok(())
        }
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Issued when an LO is added to the list. [accountId]
        LoAdded(T::AccountId),
        /// Issued when an LO is removed from the list. [accountId]
        LoRemoved(T::AccountId),
        /// Issued when an LO is updated. [accountId]
        LoUpdated(T::AccountId),
        /// Issued when an LO is imported. [accountId]
        LoImported(T::AccountId),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// The LO is already in the list.
        AlreadyExists,
        /// The LO is not in the list.
        NotFound,
        /// The Peer ID is already assigned to another LO.
        PeerIdAlreadyInUse,
        /// The host has at least one guest and cannot become a guest or be removed
        HostHasGuest,
        /// Trying to add a guest with another guest as host
        GuestOfGuest,
        /// Trying to add a guest with unknown host
        HostNotFound,
        /// Host cannot convert itself into a guest
        HostCannotConvert,
        /// Guest cannot update
        GuestCannotUpdate,
        /// LO cannot change region
        CannotChangeRegion,
		/// There are too much nodes
		TooManyNodes,
		/// The base url is too long
		BaseUrlTooLong,
		/// The PeerId is too long
		PeerIdTooLong,
	}

    #[pallet::call]
    impl<T:Config> Pallet<T> {

        /// Adds a new LO to the list
        #[pallet::call_index(0)]
        #[pallet::weight(T::WeightInfo::add_legal_officer())]
        pub fn add_legal_officer(
            origin: OriginFor<T>,
            legal_officer_id: T::AccountId,
            data: LegalOfficerDataParamOf<T>,
        ) -> DispatchResultWithPostInfo {
            T::AddOrigin::ensure_origin(origin)?;
			let bounded_data = Self::map_to_bounded_legal_officer_data(&data, false)?;
            Self::do_add_legal_officer(
                legal_officer_id,
				bounded_data,
                false,
            )
        }

        /// Removes a LO from the list
        #[pallet::call_index(1)]
        #[pallet::weight(T::WeightInfo::remove_legal_officer())]
        pub fn remove_legal_officer(
            origin: OriginFor<T>,
            legal_officer_id: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            T::RemoveOrigin::ensure_origin(origin)?;
            let to_remove = <LegalOfficerSet<T>>::get(&legal_officer_id);
            if to_remove.is_none() {
                Err(Error::<T>::NotFound)?
            } else if Self::host_has_guest(&legal_officer_id) {
                Err(Error::<T>::HostHasGuest)?
            } else {
                <LegalOfficerSet<T>>::remove(&legal_officer_id);
                Self::try_reset_legal_officer_nodes(&to_remove.unwrap())?;

                Self::deposit_event(Event::LoRemoved(legal_officer_id));
                Ok(().into())
            }
        }

        /// Updates an existing LO's data
        #[pallet::call_index(2)]
        #[pallet::weight(T::WeightInfo::update_legal_officer())]
        pub fn update_legal_officer(
			origin: OriginFor<T>,
			legal_officer_id: T::AccountId,
			data: LegalOfficerDataParamOf<T>,
        ) -> DispatchResultWithPostInfo {
            let who = ensure_signed_or_root(origin.clone())?;
            if who.is_some() && who.clone().unwrap() != legal_officer_id {
                T::UpdateOrigin::ensure_origin(origin)?;
            }
            let to_update = <LegalOfficerSet<T>>::get(&legal_officer_id);
            if to_update.is_none() {
                Err(Error::<T>::NotFound)?
            } else {
                let some_to_update = to_update.unwrap();
                let imported = Self::was_imported(&some_to_update);
                let bounded_data = Self::map_to_bounded_legal_officer_data(&data, imported)?;
                Self::ensure_host_if_guest(&bounded_data)?;
                match some_to_update {
                    LegalOfficerData::Host(_) => match bounded_data {
                        LegalOfficerData::Guest(_) => {
                            if Self::host_has_guest(&legal_officer_id) {
                                Err(Error::<T>::HostHasGuest)?
                            }
                            if who.is_some() && who.unwrap() == legal_officer_id {
                                Err(Error::<T>::HostCannotConvert)?
                            }
                        },
                        _ => (),
                    },
                    LegalOfficerData::Guest(_) => if who.is_some() && who.unwrap() == legal_officer_id {
                        Err(Error::<T>::GuestCannotUpdate)?
                    },
                }

                let source_region = Self::get_region(&some_to_update);
                let dest_region = Self::get_region(&bounded_data);
                if source_region != dest_region {
                    Err(Error::<T>::CannotChangeRegion)?
                }

                <LegalOfficerSet<T>>::set(legal_officer_id.clone(), Some(bounded_data.clone()));
                match some_to_update {
                    LegalOfficerData::Guest(_) => match bounded_data {
                        LegalOfficerData::Host(_) => Self::reset_legal_officer_nodes()?,
                        LegalOfficerData::Guest(_) => (),
                    },
                    LegalOfficerData::Host(_) => {
                        Self::reset_legal_officer_nodes()?
                    },
                }

                Self::deposit_event(Event::LoUpdated(legal_officer_id));
                Ok(().into())
            }
        }

        /// Import a host LO
        #[pallet::call_index(3)]
        #[pallet::weight(T::WeightInfo::import_host_legal_officer())]
        pub fn import_host_legal_officer(
            origin: OriginFor<T>,
            legal_officer_id: T::AccountId,
            data: HostDataParamOf<T>,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            let bounded_data = Self::map_to_bounded_host_legal_officer_data(&data, true)?;
            Self::do_add_legal_officer(
                legal_officer_id,
                bounded_data,
                true,
            )
        }

        /// Import a guest LO
        #[pallet::call_index(4)]
        #[pallet::weight(T::WeightInfo::import_guest_legal_officer())]
        pub fn import_guest_legal_officer(
            origin: OriginFor<T>,
            legal_officer_id: T::AccountId,
            host_id: T::AccountId,
        ) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            let imported = true;
            let bounded_data = LegalOfficerData::Guest(GuestData {
                host_id,
                imported,
            });
            Self::do_add_legal_officer(
                legal_officer_id,
                bounded_data,
                imported,
            )
        }
    }
}

pub type OuterOrigin<T> = <T as frame_system::Config>::RuntimeOrigin;

impl<T: Config> Pallet<T> {
	fn initialize_legal_officers(legal_officers: &Vec<(T::AccountId, GenesisHostData)>)
		where <T::Region as FromStr>::Err: Debug
	{
		for legal_officer in legal_officers {
			let region: T::Region = FromStr::from_str(&legal_officer.1.region).unwrap();
			let base_url = match legal_officer.clone().1.base_url.clone() {
				Some(url) => Some(BoundedVec::try_from(url)
					.expect("Failed to convert from unbounded url")
				),
				None => None,
			};
			let node_id = match legal_officer.clone().1.node_id {
				Some(unbounded) => {
					let bounded = BoundedVec::try_from(unbounded.0)
						.expect("Failed to convert from unbounded node_id");
					Some(BoundedPeerId::new(bounded))
				},
				None => None,
			};
			let llo_data = &LegalOfficerData::Host(
				HostData {
					node_id,
					base_url,
					region,
                    imported: false,
				}
			);
			LegalOfficerSet::<T>::insert::<&T::AccountId, &LegalOfficerDataOf<T>>(&(legal_officer.0), llo_data);
			LegalOfficerNodes::<T>::set(BoundedBTreeSet::new());
		}
	}

    fn try_reset_legal_officer_nodes(added_or_removed_data: &LegalOfficerDataOf<T>) -> Result<(), Error<T>> {
        match added_or_removed_data {
            LegalOfficerData::Host(_) => Self::reset_legal_officer_nodes(),
            _ => Ok(()),
        }
    }

    fn reset_legal_officer_nodes() -> Result<(), Error<T>> {
        let mut new_nodes: BoundedBTreeSet<BoundedPeerId<<T as pallet::Config>::MaxPeerIdLength>, <T as pallet::Config>::MaxNodes> = BoundedBTreeSet::new();
        for data in LegalOfficerSet::<T>::iter_values() {
            match data {
                LegalOfficerData::Host(host_data) => {
					match host_data.node_id {
						Some(node_id) => {
							let inserted = new_nodes.try_insert(node_id)
								.map_err(|_| Error::<T>::TooManyNodes)?;
							if !inserted {
								Err(Error::<T>::PeerIdAlreadyInUse)?
							}
						}
						None => {},
					}
                },
                _ => (),
            }
        }
        LegalOfficerNodes::<T>::set(new_nodes);
        Ok(())
    }

    pub fn ensure_legal_officer(o: T::RuntimeOrigin) -> Result<T::AccountId, BadOrigin> {
        <Self as EnsureOrigin<T::RuntimeOrigin>>::ensure_origin(o)
    }

    fn host_has_guest(host_id: &T::AccountId) -> bool {
        for data in LegalOfficerSet::<T>::iter_values() {
            match data {
                LegalOfficerData::Guest(host) =>
                    if host.host_id == *host_id { return true },
                _ => (),
            }
        }
        false
    }

    fn ensure_host_if_guest(data: &LegalOfficerDataOf<T>) -> Result<(), Error<T>> {
        match &data {
            LegalOfficerData::Guest(data) => Self::ensure_host(&data.host_id),
            _ => Ok(()),
        }
    }

    fn ensure_host(id: &T::AccountId) -> Result<(), Error<T>> {
        let potential_host = LegalOfficerSet::<T>::get(id);
        if potential_host.is_none() {
            Err(Error::<T>::HostNotFound)
        } else {
            match potential_host.unwrap() {
                LegalOfficerData::Guest(_) => Err(Error::<T>::GuestOfGuest),
                LegalOfficerData::Host(_) => Ok(()),
            }
        }
    }

    fn do_add_legal_officer(
        legal_officer_id: T::AccountId,
        data: LegalOfficerDataOf<T>,
        imported: bool,
    ) -> DispatchResultWithPostInfo {
        if <LegalOfficerSet<T>>::contains_key(&legal_officer_id) {
            Err(Error::<T>::AlreadyExists)?
        } else {
            Self::ensure_host_if_guest(&data)?;
            <LegalOfficerSet<T>>::insert(legal_officer_id.clone(), &data);
            Self::try_reset_legal_officer_nodes(&data)?;

            if imported {
                Self::deposit_event(Event::LoImported(legal_officer_id));
            } else {
                Self::deposit_event(Event::LoAdded(legal_officer_id));
            }
            Ok(().into())
        }
    }

    fn get_region(data: &LegalOfficerDataOf<T>) -> T::Region {
        match data {
            LegalOfficerData::Guest(guest_data) => Self::get_region(&LegalOfficerSet::<T>::get(&guest_data.host_id).unwrap()),
            LegalOfficerData::Host(host_data) => host_data.region,
        }
    }

	fn map_to_bounded_legal_officer_data(data: &LegalOfficerDataParamOf<T>, imported: bool) -> Result<LegalOfficerDataOf<T>, Error<T>> {
		let bounded_data = match data {
			LegalOfficerDataParam::Host(host_data) => Self::map_to_bounded_host_legal_officer_data(host_data, imported)?,
			LegalOfficerDataParam::Guest(account_id) => LegalOfficerData::Guest(GuestData {
                host_id: account_id.clone(),
                imported,
            }),
		};
		Ok(bounded_data)
	}

    fn map_to_bounded_host_legal_officer_data(host_data: &HostDataParamOf<T>, imported: bool) -> Result<LegalOfficerDataOf<T>, Error<T>> {
        let base_url = match host_data.base_url.clone() {
            Some(url) => {
                let bounded_url = BoundedVec::try_from(url)
                    .map_err(|_| Error::<T>::BaseUrlTooLong)?;
                Some(bounded_url)
            },
            None => None,
        };
        let node_id = match host_data.node_id.clone() {
            Some(peer_id) => {
                let bounded_node_id = BoundedVec::try_from(peer_id.0)
                    .map_err(|_| Error::<T>::PeerIdTooLong)?;
                Some(BoundedPeerId::new(bounded_node_id))
            }
            None => None,
        };
        Ok(LegalOfficerData::Host(
            HostData {
                node_id,
                base_url,
                region: host_data.region,
                imported,
            }
        ))
    }

    fn was_imported(data: &LegalOfficerDataOf<T>) -> bool {
        match data {
            LegalOfficerData::Host(host_data) => host_data.imported,
            LegalOfficerData::Guest(guest_data) => guest_data.imported,
        }
    }
}

impl<T: Config> EnsureOrigin<T::RuntimeOrigin> for Pallet<T> {
    type Success = T::AccountId;

    fn try_origin(o: T::RuntimeOrigin) -> Result<Self::Success, T::RuntimeOrigin> {
        <Self as IsLegalOfficer<T::AccountId, T::RuntimeOrigin>>::try_origin(o)
    }

    #[cfg(feature = "runtime-benchmarks")]
    fn try_successful_origin() -> Result<<T as frame_system::Config>::RuntimeOrigin, ()> {
        let first_member = match <LegalOfficerSet<T>>::iter().next() {
            Some(pair) => pair.0.clone(),
            None => Err(())?,
        };
        Ok(OuterOrigin::<T>::from(RawOrigin::Signed(first_member.clone())))
    }
}

impl<T: Config> IsLegalOfficer<T::AccountId, T::RuntimeOrigin> for Pallet<T> {

    fn is_legal_officer(account: &T::AccountId) -> bool {
        LegalOfficerSet::<T>::contains_key(account)
    }

    fn legal_officers() -> Vec<T::AccountId> {
        LegalOfficerSet::<T>::iter_keys().collect()
    }
}

impl<T: Config> LegalOfficerCreation<T::AccountId> for Pallet<T> {

    fn add_guest_legal_officer(
        guest_legal_officer_id: T::AccountId,
        host_legal_officer_id: T::AccountId) -> DispatchResultWithPostInfo {

        let imported = false;
        Pallet::<T>::do_add_legal_officer(guest_legal_officer_id, LegalOfficerData::Guest(GuestData {
            host_id: host_legal_officer_id,
            imported,
        }), imported)
    }
}
