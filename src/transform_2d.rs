use bevy::{prelude::*, ptr::move_as_ptr};

use crate::query_data;


#[derive(Default)]
pub struct Transform2d {
    pub translation: Vec2,
    pub rotation: Rot2,
    pub scale: Vec2,
}

pub trait MakeReference<const MUT: bool> {
    type Output<'a>;
}

impl<T: 'static> MakeReference<true> for T {
    type Output<'a> = &'a mut T;
}
impl<T: 'static> MakeReference<false> for T {
    type Output<'a> = &'a T;
}

pub struct Transform2dItem<'a, const MUT: bool>(pub <Transform as MakeReference<MUT>>::Output<'a>) where Transform: MakeReference<MUT>;

impl<'a> From<&'a Transform> for Transform2dItem<'a, false> {
    fn from(value: &'a Transform) -> Self {
        Self(value)
    }
}
impl<'a> Transform2dItem<'a, false> {
    fn from<'b>(value: Transform2dItem<'b, false>) -> Self where 'b: 'a {
        Transform2dItem(value.0)
    }
}

// I am using these to test that my my macro still works when used normally.
//query_data!(Transform2d, &, (Transform));
//query_data!(Transform2d, &mut, (Transform));

query_data!(|internal| Transform2d, &, Transform2dItem<'w, false>, (&'static Transform));

fn tester(mut blah: Single<&Transform2d>, mut commands: Commands) {
    let blah = &*blah;

    commands.spawn(Transform2d {
        translation: Vec2::new(2.3, 9.8),
        rotation: Rot2::IDENTITY,
        scale: Vec2::ONE,
    });
}

unsafe impl ::bevy::ecs::bundle::Bundle for Transform2d {
    fn component_ids(
        components: &mut ::bevy::ecs::component::ComponentsRegistrator,
    ) -> impl Iterator<Item = ::bevy::ecs::component::ComponentId> + use<> {
        core::iter::empty().chain(<Transform as ::bevy::ecs::bundle::Bundle>::component_ids(
            components,
        ))
    }
    fn get_component_ids(
        components: &::bevy::ecs::component::Components,
    ) -> impl Iterator<Item = Option<::bevy::ecs::component::ComponentId>> {
        core::iter::empty()
            .chain(<Transform as ::bevy::ecs::bundle::Bundle>::get_component_ids(components))
    }
}
unsafe impl ::bevy::ecs::bundle::BundleFromComponents for Transform2d {
    #[allow(unused_variables, non_snake_case)]
    unsafe fn from_components<__T, __F>(ctx: &mut __T, func: &mut __F) -> Self
    where
        __F: FnMut(&mut __T) -> ::bevy::ecs::ptr::OwningPtr<'_>,
    {
        let transform = unsafe { <Transform as ::bevy::ecs::bundle::BundleFromComponents>::from_components(ctx, &mut *func,) };
        Self { translation: transform.translation.xy(), rotation: todo!(), scale: transform.scale.xy() }
    }
}
impl ::bevy::ecs::bundle::DynamicBundle for Transform2d {
    type Effect = ();
    #[allow(unused_variables)]
    #[inline]
    unsafe fn get_components(
        ptr: ::bevy::ecs::ptr::MovingPtr<'_, Self>,
        func: &mut impl FnMut(::bevy::ecs::component::StorageType, ::bevy::ecs::ptr::OwningPtr<'_>),
    ) {
        let value = ptr.read();
        let transform = Transform {
            translation: value.translation.extend(0.),
            rotation: Quat::from_rotation_z(value.rotation.as_degrees()),
            scale: value.scale.extend(1.),
        };
        move_as_ptr!(transform);

        unsafe { <Transform as ::bevy::ecs::bundle::DynamicBundle>::get_components(transform, func) };
    }
    #[allow(unused_variables)]
    #[inline]
    unsafe fn apply_effect(
        ptr: ::bevy::ecs::ptr::MovingPtr<'_, core::mem::MaybeUninit<Self>>,
        func: &mut ::bevy::ecs::world::EntityWorldMut<'_>,
    ) {
    }
}