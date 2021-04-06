//! This pallet is a working space for the eutxo-on-substrate design.
//! Currently no code has been written. This is just a simple storage pallet

#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet;

pub use pallet::*;

#[pallet]
pub mod pallet {

	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	/// The Ethereum Chain Id Pallet
	#[pallet::pallet]
	pub struct Pallet<T>(PhantomData<T>);

	/// Configuration trait of this pallet.
	#[pallet::config]
	pub trait Config: frame_system::Config {}

	impl<T: Config> Get<u64> for Pallet<T> {
		fn get() -> u64 {
			Self::chain_id()
		}
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	#[pallet::storage]
	#[pallet::getter(fn chain_id)]
	pub type ChainId<T> = StorageValue<_, u64, ValueQuery>;

	#[pallet::genesis_config]
	#[derive(Default)]
	pub struct GenesisConfig {
		pub chain_id: u64,
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig {
		fn build(&self) {
			ChainId::<T>::put(self.chain_id);
		}
	}
}
