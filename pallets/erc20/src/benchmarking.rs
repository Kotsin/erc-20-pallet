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

	#[benchmark]
	fn transfer_from() {
		let value = 100u64.into();
		let caller: T::AccountId = whitelisted_caller(); //отправляет транзу
		let recipient: T::AccountId = account("Bob", 0, SEED); //получает
		let recipient_lookup = T::Lookup::unlookup(recipient.clone());
		let owner: T::AccountId = account("Alice", 0, SEED);
		let owner_lookup = T::Lookup::unlookup(owner.clone());

		<Balances<T>>::insert(owner.clone(), 1000u64);
		<Allowances<T>>::insert(owner.clone(), caller.clone(), 1000u64);
		#[extrinsic_call]
		transfer_from(RawOrigin::Signed(caller.clone()), owner_lookup, recipient_lookup, value);

		assert_eq!(<Balances<T>>::get(owner.clone()), 900u64);
		assert_eq!(<Balances<T>>::get(recipient.clone()), 100u64);
		assert_eq!(<Allowances<T>>::get(owner.clone(), caller.clone()), 900u64);
	}

	#[benchmark]
	fn approve() {
		let value = 100u64.into();
		let caller: T::AccountId = whitelisted_caller();
		let spender: T::AccountId = account("Bob", 0, SEED);
		let spender_lookup = T::Lookup::unlookup(spender.clone());

		#[extrinsic_call]
		approve(RawOrigin::Signed(caller.clone()), spender_lookup.clone(), value);

		assert_eq!(<Allowances<T>>::get(caller.clone(), spender.clone()), value);
	}

	#[benchmark]
	fn mint() {
		let value = 100u64.into();
		let caller: T::AccountId = whitelisted_caller();

		<Minters<T>>::insert(caller.clone(), ());
		#[extrinsic_call]
		mint(RawOrigin::Signed(caller.clone()), value);

		assert_eq!(<Balances<T>>::get(caller.clone()), value);
	}

	#[benchmark]
	fn burn() {
		let value = 100u64.into();
		let caller: T::AccountId = whitelisted_caller();

		<Balances<T>>::insert(caller.clone(), 1000u64);

		#[extrinsic_call]
		burn(RawOrigin::Signed(caller.clone()), value);

		assert_eq!(<Balances<T>>::get(caller.clone()), 900u64);
	}

	impl_benchmark_test_suite!(ERC20, crate::mock::new_test_ext(), crate::mock::Test);
}
