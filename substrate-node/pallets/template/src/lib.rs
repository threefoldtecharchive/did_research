#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame
use codec::{Decode, Encode};
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch, ensure, traits::Get,
};
use frame_system::ensure_signed;
use sp_core::RuntimeDebug;
use sp_std::{prelude::*, vec::Vec};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, RuntimeDebug)]
pub struct KycObject {
    pub provider: Vec<u8>,
    pub proof: [u8; 32],
}

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
    // A unique name is used to ensure that the pallet's storage items are isolated.
    // This name may be updated, but each pallet in the runtime must use a unique name.
    // ---------------------------------vvvvvvvvvvvvvv
    trait Store for Module<T: Trait> as TemplateModule {
        pub KycAttrs get(fn kyc_attributes): map hasher(blake2_128_concat) T::AccountId => Vec<KycObject>;
    }
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
    pub enum Event<T>
    where
        AccountId = <T as frame_system::Trait>::AccountId,
    {
        /// A new kyc proof has been added to the did belonging to the account, supplied by the
        /// given provider. [who, provider]
        KycProofAdded(AccountId, Vec<u8>),
    }
);

// Errors inform users that something went wrong.
decl_error! {
    pub enum Error for Module<T: Trait> {
        /// A kyc object from this provider already exists for this user.
        ProviderExists,
        /// The provider name exceeds the maximum length.
        ProviderTooLong,
    }
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Errors must be initialized if they are used by the pallet.
        type Error = Error<T>;

        // Events must be initialized if they are used by the pallet.
        fn deposit_event() = default;

        /// Allows a user to add a Kyc proof to his did
        #[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
        pub fn add_kyc_proof(origin, provider: Vec<u8>, proof: [u8;32]) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;



            ensure!(provider.len() <= 32, Error::<T>::ProviderTooLong);

            let existing_kyc_objs = KycAttrs::<T>::get(&who);

            for obj in existing_kyc_objs {
                ensure!(obj.provider != provider, Error::<T>::ProviderExists);
            }

            let new_proof = KycObject {
                provider,
                proof,
            };

            KycAttrs::<T>::mutate(&who, |list| list.push(new_proof.clone()));

            Self::deposit_event(RawEvent::KycProofAdded(who, new_proof.provider));

            Ok(())
        }
    }
}
