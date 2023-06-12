use crate::{mock::*, Error, Event, KittyName};
use frame_support::{assert_noop, assert_ok};

const INITIAL_BANANCE: u128 = 100_000;

use sp_runtime::traits::AccountIdConversion;

fn get_pallet_account() -> u64 {
	KittyPalletId::get().into_account_truncating()
}

#[test]
fn it_test_for_create() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;
		let name = KittyName([0, 0, 0, 0, 0, 0, 0, 1]);

		assert_eq!(kitty_id, KittiesModule::next_kitty_id());
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), name));
		assert_eq!(Balances::free_balance(account_id), INITIAL_BANANCE - EXISTENTIAL_DEPOSIT * 10);
		assert_eq!(Balances::free_balance(get_pallet_account()), EXISTENTIAL_DEPOSIT * 10);

		assert_eq!(kitty_id + 1, KittiesModule::next_kitty_id());
		assert_eq!(KittiesModule::kitties(kitty_id).is_some(), true);
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
		assert_eq!(KittiesModule::kitty_parents(kitty_id), None);

		let kitty = KittiesModule::kitties(kitty_id).unwrap();
		System::assert_last_event(Event::KittyCreated { 
			who: account_id,
			kitty_id,
			kitty,
		}.into());

		crate::NextKittyId::<Test>::set(crate::KittyId::max_value());
		assert_noop!(
			KittiesModule::create(RuntimeOrigin::signed(account_id), name),
			Error::<Test>::InvalidKittyId
		);
	});
}

#[test]
fn it_works_for_breed() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;
		let parent_name = KittyName([0, 0, 0, 0, 0, 0, 0, 0]);
		let parent_name_2 = KittyName([0, 0, 0, 0, 0, 0, 0, 1]);
		let child_name = KittyName([0, 0, 0, 0, 0, 0, 0, 2]);

		assert_noop!(
			KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id, parent_name),
			Error::<Test>::SameKittyId,
		);

		assert_noop!(
			KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id + 1, parent_name),
			Error::<Test>::InvalidKittyId,
		);

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), parent_name));
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), parent_name_2));

		assert_eq!(KittiesModule::next_kitty_id(), kitty_id + 2);

		assert_ok!(KittiesModule::breed(RuntimeOrigin::signed(account_id), kitty_id, kitty_id + 1, child_name));
		let create_and_breed_fees = EXISTENTIAL_DEPOSIT * 10 * 3;
		assert_eq!(Balances::free_balance(account_id), INITIAL_BANANCE - create_and_breed_fees);
		assert_eq!(Balances::free_balance(get_pallet_account()), create_and_breed_fees);

		let breed_kitty_id = 2;
		assert_eq!(KittiesModule::next_kitty_id(), breed_kitty_id + 1);
		assert_eq!(KittiesModule::kitties(breed_kitty_id).is_some(), true);
		assert_eq!(KittiesModule::kitty_owner(breed_kitty_id), Some(account_id));
		assert_eq!(KittiesModule::kitty_parents(breed_kitty_id), Some((kitty_id, kitty_id + 1)));
		
		let kitty = KittiesModule::kitties(breed_kitty_id).unwrap();
		System::assert_last_event(Event::KittyBreed { 
			who: account_id,
			kitty_id: breed_kitty_id,
			kitty,
		}.into());
	});
}

#[test]
fn it_works_for_transfer() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;
		let recipient = 2;
		let name = KittyName([0, 0, 0, 0, 0, 0, 0, 1]);

		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), name));
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));

		assert_noop!(
			KittiesModule::transfer(RuntimeOrigin::signed(recipient), recipient, kitty_id),
			Error::<Test>::NotOwner,
		);

		assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(account_id), recipient, kitty_id));
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(recipient));
		
		System::assert_last_event(Event::KittyTransferred { 
			who: account_id,
			recipient,
			kitty_id,
		}.into());

    assert_ok!(KittiesModule::transfer(RuntimeOrigin::signed(recipient), account_id, kitty_id));
    assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));
	});
}


#[test]
fn it_works_for_sale() {
	// test sale function
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;
		let account_id_2 = 2;
		let name = KittyName([0, 0, 0, 0, 0, 0, 0, 1]);

		// create kitty
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), name));
		assert_eq!(Balances::free_balance(account_id), INITIAL_BANANCE - EXISTENTIAL_DEPOSIT * 10);
		assert_eq!(Balances::free_balance(get_pallet_account()), EXISTENTIAL_DEPOSIT * 10);
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));

		// wrong owner
		assert_noop!(
			KittiesModule::sale(RuntimeOrigin::signed(account_id_2), kitty_id),
			Error::<Test>::NotOwner,
		);

		// sale success
		assert_ok!(KittiesModule::sale(RuntimeOrigin::signed(account_id), kitty_id));
		assert_eq!(KittiesModule::kitty_on_sale(kitty_id), Some(()));
		
		// emit KittyOnSale Event
		System::assert_last_event(Event::KittyOnSale { 
			who: account_id,
			kitty_id,
		}.into());
	});
}

#[test]
fn it_works_for_buy() {
	new_test_ext().execute_with(|| {
		let kitty_id = 0;
		let account_id = 1;
		let buyer = 2;
		let name = KittyName([0, 0, 0, 0, 0, 0, 0, 1]);

		// create kitty
		assert_ok!(KittiesModule::create(RuntimeOrigin::signed(account_id), name));
		assert_eq!(Balances::free_balance(account_id), INITIAL_BANANCE - EXISTENTIAL_DEPOSIT * 10);

		// confirm create success
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(account_id));

		// failed if kitty owner is your self
		assert_noop!(
			KittiesModule::buy(RuntimeOrigin::signed(account_id), kitty_id),
			Error::<Test>::AlreadyOwned,
		);

		// failed if kitty is not on sale
		assert_noop!(
			KittiesModule::buy(RuntimeOrigin::signed(buyer), kitty_id),
			Error::<Test>::NotOnSale,
		);

		// sale kitty		
		assert_ok!(KittiesModule::sale(RuntimeOrigin::signed(account_id), kitty_id));

		// buy kitty success
		assert_ok!(KittiesModule::buy(RuntimeOrigin::signed(buyer), kitty_id));
		assert_eq!(KittiesModule::kitty_owner(kitty_id), Some(buyer));
		assert_eq!(Balances::free_balance(buyer), INITIAL_BANANCE - EXISTENTIAL_DEPOSIT * 10);
		assert_eq!(Balances::free_balance(account_id), INITIAL_BANANCE);

		System::assert_last_event(Event::KittyBought { 
			who: buyer,
			kitty_id,
		}.into());
	});
}
