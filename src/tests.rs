// Tests for the Kitties Pallet.
//
// Normally this file would be split into two parts: `mock.rs` and `tests.rs`.
// The `mock.rs` file would contain all the setup code for our `TestRuntime`.
// Then `tests.rs` would only have the tests for our pallet.
// However, to minimize the project, these have been combined into this single file.
//
// Learn more about creating tests for Pallets:
// https://paritytech.github.io/polkadot-sdk/master/polkadot_sdk_docs/guides/your_first_pallet/index.html

// This flag tells rust to only run this file when running `cargo test`.
#![cfg(test)]

use crate as pallet_kitties;
use crate::*;
use frame::deps::frame_support::runtime;
use frame::deps::sp_io;
use frame::runtime::prelude::*;
use frame::testing_prelude::*;
use frame::traits::fungible::*;

type Balance = u64;
type Block = frame_system::mocking::MockBlock<TestRuntime>;

// In our "test runtime", we represent a user `AccountId` with a `u64`.
// This is just a simplification so that we don't need to generate a bunch of proper cryptographic
// public keys when writing tests. It is just easier to say "user 1 transfers to user 2".
// We create the constants `ALICE` and `BOB` to make it clear when we are representing users below.
const ALICE: u64 = 1;
const BOB: u64 = 2;

#[allow(unused)]
const DEFAULT_KITTY: Kitty<TestRuntime> = Kitty { dna: [0u8; 32], owner: 0, price: None };

#[runtime]
mod runtime {
	#[runtime::derive(
		RuntimeCall,
		RuntimeEvent,
		RuntimeError,
		RuntimeOrigin,
		RuntimeTask,
		RuntimeHoldReason,
		RuntimeFreezeReason
	)]
	#[runtime::runtime]
	/// The "test runtime" that represents the state transition function for our blockchain.
	///
	/// The runtime is composed of individual modules called "pallets", which you find see below.
	/// Each pallet has its own logic and storage, all of which can be combined together.
	pub struct TestRuntime;

	/// System: Mandatory system pallet that should always be included in a FRAME runtime.
	#[runtime::pallet_index(0)]
	pub type System = frame_system::Pallet<TestRuntime>;

	/// PalletBalances: Manages your blockchain's native currency. (i.e. DOT on Polkadot)
	#[runtime::pallet_index(1)]
	pub type PalletBalances = pallet_balances::Pallet<TestRuntime>;

	/// PalletKitties: The pallet you are building in this tutorial!
	#[runtime::pallet_index(2)]
	pub type PalletKitties = pallet_kitties::Pallet<TestRuntime>;
}

// Normally `System` would have many more configurations, but you can see that we use some macro
// magic to automatically configure most of the pallet for a "default test configuration".
#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for TestRuntime {
	type Block = Block;
	type AccountData = pallet_balances::AccountData<Balance>;
}

// Normally `pallet_balances` would have many more configurations, but you can see that we use some
// macro magic to automatically configure most of the pallet for a "default test configuration".
#[derive_impl(pallet_balances::config_preludes::TestDefaultConfig)]
impl pallet_balances::Config for TestRuntime {
	type AccountStore = System;
	type Balance = Balance;
}

// This is the configuration of our Pallet! If you make changes to the pallet's `trait Config`, you
// will also need to update this configuration to represent that.
impl pallet_kitties::Config for TestRuntime {
	type RuntimeEvent = RuntimeEvent;
	type NativeBalance = PalletBalances;
}

// We need to run most of our tests using this function: `new_test_ext().execute_with(|| { ... });`
// It simulates the blockchain database backend for our tests.
// If you forget to include this and try to access your Pallet storage, you will get an error like:
// "`get_version_1` called outside of an Externalities-provided environment."
pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::<TestRuntime>::default()
		.build_storage()
		.unwrap()
		.into()
}
#[test]
fn starting_template_is_sane() {
	new_test_ext().execute_with(|| {
		let event = Event::<TestRuntime>::Created { owner: ALICE };
		let _runtime_event: RuntimeEvent = event.into();
		let _call = Call::<TestRuntime>::create_kitty {};
		let result = PalletKitties::create_kitty(RuntimeOrigin::signed(BOB));
		assert_ok!(result);
	});
}

#[test]
fn system_and_balances_work() {
	// This test will just sanity check that we can access `System` and `PalletBalances`.
	new_test_ext().execute_with(|| {
		// We often need to add some balance to a user to test features which needs tokens.
		assert_ok!(PalletBalances::mint_into(&ALICE, 100));
		assert_ok!(PalletBalances::mint_into(&BOB, 100));
	});
}

#[test]
fn create_kitty_checks_signed() {
	new_test_ext().execute_with(|| {
		// The `create_kitty` extrinsic should work when being called by a user.
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
		// The `create_kitty` extrinsic should fail when being called by an unsigned message.
		assert_noop!(PalletKitties::create_kitty(RuntimeOrigin::none()), DispatchError::BadOrigin);
	})
}

#[test]
fn create_kitty_emits_event() {
	new_test_ext().execute_with(|| {
		// We need to set block number to 1 to view events.
		System::set_block_number(1);
		// Execute our call, and ensure it is successful.
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
		// Assert the last event by our blockchain is the `Created` event with the correct owner.
		System::assert_last_event(Event::<TestRuntime>::Created { owner: 1 }.into());
	})
}

#[test]
fn count_for_kitties_created_correctly() {
	new_test_ext().execute_with(|| {
		// Querying storage before anything is set will return `0`.
		assert_eq!(CountForKitten::<TestRuntime>::get(), 0);
		// You can `set` the value using an `u32`.
		CountForKitten::<TestRuntime>::set(1337u32);
		// You can `put` the value directly with a `u32`.
		CountForKitten::<TestRuntime>::put(1337u32);
	})
}

#[test]
fn mint_increments_count_for_kitty() {
	new_test_ext().execute_with(|| {
		// Querying storage before anything is set will return `0`.
		assert_eq!(CountForKitten::<TestRuntime>::get(), 0);
		// Call `create_kitty` which will call `mint`.
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
		// Now the storage should be `1`
		assert_eq!(CountForKitten::<TestRuntime>::get(), 1);
	})
}

#[test]
fn mint_errors_when_overflow() {
	new_test_ext().execute_with(|| {
		// Set the count to the largest value possible.
		CountForKitten::<TestRuntime>::set(u32::MAX);
		// `create_kitty` should not succeed because of safe math.
		assert_noop!(
			PalletKitties::create_kitty(RuntimeOrigin::signed(1)),
			Error::<TestRuntime>::TooManyKitties
		);
	})
}

#[test]
fn kitties_map_created_correctly() {
	new_test_ext().execute_with(|| {
		let zero_key = [0u8; 32];
		assert!(!Kitties::<TestRuntime>::contains_key(zero_key));
		Kitties::<TestRuntime>::insert(zero_key, DEFAULT_KITTY);
		assert!(Kitties::<TestRuntime>::contains_key(zero_key));
	})
}

#[test]
fn create_kitty_adds_to_map() {
	new_test_ext().execute_with(|| {
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
		assert_eq!(Kitties::<TestRuntime>::iter().count(), 1);
	})
}

#[test]
fn cannot_mint_duplicate_kitty() {
	new_test_ext().execute_with(|| {
		assert_ok!(PalletKitties::mint(ALICE, [0u8; 32]));
		assert_noop!(PalletKitties::mint(BOB, [0u8; 32]), Error::<TestRuntime>::DuplicateKitty);
	})
}

#[test]
fn kitty_struct_has_expected_traits() {
	new_test_ext().execute_with(|| {
		let kitty = DEFAULT_KITTY;
		let bytes = kitty.encode();
		let _decode_kitty = Kitty::<TestRuntime>::decode(&mut &bytes[..]).unwrap();
		assert!(Kitty::<TestRuntime>::max_encoded_len() > 0);
		let _info = Kitty::<TestRuntime>::type_info();
	})
}

#[test]
fn mint_stores_owner_in_kitty() {
	new_test_ext().execute_with(|| {
		assert_ok!(PalletKitties::mint(1337, [42u8; 32]));
	})
}

#[test]
fn create_kitty_makes_unique_kitties() {
	new_test_ext().execute_with(|| {
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(BOB)));

		assert_eq!(CountForKitten::<TestRuntime>::get(), 2);
		assert_eq!(Kitties::<TestRuntime>::iter().count(), 2);
	})
}

#[test]
fn kitties_owned_created_correctly() {
	new_test_ext().execute_with(|| {
		assert_eq!(KittiesOwned::<TestRuntime>::get(1).len(), 0);
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(BOB)));
		assert_eq!(KittiesOwned::<TestRuntime>::get(ALICE).len(), 1);
	})
}

#[test]
fn cannot_own_too_many_kitties() {
	new_test_ext().execute_with(|| {
		for _ in 1..=100 {
			assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
		}

		assert_noop!(
			PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)),
			Error::<TestRuntime>::TooManyOwned
		);
	})
}

#[test]
fn transfer_emit_event() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
		let kitty_id = Kitties::<TestRuntime>::iter_keys().collect::<Vec<[u8; 32]>>()[0];
		assert_ok!(PalletKitties::transfer(RuntimeOrigin::signed(ALICE), BOB, kitty_id));
		System::assert_last_event(
			Event::<TestRuntime>::Transferred { from: ALICE, to: BOB, kitty_id }.into(),
		);
	})
}

#[test]
fn transfer_logic_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
		let kitty = &Kitties::<TestRuntime>::iter_values().collect::<Vec<_>>()[0];
		let kitty_id = kitty.dna;

		assert_eq!(kitty.owner, ALICE);
		assert_eq!(KittiesOwned::<TestRuntime>::get(ALICE), vec![kitty_id]);
		assert_eq!(KittiesOwned::<TestRuntime>::get(BOB), vec![]);

		assert_noop!(
			PalletKitties::transfer(RuntimeOrigin::signed(ALICE), ALICE, kitty_id),
			Error::<TestRuntime>::TransferToSelf
		);

		assert_noop!(
			PalletKitties::transfer(RuntimeOrigin::signed(ALICE), BOB, [0u8; 32]),
			Error::<TestRuntime>::NoKitty
		);

		assert_noop!(
			PalletKitties::transfer(RuntimeOrigin::signed(BOB), ALICE, kitty_id),
			Error::<TestRuntime>::NotOwner
		);

		assert_ok!(PalletKitties::transfer(RuntimeOrigin::signed(ALICE), BOB, kitty_id));
		assert_eq!(KittiesOwned::<TestRuntime>::get(BOB), vec![kitty_id]);
		assert_eq!(KittiesOwned::<TestRuntime>::get(ALICE), vec![]);

		let kitty = &Kitties::<TestRuntime>::iter_values().collect::<Vec<_>>()[0];
		assert_eq!(kitty.owner, BOB);
	})
}

#[test]
fn native_balance_associated_type_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(<<TestRuntime as Config>::NativeBalance as Mutate<_>>::mint_into(&ALICE, 1337));
		assert_eq!(
			<<TestRuntime as Config>::NativeBalance as Inspect<_>>::total_balance(&ALICE),
			1337
		);
	})
}

#[test]
fn balance_of_type_works() {
	let _example_balance: BalanceOf<TestRuntime> = 1337u64;
}

#[test]
fn set_price_emit_events() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
		let kitty_id = Kitties::<TestRuntime>::iter_keys().collect::<Vec<_>>()[0];
		assert_ok!(PalletKitties::set_price(RuntimeOrigin::signed(ALICE), kitty_id, Some(1337)));
		System::assert_last_event(
			Event::<TestRuntime>::PriceSet { owner: ALICE, kitty_id, new_price: Some(1337) }.into(),
		);
	})
}

#[test]
fn set_price_logic_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));

		let kitty = &Kitties::<TestRuntime>::iter_values().collect::<Vec<_>>()[0];
		assert_eq!(kitty.price, None);

		let kitty_id = kitty.dna;

		assert_ok!(PalletKitties::set_price(RuntimeOrigin::signed(ALICE), kitty_id, Some(1337)));

		let kitty = Kitties::<TestRuntime>::get(kitty_id).unwrap();
		assert_eq!(kitty.price, Some(1337));
	})
}

#[test]
fn do_buy_kitty_emits_event() {
	new_test_ext().execute_with(|| {
		// We need to set block number to 1 to view events.
		System::set_block_number(1);
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
		let kitty_id = Kitties::<TestRuntime>::iter_keys().collect::<Vec<_>>()[0];
		assert_ok!(PalletKitties::set_price(RuntimeOrigin::signed(ALICE), kitty_id, Some(1337)));
		assert_ok!(PalletBalances::mint_into(&BOB, 100_000));
		assert_ok!(PalletKitties::buy_kitty(RuntimeOrigin::signed(BOB), kitty_id, 1337));
		// Assert the last event by our blockchain is the `Created` event with the correct owner.
		System::assert_last_event(
			Event::<TestRuntime>::Sold { buyer: BOB, kitty_id, price: 1337 }.into(),
		);
	})
}

#[test]
fn do_buy_kitty_logic_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(PalletKitties::create_kitty(RuntimeOrigin::signed(ALICE)));
		let kitty = &Kitties::<TestRuntime>::iter_values().collect::<Vec<_>>()[0];
		let kitty_id = kitty.dna;
		assert_eq!(kitty.owner, ALICE);
		assert_eq!(KittiesOwned::<TestRuntime>::get(ALICE), vec![kitty_id]);
		// Cannot buy kitty which does not exist.
		assert_noop!(
			PalletKitties::buy_kitty(RuntimeOrigin::signed(BOB), [0u8; 32], 1337),
			Error::<TestRuntime>::NoKitty
		);
		// Cannot buy kitty which is not for sale.
		assert_noop!(
			PalletKitties::buy_kitty(RuntimeOrigin::signed(BOB), kitty_id, 1337),
			Error::<TestRuntime>::NotForSale
		);
		assert_ok!(PalletKitties::set_price(RuntimeOrigin::signed(ALICE), kitty_id, Some(1337)));
		// Cannot buy kitty for a lower price.
		assert_noop!(
			PalletKitties::buy_kitty(RuntimeOrigin::signed(BOB), kitty_id, 1336),
			Error::<TestRuntime>::MaxPriceTooLow
		);
		// Cannot buy kitty if you don't have the funds.
		assert_noop!(
			PalletKitties::buy_kitty(RuntimeOrigin::signed(BOB), kitty_id, 1337),
			frame::arithmetic::ArithmeticError::Underflow
		);
		// Cannot buy kitty if it would kill your account (i.e. set your balance to 0).
		assert_ok!(PalletBalances::mint_into(&BOB, 1337));
		assert!(
			PalletKitties::buy_kitty(RuntimeOrigin::signed(BOB), kitty_id, 1337).is_err(),
			// TODO: assert_noop on DispatchError::Token(TokenError::NotExpendable)
		);
		// When everything is right, it works.
		assert_ok!(PalletBalances::mint_into(&BOB, 100_000));
		assert_ok!(PalletKitties::buy_kitty(RuntimeOrigin::signed(BOB), kitty_id, 1337));
		// State is updated correctly.
		assert_eq!(KittiesOwned::<TestRuntime>::get(BOB), vec![kitty_id]);
		let kitty = Kitties::<TestRuntime>::get(kitty_id).unwrap();
		assert_eq!(kitty.owner, BOB);
		// Price is reset to `None`.
		assert_eq!(kitty.price, None);
		// BOB transferred funds to ALICE.
		assert_eq!(PalletBalances::balance(&ALICE), 1337);
		assert_eq!(PalletBalances::balance(&BOB), 100_000);
	})
}
