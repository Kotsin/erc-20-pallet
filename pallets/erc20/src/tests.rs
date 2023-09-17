use crate::{mock::*, Error, Event};
use frame_support::{assert_noop, assert_ok};

/// Should mint tokens correctly
#[test]
fn mint_ok() {
	ExtBuilder::default().build_and_execute(|| {
		assert_ok!(ERC20Module::mint(RuntimeOrigin::signed(1), 42));
		assert_eq!(ERC20Module::balance_of(&1), 42);
	});
}

/// Should revert if mint is unauthorized
#[test]
fn mint_bad_authority() {
	ExtBuilder::default().build_and_execute(|| {
		assert_noop!(ERC20Module::mint(RuntimeOrigin::signed(2), 42), Error::<Test>::AccessControl);
		assert_eq!(ERC20Module::balance_of(&2), 0);
	});
}

/// Should transfer tokens correctly
#[test]
fn transfer_ok() {
	ExtBuilder::default().build_and_execute(|| {
		assert_ok!(ERC20Module::mint(RuntimeOrigin::signed(1), 42));
		assert_ok!(ERC20Module::transfer(RuntimeOrigin::signed(1), 2, 42));
		assert_eq!(ERC20Module::balance_of(&1), 0);
		assert_eq!(ERC20Module::balance_of(&2), 42);
	});
}

/// Should revert if not enough balance
#[test]
fn transfer_low_balance() {
	ExtBuilder::default().build_and_execute(|| {
		assert_ok!(ERC20Module::mint(RuntimeOrigin::signed(1), 42));
		assert_noop!(
			ERC20Module::transfer(RuntimeOrigin::signed(1), 2, 43),
			Error::<Test>::ERC20InsufficientBalance
		);
		assert_eq!(ERC20Module::balance_of(&1), 42);
		assert_eq!(ERC20Module::balance_of(&2), 0);
	});
}

/// Should approve correctly
#[test]
fn approve_ok() {
	ExtBuilder::default().build_and_execute(|| {
		assert_ok!(ERC20Module::mint(RuntimeOrigin::signed(1), 42));
		assert_ok!(ERC20Module::approve(RuntimeOrigin::signed(1), 2, 42));
		assert_eq!(ERC20Module::allowances(&1, &2), 42);
	});
}

/// Should transferFrom correctly
#[test]
fn transfer_from_ok() {
	ExtBuilder::default().build_and_execute(|| {
		assert_ok!(ERC20Module::mint(RuntimeOrigin::signed(1), 42));
		assert_ok!(ERC20Module::approve(RuntimeOrigin::signed(1), 2, 42));
		assert_ok!(ERC20Module::transfer_from(RuntimeOrigin::signed(2), 1, 3, 42));
		assert_eq!(ERC20Module::balance_of(&1), 0);
		assert_eq!(ERC20Module::balance_of(&2), 0);
		assert_eq!(ERC20Module::balance_of(&3), 42);
	})
}

// Should revert if not enough allowance
#[test]
fn transfer_from_low_allowance() {
	ExtBuilder::default().build_and_execute(|| {
		assert_ok!(ERC20Module::mint(RuntimeOrigin::signed(1), 43));
		assert_ok!(ERC20Module::approve(RuntimeOrigin::signed(1), 2, 42));
		assert_noop!(
			ERC20Module::transfer_from(RuntimeOrigin::signed(2), 1, 3, 43), Error::<Test>::ERC20InsufficientAllowance
		);
	})
}

// Should revert if not enough balance
#[test]
fn transfer_from_low_balance() {
	ExtBuilder::default().build_and_execute(|| {
		assert_ok!(ERC20Module::mint(RuntimeOrigin::signed(1), 42));
		assert_ok!(ERC20Module::approve(RuntimeOrigin::signed(1), 2, 10000));
		assert_noop!(
			ERC20Module::transfer_from(RuntimeOrigin::signed(2), 1, 3, 10000), Error::<Test>::ERC20InsufficientBalance
		);
	})
}

#[test]
fn burn_ok() {
	ExtBuilder::default().build_and_execute(|| {
		assert_ok!(ERC20Module::mint(RuntimeOrigin::signed(1), 42));
		assert_ok!(ERC20Module::burn(RuntimeOrigin::signed(1), 42));
		assert_eq!(ERC20Module::balance_of(&1), 0);
	})
}

#[test]
fn burn_low_balance() {
	ExtBuilder::default().build_and_execute(|| {
		assert_ok!(ERC20Module::mint(RuntimeOrigin::signed(1), 42));
		assert_noop!(ERC20Module::burn(RuntimeOrigin::signed(1), 43), Error::<Test>::ERC20InsufficientBalance);
		assert_eq!(ERC20Module::balance_of(&1), 42);
	})
}