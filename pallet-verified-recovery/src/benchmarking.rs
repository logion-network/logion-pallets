//! Benchmarking setup for pallet-verified-recovery
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused_imports)]
use crate::{Pallet as VerifiedRecovery};

use frame_benchmarking::{impl_benchmark_test_suite, v2::*, BenchmarkError};
use frame_system::RawOrigin;
use sp_std::vec::Vec;

pub trait SetupBenchmark<AccountId> {

	fn setup() -> (AccountId, Vec<AccountId>);
}

#[benchmarks]
mod benchmarks {
	use super::*;

	// Benchmark `create_vote_for_all_legal_officers` extrinsic.
	#[benchmark]
	fn create_recovery() -> Result<(), BenchmarkError> {
		let (requester, legal_officers) = T::SetupBenchmark::setup();

		#[extrinsic_call]
		_(
			RawOrigin::Signed(requester),
			legal_officers,
		);

		Ok(())
	}

	impl_benchmark_test_suite! {
		VerifiedRecovery,
		crate::mock::new_test_ext(),
		crate::mock::Test,
	}
}
