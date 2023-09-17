#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{pallet_prelude::DispatchResult, sp_runtime::traits::StaticLookup};
use frame_support::dispatch::Vec;
use frame_support::sp_runtime; // IMPORTANT! this import is overkill, can't fix atm

/// ERC20 Pallet
pub use pallet::*; //It imports all the public items from the pallet module and makes them available in the current scope.

#[cfg(test)] //complile only when running tests
mod mock;

#[cfg(test)] //complile only when running tests
mod tests;

#[cfg(feature = "runtime-benchmarks")] //only when benchmarking?
mod benchmarking; // IMPORTANT! learn what benchmarking and weights are
pub mod weights;
pub use weights::*;

// Generic type T must implement (), frame_system::Config trait (collection of methods, basically an interface)
// This trait has an associated type Lookup
// This type must implement StaticLookup
// StaticLookup has a type Source
// This is what AccountIdLookupOf is
// lookup returns either the target or lookup error
type AccountIdLookupOf<T> = <<T as frame_system::Config>::Lookup as StaticLookup>::Source; 

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		/// Type representing the weight of this pallet
		type WeightInfo: WeightInfo;

		#[pallet::constant]
		type Decimals: Get<u64>;
	}

	// STORAGE

	//The pub(super) modifier means that the type is accessible within the current module and its super modules, but not outside of them.
	/// minters
	#[pallet::storage]
	#[pallet::getter(fn minters)]
	pub(super) type Minters<T: Config> = StorageMap< //T must implement Config trait
		_, // this is prefix, what does it mean?
		Blake2_128Concat,
		T::AccountId,
		(),
		ValueQuery, // return default value
	>;

	/// total supply
	#[pallet::storage]
	#[pallet::getter(fn total_supply)]
	pub(super) type TotalSupply<T> = StorageValue<_, u64>;

	/// balance
	#[pallet::storage]
	#[pallet::getter(fn balance_of)]
	pub(super) type Balances<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId, 
		u64,
		ValueQuery, 
	>;

	/// allowances
	#[pallet::storage]
	#[pallet::getter(fn allowances)]
	pub(super) type Allowances<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		T::AccountId,
		u64,
		ValueQuery,
	>;

	#[pallet::genesis_config]
	#[derive(frame_support::DefaultNoBound)] // make empty minters by default
	pub struct GenesisConfig<T: Config> {
		pub minters: Vec<T::AccountId>,
	}

	// The build of genesis for the pallet.
	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			for admin in &self.minters {
				<Minters<T>>::insert(admin, ());
			}
		}
	}

	// EVENTS
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		Transfer { from: T::AccountId, to: T::AccountId, value: u64 },
		Approval { owner: T::AccountId, spender: T::AccountId, value: u64 },
	}

	// ERRORS.
	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
		AccessControl,
		ERC20InsufficientBalance,
		ERC20InsufficientAllowance,
	}

	// FUNCTIONS
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::transfer())]
		pub fn transfer(
			origin: OriginFor<T>,
			to: AccountIdLookupOf<T>,
			value: u64,
		) -> DispatchResult {
			let sender = ensure_signed(origin)?; //msg.sender, checks that the tx is signed
			let to = T::Lookup::lookup(to)?; // IMPORTANT! what does lookup do?
			Self::_transfer(sender, to, value)?;
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::transfer())]
		pub fn transfer_from(
			origin: OriginFor<T>,
			from: AccountIdLookupOf<T>,
			to: AccountIdLookupOf<T>,
			value: u64,
		) -> DispatchResult {
			let spender = ensure_signed(origin)?;
			let from = T::Lookup::lookup(from)?;
			let to = T::Lookup::lookup(to)?;
			Self::_spend_allowance(from.clone(), spender, value)?; // IMPORTANT! is cloning accoundid ok? Sounds expensive
			Self::_transfer(from, to, value)?;
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::transfer())]
		pub fn approve(
			origin: OriginFor<T>,
			spender: AccountIdLookupOf<T>,
			value: u64,
		) -> DispatchResult {
			let owner = ensure_signed(origin)?; 
			let spender = T::Lookup::lookup(spender)?;
			Self::_approve(owner, spender, value)?;
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::transfer())]
		pub fn mint(origin: OriginFor<T>, value: u64) -> DispatchResult {
			let _who = ensure_signed(origin)?;
			if !Minters::<T>::contains_key(_who.clone()) {
				return Err(Error::<T>::AccessControl.into())
			}
			Self::_mint(_who, value)?;
			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::transfer())]
		pub fn burn(origin: OriginFor<T>, value: u64) -> DispatchResult {
			let _who = ensure_signed(origin)?;
			Self::_burn(_who, value)?;
			Ok(())
		}
	}
}

//internal functions
impl<T: Config> Pallet<T> {
	pub fn _transfer(from: T::AccountId, to: T::AccountId, value: u64) -> DispatchResult {
		//another return type?
		//also rename internal functions somehow

		let new_balance_from = Balances::<T>::get(from.clone()).checked_sub(value).ok_or(Error::<T>::ERC20InsufficientBalance)?;
		let new_balance_to = Balances::<T>::get(to.clone()).checked_add(value).ok_or(Error::<T>::StorageOverflow)?;

		Balances::<T>::insert(from.clone(), new_balance_from); 
		Balances::<T>::insert(to.clone(), new_balance_to);
		Self::deposit_event(Event::<T>::Transfer { from, to, value });
		Ok(())
	}

	pub fn _spend_allowance(from: T::AccountId, to: T::AccountId, value: u64) -> DispatchResult {
		let current_allowance = Allowances::<T>::get(from.clone(), to.clone());

		if current_allowance != u64::MAX {
			let result = current_allowance.checked_sub(value).ok_or(Error::<T>::ERC20InsufficientAllowance)?;
			Self::_approve(from, to, result)?; 
		}
		Ok(())
	}

	pub fn _approve(owner: T::AccountId, spender: T::AccountId, value: u64) -> DispatchResult {
		Allowances::<T>::insert(owner.clone(), spender.clone(), value);
		Self::deposit_event(Event::<T>::Approval { owner, spender, value });
		Ok(())
	}

	pub fn _mint(to: T::AccountId, value: u64) -> DispatchResult {
		TotalSupply::<T>::put(value.clone());
		let new_balance = Balances::<T>::get(to.clone()).checked_add(value).ok_or(Error::<T>::StorageOverflow)?;
		Balances::<T>::insert(to, new_balance);
		Ok(())
	}

	pub fn _burn(to: T::AccountId, value: u64) -> DispatchResult {
		TotalSupply::<T>::put(value.clone());
		let new_balance = Balances::<T>::get(to.clone()).checked_sub(value).ok_or(Error::<T>::ERC20InsufficientBalance)?;
		Balances::<T>::insert(to, new_balance);
		Ok(())
	}
}
