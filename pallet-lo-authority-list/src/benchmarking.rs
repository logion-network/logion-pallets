//! Benchmarking setup for pallet-lo-authority-list
#![cfg(feature = "runtime-benchmarks")]
use super::*;

use crate::{HostData, LegalOfficerData, LegalOfficerDataOf, Pallet as LoAuthorityList};

extern crate alloc;
use alloc::string::ToString;

use frame_benchmarking::{account, impl_benchmark_test_suite, v2::*, BenchmarkError};
use frame_support::{assert_ok, traits::OriginTrait};
use frame_system::RawOrigin;

use sp_core::OpaquePeerId;
use sp_std::vec;

#[benchmarks]
mod benchmarks {
    use super::*;

    // Benchmark `add_legal_officer` extrinsic with the worst possible conditions:
    // * Add host legal officer (causes re-computation of nodes set).
    // * There are already "many" legal officers.
	//
	// TODO: make this call at worst O(N) in number of LOs
    #[benchmark]
    fn add_legal_officer() -> Result<(), BenchmarkError> {
		let initial_lo_count = LoAuthorityList::<T>::legal_officers().len();
        add_many_legal_officers::<T>();

        #[extrinsic_call]
        _(
            RawOrigin::Root,
            account_num::<T>(MANY_LO_COUNT),
            host_data_num::<T>(MANY_LO_COUNT),
        );

        assert_eq!(
            LoAuthorityList::<T>::legal_officers().len(),
            initial_lo_count + (MANY_LO_COUNT as usize) + 1
        );

        Ok(())
    }

    // Benchmark `remove_legal_officer` extrinsic with the worst possible conditions:
    // * Remove host legal officer (causes re-computation of nodes set).
    // * There are already "many" legal officers.
	//
	// TODO: make this call at worst O(N) in number of LOs
    #[benchmark]
    fn remove_legal_officer() -> Result<(), BenchmarkError> {
		let initial_lo_count = LoAuthorityList::<T>::legal_officers().len();
        add_many_legal_officers::<T>();

        #[extrinsic_call]
        _(
            RawOrigin::Root,
            account_num::<T>(MANY_LO_COUNT - 1),
        );

        assert_eq!(
            LoAuthorityList::<T>::legal_officers().len(),
            initial_lo_count + (MANY_LO_COUNT as usize) - 1
        );

        Ok(())
    }

	// Benchmark `update_legal_officer` extrinsic with the worst possible conditions:
    // * Update host legal officer (causes re-computation of nodes set).
    // * There are already "many" legal officers.
	//
	// TODO: make this call at worst O(N) in number of LOs
    #[benchmark]
    fn update_legal_officer() -> Result<(), BenchmarkError> {
		let initial_lo_count = LoAuthorityList::<T>::legal_officers().len();
        add_many_legal_officers::<T>();

        #[extrinsic_call]
        _(
            RawOrigin::Root,
            account_num::<T>(MANY_LO_COUNT - 1),
			LegalOfficerData::Host(HostData {
				node_id: Some(OpaquePeerId(vec![(MANY_LO_COUNT - 1) as u8])),
				base_url: Some(base_url_num::<T>(MANY_LO_COUNT)), // Change base URL
				region: T::Region::from_str("Europe").ok().unwrap(),
			}),
        );

        assert_eq!(
            LoAuthorityList::<T>::legal_officers().len(),
            initial_lo_count + (MANY_LO_COUNT as usize)
        );

        Ok(())
    }

    impl_benchmark_test_suite! {
        LoAuthorityList,
        crate::mock::new_test_ext(),
        crate::mock::Test,
    }
}

fn add_many_legal_officers<T: Config>() {
	for i in 0..MANY_LO_COUNT {
		assert_ok!(LoAuthorityList::<T>::add_legal_officer(
			T::RuntimeOrigin::root(),
			account_num::<T>(i),
			host_data_num::<T>(i)
		));
	}
}

const MANY_LO_COUNT: u32 = 50;

fn account_num<T: Config>(i: u32) -> T::AccountId {
    account("lo", i, SEED)
}

const SEED: u32 = 0;

fn base_url_num<T: Config>(i: u32) -> Vec<u8> {
	let prefix = "https://node".as_bytes().to_vec();
	let number = i.to_string().as_bytes().to_vec();
	let suffix = ".logion.network".as_bytes().to_vec();
	[prefix, number, suffix].concat()

}

fn host_data_num<T: Config>(i: u32) -> LegalOfficerDataOf<T> {
    LegalOfficerData::Host(HostData {
        node_id: Some(OpaquePeerId(vec![i as u8])),
        base_url: Some(base_url_num::<T>(i)),
        region: T::Region::from_str("Europe").ok().unwrap(),
    })
}
