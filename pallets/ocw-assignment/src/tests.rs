use frame_support::sp_runtime::offchain::{testing, OffchainWorkerExt};

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

fn expect_request(state: &mut testing::OffchainState) {
	state.expect_request(testing::PendingRequest {
		method: "GET".into(),
		uri: "https://min-api.cryptocompare.com/data/price?fsym=ETH&tsyms=ETH,USD,EUR".into(),
		response: Some(PRICE_RESPONSE.to_vec()),
		sent: true,
		..Default::default()
	});
}
