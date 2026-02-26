pub use crate::BundleEffect;
use bevy::ecs::world::EntityWorldMut;

pub trait BundleEffect {
    fn effect(self, entity_world: &mut EntityWorldMut);
}

// TO DO: I wonder if it is possible to have this work like a regular Bundle derive macro, if the type doesn't implement BundleEffect. Probably using try_as_dyn.
#[macro_export]
macro_rules! BundleEffect {
    derive() ($(#[$($_:tt)*])* $visibility:vis struct $name:ident <$($(const $const_generic_ident:ident:)? $generic_type:ty $(= $default:tt)?),*> $($__:tt)*) => {
        BundleEffect!(|internal| self_type: $name<$($crate::bundle::BundleEffect!(|generic_get_ident| generic: $(const $const_generic_ident:)? $generic_type)),*>, to_type: <$name<$($crate::bundle::BundleEffect!(|generic_get_ident| generic: $(const $const_generic_ident:)? $generic_type)),*> as $crate::bundle::BundleEffect>::To, impl_generics: ($($(const $const_generic_ident:)? $generic_type),*), component_ids_return_type: $crate::bundle::BundleEffect!(|construct| input: ($($(const $const_generic_ident:)? $generic_type,)*), output: (impl Iterator<Item = ::bevy::ecs::component::ComponentId>)));
    };
    derive() ($(#[$($_:tt)*])* $visibility:vis struct $name:ident $($__:tt)*) => {
        BundleEffect!(|internal| self_type: $name, to_type: <$name as $crate::bundle::BundleEffect>::To, impl_generics: , component_ids_return_type: impl Iterator<Item = ::bevy::ecs::component::ComponentId> + use<>);
    };

    (|construct| input: (const $const_generic_ident:ident: $const_generic_type:ty, $($input:tt)*), output: ($($output:tt)*)) => {
        $crate::bundle::BundleEffect!(|construct| input: ($($input)*), output: ($($output)* + use<$const_generic_ident>))
    };

    (|construct| input: (), output: ($($output:tt)*)) => {
        $($output)*
    };

    (|generic_get_ident| generic: const $const_generic_ident:ident: $const_generic_type:ty) => {
        $const_generic_ident
    };

    (|generic_get_use| generic: const $const_generic_ident:ident: $const_generic_type:ty) => {
        use<$const_generic_ident>
    };

    (|internal| self_type: $self_type:ty, to_type: $to_type:ty, impl_generics: $(($($impl_generics:tt)*))?, component_ids_return_type: $($component_ids_return_type:tt)*) => {
        unsafe impl$(<$($impl_generics)*>)? ::bevy::ecs::bundle::Bundle for $self_type {
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

        impl$(<$($impl_generics)*>)? ::bevy::ecs::bundle::DynamicBundle for $self_type {
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
