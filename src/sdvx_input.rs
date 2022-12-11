use std::collections::VecDeque;
use std::cmp::Ordering;

use bevy::prelude::*;

use crate::consts;

#[derive(Component)]
pub struct SdvxInput {
    pub kind: SdvxInputKind,
}

impl SdvxInput {
    pub const fn new(kind: SdvxInputKind) -> Self {
        Self { kind }
    }
}

#[derive(Component, PartialEq, Eq, Clone, Copy)]
pub enum SdvxInputKind {
    Button(ButtonKind),
    Vol(VolKind),
}

#[derive(Component, PartialEq, Eq, Clone, Copy)]
pub enum ButtonKind {
    BTA,
    BTB,
    BTC,
    BTD,
    FXL,
    FXR,
}

impl ButtonKind {
    pub const fn from_gamepad_button_type(button_type: &GamepadButtonType) -> Option<Self> {
        match button_type {
            GamepadButtonType::West => Some(Self::BTA),
            GamepadButtonType::South => Some(Self::BTB),
            GamepadButtonType::East => Some(Self::BTC),
            GamepadButtonType::North => Some(Self::BTD),
            GamepadButtonType::LeftTrigger => Some(Self::FXL),
            GamepadButtonType::RightTrigger => Some(Self::FXR),
            _ => None,
        }
    }
}

#[derive(Component, PartialEq, Eq, Clone, Copy)]
pub enum VolKind {
    Left(VolRotation),
    Right(VolRotation),
}

#[derive(Component, PartialEq, Eq, Clone, Copy)]
pub enum VolRotation {
    Left,
    Right,
}

#[derive(Resource, Default, Debug)]
pub struct BeamConfig;

impl BeamConfig {
    pub const fn color(kind: &SdvxInputKind) -> Color {
        match kind {
            SdvxInputKind::Button(ButtonKind::BTA | ButtonKind::BTB | ButtonKind::BTC | ButtonKind::BTD) => consts::BUTTON_COLOR,
            SdvxInputKind::Button(ButtonKind::FXL | ButtonKind::FXR) => consts::FX_COLOR,
            SdvxInputKind::Vol(VolKind::Left(_)) => consts::VOLL_COLOR,
            SdvxInputKind::Vol(VolKind::Right(_)) => consts::VOLR_COLOR,
        }
    }

    pub const fn pos(kind: &SdvxInputKind) -> Vec3 {
        match kind {
            SdvxInputKind::Button(ButtonKind::BTA) => consts::BTA_POS,
            SdvxInputKind::Button(ButtonKind::BTB) => consts::BTB_POS,
            SdvxInputKind::Button(ButtonKind::BTC) => consts::BTC_POS,
            SdvxInputKind::Button(ButtonKind::BTD) => consts::BTD_POS,
            SdvxInputKind::Button(ButtonKind::FXL) => consts::FXL_POS,
            SdvxInputKind::Button(ButtonKind::FXR) => consts::FXR_POS,
            SdvxInputKind::Vol(VolKind::Left(VolRotation::Left)) => consts::VOLLL_POS,
            SdvxInputKind::Vol(VolKind::Left(VolRotation::Right)) => consts::VOLLR_POS,
            SdvxInputKind::Vol(VolKind::Right(VolRotation::Left)) => consts::VOLRL_POS,
            SdvxInputKind::Vol(VolKind::Right(VolRotation::Right)) => consts::VOLRR_POS,
        }
    }

    pub const fn size(kind: &SdvxInputKind) -> Vec2 {
        match kind {
            SdvxInputKind::Button(ButtonKind::BTA | ButtonKind::BTB | ButtonKind::BTC | ButtonKind::BTD) => consts::BUTTON_SIZE,
            SdvxInputKind::Button(ButtonKind::FXL | ButtonKind::FXR) => consts::FX_SIZE,
            SdvxInputKind::Vol(_) => consts::VOL_SIZE,
        }
    }
}


#[derive(Resource, Default, Debug)]
pub struct VolState {
    left_values: VecDeque<f32>,
    right_values: VecDeque<f32>,
}

impl VolState {
    // Must be even
    const BUF_SIZE: usize = 6;

    pub fn push_left(&mut self, value: f32) {
        self.left_values.push_back(value);
        if self.left_values.len() > Self::BUF_SIZE {
            self.left_values.pop_front();
        }
    }

    pub fn push_right(&mut self, value: f32) {
        self.right_values.push_back(value);
        if self.right_values.len() > Self::BUF_SIZE {
            self.right_values.pop_front();
        }
    }

    pub fn get_left_vol_rotation(&self) -> Option<VolRotation> {
        if self.left_values.len() < Self::BUF_SIZE {
            return None;
        }

        let mut diffs = Vec::new();
        for i in 1..self.left_values.len() {
            let prev = self.left_values[i-1];
            let curr = self.left_values[i];
            if prev < curr || (prev == 1.0 && curr == -1.0) {
                diffs.push(1);
            } else {
                diffs.push(-1);
            }
        }
        match diffs.iter().sum::<i32>().cmp(&0) {
            Ordering::Greater => Some(VolRotation::Right),
            Ordering::Less => Some(VolRotation::Left),
            Ordering::Equal => panic!(),
        }
    }

    pub fn get_right_vol_rotation(&self) -> Option<VolRotation> {
        if self.right_values.len() < Self::BUF_SIZE {
            return None;
        }

        let mut diffs = Vec::new();
        for i in 1..self.right_values.len() {
            let prev = self.right_values[i-1];
            let curr = self.right_values[i];
            if prev < curr || (prev == 1.0 && curr == -1.0) {
                diffs.push(1);
            } else {
                diffs.push(-1);
            }
        }
        match diffs.iter().sum::<i32>().cmp(&0) {
            Ordering::Greater => Some(VolRotation::Left),
            Ordering::Less => Some(VolRotation::Right),
            Ordering::Equal => panic!(),
        }
    }

    pub fn clear_left(&mut self) {
        self.left_values.clear()
    }

    pub fn clear_right(&mut self) {
        self.right_values.clear()
    }
}
