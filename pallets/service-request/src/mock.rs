use crate as service_request;
use frame_support::{parameter_types};
use sp_core::{H256, Encode, Decode, RuntimeDebug};
use sp_io::TestExternalities;
use frame_system as system;
use sp_runtime::{
	testing::Header,
	traits::{IdentityLookup, BlakeTwo256},
};
use pallet_balances::AccountData;
use scale_info::TypeInfo;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub type AccountId = u64;

frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
		ServiceRequest: service_request::{Pallet, Call, Storage, Event<T>},
		Labs: labs::{Pallet, Call, Storage, Event<T>},
		Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
		Services: services::{Pallet, Call, Storage, Event<T>},
		Certifications: certifications::{Pallet, Call, Storage, Event<T>},
		UserProfile: user_profile::{Pallet, Call, Storage, Event<T>}
	}
);

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const SS58Prefix: u8 = 42;
	pub BlockWeights: frame_system::limits::BlockWeights =
	frame_system::limits::BlockWeights::simple_max(1024);
}

impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type AccountId = AccountId;
	type Call = Call;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type Header = Header;
	type Event = Event;
	type Origin = Origin;
	type BlockHashCount = BlockHashCount;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type AccountData = AccountData<Balance>;
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
}

type Balance = u64;

parameter_types! {
	pub static ExistentialDeposit: Balance = 0;
}

impl pallet_balances::Config for Test {
	type MaxLocks = ();
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// The ubiquitous event type.
	type Event = Event;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
}

impl service_request::Config for Test {
	type Event = Event;
	type TimeProvider = Timestamp;
	type Currency = Balances;
	type Labs = Labs;
}

impl labs::Config for Test {
	type Event = Event;
	type Currency = Balances;
	type Certifications = Certifications;
	type EthereumAddress = EthereumAddress;
	type Services = Services;
	type UserProfile = UserProfile;
	type WeightInfo = ();
}

impl services::Config for Test {
	type Event = Event;
	type Currency = Balances;
	type ServiceOwner = Labs;
	type WeightInfo = ();
}

impl certifications::Config for Test {
	type Event = Event;
	type CertificationOwner = Labs;
	type WeightInfo = ();
}

#[derive(Clone, Copy, PartialEq, Eq, Encode, Decode, Default, RuntimeDebug, TypeInfo)]
pub struct EthereumAddress(pub [u8; 20]);

impl user_profile::Config for Test {
    type Event = Event;
    type EthereumAddress = EthereumAddress;
	type WeightInfo = ();
}


pub type Moment = u64;
pub const MILLISECS_PER_BLOCK: Moment = 6000;
pub const SLOT_DURATION: Moment = MILLISECS_PER_BLOCK;

parameter_types! {
	pub const MinimumPeriod: Moment = SLOT_DURATION / 2;
}

impl pallet_timestamp::Config for Test {
	/// A timestamp: milliseconds since the unix epoch.
	type Moment = Moment;
	type OnTimestampSet = ();
	type MinimumPeriod = MinimumPeriod;
	type WeightInfo = ();
}

pub struct ExternalityBuilder {
	existential_deposit: u64,
}

impl Default for ExternalityBuilder {
	fn default() -> Self {
		Self { existential_deposit: 1 }
	}
}

impl ExternalityBuilder {
	pub fn existential_deposit(mut self, existential_deposit: u64) -> Self {
		self.existential_deposit = existential_deposit;
		self
	}
	pub fn set_associated_consts(&self) {
		EXISTENTIAL_DEPOSIT.with(|v| *v.borrow_mut() = self.existential_deposit);
	}
	pub fn build(&self) -> TestExternalities {
		self.set_associated_consts();
		let mut storage = system::GenesisConfig::default().build_storage::<Test>().unwrap();
		pallet_balances::GenesisConfig::<Test> {
			balances: {
				vec![]
			},
		}
		.assimilate_storage(&mut storage)
		.unwrap();
		let mut ext = sp_io::TestExternalities::new(storage);
		ext.execute_with(|| System::set_block_number(1));
		ext
	}
}
