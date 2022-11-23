use crate::*;
use frame_benchmarking::benchmarks;
use frame_system::RawOrigin;
use sp_std::vec;
use sp_std::vec::Vec;

fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

benchmarks! {
	add_employee {
		let i in 1 .. 100;

		let id = vec![i as u8; i as usize];
		let name: Vec<u8> = "Sachin".as_bytes().into();
		let company_name: Vec<u8> = "MindfireSolutions".as_bytes().into();
		let (day, month, year) = (1u8, 1u8, 2022u16);
	}: _(RawOrigin::Root, id.clone(), name.clone(), company_name.clone(), day, month, year)
	verify {
		// Verify event
		let employee = Employee {
			id: id.try_into().unwrap(),
			name: name.try_into().unwrap(),
			company_name: company_name.try_into().unwrap(),
			dob: (day, month, year)
		};
		let event = Event::<T>::EmployeeAdded {
			employee: employee.clone()
		};
		assert_last_event::<T>(event.into());

		// Verify storage
		assert_eq!(Employees::<T>::get(&employee.id), Some(employee));
	}

	update_employee {
		let i in 1 .. 100;

		let old_id: Vec<u8> = vec![i as u8; i as usize];
		let old_name: Vec<u8> = "Sachin".as_bytes().into();
		let old_company_name: Vec<u8> = "MindfireSolutions".as_bytes().into();
		let old_dob = (1u8, 1u8, 2022u16);

		let old_employee = Employee {
			id: old_id.try_into().unwrap(),
			name: old_name.try_into().unwrap(),
			company_name: old_company_name.try_into().unwrap(),
			dob: old_dob,
		};

		Employees::<T>::insert(old_employee.id.clone(), old_employee.clone());

		let mut new_employee = old_employee.clone();
		let new_name: Vec<u8> = "Charakhwal".as_bytes().into();
		new_employee.name = new_name.try_into().unwrap();
	}: _(
		RawOrigin::Root,
		new_employee.id.clone().into(),
		new_employee.name.clone().into(),
		new_employee.company_name.clone().into(),
		new_employee.dob.0,
		new_employee.dob.1,
		new_employee.dob.2
	)
	verify {
		assert_last_event::<T>(Event::<T>::EmployeeUpdated {
			old: old_employee,
			new: new_employee.clone()
		}.into());

		assert_eq!(Employees::<T>::get(&new_employee.id), Some(new_employee));
	}

	remove_employee {
		let i in 1 .. 100;

		let id: Vec<u8> = vec![i as u8; i as usize];
		let name: Vec<u8> = "Sachin".as_bytes().into();
		let company_name: Vec<u8> = "MindfireSolutions".as_bytes().into();
		let dob = (1u8, 1u8, 2022u16);

		let e = Employee {
			id: id.try_into().unwrap(),
			name: name.try_into().unwrap(),
			company_name: company_name.try_into().unwrap(),
			dob,
		};

		Employees::<T>::insert(e.id.clone(), e.clone());
	}: _(RawOrigin::Root, vec![i as u8; i as usize])
	verify {
		assert_last_event::<T>(Event::<T>::EmployeeRemoved { id: e.id.clone() }.into());
		assert_eq!(Employees::<T>::get(&e.id), None);
	}

	impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}
