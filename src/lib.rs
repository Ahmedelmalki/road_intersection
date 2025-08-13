use macroquad::prelude::*;

#[allow(deprecated)]
use ::rand::{thread_rng, Rng};

pub const WINDOW_WIDTH: f32 = 1000.0;
pub const WINDOW_HEIGHT: f32 = 900.0;

pub const LANE_WIDTH: f32 = 60.0;
pub const CAR_LENGTH: f32 = 40.0;
pub const CAR_WIDTH: f32 = 20.0;
pub const SAFETY_GAP: f32 = 20.0;
pub const CAR_VELOCITY: f32 = 1.5;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Route { // turns
    Straight,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vehicle {
   pub x: f32,
   pub y: f32,
   pub direction: Direction,
   pub route: Route,
   pub velocity: f32,
   pub color: Color,
}

impl Vehicle {
    pub fn new(direction: Direction, route: Route) -> Self {
        let (x, y) = match direction {
            Direction::North => (WINDOW_WIDTH / 2.0 + (LANE_WIDTH / 4.0), 0.0 - CAR_LENGTH), // x and y of starting point
            Direction::South => (WINDOW_WIDTH / 2.0 - (LANE_WIDTH / 4.0), WINDOW_HEIGHT + CAR_LENGTH),
            Direction::East => (0.0 - CAR_LENGTH, WINDOW_HEIGHT / 2.0 - (LANE_WIDTH / 4.0)),
            Direction::West => (WINDOW_WIDTH + CAR_LENGTH, WINDOW_HEIGHT / 2.0 + (LANE_WIDTH / 4.0)),
        };
        #[allow(deprecated)]
        let color = match thread_rng().gen_range(0..3) {
            0 => RED,
            1 => GREEN,
            _ => BLUE,
        };

        Vehicle {
            x,
            y,
            direction,
            route,
            velocity: CAR_VELOCITY,
            color,
        }
    }
}

// #[derive(Copy, Clone, Debug, PartialEq)]
// struct TrafficLight {
//     x: f32,
//     y: f32,
//     state: Color,
//     direction: Direction,
// }

// impl TrafficLight {
//     fn new(direction: Direction) -> Self {
//         let (x, y) = match direction {
//             Direction::North => (WINDOW_WIDTH / 2.0 + (LANE_WIDTH / 2.0), WINDOW_HEIGHT / 2.0 + (LANE_WIDTH / 2.0)),
//             Direction::South => (WINDOW_WIDTH / 2.0 - (LANE_WIDTH / 2.0), WINDOW_HEIGHT / 2.0 - (LANE_WIDTH / 2.0)),
//             Direction::East => (WINDOW_WIDTH / 2.0 + (LANE_WIDTH / 2.0), WINDOW_HEIGHT / 2.0 - (LANE_WIDTH / 2.0)),
//             Direction::West => (WINDOW_WIDTH / 2.0 - (LANE_WIDTH / 2.0), WINDOW_HEIGHT / 2.0 + (LANE_WIDTH / 2.0)),
//         };

//         TrafficLight {
//             x: x - TRAFFIC_LIGHT_SIZE,
//             y: y - TRAFFIC_LIGHT_SIZE,
//             state: RED,
//             direction,
//         }
//     }
// }


pub fn window_conf() -> Conf { // short for config
    Conf {
        window_title: "Road Intersection".to_owned(),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        fullscreen: false,
        ..Default::default()
    }
}