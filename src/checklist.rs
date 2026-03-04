use bevy::{color::palettes::css::{BLACK, GREY, WHITE}, prelude::*, ui_widgets::observe};
use duck_back::Else;
use crate::bundle::BundleEffect;

//trace_macros!(true);
#[derive(BundleEffect)]
pub struct CheckList<const TESTER: bool, T>(pub T);

// BundleEffect!
// (|2: Implementation|
//     self_type(CheckList <TESTER, T,>),
//     impl_generics(<const TESTER: bool, T: Send + Sync + 'static,>),
//     component_ids_return_type(impl Iterator <Item = ::bevy::ecs::component::ComponentId> + use<TESTER, T>),
// );
//trace_macros!(false);

impl<const TESTER: bool, T> BundleEffect for CheckList<TESTER, T> {
    fn effect(self, entity_world: &mut EntityWorldMut) {
        let entity = entity_world.id();

        #[derive(Component)]
        struct CheckedText;

        #[derive(Component)]
        struct Done;

        let list = ["Get dressed.", "Make bed.", "Brush hair."];
        let font = entity_world
            .resource::<AssetServer>()
            .load("domine_regular.ttf");

        entity_world.insert(Node {
            width: percent(100),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            ..default()
        });

        for item in list {
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
                //commands.run_system_cached(self.0);
                info!("Done.");
            }),
        ));
    }
}