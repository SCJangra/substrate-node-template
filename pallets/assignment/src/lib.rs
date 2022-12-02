#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

pub mod weights;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::{weights::*, ToVtbc};
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_std::prelude::*;

	// A type to hold strings
	type BVec = BoundedVec<u8, ConstU32<100>>;

	// Tokey type
	pub type Vtbc = u64;

	// 1 Vtbc has a fixed price of 5 Dollars
	pub const VTBC_PRICE: u64 = 5;

	#[derive(Encode, Decode, Clone, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
	#[cfg_attr(feature = "std", derive(Debug))]
	pub struct Employee {
		pub id: BVec,
		pub name: BVec,
		pub company_name: BVec,
		pub dob: (u8, u8, u16),
		pub vtbc_balance: Vtbc,
	}

	#[pallet::pallet]
	// #[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub type Employees<T: Config> = StorageMap<_, Blake2_128Concat, BVec, Employee>;

	#[pallet::type_value]
	pub fn DefaultVtbcSupply<T: Config>() -> Vtbc {
		1_000_000
	}

	/// Current Vtbc supply
	#[pallet::storage]
	pub type VtbcSupply<T: Config> = StorageValue<_, Vtbc, ValueQuery, DefaultVtbcSupply<T>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		EmployeeAdded { employee: Employee },
		EmployeeUpdated { old: Employee, new: Employee },
		EmployeeRemoved { id: BVec },
	}

	#[pallet::error]
	pub enum Error<T> {
		// Employee already exists
		AlreadyExists,
		// Employee not found
		NotFound,
		// No more Vtbc available to supply
		InsufficientFunds,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(T::WeightInfo::add_employee(id.len() as u32))]
		pub fn add_employee(
			origin: OriginFor<T>,
			id: Vec<u8>,
			name: Vec<u8>,
			company_name: Vec<u8>,
			day: u8,
			month: u8,
			year: u16,
		) -> DispatchResult {
			frame_system::EnsureRoot::ensure_origin(origin)?;

			let bid: BVec = id.try_into().unwrap();
			let bname: BVec = name.try_into().unwrap();
			let bconpany_name: BVec = company_name.try_into().unwrap();

			ensure!(!Employees::<T>::contains_key(&bid), Error::<T>::AlreadyExists);

			let mut e = Employee {
				id: bid.clone(),
				name: bname,
				company_name: bconpany_name,
				dob: (day, month, year),
				vtbc_balance: 0,
			};

			let price = pallet_ocw_assignment::Pallet::<T>::current_price();
			let price = Self::parse_price(&price);

			Self::supply_vtbc_to(&mut e, price.to_vtbc())?;

			Employees::<T>::insert(bid, e.clone());

			Self::deposit_event(Event::EmployeeAdded { employee: e });

			Ok(())
		}

		#[pallet::weight(T::WeightInfo::update_employee(id.len() as u32))]
		pub fn update_employee(
			origin: OriginFor<T>,
			id: Vec<u8>,
			name: Vec<u8>,
			company_name: Vec<u8>,
			day: u8,
			month: u8,
			year: u16,
		) -> DispatchResult {
			frame_system::EnsureRoot::ensure_origin(origin)?;

			let bid: BVec = id.try_into().unwrap();
			let bname: BVec = name.try_into().unwrap();
			let bconpany_name: BVec = company_name.try_into().unwrap();

			let old: Employee = Employees::<T>::get(&bid).ok_or(Error::<T>::NotFound)?;
			let mut new: Employee = old.clone();

			new.name = bname;
			new.company_name = bconpany_name;
			new.dob = (day, month, year);

			Employees::<T>::insert(new.id.clone(), new.clone());

			Self::deposit_event(Event::EmployeeUpdated { old, new });

			Ok(())
		}

		#[pallet::weight(T::WeightInfo::remove_employee(id.len() as u32))]
		pub fn remove_employee(origin: OriginFor<T>, id: Vec<u8>) -> DispatchResult {
			frame_system::EnsureRoot::ensure_origin(origin)?;

			let bid: BVec = id.try_into().unwrap();

			Employees::<T>::remove(&bid);

			Self::deposit_event(Event::EmployeeRemoved { id: bid });

			Ok(())
		}
	}

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_ocw_assignment::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;
	}
}

impl<T: Config> Pallet<T> {
	fn supply_vtbc_to(employee: &mut Employee, vtbc: Vtbc) -> Result<(), Error<T>> {
		VtbcSupply::<T>::get().checked_sub(vtbc).ok_or(Error::<T>::InsufficientFunds)?;

		employee.vtbc_balance += vtbc;

		Ok(())
	}

	fn parse_price(price: &[u8]) -> u64 {
		// remove curly brackets
		let price = &price[1..price.len() - 1];
		let price = sp_std::str::from_utf8(price).unwrap();
		let (_, price) = price.split_once(':').unwrap();
		let price: f64 = price.parse().unwrap();

		unsafe { price.to_int_unchecked::<u64>() }
	}
}

pub trait ToVtbc {
	fn to_vtbc(self) -> Vtbc;
}

impl ToVtbc for u64 {
	fn to_vtbc(self) -> Vtbc {
		self / VTBC_PRICE
	}
}

impl ToVtbc for f64 {
	fn to_vtbc(self) -> Vtbc {
		let v: u64 = unsafe { self.to_int_unchecked::<u64>() };
		v / VTBC_PRICE
	}
}

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
