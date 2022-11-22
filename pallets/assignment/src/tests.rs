use crate::mock::*;
use crate::pallet;
use frame_support::*;

#[test]
fn employee() {
	new_test_ext().execute_with(|| {
		assert_ok!(Assignment::add_employee(
			RuntimeOrigin::root(),
			"1".as_bytes().into(),
			"Sachin".as_bytes().into(),
			"MindfireSolutions".as_bytes().into(),
			1,
			1,
			2022
		));

		assert_err!(
			Assignment::add_employee(
				RuntimeOrigin::root(),
				// An employee with this id already exists, so this should return an error
				"1".as_bytes().into(),
				"Sachin".as_bytes().into(),
				"MindfireSolutions".as_bytes().into(),
				1,
				1,
				2022
			),
			pallet::Error::<Test>::AlreadyExists
		);

		assert_ok!(Assignment::update_employee(
			RuntimeOrigin::root(),
			"1".as_bytes().into(),
			"Charakhwal".as_bytes().into(),
			"MindfireSolutions".as_bytes().into(),
			1,
			1,
			2022
		));

		assert_err!(
			Assignment::update_employee(
				RuntimeOrigin::root(),
				// There is no employee with id '2', so cannot update
				"2".as_bytes().into(),
				"Sachin Charakhwal".as_bytes().into(),
				"MindfireSolutions".as_bytes().into(),
				1,
				1,
				2022
			),
			pallet::Error::<Test>::NotFound
		);

		assert_ok!(Assignment::remove_employee(RuntimeOrigin::root(), "1".as_bytes().into()));
	});
}
