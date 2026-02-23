// Spin a wheel to pick what task you must do. When completed you get a gem.
// Gems can be used to purchase specific tasks, or re-roll.
// All tasks, including 'fun' ones, like playing games, are on the wheel.
// 'fun' ones are just less likely.
// Perhaps wave function collapse to ensure tasks get completed one one of the rolls of that day.
use bevy::{
    color::palettes::css::{BLUE, GREEN, LIGHT_BLUE, NAVY, RED, YELLOW},
    prelude::*,
    window::{WindowMode, WindowResolution},
};

mod transform_2d;
mod query_data;
mod spinner;
use spinner::*;

use crate::transform_2d::Transform2d;

fn main() {
    let mut window = Window::default();
    window.set_maximized(true);

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(window),
            ..default()
        }))
        .add_systems(Startup, start)
        .add_systems(Update, rotate)
        .run();
}

fn start(mut commands: Commands) {
    commands.spawn(Camera2d);

    let colour_order = [RED, BLUE, GREEN, YELLOW];
    let amount = 40;

    let degrees_per_slice = 360. / amount as f32;
    let stops = (0..amount)
        .flat_map(|index| {
            let start = degrees_per_slice * index as f32;
            let end = (start + degrees_per_slice).to_radians();
            let colour = colour_order[index % colour_order.len()].into();

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

    commands.spawn((
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
    ));

    commands.spawn((Transform2d {
        translation: Vec2::new(2.3, 9.8),
        rotation: Rot2::IDENTITY,
        scale: Vec2::ONE,
    }, Marker));
}

#[derive(Component)]
struct Marker;

fn rotate(mut gradient: Single<&mut BackgroundGradient>, time: Res<Time>, bad: Single<&Transform2d, With<Marker>>) {
    let Gradient::Conic(gradient) = &mut gradient.0[0] else {
        error!("Failed to get gradient.");
        return;
    };

    gradient.start = (gradient.start.to_degrees() + 25. * time.delta_secs()).to_radians();

    info!("{}", bad.0.translation);
}
