#![windows_subsystem = "windows"]

use bevy::input::gamepad::{GamepadEvent, GamepadEventType};
use bevy::prelude::*;
use bevy::time::Stopwatch;
use sdvx_keyoverlay::*;

fn main() {
    App::new()
        .init_resource::<VolState>()
        .init_resource::<BeamConfig>()
        .insert_resource(ClearColor(Color::NONE))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "sdvx-keyoverlay".to_string(),
                width: WINDOW_WIDTH,
                height: 400.0,
                // transparent: true,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup)
        .add_system(move_input)
        .add_system(despawn_beam)
        .add_system(grow_input)
        .add_system(stop_growing_vol_beam)
        .add_system(gamepad_events)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Component)]
struct Moving {
    time: Stopwatch,
}

impl Moving {
    fn new() -> Self {
        Self {
            time: Stopwatch::new(),
        }
    }
}

#[derive(Component)]
struct Growing {
    time: Stopwatch,
}

impl Growing {
    fn new() -> Self {
        Self {
            time: Stopwatch::new(),
        }
    }
}

fn move_input(mut query: Query<&mut Transform, With<Moving>>, timer: Res<Time>) {
    for mut transform in &mut query {
        let delta = SCROLL_SPEED * timer.delta_seconds();
        transform.translation += Vec3::new(0.0, delta, 0.0);
    }
}

fn despawn_beam(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Moving), Without<Growing>>,
    timer: Res<Time>,
) {
    for (entity, mut beam) in &mut query {
        beam.time.tick(timer.delta());
        if beam.time.elapsed_secs() > 3.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn grow_input(mut query: Query<(&mut Transform, &mut Growing)>, timer: Res<Time>) {
    for (mut transform, mut beam) in &mut query {
        beam.time.tick(timer.delta());

        let delta = SCROLL_SPEED * timer.delta_seconds();
        transform.scale += Vec3::new(0.0, delta, 0.0);
        transform.translation += Vec3::new(0.0, -delta / 2.0, 0.0);
    }
}

fn stop_growing_vol_beam(
    mut commands: Commands,
    query: Query<(Entity, &SdvxInput, &Growing)>,
    mut vol_state: ResMut<VolState>,
) {
    for (entity, input, beam) in query.iter() {
        if let SdvxInputKind::Vol(ref vol) = input.kind {
            if beam.time.elapsed_secs() > 0.05 {
                commands.entity(entity).remove::<Growing>();

                match vol {
                    VolKind::Left(_) => vol_state.clear_left(),
                    VolKind::Right(_) => vol_state.clear_right(),
                }
            }
        }
    }
}

fn gamepad_events(
    mut gamepad_event: EventReader<GamepadEvent>,
    mut commands: Commands,
    mut query: Query<(Entity, &SdvxInput, &mut Growing)>,
    mut vol_state: ResMut<VolState>,
    beam_config: Res<BeamConfig>,
) {
    for event in gamepad_event.iter() {
        match event.event_type {
            GamepadEventType::ButtonChanged(button_type, value) => {
                if let Some(kind) = ButtonKind::from_gamepad_button_type(&button_type) {
                    if value == 1.0 {
                        // Button pressed
                        spawn_beam(SdvxInputKind::Button(kind), &mut commands, &beam_config);
                    } else if value == 0.0 {
                        // Button released
                        for (entity, input, _) in query.iter() {
                            if input.kind == SdvxInputKind::Button(kind) {
                                commands.entity(entity).remove::<Growing>();
                            }
                        }
                    }
                }
            }
            GamepadEventType::AxisChanged(GamepadAxisType::LeftStickX, value) => {
                vol_state.push_left(value);
                if let Some(rot) = vol_state.get_left_vol_rotation() {
                    if let Some((_, _, mut beam)) = query.iter_mut().find(|(_, input, _)| {
                        if let SdvxInputKind::Vol(VolKind::Left(prev_rot)) = &input.kind {
                            prev_rot == &rot
                        } else {
                            false
                        }
                    }) {
                        beam.time.reset();
                    } else {
                        spawn_beam(
                            SdvxInputKind::Vol(VolKind::Left(rot)),
                            &mut commands,
                            &beam_config,
                        );
                    }
                }
            }
            GamepadEventType::AxisChanged(GamepadAxisType::LeftStickY, value) => {
                vol_state.push_right(value);
                if let Some(rot) = vol_state.get_right_vol_rotation() {
                    if let Some((_, _, mut beam)) = query.iter_mut().find(|(_, input, _)| {
                        if let SdvxInputKind::Vol(VolKind::Right(prev_rot)) = &input.kind {
                            prev_rot == &rot
                        } else {
                            false
                        }
                    }) {
                        beam.time.reset();
                    } else {
                        spawn_beam(
                            SdvxInputKind::Vol(VolKind::Right(rot)),
                            &mut commands,
                            &beam_config,
                        );
                    }
                }
            }
            _ => {}
        }
    }
}

fn spawn_beam(kind: SdvxInputKind, commands: &mut Commands, _beam_config: &Res<BeamConfig>) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: BeamConfig::color(&kind),
                custom_size: Some(BeamConfig::size(&kind)),
                ..default()
            },
            transform: Transform::from_translation(BeamConfig::pos(&kind)),
            ..default()
        },
        Moving::new(),
        Growing::new(),
        SdvxInput::new(kind),
    ));
}
