use crate::{Config, Kitties, Kitty, KittyId, KittyName, Pallet};
use frame_support::{
	pallet_prelude::*, storage::StoragePrefixedMap, traits::GetStorageVersion, weights::Weight,
};

use sp_runtime::sp_std::vec::Vec;

use frame_support::{migration::storage_key_iter, Blake2_128Concat};

#[derive(
	Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq, Default, TypeInfo, MaxEncodedLen,
)]
pub struct KittyV0(pub [u8; 16]);

// declare a struct for Kitty v1
#[derive(
	Encode, Decode, Clone, Copy, RuntimeDebug, PartialEq, Eq, Default, TypeInfo, MaxEncodedLen,
)]
pub struct KittyV1 {
	pub dna: [u8; 16],
	pub name: [u8; 4],
}

pub fn migrate_to_v2<T: Config>() -> Weight {
	let on_chain_version = Pallet::<T>::on_chain_storage_version();
	let current_version = Pallet::<T>::current_storage_version();

	if current_version != 2 {
		return Weight::zero()
	} else {
		let module = Kitties::<T>::module_prefix();
		let item = Kitties::<T>::storage_prefix();

		// v0 -> v2
		if on_chain_version == 0 {
			for (index, kitty) in
				storage_key_iter::<KittyId, KittyV0, Blake2_128Concat>(module, item).drain()
			{
				let new_kitty = Kitty {
					dna: kitty.0,
					name: KittyName([b'a', b'a', b'a', b'a', b'a', b'a', b'a', b'a']),
				};
				Kitties::<T>::insert(index, &new_kitty);
			}
			Weight::zero()
		} else if on_chain_version == 1 {
      // v1 -> v2
			for (index, kitty) in
				storage_key_iter::<KittyId, KittyV1, Blake2_128Concat>(module, item).drain()
			{
				let new_kitty = Kitty {
					dna: kitty.dna,
					name: KittyName(
						[0, 0, 0, 0]
							.iter()
							.chain(kitty.name.iter())
							.cloned()
							.collect::<Vec<u8>>()
							.try_into()
							.unwrap(),
					),
				};

				Kitties::<T>::insert(index, &new_kitty);
			}
			Weight::zero()
		} else {
			return Weight::zero()
		}
	}
}
