//! Benchmarking setup for pallet-lo-authority-list
#![cfg(feature = "runtime-benchmarks")]
use super::*;

use crate::{Pallet as LoAuthorityList};

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
    // * There are already (MaxNodes - 1) host legal officers.
    #[benchmark]
    fn add_legal_officer() -> Result<(), BenchmarkError> {
		let initial_lo_count = LoAuthorityList::<T>::legal_officers().len() as u32;
        add_legal_officers::<T>(max_lo_count::<T>() - initial_lo_count - 1);

        #[extrinsic_call]
        _(
            RawOrigin::Root,
            account_num::<T>(max_lo_count::<T>() - 1),
            host_data_num::<T>(max_lo_count::<T>() - 1),
        );

        assert_eq!(
            LoAuthorityList::<T>::legal_officers().len(),
            max_lo_count::<T>() as usize,
        );

        Ok(())
    }

    // Benchmark `remove_legal_officer` extrinsic with the worst possible conditions:
    // * Remove host legal officer (causes re-computation of nodes set).
    // * There are already MaxNodes host legal officers.
    #[benchmark]
    fn remove_legal_officer() -> Result<(), BenchmarkError> {
		let initial_lo_count = LoAuthorityList::<T>::legal_officers().len() as u32;
        add_legal_officers::<T>(max_lo_count::<T>() - initial_lo_count);

        #[extrinsic_call]
        _(
            RawOrigin::Root,
            account_num::<T>(0),
        );

        assert_eq!(
            LoAuthorityList::<T>::legal_officers().len(),
            (max_lo_count::<T>() - 1) as usize,
        );

        Ok(())
    }

	// Benchmark `update_legal_officer` extrinsic with the worst possible conditions:
    // * Update host legal officer (causes re-computation of nodes set).
    // * There are already MaxNodes host legal officers.
    #[benchmark]
    fn update_legal_officer() -> Result<(), BenchmarkError> {
		let initial_lo_count = LoAuthorityList::<T>::legal_officers().len() as u32;
        add_legal_officers::<T>(max_lo_count::<T>() - initial_lo_count);

        #[extrinsic_call]
        _(
            RawOrigin::Root,
            account_num::<T>(0),
			LegalOfficerDataParam::Host(HostDataParam {
				node_id: Some(OpaquePeerId(vec![0u8])),
				base_url: Some(base_url_num::<T>(0)), // Change base URL
				region: T::Region::from_str("Europe").ok().unwrap(),
			}),
        );

        assert_eq!(
            LoAuthorityList::<T>::legal_officers().len(),
            max_lo_count::<T>() as usize,
        );

        Ok(())
    }

    // Benchmark `import_host_legal_officer` extrinsic with the worst possible conditions:
    // * There are already (MaxNodes - 1) host legal officers.
    #[benchmark]
    fn import_host_legal_officer() -> Result<(), BenchmarkError> {
        let initial_lo_count = LoAuthorityList::<T>::legal_officers().len() as u32;
        add_legal_officers::<T>(max_lo_count::<T>() - initial_lo_count - 1);

        #[extrinsic_call]
        _(
            RawOrigin::Root,
            account_num::<T>(max_lo_count::<T>() - 1),
            host_data_param_num::<T>(max_lo_count::<T>() - 1),
        );

        assert_eq!(
            LoAuthorityList::<T>::legal_officers().len(),
            max_lo_count::<T>() as usize,
        );

        Ok(())
    }

    // Benchmark `import_guest_legal_officer` extrinsic with the worst possible conditions:
    // * There are already (MaxNodes - 1) host legal officers.
    #[benchmark]
    fn import_guest_legal_officer() -> Result<(), BenchmarkError> {
        let initial_lo_count = LoAuthorityList::<T>::legal_officers().len() as u32;
        add_legal_officers::<T>(max_lo_count::<T>() - initial_lo_count - 1);

        #[extrinsic_call]
        _(
            RawOrigin::Root,
            account_num::<T>(max_lo_count::<T>() - 1),
            account_num::<T>(0),
        );

        assert_eq!(
            LoAuthorityList::<T>::legal_officers().len(),
            max_lo_count::<T>() as usize,
        );

        Ok(())
    }

    impl_benchmark_test_suite! {
        LoAuthorityList,
        crate::mock::new_test_ext(),
        crate::mock::Test,
    }
}

fn add_legal_officers<T: Config>(number: u32) {
	for i in 0..number {
		assert_ok!(LoAuthorityList::<T>::add_legal_officer(
			T::RuntimeOrigin::root(),
			account_num::<T>(i),
			host_data_num::<T>(i)
		));
	}
}

fn max_lo_count<T: Config>() -> u32 {
    T::MaxNodes::get()
}

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

fn host_data_num<T: Config>(i: u32) -> LegalOfficerDataParamOf<T> {
    LegalOfficerDataParam::Host(host_data_param_num::<T>(i))
}

fn host_data_param_num<T: Config>(i: u32) -> HostDataParamOf<T> {
    HostDataParam {
        node_id: Some(OpaquePeerId(vec![i as u8])),
        base_url: Some(base_url_num::<T>(i)),
        region: T::Region::from_str("Europe").ok().unwrap(),
    }
}
