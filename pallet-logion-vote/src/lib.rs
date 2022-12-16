#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::codec::{Decode, Encode};
use frame_support::dispatch::Vec;
use scale_info::TypeInfo;

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo)]
pub struct Vote<LocId, AccountId> {
    loc_id: LocId,
    ballots: Vec<Ballot<AccountId>>,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo)]
pub struct Ballot<AccountId> {
    voter: AccountId,
    status: BallotStatus,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, Debug, TypeInfo)]
pub enum BallotStatus {
    NotVoted,
    VotedYes,
    VotedNo,
}

pub type VoteId = u64;

#[frame_support::pallet]
pub mod pallet {
    use codec::HasCompact;
    use frame_support::{
        dispatch::DispatchResultWithPostInfo,
        pallet_prelude::*,
    };
    use frame_system::pallet_prelude::OriginFor;
    use logion_shared::{IsLegalOfficer, LocValidity};
    use super::*;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// LOC identifier
        type LocId: Member + Parameter + Default + Copy + HasCompact;

        /// The overarching event type.
        type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

        /// Query for checking that a signer is a legal officer
        type IsLegalOfficer: IsLegalOfficer<Self::AccountId, Self::RuntimeOrigin>;

        /// Query for checking the existence of a closed Identity LOC
        type LocValidity: LocValidity<Self::LocId, Self::AccountId>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    #[pallet::without_storage_info]
    pub struct Pallet<T>(_);

    ///
    #[pallet::storage]
    #[pallet::getter(fn last_vote_id)]
    pub type LastVoteId<T> = StorageValue<_, VoteId, ValueQuery>;

    /// Votes
    #[pallet::storage]
    #[pallet::getter(fn votes)]
    pub type Votes<T> = StorageMap<_, Blake2_128Concat, VoteId, Vote<<T as Config>::LocId, <T as frame_system::Config>::AccountId>>;

    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Issued upon new Vote creation. [voteId, legalOfficers]
        VoteCreated(VoteId, Vec<T::AccountId>)
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Given LOC is not valid (not found, or not closed or void) or does not belong to vote requester.
        InvalidLoc,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {

        /// Creates a new Vote.
        #[pallet::weight(0)]
        pub fn create_vote_for_all_legal_officers(
            origin: OriginFor<T>,
            #[pallet::compact] loc_id: T::LocId,
        ) -> DispatchResultWithPostInfo {
            let who = T::IsLegalOfficer::ensure_origin(origin.clone())?;
            let legal_officers = T::IsLegalOfficer::legal_officers();
            if T::LocValidity::loc_valid_with_owner(&loc_id, &who) {
                let ballots: Vec<Ballot<<T as frame_system::Config>::AccountId>> = legal_officers
                    .iter()
                    .map(|legal_officer| Ballot { voter: legal_officer.clone(), status: BallotStatus::NotVoted })
                    .collect();
                let vote_id = <LastVoteId<T>>::get() + 1;
                <Votes<T>>::insert(vote_id, Vote {
                    loc_id,
                    ballots,
                });
                <LastVoteId<T>>::set(vote_id);
                Self::deposit_event(Event::VoteCreated(vote_id, legal_officers));
                Ok(().into())
            } else {
                Err(Error::<T>::InvalidLoc)?
            }
        }
    }
}
