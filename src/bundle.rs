pub use crate::SimpleBundle;

pub trait SimpleBundle {
    type To: bevy::ecs::bundle::Bundle;

    fn get_components(self) -> Self::To;
}

#[derive(SimpleBundle)]
struct Tester;

impl SimpleBundle for Tester {
    type To = (bevy::prelude::Transform, bevy::prelude::Sprite);

    fn get_components(self) -> Self::To {
        todo!()
    }
}

#[macro_export]
macro_rules! SimpleBundle {
    derive() ($(#[$($any_attribute:tt)*])* $visibility:vis struct $name:ident $($_:tt)*) => {
        SimpleBundle!(|internal| self_type: $name, to_type: <$name as $crate::bundle::SimpleBundle>::To);
    };

    (|internal| self_type: $self_type:ty, to_type: $to_type:ty) => {
        unsafe impl ::bevy::ecs::bundle::Bundle for $self_type {
            fn component_ids(
                components: &mut ::bevy::ecs::component::ComponentsRegistrator,
            ) -> impl Iterator<Item = ::bevy::ecs::component::ComponentId> + use<> {
                <$to_type as ::bevy::ecs::bundle::Bundle>::component_ids(components)
            }

            fn get_component_ids(
                components: &::bevy::ecs::component::Components,
            ) -> impl Iterator<Item = Option<::bevy::ecs::component::ComponentId>> {
                <$to_type as ::bevy::ecs::bundle::Bundle>::get_component_ids(components)
            }
        }
        // unsafe impl ::bevy::ecs::bundle::BundleFromComponents for $self_type {
        //     #[allow(unused_variables, non_snake_case)]
        //     unsafe fn from_components<__T, __F>(ctx: &mut __T, func: &mut __F) -> Self
        //     where
        //         __F: FnMut(&mut __T) -> ::bevy::ecs::ptr::OwningPtr<'_>,
        //     {
        //         let components = <$to_type as ::bevy::ecs::bundle::BundleFromComponents>::from_components(ctx, &mut *func);
        //         let transform = unsafe {
        //             <Transform as ::bevy::ecs::bundle::BundleFromComponents>::from_components(
        //                 ctx, &mut *func,
        //             )
        //         };
        //         Self {
        //             translation: transform.translation.xy(),
        //             rotation: Rot2::radians(transform.rotation.to_euler(EulerRot::XYZ).2),
        //             scale: transform.scale.xy(),
        //         }
        //     }
        // }
        impl ::bevy::ecs::bundle::DynamicBundle for $self_type {
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