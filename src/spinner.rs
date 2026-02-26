use crate::{bundle::SimpleBundle, query_data::SimpleQueryData};
use bevy::{
    color::palettes::css::{BLUE, GREEN, RED, YELLOW}, ecs::{lifecycle::HookContext, world::DeferredWorld}, prelude::*
};
use duck_back::Else;

#[derive(Component)]
#[component(on_add = Self::on_add)]
pub struct TextFontLoad(pub &'static str);

impl TextFontLoad {
    fn on_add(mut world: DeferredWorld, context: HookContext) {
        panic!("?");
        let font = world.entity(context.entity).get::<Self>().else_error()?.0;
        let font = world.resource::<AssetServer>().load(font);
        world.entity_mut(context.entity).get_mut::<TextFont>().else_error()?.font = font;
    }
}

const COLOUR_ORDER: [Srgba; 4] = [RED, BLUE, GREEN, YELLOW];
#[derive(SimpleBundle, SimpleQueryData)]
pub struct Spinner<const LENGTH: usize = 0>(pub [&'static str; LENGTH]);

impl<const LENGTH: usize> SimpleBundle for Spinner<LENGTH> {
    type To = impl Bundle;

    fn get_components(self) -> Self::To {
        let degrees_per_slice = 360. / LENGTH as f32;
        let stops = (0..LENGTH)
            .flat_map(|index| {
                let start = degrees_per_slice * index as f32;
                let end = (start + degrees_per_slice).to_radians();
                let colour = COLOUR_ORDER[index % COLOUR_ORDER.len()].into();

                let slice = AngularColorStop {
                    color: colour,
                    angle: Some(start.to_radians()),
                    hint: end,
                };
                let avoid_blend = AngularColorStop {
                    color: colour,
                    angle: Some(end),
                    hint: end,
                };

                [slice, avoid_blend]
            })
            .collect();

        (
            Node {
                height: percent(95),
                aspect_ratio: Some(1.),
                border_radius: BorderRadius::MAX,
                justify_self: JustifySelf::Center,
                align_self: AlignSelf::Center,
                ..default()
            },
            BackgroundGradient::from(ConicGradient {
                start: 0.,
                stops,
                position: UiPosition::CENTER,
                ..default()
            }),
            children![
                Text::new("Tester. Let us see."),
                TextFont {
                    font_size: 50.,
                    ..default()
                },
                TextFontLoad("domine_regular.ttf"),
            ],
        )
    }
}

impl<const LENGTH: usize> SimpleQueryData<false> for Spinner<LENGTH> {
    type Fetch = &'static BackgroundGradient;
    type Item<'w> = &'w BackgroundGradient;

    fn fetch<'w, 's>(
        fetch: <Self::Fetch as bevy::ecs::query::QueryData>::Item<'w, 's>,
    ) -> Self::Item<'w> {
        fetch
    }

    fn shrink<'wlong: 'wshort, 'wshort>(item: Self::Item<'wlong>) -> Self::Item<'wshort> {
        item
    }
}
impl<const LENGTH: usize> SimpleQueryData<true> for Spinner<LENGTH> {
    type Fetch = &'static mut BackgroundGradient;
    type Item<'w> = Mut<'w, BackgroundGradient>;

    fn fetch<'w, 's>(fetch: <Self::Fetch as bevy::ecs::query::QueryData>::Item<'w, 's>) -> Self::Item<'w> {
        fetch
    }

    fn shrink<'wlong: 'wshort, 'wshort>(item: Self::Item<'wlong>) -> Self::Item<'wshort> {
        item
    }
}

#[derive(Bundle)]
#[bundle(ignore_from_components)]
pub struct Hope(pub bevy::ecs::spawn::SpawnRelatedBundle<bevy::prelude::ChildOf, bevy::prelude::Spawn<TextFontLoad>>);
