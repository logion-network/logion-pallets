//! Autogenerated weights for pallet_logion_loc
//!
//! The [original template](https://github.com/paritytech/substrate/blob/630422d6108cbaaca893ab213dde69f3bdaa1f6b/.maintain/frame-weight-template.hbs)
//! was disclosed under [Apache 2.0 license](http://www.apache.org/licenses/LICENSE-2.0).
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2022-02-08, STEPS: `[20, ]`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 128

// Executed Command:
// ./target/release/logion-node
// benchmark
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet_logion_loc
// --extrinsic
// *
// --steps
// 20
// --repeat
// 10
// --output
// ./pallets/logion_loc/src/weights.rs
// --template
// ./scripts/weights-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_logion_loc.
pub trait WeightInfo {
    fn create_polkadot_identity_loc() -> Weight;
    fn create_logion_identity_loc() -> Weight;
    fn create_polkadot_transaction_loc() -> Weight;
    fn create_logion_transaction_loc() -> Weight;
    fn add_metadata() -> Weight;
    fn add_file() -> Weight;
    fn add_link() -> Weight;
    fn close() -> Weight;
    fn make_void() -> Weight;
    fn make_void_and_replace() -> Weight;
    fn create_collection_loc() -> Weight;
    fn add_collection_item() -> Weight;
    fn nominate_issuer() -> Weight;
    fn dismiss_issuer() -> Weight;
    fn set_issuer_selection() -> Weight;
    fn add_tokens_record() -> Weight;
    fn create_other_identity_loc() -> Weight;
    fn sponsor() -> Weight;
	fn withdraw_sponsorship() -> Weight;
    fn acknowledge_metadata() -> Weight;
    fn acknowledge_file() -> Weight;
    fn acknowledge_link() -> Weight;
}

/// Weights for pallet_logion_loc using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
    fn create_polkadot_identity_loc() -> Weight {
        Weight::from_parts(29_862_000, 0)
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    fn create_logion_identity_loc() -> Weight {
        Weight::from_parts(20_945_000, 0)
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    fn create_polkadot_transaction_loc() -> Weight {
        Weight::from_parts(26_316_000, 0)
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    fn create_logion_transaction_loc() -> Weight {
        Weight::from_parts(30_288_000, 0)
            .saturating_add(T::DbWeight::get().reads(4))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    fn add_metadata() -> Weight {
        Weight::from_parts(11_979_000, 0)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    fn add_file() -> Weight {
        Weight::from_parts(11_971_000, 0)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    fn add_link() -> Weight {
        Weight::from_parts(16_067_000, 0)
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    fn close() -> Weight {
        Weight::from_parts(22_224_000, 0)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    fn make_void() -> Weight {
        Weight::from_parts(22_360_000, 0)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    fn make_void_and_replace() -> Weight {
        Weight::from_parts(32_724_000, 0)
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    fn create_collection_loc() -> Weight {
        Weight::from_parts(29_219_000, 0)
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    fn add_collection_item() -> Weight {
        Weight::from_parts(31_621_000, 0)
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    fn nominate_issuer() -> Weight {
        Weight::from_parts(11_971_000, 0)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    fn dismiss_issuer() -> Weight {
        Weight::from_parts(11_971_000, 0)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    fn set_issuer_selection() -> Weight {
        Weight::from_parts(11_971_000, 0)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    fn add_tokens_record() -> Weight {
        Weight::from_parts(31_621_000, 0)
            .saturating_add(T::DbWeight::get().reads(3))
            .saturating_add(T::DbWeight::get().writes(2))
    }
    fn create_other_identity_loc() -> Weight {
        Weight::from_parts(20_945_000, 0)
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    fn sponsor() -> Weight {
        Weight::from_parts(20_945_000, 0)
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(1))
    }
	fn withdraw_sponsorship() -> Weight {
        Weight::from_parts(20_945_000, 0)
            .saturating_add(T::DbWeight::get().reads(2))
            .saturating_add(T::DbWeight::get().writes(1))
    }
    fn acknowledge_metadata() -> Weight {
        Weight::from_parts(11_979_000, 0)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }

    fn acknowledge_file() -> Weight {
        Weight::from_parts(11_979_000, 0)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }

    fn acknowledge_link() -> Weight {
        Weight::from_parts(11_979_000, 0)
            .saturating_add(T::DbWeight::get().reads(1))
            .saturating_add(T::DbWeight::get().writes(1))
    }
}

// For backwards compatibility and tests
impl WeightInfo for () {
    fn create_polkadot_identity_loc() -> Weight {
        Weight::from_parts(29_862_000, 0)
            .saturating_add(RocksDbWeight::get().reads(3))
            .saturating_add(RocksDbWeight::get().writes(2))
    }
    fn create_logion_identity_loc() -> Weight {
        Weight::from_parts(20_945_000, 0)
            .saturating_add(RocksDbWeight::get().reads(2))
            .saturating_add(RocksDbWeight::get().writes(1))
    }
    fn create_polkadot_transaction_loc() -> Weight {
        Weight::from_parts(26_316_000, 0)
            .saturating_add(RocksDbWeight::get().reads(3))
            .saturating_add(RocksDbWeight::get().writes(2))
    }
    fn create_logion_transaction_loc() -> Weight {
        Weight::from_parts(30_288_000, 0)
            .saturating_add(RocksDbWeight::get().reads(4))
            .saturating_add(RocksDbWeight::get().writes(2))
    }
    fn add_metadata() -> Weight {
        Weight::from_parts(11_979_000, 0)
            .saturating_add(RocksDbWeight::get().reads(1))
            .saturating_add(RocksDbWeight::get().writes(1))
    }
    fn add_file() -> Weight {
        Weight::from_parts(11_971_000, 0)
            .saturating_add(RocksDbWeight::get().reads(1))
            .saturating_add(RocksDbWeight::get().writes(1))
    }
    fn add_link() -> Weight {
        Weight::from_parts(16_067_000, 0)
            .saturating_add(RocksDbWeight::get().reads(2))
            .saturating_add(RocksDbWeight::get().writes(1))
    }
    fn close() -> Weight {
        Weight::from_parts(22_224_000, 0)
            .saturating_add(RocksDbWeight::get().reads(1))
            .saturating_add(RocksDbWeight::get().writes(1))
    }
    fn make_void() -> Weight {
        Weight::from_parts(22_360_000, 0)
            .saturating_add(RocksDbWeight::get().reads(1))
            .saturating_add(RocksDbWeight::get().writes(1))
    }
    fn make_void_and_replace() -> Weight {
        Weight::from_parts(32_724_000, 0)
            .saturating_add(RocksDbWeight::get().reads(2))
            .saturating_add(RocksDbWeight::get().writes(2))
    }
    fn create_collection_loc() -> Weight {
        Weight::from_parts(29_219_000, 0)
            .saturating_add(RocksDbWeight::get().reads(3))
            .saturating_add(RocksDbWeight::get().writes(2))
    }
    fn add_collection_item() -> Weight {
        Weight::from_parts(31_621_000, 0)
            .saturating_add(RocksDbWeight::get().reads(3))
            .saturating_add(RocksDbWeight::get().writes(2))
    }
    fn nominate_issuer() -> Weight {
        Weight::from_parts(11_971_000, 0)
            .saturating_add(RocksDbWeight::get().reads(1))
            .saturating_add(RocksDbWeight::get().writes(1))
    }
    fn dismiss_issuer() -> Weight {
        Weight::from_parts(11_971_000, 0)
            .saturating_add(RocksDbWeight::get().reads(1))
            .saturating_add(RocksDbWeight::get().writes(1))
    }
    fn set_issuer_selection() -> Weight {
        Weight::from_parts(11_971_000, 0)
            .saturating_add(RocksDbWeight::get().reads(1))
            .saturating_add(RocksDbWeight::get().writes(1))
    }
    fn add_tokens_record() -> Weight {
        Weight::from_parts(31_621_000, 0)
            .saturating_add(RocksDbWeight::get().reads(3))
            .saturating_add(RocksDbWeight::get().writes(2))
    }
    fn create_other_identity_loc() -> Weight {
        Weight::from_parts(20_945_000, 0)
            .saturating_add(RocksDbWeight::get().reads(2))
            .saturating_add(RocksDbWeight::get().writes(1))
    }
    fn sponsor() -> Weight {
        Weight::from_parts(20_945_000, 0)
            .saturating_add(RocksDbWeight::get().reads(2))
            .saturating_add(RocksDbWeight::get().writes(1))
    }
	fn withdraw_sponsorship() -> Weight {
        Weight::from_parts(20_945_000, 0)
            .saturating_add(RocksDbWeight::get().reads(2))
            .saturating_add(RocksDbWeight::get().writes(1))
    }
    fn acknowledge_metadata() -> Weight {
        Weight::from_parts(11_979_000, 0)
            .saturating_add(RocksDbWeight::get().reads(1))
            .saturating_add(RocksDbWeight::get().writes(1))
    }
    fn acknowledge_file() -> Weight {
        Weight::from_parts(11_979_000, 0)
            .saturating_add(RocksDbWeight::get().reads(1))
            .saturating_add(RocksDbWeight::get().writes(1))
    }
    fn acknowledge_link() -> Weight {
        Weight::from_parts(11_979_000, 0)
            .saturating_add(RocksDbWeight::get().reads(1))
            .saturating_add(RocksDbWeight::get().writes(1))
    }
}
