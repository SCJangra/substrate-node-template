#![cfg_attr(not(feature = "std"), no_std)]
// `construct_runtime!` does a lot of recursion and requires us to increase the limit to 256.
#![recursion_limit = "256"]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::traits::Hooks;
	use frame_system::pallet_prelude::BlockNumberFor;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn offchain_worker(_block_number: T::BlockNumber) {
			let price = match Self::get_eth_price() {
				Ok(p) => p,
				Err(e) => {
					frame_support::log::error!("{e}");
					return;
				},
			};

			let price = sp_std::str::from_utf8(&price).unwrap();

			frame_support::log::info!("Current price is {price}");
		}
	}
}

use frame_support::sp_runtime::offchain::http;
use sp_std::vec::Vec;
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
