use bevy::prelude::*;
use bevy::{
    input::gamepad::{GamepadEvent, GamepadEventType},
    sprite::MaterialMesh2dBundle,
};
use std::f32::consts::PI;

static BUTTON_BEAM_WIDTH: f32 = 50.0;
static SCROLL_SPEED: f32 = 200.0;

#[derive(Component)]
struct Moving;

#[derive(Component)]
struct Growing;

// 押した時刻を保存しておいて一定期間後に削除する
#[derive(Component)]
struct SdvxInput {
    kind: SdvxInputKind,
}

impl SdvxInput {
    fn new(kind: SdvxInputKind) -> Self {
        Self { kind }
    }
}

#[derive(PartialEq, Eq)]
enum SdvxInputKind {
    BTA,
    BTB,
    BTC,
    BTD,
    FXL,
    FXR,
    VolLL,
    VolLR,
    VolRL,
    VolRR,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(move_input)
        .add_system(grow_input)
        .add_system(gamepad_events)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn move_input(mut query: Query<&mut Transform, With<Moving>>, timer: Res<Time>) {
    for mut transform in &mut query {
        let delta = SCROLL_SPEED * timer.delta_seconds();
        transform.translation += Vec3::new(0.0, delta, 0.0);
    }
}

fn grow_input(mut query: Query<&mut Transform, With<Growing>>, timer: Res<Time>) {
    for mut transform in &mut query {
        let delta = SCROLL_SPEED * timer.delta_seconds();
        transform.scale += Vec3::new(0.0, delta, 0.0);
        transform.translation += Vec3::new(0.0, -delta / 2.0, 0.0);
    }
}

fn gamepad_events(
    mut gamepad_event: EventReader<GamepadEvent>,
    mut commands: Commands,
    query: Query<(Entity, &SdvxInput), With<Growing>>,
) {
    for event in gamepad_event.iter() {
        match event.event_type {
            GamepadEventType::Connected(_) => {
                info!("{:?} Connected", event.gamepad);
            }
            GamepadEventType::Disconnected => {
                info!("{:?} Disconnected", event.gamepad);
            }
            GamepadEventType::ButtonChanged(GamepadButtonType::West, value) if value == 1.0 => {
                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgb(1.0, 1.0, 1.0),
                            custom_size: Some(Vec2::new(BUTTON_BEAM_WIDTH, 1.0)),
                            ..default()
                        },
                        ..default()
                    },
                    Moving {},
                    Growing {},
                    SdvxInput::new(SdvxInputKind::BTA),
                ));
                info!("West of {:?} is changed to {}", event.gamepad, value);
            }
            GamepadEventType::ButtonChanged(GamepadButtonType::West, value) if value == 0.0 => {
                for (entity, input) in query.iter() {
                    if input.kind == SdvxInputKind::BTA {
                        commands.entity(entity).remove::<Growing>();
                    }
                }
            }
            GamepadEventType::AxisChanged(axis_type, value) => {
                info!(
                    "{:?} of {:?} is changed to {}",
                    axis_type, event.gamepad, value
                );
            }
            _ => {
                info!("{event:?}");
            }
        }
    }
}
