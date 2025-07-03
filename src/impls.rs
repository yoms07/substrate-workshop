use super::*;
use frame::prelude::*;

impl<T: Config> Pallet<T> {
	pub fn mint(owner: T::AccountId, dna: [u8; 32]) -> DispatchResult {
		ensure!(!Kitties::<T>::contains_key(dna), Error::<T>::DuplicateKitty);
		let current_count = CountForKitten::<T>::get();
		let new_count = current_count.checked_add(1).ok_or(Error::<T>::TooManyKitties)?;
		let kitty = Kitty { dna, owner: owner.clone() };
		Kitties::<T>::insert(dna, kitty);
		CountForKitten::<T>::set(new_count);
		Self::deposit_event(Event::<T>::Created { owner });
		Ok(())
	}
}
