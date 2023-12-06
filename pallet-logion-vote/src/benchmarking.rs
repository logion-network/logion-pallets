//! Benchmarking setup for pallet-logion-loc
#![cfg(feature = "runtime-benchmarks")]
use super::*;

use crate::{Pallet as Vote};

use frame_benchmarking::{impl_benchmark_test_suite, v2::*, BenchmarkError};
use frame_support::assert_ok;
use frame_system::RawOrigin;
use sp_std::vec;

pub trait LocSetup<LocId, AccountId> {

	fn setup_vote_loc() -> (LocId, AccountId);
}

#[benchmarks]
mod benchmarks {
	use super::*;

	// Benchmark `create_vote_for_all_legal_officers` extrinsic.
	#[benchmark]
	fn create_vote_for_all_legal_officers() -> Result<(), BenchmarkError> {
		let (loc_id, legal_officer_id) = T::LocSetup::setup_vote_loc();

		#[extrinsic_call]
		_(
			RawOrigin::Signed(legal_officer_id),
			loc_id,
		);

		assert!(Vote::<T>::votes(Vote::<T>::last_vote_id()).is_some());

		Ok(())
	}

	// Benchmark `vote` extrinsic.
	#[benchmark]
	fn vote() -> Result<(), BenchmarkError> {
		let (loc_id, legal_officer_id) = T::LocSetup::setup_vote_loc();
		assert_ok!(Vote::<T>::create_vote_for_all_legal_officers(
			RawOrigin::Signed(legal_officer_id.clone()).into(),
			loc_id,
		));

		#[extrinsic_call]
		_(
			RawOrigin::Signed(legal_officer_id),
			Vote::<T>::last_vote_id(),
			true
		);

		Ok(())
	}

	impl_benchmark_test_suite! {
		Vote,
		crate::mock::new_test_ext(),
		crate::mock::Test,
	}
}
