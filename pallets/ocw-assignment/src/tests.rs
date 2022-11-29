use frame_support::sp_runtime::offchain::{testing, OffchainWorkerExt, TransactionPoolExt};
use parity_scale_codec::Decode;
use sp_keystore::{testing::KeyStore, KeystoreExt};
use std::sync::Arc;

use crate::mock::*;

const PRICE_RESPONSE: &[u8] = br#"{"ETH":1,"USD":100,"EUR":100}"#;

#[test]
fn should_make_http_call() {
	let (offchain, state) = testing::TestOffchainExt::new();
	let mut t = new_test_ext();
	t.register_extension(OffchainWorkerExt::new(offchain));

	expect_request(&mut state.write());

	t.execute_with(|| {
		let price = OcwAssignment::get_eth_price().unwrap();
		assert_eq!(&price[..], PRICE_RESPONSE);
	});
}

#[test]
fn should_submit_unsigned_transactions() {
	let (offchain, _offchain_state) = testing::TestOffchainExt::new();
	let (pool, pool_state) = testing::TestTransactionPoolExt::new();

	let keystore = KeyStore::new();

	let mut t = new_test_ext();
	t.register_extension(OffchainWorkerExt::new(offchain));
	t.register_extension(TransactionPoolExt::new(pool));
	t.register_extension(KeystoreExt(Arc::new(keystore)));

	t.execute_with(|| {
		OcwAssignment::submit_unsigned(PRICE_RESPONSE, 1).unwrap();

		let tx = pool_state.write().transactions.pop().unwrap();
		assert!(pool_state.read().transactions.is_empty());

		let tx = Extrinsic::decode(&mut &*tx).unwrap();
		assert_eq!(tx.signature, None);
		assert_eq!(
			tx.call,
			RuntimeCall::OcwAssignment(crate::Call::get_price_unsigned {
				block_number: 1,
				price: PRICE_RESPONSE.try_into().unwrap()
			})
		);
	})
}

fn expect_request(state: &mut testing::OffchainState) {
	state.expect_request(testing::PendingRequest {
		method: "GET".into(),
		uri: "https://min-api.cryptocompare.com/data/price?fsym=ETH&tsyms=ETH,USD,EUR".into(),
		response: Some(PRICE_RESPONSE.to_vec()),
		sent: true,
		..Default::default()
	});
}
