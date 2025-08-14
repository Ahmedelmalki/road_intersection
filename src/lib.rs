use macroquad::prelude::*;

#[allow(deprecated)]
use ::rand::{ thread_rng, Rng };

pub const WINDOW_WIDTH: f32 = 600.0;
pub const WINDOW_HEIGHT: f32 = 600.0;
pub const THICKNESS: f32 = 100.0;

pub const LANE_WIDTH: f32 = THICKNESS / 2.0;
pub const CAR_LENGTH: f32 = 40.0;
pub const CAR_WIDTH: f32 = 40.0;
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
pub enum Route {
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
    pub fn new(direction: Direction) -> Self {
        let (x, y) = match direction {
            Direction::North => (WINDOW_WIDTH / 2.0 - LANE_WIDTH + 2.0, 0.0 - CAR_LENGTH),
            Direction::South => (WINDOW_WIDTH / 2.0 + 2.0, WINDOW_HEIGHT + CAR_LENGTH + 2.0),
            Direction::East => (0.0 - CAR_LENGTH, WINDOW_HEIGHT / 2.0 + 2.0),
            Direction::West => (WINDOW_WIDTH + CAR_LENGTH, WINDOW_HEIGHT / 2.0 - LANE_WIDTH),
        };
        #[allow(deprecated)]
        let color = match thread_rng().gen_range(0..3) {
            0 => ORANGE,
            1 => GREEN,
            _ => BLUE,
        };
        let route = match color {
            ORANGE => Route::Left,
            GREEN => Route::Right,
            _ => Route::Straight,
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

pub fn window_conf() -> Conf {
    // short for config
    Conf {
        window_title: "Road Intersection".to_owned(),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        fullscreen: false,
        ..Default::default()
    }
}

// Note: if u moved the caller of this fn to this file remove "pub"
pub fn render_route(w: f32, h: f32) {
    // rendering the outer rects
    draw_line(0.0, h / 2.0, w, h / 2.0, THICKNESS, BLACK);
    draw_line(w / 2.0, 0.0, w / 2.0, h, THICKNESS, BLACK);
    // rendering the inner lane separater
    draw_line(w / 2.0, 0.0, w / 2.0, h, 2.0, WHITE);
    draw_line(0.0, h / 2.0, w, h / 2.0, 2.0, WHITE);
    draw_rectangle(
        // turning point
        WINDOW_WIDTH / 2.0 - THICKNESS / 2.0,
        WINDOW_HEIGHT / 2.0 - THICKNESS / 2.0,
        THICKNESS,
        THICKNESS,
        BLACK
    );
}
// ############################ Traffic lights logic ##################################

use std::thread;
use std::time::Duration;
pub struct TrafficLight {
    x: f32,
    y: f32,
    id: u8,
    color: Color,
    state: State,
}

pub enum State {
    ON,
    OFF,
}

impl TrafficLight {
    fn new(x: f32, y: f32, id: u8) -> Self {
        Self { x, y, id, color: RED, state: State::OFF }
    }
    // capacity = floor(lane_length / (vehicle_length + safety_gap))
    // The primary function of your traffic light system is to avoid collisions between vehicles passing through the intersection, while dynamically adapting to congestion.
    pub fn controler(lights: &mut Vec<TrafficLight>) {
        for light in lights {
            light.color = GREEN;
            light.state = State::ON;
            thread::sleep(Duration::from_secs(10));
        }
    }
}

pub fn render_trafic_lights() {
    let s = 50.0; // square size
    let top_left = TrafficLight::new(
        WINDOW_WIDTH / 2.0 - THICKNESS / 2.0 - s,
        WINDOW_HEIGHT / 2.0 - THICKNESS / 2.0 - s,
        1
    );

    let top_right = TrafficLight::new(
        WINDOW_WIDTH / 2.0 + THICKNESS / 2.0,
        WINDOW_HEIGHT / 2.0 - THICKNESS / 2.0 - s,
        2
    );
    let bottom_right = TrafficLight::new(
        WINDOW_WIDTH / 2.0 + THICKNESS / 2.0,
        WINDOW_HEIGHT / 2.0 + THICKNESS / 2.0,
        3
    );
    let bottom_left = TrafficLight::new(
        WINDOW_WIDTH / 2.0 - THICKNESS / 2.0 - s,
        WINDOW_HEIGHT / 2.0 + THICKNESS / 2.0,
        4
    );
    draw_rectangle(top_left.x, top_left.y, s, s, RED);
    draw_rectangle(top_right.x, top_right.y, s, s, RED);
    draw_rectangle(bottom_left.x, bottom_left.y, s, s, RED);
    draw_rectangle(bottom_right.x, bottom_right.y, s, s, RED);
   
}

