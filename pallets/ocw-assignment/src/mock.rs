use frame_support::{
	sp_io,
	sp_runtime::{app_crypto::sr25519, testing, traits as rt_traits},
	traits::{ConstU32, ConstU64},
};
use frame_system::mocking;

type UncheckedExtrinsic = mocking::MockUncheckedExtrinsic<Test>;
type Block = mocking::MockBlock<Test>;

frame_support::construct_runtime!(
	pub enum Test
	where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic, {
		System: frame_system,
		OcwAssignment: crate,
	}
);

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = testing::H256;
	type Hashing = rt_traits::BlakeTwo256;
	type AccountId = sr25519::Public;
	type Lookup = rt_traits::IdentityLookup<Self::AccountId>;
	type Header = testing::Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ();
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

// ╭──────────────────────────────────────────────────────────╮
// │ Impl traits related to offchain workers                  │
// ╰──────────────────────────────────────────────────────────╯
pub type Extrinsic = testing::TestXt<RuntimeCall, ()>;
type AccountId =
	<<sr25519::Signature as rt_traits::Verify>::Signer as rt_traits::IdentifyAccount>::AccountId;

impl frame_system::offchain::SigningTypes for Test {
	type Public = <sr25519::Signature as rt_traits::Verify>::Signer;
	type Signature = sr25519::Signature;
}

impl<LocalCall> frame_system::offchain::SendTransactionTypes<LocalCall> for Test
where
	RuntimeCall: From<LocalCall>,
{
	type OverarchingCall = RuntimeCall;
	type Extrinsic = Extrinsic;
}

impl<LocalCall> frame_system::offchain::CreateSignedTransaction<LocalCall> for Test
where
	RuntimeCall: From<LocalCall>,
{
	fn create_transaction<C: frame_system::offchain::AppCrypto<Self::Public, Self::Signature>>(
		call: RuntimeCall,
		_public: <sr25519::Signature as rt_traits::Verify>::Signer,
		_account: AccountId,
		nonce: u64,
	) -> Option<(RuntimeCall, <Extrinsic as rt_traits::Extrinsic>::SignaturePayload)> {
		Some((call, (nonce, ())))
	}
}
// ────────────────────────────────────────────────────────────

// ╭──────────────────────────────────────────────────────────╮
// │ Pallet OcwAssignment                                     │
// ╰──────────────────────────────────────────────────────────╯
impl crate::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type AuthorityId = crate::crypto::AuthorityId;
	type UnsignedInterval = rt_traits::ConstU64<1>;
}
// ────────────────────────────────────────────────────────────

// ╭──────────────────────────────────────────────────────────╮
// │ Exports                                                  │
// ╰──────────────────────────────────────────────────────────╯
pub fn new_test_pub() -> sr25519::Public {
	sr25519::Public::from_raw([1u8; 32])
}
pub fn new_test_ext() -> sp_io::TestExternalities {
	sp_io::TestExternalities::default()
}
// ────────────────────────────────────────────────────────────
