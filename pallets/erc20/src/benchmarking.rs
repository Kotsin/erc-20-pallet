//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]

use super::*;

#[allow(unused)]
use crate::Pallet as ERC20;
use frame_benchmarking::v2::*;
use frame_system::RawOrigin;

const SEED: u32 = 0;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn transfer() { 
		let value = 100u64.into();
		let caller: T::AccountId = whitelisted_caller();

		let recipient: T::AccountId = account("Bob", 0, SEED);
		let recipient_lookup = T::Lookup::unlookup(recipient.clone());

		<Balances<T>>::insert(caller.clone(), 1000u64);
		#[extrinsic_call]
		transfer(RawOrigin::Signed(caller.clone()), recipient_lookup, value);
		

		assert_eq!(<Balances<T>>::get(caller.clone()), 900u64);
		assert_eq!(<Balances<T>>::get(recipient.clone()), 100u64);
	}

	impl_benchmark_test_suite!(ERC20, crate::mock::new_test_ext(), crate::mock::Test);
}
