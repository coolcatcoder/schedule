use crate::{bundle::BundleEffect, query_data::SimpleQueryData, transform_2d::Transform2d};
use bevy::{
    color::palettes::css::{BLUE, GREEN, RED, YELLOW},
    prelude::*,
};

const COLOUR_ORDER: [Srgba; 4] = [RED, BLUE, GREEN, YELLOW];
#[derive(BundleEffect, SimpleQueryData)]
pub struct Spinner<const LENGTH: usize = 0>(pub [&'static str; LENGTH]);

impl<const LENGTH: usize> BundleEffect for Spinner<LENGTH> {
    fn effect(self, entity_world: &mut EntityWorldMut) {
        let font = entity_world
            .resource::<AssetServer>()
            .load("domine_regular.ttf");

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

        entity_world.insert((
            Node {
                height: percent(95),
                aspect_ratio: Some(1.),
                border_radius: BorderRadius::MAX,
                justify_self: JustifySelf::Center,
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundGradient::from(ConicGradient {
                start: 0.,
                stops,
                position: UiPosition::CENTER,
                ..default()
            }),
        ));

        entity_world.with_child((
            Node {
                width: percent(100),
                height: percent(100),
                position_type: PositionType::Absolute,

                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            UiTransform {
                rotation: Rot2::degrees(degrees_per_slice * 0.5),
                ..default()
            },
            children![(
                Node {
                    width: px(250),
                    height: px(250),
                    bottom: percent(35),

                    //align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                //BackgroundColor(BLUE.into()),
                GlobalZIndex(1),
                children![(
                    Text::new("Play games for 1 hour!"),
                    TextFont {
                        font,
                        font_size: 30.,
                        ..default()
                    },
                )],
            )],
        ));

        entity_world.world_scope(|world: &mut World| {
            //world.spawn();
            // world.spawn((
            //     Node {
            //         left: percent(50),

            //         align_items: AlignItems::Center,
            //         justify_content: JustifyContent::Center,
            //         ..default()
            //     },
            //     BackgroundColor(BLUE.into()),
            //     GlobalZIndex(1),
            //     children![(
            //         Text::new("Tester. Let us see."),
            //         TextFont {
            //             font,
            //             font_size: 30.,
            //             ..default()
            //         },
            //     )],
            // ));
        });
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

    fn fetch<'w, 's>(
        fetch: <Self::Fetch as bevy::ecs::query::QueryData>::Item<'w, 's>,
    ) -> Self::Item<'w> {
        fetch
    }

    fn shrink<'wlong: 'wshort, 'wshort>(item: Self::Item<'wlong>) -> Self::Item<'wshort> {
        item
    }
}
