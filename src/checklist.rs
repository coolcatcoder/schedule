use crate::{ObsidianFile, bundle::BundleEffect};
use bevy::{
    color::palettes::css::{BLACK, GREY, WHITE},
    prelude::*,
    ui_widgets::observe,
};
use duck_back::Else;

pub fn checklist(commands: &mut Commands, path: &str, then: fn(Commands)) {
    let file = ObsidianFile::open(path).else_error()?;

    if !file
        .properties()
        .any(|(name, content)| name == "tags" && content.contains("- checklist"))
    {
        warn!("Checklist {path} is not tagged with checklist.");
    }

    commands.queue(move |world: &mut World| {
        #[derive(Component)]
        struct CheckedText;

        #[derive(Component)]
        struct Done;

        let tasks = file.rest().lines().filter_map(|line| {
            if let Some(task) = line.strip_prefix("- [ ] ") {
                Some((false, task))
            } else if let Some(task) = line.strip_prefix("- [x] ") {
                Some((true, task))
            } else {
                None
            }
        });

        let font = world
            .resource::<AssetServer>()
            .load("domine_regular.ttf");

        let mut entity_world = world.spawn(Node {
            width: percent(100),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            ..default()
        });
        let entity = entity_world.id();

        for (checked, task) in tasks {
            let checked = if checked {
                "X"
            } else {
                ""
            };

            entity_world.with_child((
                Node {
                    width: percent(100),
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                children![
                    (
                        observe(
                            |on: On<Pointer<Over>>, mut colour: Query<&mut BorderColor>| {
                                *colour.get_mut(on.entity).else_error()? = BorderColor::all(WHITE);
                            }
                        ),
                        observe(
                            |on: On<Pointer<Out>>, mut colour: Query<&mut BorderColor>| {
                                *colour.get_mut(on.entity).else_error()? = BorderColor::all(GREY);
                            }
                        ),
                        observe(
                            |on: On<Pointer<Click>>,
                             children: Query<&Children>,
                             mut text: Query<&mut Text, With<CheckedText>>, mut done: Single<&mut Visibility, With<Done>>| {
                                let children = children.get(on.entity).else_error()?;
                                let child =
                                    children.first().else_error()?;
                                let mut child = text.get_mut(*child).else_error()?;

                                if child.0 == "X" {
                                    child.0 = "".into();
                                } else {
                                    child.0 = "X".into();
                                }

                                for text in text {
                                    if text.0 != "X" {
                                        **done = Visibility::Hidden;
                                        return;
                                    }
                                }

                                info!("All ticked!");
                                **done = Visibility::Visible;
                            }
                        ),
                        Node {
                            width: px(30),
                            height: px(30),
                            border: px(2).all(),
                            border_radius: BorderRadius::all(px(3)),
                            align_self: AlignSelf::Center,
                            ..default()
                        },
                        BorderColor::all(GREY),
                        children![(
                            Node {
                                width: px(16),
                                height: px(16),
                                position_type: PositionType::Absolute,
                                left: px(4),
                                top: px(4),
                                ..default()
                            },
                            BackgroundColor(BLACK.into()),
                            CheckedText,
                            Text::new(checked),
                            TextFont {
                                font: font.clone(),
                                font_size: 15.,
                                ..default()
                            },
                        )],
                    ),
                    (
                        Text::new(task),
                        TextFont {
                            font: font.clone(),
                            font_size: 60.,
                            ..default()
                        },
                    ),
                ],
            ));
        }

        entity_world.with_child((
            Done,
            Visibility::Hidden,
            Text::new("Done."),
            TextFont {
                font: font.clone(),
                font_size: 80.,
                ..default()
            },
            observe(move |_: On<Pointer<Click>>, mut commands: Commands| {
                commands.entity(entity).despawn();
                info!("Done.");

                then(commands);
            }),
        ));
    });
}

#[derive(BundleEffect)]
pub struct CheckList<const LENGTH: usize>(pub [&'static str; LENGTH], pub fn(Commands));

impl<const LENGTH: usize> BundleEffect for CheckList<LENGTH> {
    fn effect(self, entity_world: &mut EntityWorldMut) {
        let entity = entity_world.id();

        #[derive(Component)]
        struct CheckedText;

        #[derive(Component)]
        struct Done;

        let font = entity_world
            .resource::<AssetServer>()
            .load("domine_regular.ttf");

        entity_world.insert(Node {
            width: percent(100),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            ..default()
        });

        for item in self.0 {
            entity_world.with_child((
                Node {
                    width: percent(100),
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Row,
                    ..default()
                },
                children![
                    (
                        observe(
                            |on: On<Pointer<Over>>, mut colour: Query<&mut BorderColor>| {
                                *colour.get_mut(on.entity).else_error()? = BorderColor::all(WHITE);
                            }
                        ),
                        observe(
                            |on: On<Pointer<Out>>, mut colour: Query<&mut BorderColor>| {
                                *colour.get_mut(on.entity).else_error()? = BorderColor::all(GREY);
                            }
                        ),
                        observe(
                            |on: On<Pointer<Click>>,
                             children: Query<&Children>,
                             mut text: Query<&mut Text, With<CheckedText>>, mut done: Single<&mut Visibility, With<Done>>| {
                                let children = children.get(on.entity).else_error()?;
                                let child =
                                    children.first().else_error()?;
                                let mut child = text.get_mut(*child).else_error()?;

                                if child.0 == "X" {
                                    child.0 = "".into();
                                } else {
                                    child.0 = "X".into();
                                }

                                for text in text {
                                    if text.0 != "X" {
                                        **done = Visibility::Hidden;
                                        return;
                                    }
                                }

                                info!("All ticked!");
                                **done = Visibility::Visible;
                            }
                        ),
                        Node {
                            width: px(30),
                            height: px(30),
                            border: px(2).all(),
                            border_radius: BorderRadius::all(px(3)),
                            align_self: AlignSelf::Center,
                            ..default()
                        },
                        BorderColor::all(GREY),
                        children![(
                            Node {
                                width: px(16),
                                height: px(16),
                                position_type: PositionType::Absolute,
                                left: px(4),
                                top: px(4),
                                ..default()
                            },
                            BackgroundColor(BLACK.into()),
                            CheckedText,
                            Text::new(""),
                            TextFont {
                                font: font.clone(),
                                font_size: 15.,
                                ..default()
                            },
                        )],
                    ),
                    (
                        Text::new(item),
                        TextFont {
                            font: font.clone(),
                            font_size: 60.,
                            ..default()
                        },
                    ),
                ],
            ));
        }

        entity_world.with_child((
            Done,
            Visibility::Hidden,
            Text::new("Done."),
            TextFont {
                font: font.clone(),
                font_size: 80.,
                ..default()
            },
            observe(move |_: On<Pointer<Click>>, mut commands: Commands| {
                commands.entity(entity).despawn();
                info!("Done.");

                (self.1)(commands);
            }),
        ));
    }
}
