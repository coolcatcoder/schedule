#![feature(macro_derive)]
#![feature(impl_trait_in_assoc_type)]
#![feature(trace_macros)]

// Spin a wheel to pick what task you must do. When completed you get a gem.
// Gems can be used to purchase specific tasks, or re-roll.
// All tasks, including 'fun' ones, like playing games, are on the wheel.
// 'fun' ones are just less likely.
// Perhaps wave function collapse to ensure tasks get completed one one of the rolls of that day.
use bevy::{
    color::palettes::css::{BLACK, WHITE}, input_focus::InputDispatchPlugin, picking::hover::Hovered, prelude::*, ui_widgets::{Checkbox, UiWidgetsPlugins, checkbox_self_update, observe}
};

mod bundle;
mod query_data;
mod spinner;
mod transform_2d;
use spinner::*;

use crate::transform_2d::Transform2d;

fn main() {
    let mut window = Window::default();
    window.set_maximized(true);

    App::new()
        .add_plugins((
            InputDispatchPlugin,
            UiWidgetsPlugins,
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(window),
                ..default()
            }),
        ))
        .add_systems(Startup, start)
        .add_systems(Update, rotate)
        .run();
}

fn start(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.spawn(Spinner(["a"; 10]));

    let list = ["Get dressed.", "Make bed.", "Brush hair."];
    let font = asset_server.load("domine_regular.ttf");

    let mut root = commands.spawn(Node {
        flex_direction: FlexDirection::Column,
        ..default()
    });

    for item in list {
        root.with_child((
            Node {
                flex_direction: FlexDirection::Row,
                ..default()
            },
            children![
                (
                    Text::new(item),
                    TextFont {
                        font: font.clone(),
                        font_size: 30.,
                        ..default()
                    },
                ),
                (
                    observe(checkbox_self_update),
                    Node {
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    Pickable {
                        should_block_lower: false,
                        is_hoverable: true,
                    },
                    Checkbox,
                    Hovered::default(),
                    children![(
                        Node {
                            width: px(16),
                            height: px(16),
                            border: px(2).all(),
                            border_radius: BorderRadius::all(px(3)),
                            ..default()
                        },
                        BorderColor::all(WHITE),
                        children![(
                            Node {
                                width: px(8),
                                height: px(8),
                                position_type: PositionType::Absolute,
                                left: px(2),
                                top: px(2),
                                ..default()
                            },
                            BackgroundColor(BLACK.into()),
                        )],
                    )],
                )
            ],
        ));
    }
}

fn rotate(mut gradient: Single<(&mut Spinner, &mut UiTransform)>, time: Res<Time>) {
    // let Gradient::Conic(gradient) = &mut gradient.0.0[0] else {
    //     error!("Failed to get gradient.");
    //     return;
    // };

    // gradient.start = (gradient.start.to_degrees() + 25. * time.delta_secs()).to_radians();

    gradient.1.rotation = Rot2::degrees(gradient.1.rotation.as_degrees() + 25. * time.delta_secs());
}
