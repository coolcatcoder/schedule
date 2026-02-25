use std::{
    ops::{Deref, DerefMut},
    ptr::null_mut,
};

use bevy::prelude::*;

use crate::{bundle::SimpleBundle, query_data::SimpleQueryData};

#[derive(Default, SimpleQueryData, SimpleBundle)]
pub struct Transform2d {
    pub translation: Vec2,
    pub rotation: Rot2,
    pub scale: Vec2,
}

impl SimpleBundle for Transform2d {
    type To = Transform;

    fn get_components(self) -> Self::To {
        Transform {
            translation: self.translation.extend(0.),
            rotation: Quat::from_rotation_z(self.rotation.as_degrees()),
            scale: self.scale.extend(1.),
        }
    }
}

impl SimpleQueryData<false> for Transform2d {
    type Fetch = &'static Transform;
    type Item<'w> = Transform2dItem<'w, false>;

    fn shrink<'wlong: 'wshort, 'wshort>(item: Self::Item<'wlong>) -> Self::Item<'wshort> {
        Transform2dItem {
            translation: item.translation,
            rotation: item.rotation,
            scale: item.scale,
        }
    }

    fn fetch<'w, 's>(
        fetch: <Self::Fetch as bevy::ecs::query::QueryData>::Item<'w, 's>,
    ) -> Self::Item<'w> {
        Transform2dItem {
            translation: ref_vec3_to_ref_vec2(&fetch.translation),
            rotation: &fetch.rotation,
            scale: ref_vec3_to_ref_vec2(&fetch.scale),
        }
    }
}
impl SimpleQueryData<true> for Transform2d {
    type Fetch = &'static mut Transform;
    type Item<'w> = Transform2dItemMut<'w>;

    fn shrink<'wlong: 'wshort, 'wshort>(item: Self::Item<'wlong>) -> Self::Item<'wshort> {
        Transform2dItemMut(
            Transform2dItem {
                translation: item.0.translation,
                rotation: item.0.rotation,
                scale: item.0.scale,
            },
            item.1,
        )
    }

    fn fetch<'w, 's>(
        fetch: <Self::Fetch as bevy::ecs::query::QueryData>::Item<'w, 's>,
    ) -> Self::Item<'w> {
        // Bevy would not have to do this.
        // I have to, due to not having access to the fields of Mut.
        let mut stolen_value: *mut Transform = null_mut();
        let change_detection = fetch.map_unchanged(|value| {
            static mut WEIRD: &mut () = &mut ();

            stolen_value = value;
            // SAFETY: This is blatantly unsound. We just never ever access the value, and therefore hopefully it will be okay.
            unsafe { WEIRD }
        });
        let value = unsafe { &mut *stolen_value };

        Transform2dItemMut(
            Transform2dItem {
                translation: mut_vec3_to_mut_vec2(&mut value.translation),
                rotation: &mut value.rotation,
                scale: mut_vec3_to_mut_vec2(&mut value.scale),
            },
            change_detection,
        )
    }
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

impl<'a> Transform2dItemMut<'a> {
    pub fn is_changed(&self) -> bool {
        self.1.is_changed()
    }
}
