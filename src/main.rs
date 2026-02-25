#![feature(macro_derive)]
#![feature(impl_trait_in_assoc_type)]
#![feature(trace_macros)]

// Spin a wheel to pick what task you must do. When completed you get a gem.
// Gems can be used to purchase specific tasks, or re-roll.
// All tasks, including 'fun' ones, like playing games, are on the wheel.
// 'fun' ones are just less likely.
// Perhaps wave function collapse to ensure tasks get completed one one of the rolls of that day.
use bevy::prelude::*;

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
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(window),
            ..default()
        }))
        .add_systems(Startup, start)
        .add_systems(Update, (rotate, tester))
        .run();
}

fn start(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn(Spinner(["a"; 10]));

    commands.spawn((
        Transform2d {
            translation: Vec2::new(2.3, 9.8),
            rotation: Rot2::IDENTITY,
            scale: Vec2::ONE,
        },
        Marker,
    ));
}

#[derive(Component)]
struct Marker;

fn rotate(
    mut gradient: Single<&mut BackgroundGradient>,
    time: Res<Time>,
    mut bad: Single<&mut Transform2d, With<Marker>>,
) {
    let Gradient::Conic(gradient) = &mut gradient.0[0] else {
        error!("Failed to get gradient.");
        return;
    };

    gradient.start = (gradient.start.to_degrees() + 25. * time.delta_secs()).to_radians();

    info!("{}", bad.translation);
    bad.translation.x += 1.;
}

fn tester(bad: Single<&mut Transform2d, With<Marker>>) {
    info!("{}", bad.is_changed());
}
