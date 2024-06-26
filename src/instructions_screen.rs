use bevy::prelude::*;

use crate::game_state::GameState;

#[derive(Component)]
struct InstructionsScreenComponent;

#[derive(Component)]
struct PressAnyKeyText;

pub struct InstructionsScreenPlugin;
impl Plugin for InstructionsScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(OnExit(GameState::Instructions), despawn)
            .add_systems(Update, blink.run_if(in_state(GameState::Instructions)));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut press_any_key_text = TextBundle::from_section(
        "PRESS ANY KEY",
        TextStyle {
            font: asset_server.load("PixelifySans-Medium.ttf"),
            font_size: 32.0,
            ..default()
        },
    );
    press_any_key_text.visibility = Visibility::Visible;

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    height: Val::Percent(100.0),
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            InstructionsScreenComponent,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "MOVE WITH",
                TextStyle {
                    font: asset_server.load("PixelifySans-Medium.ttf"),
                    font_size: 32.0,
                    ..default()
                },
            ));

            parent.spawn(NodeBundle {
                style: Style {
                    height: Val::Px(142.0),
                    width: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            });

            parent.spawn((press_any_key_text, PressAnyKeyText));
        });

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("instructions.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(192., 128.)),
                ..Default::default()
            },
            ..Default::default()
        },
        InstructionsScreenComponent,
    ));
}

fn despawn(
    instructions_query: Query<Entity, With<InstructionsScreenComponent>>,
    mut commands: Commands,
) {
    for entity in instructions_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn blink(
    mut text_query: Query<&mut Visibility, With<PressAnyKeyText>>,
    time: Res<Time>,
    mut elapsed_time: Local<f32>,
) {
    *elapsed_time += time.delta_seconds();

    let mut text_vis = text_query.single_mut();
    if *elapsed_time > 1.0 {
        if *text_vis == Visibility::Hidden {
            *text_vis = Visibility::Visible;
        } else if *text_vis == Visibility::Visible {
            *text_vis = Visibility::Hidden;
        }

        *elapsed_time = 0.;
    }
}
