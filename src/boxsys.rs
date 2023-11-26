use std::io::Write;

use bevy::prelude::*;
type Position = (f32, f32);

static mut _UP: Position = (0., -100.);
static mut _RIGHT: Position = (400., -250.);
static mut _DOWN: Position = (0., -400.);
static mut _LEFT: Position = (-400., -250.);

pub static mut UP: Position = (0., 0.);
pub static mut RIGHT: Position = (0., 0.);
pub static mut DOWN: Position = (0., 0.);
pub static mut LEFT: Position = (0., 0.);
pub struct MainPlugin;
#[derive(Component)]
pub struct Bar {
    pub t: BarType,
}

pub enum BarType {
    Up,
    Right,
    Down,
    Left,
}

impl Plugin for MainPlugin {
    fn build(&self, app: &mut App) {
        println!("running Box System");
        app.add_systems(Update, box_movement);
    }
}

fn box_movement(mut bar: Query<(&mut Transform, &Bar), With<Bar>>) {
    // change_size(0., -0.5, 0., -0.5);
    let mut bar_iter = bar.iter_mut();
    let (up, _) = bar_iter.next().unwrap();
    let (right, _) = bar_iter.next().unwrap();
    let (down, _) = bar_iter.next().unwrap();
    let (left, _) = bar_iter.next().unwrap();

    unsafe {
        UP = (up.translation.x, up.translation.y);
        RIGHT = (right.translation.x, right.translation.y);
        DOWN = (down.translation.x, down.translation.y);
        LEFT = (left.translation.x, left.translation.y);
    }

    for (mut bar_transform, bar) in bar.iter_mut() {
        std::io::stdout().flush().unwrap();
        let pos = match bar.t {
            BarType::Up => unsafe { (_UP, _LEFT.0 - _RIGHT.0, 5.) },
            BarType::Right => unsafe { (_RIGHT, 5., _UP.1 - _DOWN.1) },
            BarType::Down => unsafe { (_DOWN, _LEFT.0 - _RIGHT.0, 5.) },
            BarType::Left => unsafe { (_LEFT, 5., _UP.1 - _DOWN.1) },
        };

        bar_transform.translation.x += (pos.0 .0 - bar_transform.translation.x) / 30.;
        bar_transform.translation.y += (pos.0 .1 - bar_transform.translation.y) / 30.;

        bar_transform.scale.x = f32::abs(pos.1);
        bar_transform.scale.y = f32::abs(pos.2);
    }
}

pub fn change_size(u: f32, r: f32, d: f32, l: f32) {
    unsafe {
        _UP.1 += u / 2.;
        _RIGHT.0 += r / 2.;
        _DOWN.1 -= d / 2.;
        _LEFT.0 -= l / 2.;

        _UP.0 = (_LEFT.0 + _RIGHT.0) / 2.;
        _RIGHT.1 = (_UP.1 + _DOWN.1) / 2.;
        _DOWN.0 = (_LEFT.0 + _RIGHT.0) / 2.;
        _LEFT.1 = (_UP.1 + _DOWN.1) / 2.;
    }
}

pub fn change_pos(x: f32, y: f32) {
    change_size(y, x, -y, -x);
}

pub fn set_size(u: f32, r: f32, d: f32, l: f32) {
    unsafe {
        _UP.1 = u;
        _RIGHT.0 = r;
        _DOWN.1 = d;
        _LEFT.0 = l;
    }
}
