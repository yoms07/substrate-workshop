use super::*;
use frame::prelude::*;

impl<T: Config> Pallet<T> {
	pub fn mint(owner: T::AccountId) -> DispatchResult {
		let current_count = CountForKitten::<T>::get().unwrap_or(0);
		let new_count = current_count + 1;

		CountForKitten::<T>::set(Some(new_count));
		Self::deposit_event(Event::<T>::Created { owner });
		Ok(())
	}
}
