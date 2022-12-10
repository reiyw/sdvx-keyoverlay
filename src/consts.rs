use bevy::prelude::*;

// TODO: Put these in the config Resource.

pub const SCROLL_SPEED: f32 = 300.0;

pub const BUTTON_GAP: f32 = 2.0;
pub const BUTTON_BEAM_WIDTH: f32 = 50.0;
pub const VOL_BEAM_WIDTH: f32 = 40.0;

pub const WINDOW_WIDTH: f32 = BUTTON_BEAM_WIDTH * 4.0 + BUTTON_GAP * 5.0 + VOL_BEAM_WIDTH * 4.0;

pub const BUTTON_SIZE: Vec2 = Vec2::new(BUTTON_BEAM_WIDTH, 1.0);
pub const FX_SIZE: Vec2 = Vec2::new(BUTTON_BEAM_WIDTH * 2.0 + BUTTON_GAP, 1.0);
pub const VOL_SIZE: Vec2 = Vec2::new(VOL_BEAM_WIDTH, 1.0);

pub const BTA_POS: Vec3 = Vec3::new(-BUTTON_BEAM_WIDTH * 1.5 - BUTTON_GAP * 1.5, -200.0, 1.0);
pub const BTB_POS: Vec3 = Vec3::new(-BUTTON_BEAM_WIDTH * 0.5 - BUTTON_GAP * 0.5, -200.0, 1.0);
pub const BTC_POS: Vec3 = Vec3::new(BUTTON_BEAM_WIDTH * 0.5 + BUTTON_GAP * 0.5, -200.0, 1.0);
pub const BTD_POS: Vec3 = Vec3::new(BUTTON_BEAM_WIDTH * 1.5 + BUTTON_GAP * 1.5, -200.0, 1.0);

pub const FXL_POS: Vec3 = Vec3::new(-BUTTON_BEAM_WIDTH - BUTTON_GAP, -200.0, 0.0);
pub const FXR_POS: Vec3 = Vec3::new(BUTTON_BEAM_WIDTH + BUTTON_GAP, -200.0, 0.0);

pub const VOLLL_POS: Vec3 = Vec3::new(
    -BUTTON_BEAM_WIDTH * 2.0 - BUTTON_GAP * 2.5 - VOL_BEAM_WIDTH * 1.5,
    -200.0,
    0.0,
);
pub const VOLLR_POS: Vec3 = Vec3::new(
    -BUTTON_BEAM_WIDTH * 2.0 - BUTTON_GAP * 2.5 - VOL_BEAM_WIDTH * 0.5,
    -200.0,
    0.0,
);
pub const VOLRL_POS: Vec3 = Vec3::new(
    BUTTON_BEAM_WIDTH * 2.0 + BUTTON_GAP * 2.5 + VOL_BEAM_WIDTH * 0.5,
    -200.0,
    0.0,
);
pub const VOLRR_POS: Vec3 = Vec3::new(
    BUTTON_BEAM_WIDTH * 2.0 + BUTTON_GAP * 2.5 + VOL_BEAM_WIDTH * 1.5,
    -200.0,
    0.0,
);

pub const BUTTON_COLOR: Color = Color::WHITE;
pub const FX_COLOR: Color = Color::rgb(0.96, 0.56, 0.0);
pub const VOLL_COLOR: Color = Color::rgb(0.05, 0.71, 0.90);
pub const VOLR_COLOR: Color = Color::rgb(0.75, 0.25, 0.56);
