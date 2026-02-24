use std::{
    ops::{Deref, DerefMut},
    ptr::null_mut,
};

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

pub struct Transform2dItem<'a, const MUT: bool>
where
    Vec2: MakeReference<MUT>,
    Quat: MakeReference<MUT>,
{
    pub translation: <Vec2 as MakeReference<MUT>>::Output<'a>,
    pub rotation: <Quat as MakeReference<MUT>>::Output<'a>,
    pub scale: <Vec2 as MakeReference<MUT>>::Output<'a>,
}

pub struct Transform2dItemMut<'a>(Transform2dItem<'a, true>, Mut<'a, ()>);

impl<'a> Deref for Transform2dItemMut<'a> {
    type Target = Transform2dItem<'a, true>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for Transform2dItemMut<'a> {
    #[track_caller]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.1.set_changed();
        &mut self.0
    }
}

fn mut_vec3_to_mut_vec2(value: &mut Vec3) -> &mut Vec2 {
    let value: &mut [f32; 3] = value.as_mut();
    let value: *mut [f32; 3] = value;
    let value: *mut [f32; 2] = value as *mut [f32; 2];
    let value: *mut Vec2 = value as *mut Vec2;
    // SAFETY: Miri allows it.
    let value: &mut Vec2 = unsafe { &mut *value };
    value
}

fn ref_vec3_to_ref_vec2(value: &Vec3) -> &Vec2 {
    let value: &[f32; 3] = value.as_ref();
    let value: *const [f32; 3] = value;
    let value: *const [f32; 2] = value as *const [f32; 2];
    let value: *const Vec2 = value as *const Vec2;
    // SAFETY: Miri allows it.
    let value: &Vec2 = unsafe { &*value };
    value
}

impl<'a> From<Mut<'a, Transform>> for Transform2dItemMut<'a> {
    fn from(value: Mut<'a, Transform>) -> Self {
        // Bevy would not have to do this.
        // I have to, due to not having access to the fields of Mut.
        let mut stolen_value: *mut Transform = null_mut();
        let change_detection = value.map_unchanged(|value| {
            static mut WEIRD: &mut () = &mut ();

            stolen_value = value;
            // SAFETY: This is blatantly unsound. We just never ever access the value, and therefore hopefully it will be okay.
            unsafe { WEIRD }
        });
        let value = unsafe { &mut *stolen_value };

        Self(
            Transform2dItem {
                translation: mut_vec3_to_mut_vec2(&mut value.translation),
                rotation: &mut value.rotation,
                scale: mut_vec3_to_mut_vec2(&mut value.scale),
            },
            change_detection,
        )
    }
}

impl<'a> Transform2dItemMut<'a> {
    fn from<'b>(value: Transform2dItemMut<'b>) -> Transform2dItemMut<'a>
    where
        'b: 'a,
    {
        Transform2dItemMut(
            Transform2dItem {
                translation: value.0.translation,
                rotation: value.0.rotation,
                scale: value.0.scale,
            },
            value.1,
        )
    }

    pub fn is_changed(&self) -> bool {
        self.1.is_changed()
    }
}

impl<'a> From<&'a Transform> for Transform2dItem<'a, false> {
    fn from(value: &'a Transform) -> Self {
        Self {
            translation: ref_vec3_to_ref_vec2(&value.translation),
            rotation: &value.rotation,
            scale: ref_vec3_to_ref_vec2(&value.scale),
        }
    }
}

impl<'a> Transform2dItem<'a, false> {
    fn from<'b>(value: Transform2dItem<'b, false>) -> Self
    where
        'b: 'a,
    {
        Transform2dItem {
            translation: value.translation,
            rotation: value.rotation,
            scale: value.scale,
        }
    }
}

query_data!(|internal| Transform2d, &, Transform2dItem<'w, false>, (&'static Transform));
query_data!(|internal| Transform2d, &mut, Transform2dItemMut<'w>, (&'static mut Transform));

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
        let transform = unsafe {
            <Transform as ::bevy::ecs::bundle::BundleFromComponents>::from_components(
                ctx, &mut *func,
            )
        };
        Self {
            translation: transform.translation.xy(),
            rotation: Rot2::radians(transform.rotation.to_euler(EulerRot::XYZ).2),
            scale: transform.scale.xy(),
        }
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

        unsafe {
            <Transform as ::bevy::ecs::bundle::DynamicBundle>::get_components(transform, func)
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
