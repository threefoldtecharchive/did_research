#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame
use codec::{Decode, Encode};
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure, traits::Get,
};
use frame_system::ensure_signed;
use sp_core::RuntimeDebug;
use sp_std::{prelude::*, vec::Vec};

#[cfg(test)]
mod mock;

/// Attributes or properties that make an identity.
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Encode, Decode, Default, RuntimeDebug)]
pub struct Attribute {
    pub name: Vec<u8>,
    pub value: Vec<u8>,
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
        pub Attributes get(fn attributes_of): map hasher(blake2_128_concat) T::AccountId => Vec<Attribute>;
        /// Identity owner.
        pub OwnerOf get(fn owner_of): map hasher(blake2_128_concat) T::AccountId => Option<T::AccountId>;
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
        AttributeAdded(AccountId,Vec<u8>),
    }
);

// Errors inform users that something went wrong.
decl_error! {
    pub enum Error for Module<T: Trait> {
        AttributeExists,
        AttributeCreationFailed,
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        // Errors must be initialized if they are used by the pallet.
        type Error = Error<T>;

        // Events must be initialized if they are used by the pallet.
        fn deposit_event() = default;

        /// Allows a user to add an attribute to it's account
        #[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
        pub fn add_attribute(
            origin,
            identity: T::AccountId,
            name: Vec<u8>, 
            value: Vec<u8>, 
        ) -> DispatchResult {
            ensure_signed(origin)?;
            ensure!(name.len() <= 64, Error::<T>::AttributeCreationFailed);

            Self::create_attribute(&identity, &name, &value)?;
            Self::deposit_event(RawEvent::AttributeAdded(identity, name));
            Ok(())
        }
    }
}

impl<T: Trait> Module<T> {
    pub fn create_attribute(
        identity: &T::AccountId,
        name: &[u8],
        value: &[u8],
    ) -> DispatchResult {
        let existing_kyc_objs = Attributes::<T>::get(&identity);

        for obj in existing_kyc_objs {
            ensure!(obj.name != name, Error::<T>::AttributeExists);
        }

        let new_attribute = Attribute {
            name: (&name).to_vec(),
            value: (&value).to_vec(),
        };

        Attributes::<T>::mutate(&identity, |list| list.push(new_attribute.clone()));

        Ok(())
    }
}