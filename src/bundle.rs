pub use crate::SimpleBundle;

pub trait SimpleBundle {
    type To: bevy::ecs::bundle::Bundle;

    fn get_components(self) -> Self::To;
}

#[macro_export]
macro_rules! SimpleBundle {
    derive() ($(#[$($_:tt)*])* $visibility:vis struct $name:ident <$($(const $const_generic_ident:ident:)? $generic_type:ty),*> $($__:tt)*) => {
        SimpleBundle!(|internal| self_type: $name<$($crate::bundle::SimpleBundle!(|generic_get_ident| generic: $(const $const_generic_ident:)? $generic_type)),*>, to_type: <$name<$($crate::bundle::SimpleBundle!(|generic_get_ident| generic: $(const $const_generic_ident:)? $generic_type)),*> as $crate::bundle::SimpleBundle>::To, impl_generics: ($($(const $const_generic_ident:)? $generic_type),*), component_ids_return_type: $crate::bundle::SimpleBundle!(|construct| input: ($($(const $const_generic_ident:)? $generic_type,)*), output: (impl Iterator<Item = ::bevy::ecs::component::ComponentId>)));
    };
    derive() ($(#[$($_:tt)*])* $visibility:vis struct $name:ident $($__:tt)*) => {
        SimpleBundle!(|internal| self_type: $name, to_type: <$name as $crate::bundle::SimpleBundle>::To, impl_generics: , component_ids_return_type: impl Iterator<Item = ::bevy::ecs::component::ComponentId> + use<>);
    };

    (|construct| input: (const $const_generic_ident:ident: $const_generic_type:ty, $($input:tt)*), output: ($($output:tt)*)) => {
        $crate::bundle::SimpleBundle!(|construct| input: ($($input)*), output: ($($output)* + use<$const_generic_ident>))
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
                components: &mut ::bevy::ecs::component::ComponentsRegistrator,
            ) -> $($component_ids_return_type)* {
                <$to_type as ::bevy::ecs::bundle::Bundle>::component_ids(components)
            }

            fn get_component_ids(
                components: &::bevy::ecs::component::Components,
            ) -> impl Iterator<Item = Option<::bevy::ecs::component::ComponentId>> {
                <$to_type as ::bevy::ecs::bundle::Bundle>::get_component_ids(components)
            }
        }

        impl$(<$($impl_generics)*>)? ::bevy::ecs::bundle::DynamicBundle for $self_type {
            type Effect = ();
            #[allow(unused_variables)]
            #[inline]
            unsafe fn get_components(
                ptr: ::bevy::ecs::ptr::MovingPtr<'_, Self>,
                func: &mut impl FnMut(::bevy::ecs::component::StorageType, ::bevy::ecs::ptr::OwningPtr<'_>),
            ) {
                let value = ptr.read();
                let components = <$self_type as $crate::bundle::SimpleBundle>::get_components(value);
                ::bevy::ecs::ptr::move_as_ptr!(components);

                unsafe {
                    <$to_type as ::bevy::ecs::bundle::DynamicBundle>::get_components(components, func)
                };
            }
            #[allow(unused_variables)]
            #[inline]
            unsafe fn apply_effect(
                ptr: ::bevy::ecs::ptr::MovingPtr<'_, core::mem::MaybeUninit<Self>>,
                func: &mut ::bevy::ecs::world::EntityWorldMut<'_>,
            ) {
            }
        }
    };
}
