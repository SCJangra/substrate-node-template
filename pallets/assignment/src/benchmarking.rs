use crate::*;
use frame_benchmarking::benchmarks;
use frame_system::RawOrigin;
use sp_std::vec;
use sp_std::vec::Vec;

benchmarks! {
	add_employee {
		let i in 1 .. 100;
	}: _(
		RawOrigin::Root,
		vec![i as u8; i as usize],
		"Sachin".as_bytes().into(),
		"MindfireSolutions".as_bytes().into(),
		1,
		1,
		2022
	)

	update_employee {
		let i in 1 .. 100 => {
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

			Employees::<T>::insert(e.id.clone(), e);
		};
	}: _(
		RawOrigin::Root,
		vec![i as u8; i as usize],
		"Charakhwal".as_bytes().into(),
		"MindfireSolutions".as_bytes().into(),
		1,
		1,
		2022
	)

	remove_employee {
		let i in 1 .. 100 => {
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

			Employees::<T>::insert(e.id.clone(), e);
		};
	}: _(RawOrigin::Root, vec![i as u8; i as usize])

	impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}
