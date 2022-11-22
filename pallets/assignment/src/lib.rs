#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_std::prelude::*;

	// A type to hold strings
	type BVec = BoundedVec<u8, ConstU32<100>>;

	#[derive(Encode, Decode, Clone, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
	#[cfg_attr(feature = "std", derive(Debug))]
	pub struct Employee {
		id: BVec,
		name: BVec,
		company_name: BVec,
		dob: (u8, u8, u16),
	}

	#[pallet::pallet]
	// #[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub(super) type Employees<T: Config> = StorageMap<_, Blake2_128Concat, BVec, Employee>;

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
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
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

			let e = Employee {
				id: bid.clone(),
				name: bname,
				company_name: bconpany_name,
				dob: (day, month, year),
			};

			Employees::<T>::insert(bid, e.clone());

			Self::deposit_event(Event::EmployeeAdded { employee: e });

			Ok(())
		}

		#[pallet::weight(0)]
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

		#[pallet::weight(0)]
		pub fn remove_employee(origin: OriginFor<T>, id: Vec<u8>) -> DispatchResult {
			frame_system::EnsureRoot::ensure_origin(origin)?;

			let bid: BVec = id.try_into().unwrap();

			Employees::<T>::remove(&bid);

			Self::deposit_event(Event::EmployeeRemoved { id: bid });

			Ok(())
		}
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}
}

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;
