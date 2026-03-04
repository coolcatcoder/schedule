#![feature(macro_derive)]
#![feature(impl_trait_in_assoc_type)]
#![feature(trace_macros)]
#![feature(log_syntax)]

// Spin a wheel to pick what task you must do. When completed you get a gem.
// Gems can be used to purchase specific tasks, or re-roll.
// All tasks, including 'fun' ones, like playing games, are on the wheel.
// 'fun' ones are just less likely.
// Perhaps wave function collapse to ensure tasks get completed one one of the rolls of that day.
use bevy::prelude::*;

mod bundle;
mod query_data;
mod spinner;
mod checklist;
mod transform_2d;
use duck_back::Else;

use crate::{checklist::CheckList, spinner::Spinner};

fn main() {
    let mut window = Window::default();
    window.set_maximized(true);

    App::new()
        .add_plugins(
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(window),
                ..default()
            }),
        )
        .add_systems(Startup, start)
        .add_systems(Update, rotate)
        .run();
}

fn start(mut commands: Commands) {
    commands.spawn(Camera2d);

    //commands.spawn(Spinner(["a"; 10]));
    //commands.spawn(CheckList::<true, ()>(()));
}

fn rotate(gradient: Option<Single<(&mut Spinner, &mut UiTransform)>>, time: Res<Time>) {
    let mut gradient = gradient.else_return()?;

    gradient.1.rotation = Rot2::degrees(gradient.1.rotation.as_degrees() + 25. * time.delta_secs());
}
