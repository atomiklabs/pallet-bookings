#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use sp_runtime::{
		traits::{Saturating},
	};
	use frame_support::{
		dispatch::{DispatchResultWithPostInfo},
		pallet_prelude::*
	};
	use frame_system::pallet_prelude::*;

	#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug)]
	pub enum BookingStatus {
		Created,
		Active,
		Completed,
	}

	#[derive(Encode, Decode, Clone, Default, Eq, PartialEq, RuntimeDebug)]
	pub struct BookingConfig<BlockNumber, BookingStatus> {
		start: BlockNumber,
		end: BlockNumber,
		status: BookingStatus,
	}

	// type BookingConfigOf<T> = BookingConfig<<T as frame_system::Config>::BlockNumber, BookingStatus>;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn booking)]
	pub type Booking<T: Config> = StorageValue<_, BookingConfig<T::BlockNumber, BookingStatus>>;

	#[pallet::event]
	#[pallet::metadata(T::AccountId = "AccountId")]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		CreateBooking(T::BlockNumber, T::BlockNumber, BookingStatus),
		SomethingStored(u32, T::AccountId),
	}

	#[pallet::error]
	pub enum Error<T> {
		NoneValue,
		StorageOverflow,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T:Config> Pallet<T> {

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn create_booking(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			let _caller = ensure_signed(origin)?;

			let time = frame_system::Pallet::<T>::block_number();

			let new_booking = BookingConfig {
				start: time,
				end: time.saturating_add(10_u32.into()),
				status: BookingStatus::Created,
			};

			Booking::<T>::set(Some(new_booking.clone()));

			Self::deposit_event(Event::<T>::CreateBooking(new_booking.start, new_booking.end, new_booking.status));

			Ok(().into())
		}

		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn complete_booking(origin: OriginFor<T>) -> DispatchResultWithPostInfo {
			ensure_signed(origin)?;

			Booking::<T>::mutate(|mut booking| {
				if let Some(config) = &mut booking {
					config.status = BookingStatus::Completed;
				}
			});

			Ok(().into())
		}
	}
}
