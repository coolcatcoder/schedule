#![feature(macro_derive)]
#![feature(trace_macros)]
#![feature(log_syntax)]
#![feature(try_as_dyn)]

use std::{
    any::try_as_dyn,
    fmt::{Debug, Display},
    ops::Range,
    panic::Location,
    time::Duration,
};

// Spin a wheel to pick what task you must do. When completed you get a gem.
// Gems can be used to purchase specific tasks, or re-roll.
// All tasks, including 'fun' ones, like playing games, are on the wheel.
// 'fun' ones are just less likely.
// Perhaps wave function collapse to ensure tasks get completed one one of the rolls of that day.
use bevy::{
    color::palettes::css::{BLACK, WHITE},
    prelude::*,
};

mod bundle;
mod checklist;
mod query_data;
mod spinner;
mod transform_2d;
use duck_back::Else;

use crate::{
    checklist::{CheckList, checklist},
    spinner::{Speed, Spinner, spinner},
};

fn main() {
    let mut window = Window::default();
    window.set_maximized(true);

    let start = if false { start } else { start_debug };

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

    checklist(&mut commands, "Morning.md", |mut commands: Commands| {
        earn_gem(&mut commands, |mut commands| {
            commands.run_system_cached(next);
        });
    });
}

struct TaskSlice {
    title: String,
    seconds: Option<f32>,
}
impl TaskSlice {
    fn text(&self) -> Text {
        Text::new(&self.title)
    }
}

fn next(mut commands: Commands) {
    let tasks_file = ObsidianFile::open("Tasks.md").else_error()?;

    let mut tasks: Vec<TaskSlice> = vec![];

    for line in tasks_file.rest().lines().rev() {
        if line.starts_with("| -") {
            break;
        }

        let Some(line) = line.strip_prefix("| ") else {
            error!("Failed to parse line.");
            continue;
        };

        let mut cells = line.split("|");

        let Some(title) = cells.next() else {
            error!("Failed to parse line.");
            continue;
        };
        let title = title.trim();

        let Some(duration) = cells.next() else {
            error!("Failed to parse line.");
            continue;
        };
        let duration = duration.trim();
        info!("{:?}", duration);
        let duration = if duration.contains("until completed") {
            None
        } else if let Some(hours) = duration
            .strip_suffix("hours")
            .or(duration.strip_suffix("hour"))
        {
            let Ok(hours) = hours.trim().parse::<f32>() else {
                error!("Failed to parse line.");
                continue;
            };
            Some(hours * 60. * 60.)
        } else if let Some(minutes) = duration
            .strip_suffix("minutes")
            .or(duration.strip_suffix("minute"))
        {
            let Ok(minutes) = minutes.trim().parse::<f32>() else {
                error!("Failed to parse line.");
                continue;
            };
            Some(minutes * 60.)
        } else {
            error!("Failed to parse line.");
            None
        };

        tasks.push(TaskSlice {
            title: title.to_owned(),
            seconds: duration,
        });
    }

    let file = ObsidianFile::open("2026-03-11.md").else_error()?;

    spinner(&mut commands, tasks, |mut commands, task| {
        show_task(&mut commands, task, |mut commands| {
            commands.run_system_cached(next);
        });
    });
}

fn start_debug(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.run_system_cached(next);
}

fn show_task(commands: &mut Commands, task: TaskSlice, then: fn(Commands)) {
    let text = task.title.to_owned();
    commands.queue(move |world: &mut World| {
        let font = world.resource::<AssetServer>().load("domine_regular.ttf");

        let mut root = world.spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceEvenly,
            width: percent(100),
            height: percent(100),
            ..default()
        });
        let root_entity = root.id();

        root.with_children(|entity| {
            entity.spawn((
                Text::new(text),
                TextFont {
                    font: font.clone(),
                    font_size: 50.,
                    ..default()
                },
            ));

            if let Some(seconds) = task.seconds {
                entity.spawn((
                    Text::new(""),
                    TextFont {
                        font: font.clone(),
                        font_size: 50.,
                        ..default()
                    },
                ));
            } else {
                info!("?");
                entity.spawn((
                    Text::new("Until completed."),
                    TextFont {
                        font: font.clone(),
                        font_size: 50.,
                        ..default()
                    },
                ));
            }

            entity
                .spawn((
                    Node {
                        width: px(200),
                        height: px(80),
                        border: px(5).all(),
                        border_radius: BorderRadius::all(px(10)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BorderColor::all(WHITE),
                    BackgroundColor(BLACK.into()),
                ))
                .with_child((
                    Text::new("Continue"),
                    TextFont {
                        font: font.clone(),
                        font_size: 30.,
                        ..default()
                    },
                ))
                .observe(move |_: On<Pointer<Click>>, mut commands: Commands| {
                    commands.entity(root_entity).despawn();
                    then(commands);
                });
        });
    });
}

fn earn_gem(commands: &mut Commands, then: fn(Commands)) {
    commands.queue(move |world: &mut World| {
        let font = world.resource::<AssetServer>().load("domine_regular.ttf");

        let mut root = world.spawn(Node {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::SpaceEvenly,
            width: percent(100),
            height: percent(100),
            ..default()
        });
        let root_entity = root.id();

        root.with_children(|entity| {
            entity.spawn((
                Text::new("You have earned a gem!\nGo grab one and place it in your jar!"),
                TextFont {
                    font: font.clone(),
                    font_size: 50.,
                    ..default()
                },
            ));

            entity
                .spawn((
                    Node {
                        width: px(200),
                        height: px(80),
                        border: px(5).all(),
                        border_radius: BorderRadius::all(px(10)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BorderColor::all(WHITE),
                    BackgroundColor(BLACK.into()),
                ))
                .with_child((
                    Text::new("Continue"),
                    TextFont {
                        font: font.clone(),
                        font_size: 30.,
                        ..default()
                    },
                ))
                .observe(move |_: On<Pointer<Click>>, mut commands: Commands| {
                    commands.entity(root_entity).despawn();
                    then(commands);
                });
        });
    });
}

fn rotate(
    spinner: Option<Single<(Entity, &mut Speed, &mut UiTransform)>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    let mut spinner = spinner.else_return()?;
    let (entity, speed, transform) = &mut *spinner;

    transform.rotation =
        Rot2::degrees(transform.rotation.as_degrees() + speed.speed * time.delta_secs());
    speed.stop -= time.delta_secs();

    if speed.stop <= 0. {
        speed.speed -= 1000. * time.delta_secs();
        speed.speed = speed.speed.max(0.);
    }

    if speed.speed == 0. {
        info!("Done.");
        commands.entity(*entity).despawn();

        let degrees = -transform.rotation.as_degrees();

        let degrees = if degrees.is_sign_negative() {
            degrees + 360.
        } else {
            degrees
        };

        let index = degrees / speed.degrees_per_slice;
        let index = index + index.signum() * -0.5;
        let index = index.round() as usize;
        info!("{index}");

        (speed.then)(commands, speed.slices.swap_remove(index));
    }
}

struct DetachedStr(Range<usize>);
impl DetachedStr {
    fn get<'a>(&self, string: &'a str) -> &'a str {
        &string[self.0.clone()]
    }

    /// Remove last element;
    fn pop(&mut self) {
        self.0.end -= 1;
    }
}

// struct Error<T: 'static>(T);

// impl<T: 'static> Debug for Error<T> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         if let Some(error) = try_as_dyn::<Self, dyn Display>(self) {
//             error.fmt(f)
//         } else if let Some(error) = try_as_dyn::<Self, dyn Debug>(self) {
//             error.fmt(f)
//         } else {
//             Ok(())
//         }
//     }
// }

type Result<T, E = Box<dyn Error>> = core::result::Result<T, E>;

trait Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

impl<T: 'static> Error for T {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(error) = try_as_dyn::<Self, dyn Display>(self) {
            error.fmt(f)
        } else if let Some(error) = try_as_dyn::<Self, dyn Debug>(self) {
            error.fmt(f)
        } else {
            Ok(())
        }
    }
}

impl Display for dyn Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Error>::fmt(self, f)
    }
}

impl<T: 'static + Debug> From<T> for Box<dyn Error> {
    fn from(value: T) -> Self {
        Box::new(value)
    }
}

struct ObsidianFile {
    file: String,
    properties: Vec<(DetachedStr, DetachedStr)>,
    rest: DetachedStr,
}

impl ObsidianFile {
    fn checkboxes(&self) -> impl Iterator<Item = (bool, &str)> {
        if !self
            .properties()
            .any(|(name, content)| name == "tags" && content.contains("- checklist"))
        {
            warn!("Checklist is not tagged with checklist.");
        }

        self.rest().lines().filter_map(|line| {
            if let Some(task) = line.strip_prefix("- [ ] ") {
                Some((false, task))
            } else if let Some(task) = line.strip_prefix("- [x] ") {
                Some((true, task))
            } else {
                None
            }
        })
    }

    fn list_items(&self) -> impl Iterator<Item = &str> {
        if !self
            .properties()
            .any(|(name, content)| name == "tags" && content.contains("- list"))
        {
            warn!("List is not tagged with list.");
        }

        self.rest()
            .lines()
            .filter_map(|line| line.strip_prefix("- "))
    }

    fn properties(&self) -> impl Iterator<Item = (&str, &str)> {
        self.properties
            .iter()
            .map(|(name, content)| (name.get(&self.file), content.get(&self.file)))
    }

    fn rest(&self) -> &str {
        self.rest.get(&self.file)
    }

    fn open(path: &str) -> Result<ObsidianFile> {
        let file = String::from_utf8(
            std::fs::read(format!(
                "/home/coolcatcoder/Documents/GitHub/random_notes/{path}"
            ))
            .map_err(|error| Location::caller())?,
        )
        .unwrap();

        enum Stage {
            Start,
            Properties,
            Rest,
        }
        let mut stage = Stage::Start;

        struct Unparsed<'a>(Range<usize>, &'a str);
        impl<'a> Unparsed<'a> {
            fn equals(&self, string: &'static str) -> bool {
                if string.len() != self.0.len() {
                    return false;
                }

                for (char_a, char_b_index) in string.chars().zip(self.0.clone()) {
                    let char_b = self.1.chars().nth(char_b_index).unwrap();
                    if char_a != char_b {
                        return false;
                    }
                }

                true
            }

            fn ends_with(&self, string: &'static str) -> bool {
                if string.len() > self.0.len() {
                    return false;
                }

                for (char_a, char_b_index) in string.chars().rev().zip(self.0.clone().rev()) {
                    let char_b = self.1.chars().nth(char_b_index).unwrap();
                    if char_a != char_b {
                        return false;
                    }
                }

                true
            }

            fn clear(&mut self) {
                self.0.start = self.0.end;
            }

            /// Does not include the character in the new string.
            /// TO DO: Remove len() calls, it should instead be count(), as we don't know if any non-ascii characters are in the string.
            #[track_caller]
            fn until(&self, character: char) -> Result<DetachedStr> {
                info!("Start until.");
                let mut characters = self.1.chars();
                let checker = characters.nth_back(self.1.len() - 1 - self.0.end).unwrap();
                info!("Checker: {checker}");
                let reversed_characters = characters.rev();

                let mut back = 0;
                let mut found = false;

                for character_in_string in reversed_characters {
                    info!("character_in_string {character_in_string}");
                    back += 1;
                    if character == character_in_string {
                        found = true;
                        break;
                    }
                }

                if found {
                    info!("self.0.end {}", self.0.end);
                    info!("back {back}");

                    let start = self.0.end - back;

                    // start + 1 as we aren't including the character at the start.
                    Ok(DetachedStr((start + 1)..self.0.end))
                } else {
                    Err(Location::caller())?
                }
            }
        }

        let mut unparsed = Unparsed(0..0, &file);
        let mut properties: Vec<(DetachedStr, DetachedStr)> = vec![];

        for _ in file.chars() {
            //info!("Current char: {char}");
            unparsed.0.end += 1;

            match stage {
                Stage::Start => {
                    if unparsed.ends_with("---") {
                        info!("Entering properties.");
                        stage = Stage::Properties;
                        unparsed.clear();
                    }
                }
                Stage::Properties => {
                    if unparsed.ends_with(":") {
                        let mut name = unparsed.until('\n')?;
                        name.pop();

                        if let Some(previous) = properties.last_mut() {
                            let previous_content = DetachedStr(unparsed.0.start..name.0.start);
                            previous.1 = previous_content;
                        }

                        properties.push((name, DetachedStr(0..0)));
                        unparsed.clear();
                    } else if unparsed.ends_with("---") {
                        info!("Leaving properties.");

                        if let Some(previous) = properties.last_mut() {
                            let previous_content =
                                DetachedStr(unparsed.0.start..(unparsed.0.end - 3));
                            previous.1 = previous_content;
                        }

                        stage = Stage::Rest;
                        unparsed.clear();
                    }
                }
                _ => (),
            }
        }

        info!("Properties:");

        for property in &properties {
            info!("{:?}\n{:?}", property.0.get(&file), property.1.get(&file));
        }

        let rest = DetachedStr(unparsed.0.clone());
        Ok(ObsidianFile {
            file,
            properties,
            rest,
        })
    }
}
