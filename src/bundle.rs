pub use crate::BundleEffect;
use bevy::ecs::world::EntityWorldMut;

pub trait BundleEffect {
    fn effect(self, entity_world: &mut EntityWorldMut);
}

// TO DO: I wonder if it is possible to have this work like a regular Bundle derive macro, if the type doesn't implement BundleEffect. Probably using try_as_dyn.
#[macro_export]
macro_rules! BundleEffect {
    derive() ($(#[$_:meta])* $__:vis struct $name:ident <$($(const $const_generic_ident:ident:)? $generic_type:ty $(: $($bounds:ident)++)? $(= $default:tt)?),*> $($___:tt)*) => {
        $crate::bundle::BundleEffect!(|1: Process Generics|
            remaining_generics_to_check($($(const $const_generic_ident:)? $generic_type,)*),
            self_type($name<),
            impl_generics(<$($(const $const_generic_ident:)? $generic_type $(: $($bounds)++)?,)*>),
            component_ids_return_type(impl Iterator<Item = ::bevy::ecs::component::ComponentId> + use<),
        );
    };
    derive() ($(#[$_:meta])* $__:vis struct $name:ident $($___:tt)*) => {
        $crate::bundle::BundleEffect!(|1: Process Generics|
            remaining_generics_to_check(),
            self_type($name<),
            impl_generics(),
            component_ids_return_type(impl Iterator<Item = ::bevy::ecs::component::ComponentId> + use<),
        );
    };

    (|1: Process Generics|
        remaining_generics_to_check(const $generic_ident:ident: $generic_type:ty, $($remaining_generics_to_check:tt)*),
        self_type($($self_type:tt)*),
        impl_generics($($impl_generics:tt)*),
        component_ids_return_type($($component_ids_return_type:tt)*),
    ) => {
        $crate::bundle::BundleEffect!(|1: Process Generics|
            remaining_generics_to_check($($remaining_generics_to_check)*),
            self_type($($self_type)* $generic_ident,),
            impl_generics($($impl_generics)*),
            component_ids_return_type($($component_ids_return_type)* $generic_ident,),
        );
    };
    (|1: Process Generics|
        remaining_generics_to_check($generic_type:ty, $($remaining_generics_to_check:tt)*),
        self_type($($self_type:tt)*),
        impl_generics($($impl_generics:tt)*),
        component_ids_return_type($($component_ids_return_type:tt)*),
    ) => {
        $crate::bundle::BundleEffect!(|1: Process Generics|
            remaining_generics_to_check($($remaining_generics_to_check)*),
            self_type($($self_type)* $generic_type,),
            impl_generics($($impl_generics)*),
            component_ids_return_type($($component_ids_return_type)* $generic_type,),
        );
    };
    (|1: Process Generics|
        remaining_generics_to_check(),
        self_type($($self_type:tt)*),
        impl_generics($($impl_generics:tt)*),
        component_ids_return_type($($component_ids_return_type:tt)*),
    ) => {
        log_syntax!{
            $crate::bundle::BundleEffect!(|2: Implementation|
                self_type($($self_type)*>),
                impl_generics($($impl_generics)*),
                component_ids_return_type($($component_ids_return_type)*>),
            );
        }
        $crate::bundle::BundleEffect!(|2: Implementation|
            self_type($($self_type)*>),
            impl_generics($($impl_generics)*),
            component_ids_return_type($($component_ids_return_type)*>),
        );
    };

    //(|internal| self_type: $self_type:ty, to_type: $to_type:ty, impl_generics: $(($($impl_generics:tt)*))?, component_ids_return_type: $($component_ids_return_type:tt)*) => {
    (|2: Implementation|
        self_type($($self_type:tt)*),
        impl_generics($($impl_generics:tt)*),
        component_ids_return_type($($component_ids_return_type:tt)*),
    ) => {
        unsafe impl$($impl_generics)* ::bevy::ecs::bundle::Bundle for $($self_type)* {
            fn component_ids(
                _: &mut ::bevy::ecs::component::ComponentsRegistrator,
            ) -> $($component_ids_return_type)* {
                core::iter::empty()
            }

            fn get_component_ids(
                _: &::bevy::ecs::component::Components,
            ) -> impl Iterator<Item = Option<::bevy::ecs::component::ComponentId>> {
                core::iter::empty()
            }
        }

        impl$($impl_generics)* ::bevy::ecs::bundle::DynamicBundle for $($self_type)* {
            type Effect = Self;
            #[allow(unused_variables)]
            #[inline]
            unsafe fn get_components(
                _: ::bevy::ecs::ptr::MovingPtr<'_, Self>,
                _: &mut impl FnMut(::bevy::ecs::component::StorageType, ::bevy::ecs::ptr::OwningPtr<'_>),
            ) {

            }
            #[allow(unused_variables)]
            #[inline]
            unsafe fn apply_effect(
                ptr: ::bevy::ecs::ptr::MovingPtr<'_, core::mem::MaybeUninit<Self>>,
                func: &mut ::bevy::ecs::world::EntityWorldMut<'_>,
            ) {
                // SAFETY: We don't interact with the pointer in get_components, so this should be fine.
                let ptr = unsafe{ ptr.assume_init() };
                let value = ptr.read();

                <Self as $crate::bundle::BundleEffect>::effect(value, func);
            }
        }
    };
}
