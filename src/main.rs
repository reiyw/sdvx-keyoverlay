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
                transparent: false,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup)
        .add_system(move_input)
        .add_system(despawn_beam)
        .add_system(grow_input)
        .add_system(track_kps)
        .add_system(stop_growing_vol_beam)
        .add_system(gamepad_events)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let text_style = TextStyle {
        font: asset_server.load("C:/Windows/Fonts/arial.ttf"),
        font_size: 20.0,
        color: Color::WHITE,
    };
    commands.spawn((
        TextBundle::from_sections([
            TextSection::new("KPS: ", text_style.clone()),
            TextSection::from_style(text_style),
        ]),
        KpsText,
    ));
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

#[derive(Component)]
struct KpsText;

#[derive(Component, Default)]
struct MetricsTrackingTarget {
    time: Stopwatch,
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

fn track_kps(
    mut commands: Commands,
    mut target_query: Query<(Entity, &mut MetricsTrackingTarget)>,
    mut text_query: Query<&mut Text, With<KpsText>>,
    timer: Res<Time>,
) {
    let mut count = 0;
    for (entity, mut target) in &mut target_query {
        target.time.tick(timer.delta());
        if target.time.elapsed_secs() <= 1.0 {
            count += 1;
        } else {
            commands.entity(entity).remove::<MetricsTrackingTarget>();
        }
    }
    for mut text in &mut text_query {
        text.sections[1].value = count.to_string();
    }
}

fn stop_growing_vol_beam(
    mut commands: Commands,
    query: Query<(Entity, &VolKind, &Growing)>,
    mut vol_state: ResMut<VolState>,
) {
    let left_overlapping = query
        .iter()
        .any(|(_, kind, _)| matches!(kind, VolKind::Left(VolRotation::Left)))
        && query
            .iter()
            .any(|(_, kind, _)| matches!(kind, VolKind::Left(VolRotation::Right)));
    let right_overlapping = query
        .iter()
        .any(|(_, kind, _)| matches!(kind, VolKind::Right(VolRotation::Left)))
        && query
            .iter()
            .any(|(_, kind, _)| matches!(kind, VolKind::Right(VolRotation::Right)));

    for (entity, _, beam) in query.iter() {
        if beam.time.elapsed_secs() > 0.05 {
            commands.entity(entity).remove::<Growing>();

            if !left_overlapping {
                vol_state.clear_left();
            }
            if !right_overlapping {
                vol_state.clear_right();
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
    let mut cmd = commands.spawn((
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
    if let SdvxInputKind::Button(_) = kind {
        cmd.insert(MetricsTrackingTarget::default());
    }
    if let SdvxInputKind::Vol(kind) = kind {
        cmd.insert(kind);
    }
}
