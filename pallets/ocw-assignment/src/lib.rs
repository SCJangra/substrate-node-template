#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

pub use pallet::*;

use frame_support::log;
use sp_std::vec::Vec;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		pallet_prelude::{
			BoundedVec, DispatchResult, InvalidTransaction, IsType, StorageValue, ValidTransaction,
			ValueQuery,
		},
		traits::{ConstU32, Get, Hooks},
		unsigned::{TransactionSource, TransactionValidity, ValidateUnsigned},
	};
	use frame_system::{
		ensure_none,
		offchain::{SendTransactionTypes, SubmitTransaction},
		pallet_prelude::{BlockNumberFor, OriginFor},
	};

	use super::*;

	type BVec = BoundedVec<u8, ConstU32<100>>;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	/// The block at which the nest unsigned transaction may be submitted
	pub type NextUnsignedAt<T: Config> = StorageValue<_, T::BlockNumber, ValueQuery>;

	#[pallet::config]
	pub trait Config: frame_system::Config + SendTransactionTypes<Call<Self>> {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type UnsignedInterval: Get<Self::BlockNumber>;
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		NewPriceUnsigned { price: BVec },
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(block_number: T::BlockNumber) {
			// Get price
			let price = match Self::get_eth_price() {
				Ok(p) => p,
				Err(e) => {
					log::error!("{e}");
					return;
				},
			};

			// Log price
			let price_str = sp_std::str::from_utf8(&price).unwrap();
			log::info!("Current price is {price_str}");

			// Submit transaction
			if <NextUnsignedAt<T>>::get() > block_number {
				return;
			}

			let call = Call::<T>::set_current_price_unsigned { block_number, price };
			let res = SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into());
			if res.is_err() {
				log::error!("Could not submit unsigned transaction");
			}
		}
	}

	#[pallet::validate_unsigned]
	impl<T: Config> ValidateUnsigned for Pallet<T> {
		type Call = Call<T>;
		fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
			if let Self::Call::set_current_price_unsigned { block_number, .. } = call {
				let next_unsigned_at = <NextUnsignedAt<T>>::get();
				let current_block = <frame_system::Pallet<T>>::block_number();

				if &next_unsigned_at > block_number {
					return InvalidTransaction::Stale.into();
				}

				if block_number > &current_block {
					return InvalidTransaction::Future.into();
				}

				return ValidTransaction::with_tag_prefix("OcwAssignment")
					.priority(u64::max_value())
					.and_provides(next_unsigned_at)
					.longevity(3)
					.propagate(true)
					.build();
			}

			InvalidTransaction::Call.into()
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(0)]
		pub fn set_current_price_unsigned(
			origin: OriginFor<T>,
			_block_number: T::BlockNumber,
			price: Vec<u8>,
		) -> DispatchResult {
			ensure_none(origin)?;

			let price: BVec = price.try_into().unwrap();

			let current_block = <frame_system::Pallet<T>>::block_number();
			<NextUnsignedAt<T>>::put(current_block + T::UnsignedInterval::get());

			Self::deposit_event(Event::NewPriceUnsigned { price });
			Ok(())
		}
	}
}

use frame_support::sp_runtime::offchain::http;
impl<T: Config> Pallet<T> {
	fn get_eth_price() -> Result<Vec<u8>, &'static str> {
		let res = http::Request::get(
			"https://min-api.cryptocompare.com/data/price?fsym=ETH&tsyms=ETH,USD,EUR",
		)
		.send()
		.map_err(|_| "Could not send GET")?
		.wait()
		.map_err(|_| "Could not get response from endpoint")?;

		if res.code != 200 {
			return Err("Got invalid status code");
		}

		Ok(res.body().collect::<Vec<u8>>())
	}
}
