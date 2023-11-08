use std::io::Write;

use bevy::prelude::*;
type Position = (f32, f32);
static mut UP: Position = (0., -100.);
static mut RIGHT: Position = (400., -250.);
static mut DOWN: Position = (0., -400.);
static mut LEFT: Position = (-400., -250.);
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
    // let mut bar_iter = bar.iter_mut();
    // let mut up = bar_iter.next().unwrap();
    // let mut right = bar_iter.next().unwrap();
    // let mut down = bar_iter.next().unwrap();
    // let mut left = bar_iter.next().unwrap();
    for (mut bar_transform, bar) in bar.iter_mut() {
        std::io::stdout().flush().unwrap();
        let pos = match bar.t {
            BarType::Up => unsafe { (UP, LEFT.0 - RIGHT.0, 5.) },
            BarType::Right => unsafe { (RIGHT, 5., UP.1 - DOWN.1) },
            BarType::Down => unsafe { (DOWN, LEFT.0 - RIGHT.0, 5.) },
            BarType::Left => unsafe { (LEFT, 5., UP.1 - DOWN.1) },
        };

        bar_transform.translation.x += (pos.0 .0 - bar_transform.translation.x) / 30.;
        bar_transform.translation.y += (pos.0 .1 - bar_transform.translation.y) / 30.;

        bar_transform.scale.x = f32::abs(pos.1);
        bar_transform.scale.y = f32::abs(pos.2);
    }
}

pub fn change_size(u: f32, r: f32, d: f32, l: f32) {
    unsafe {
        UP.1 += u / 2.;
        RIGHT.0 += r / 2.;
        DOWN.1 -= d / 2.;
        LEFT.0 -= l / 2.;

        UP.0 = (LEFT.0 + RIGHT.0) / 2.;
        RIGHT.1 = (UP.1 + DOWN.1) / 2.;
        DOWN.0 = (LEFT.0 + RIGHT.0) / 2.;
        LEFT.1 = (UP.1 + DOWN.1) / 2.;
    }
}

pub fn change_pos(x: f32, y: f32) {
    change_size(y, x, -y, -x);
}

pub fn set_size(u: f32, r: f32, d: f32, l: f32) {
    unsafe {
        UP.1 = u;
        RIGHT.0 = r;
        DOWN.1 = d;
        LEFT.0 = l;
    }
}
