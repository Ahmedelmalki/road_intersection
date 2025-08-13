use macroquad::prelude::*;

#[allow(deprecated)]
use ::rand::{ thread_rng, Rng };

pub const WINDOW_WIDTH: f32 = 1330.0;
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
            Direction::North => (WINDOW_WIDTH / 2.0 - LANE_WIDTH + 2.0, 0.0 - CAR_LENGTH), 
            Direction::South => (WINDOW_WIDTH / 2.0 + 2.0, WINDOW_HEIGHT + CAR_LENGTH + 2.0),
            Direction::East => (0.0 - CAR_LENGTH, WINDOW_HEIGHT / 2.0 - LANE_WIDTH),
            Direction::West => (WINDOW_WIDTH + CAR_LENGTH, WINDOW_HEIGHT / 2.0 +  2.0),
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

pub fn moving_cars(vehicles: &mut Vec<Vehicle>) {
    // Mise a jour position des voitures
    for v in vehicles {
        match v.direction {
            Direction::North => {
                v.y += v.velocity;
            }
            Direction::South => {
                v.y -= v.velocity;
            }
            Direction::East => {
                v.x += v.velocity;
            }
            Direction::West => {
                v.x -= v.velocity;
            }
        }
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
}

pub fn render_trafic_lights() {
    let square_size = 50.0;
    draw_rectangle(
        WINDOW_WIDTH / 2.0 - THICKNESS / 2.0 - square_size,
        WINDOW_HEIGHT / 2.0 - THICKNESS / 2.0 - square_size,
        square_size,
        square_size,
        RED
    );
    draw_rectangle(
        WINDOW_WIDTH / 2.0 + THICKNESS / 2.0,
        WINDOW_HEIGHT / 2.0 - THICKNESS / 2.0 - square_size,
        square_size,
        square_size,
        RED
    );
    draw_rectangle(
        WINDOW_WIDTH / 2.0 - THICKNESS / 2.0 - square_size,
        WINDOW_HEIGHT / 2.0 + THICKNESS / 2.0,
        square_size,
        square_size,
        RED
    );
    draw_rectangle(
        WINDOW_WIDTH / 2.0 + THICKNESS / 2.0,
        WINDOW_HEIGHT / 2.0 + THICKNESS / 2.0,
        square_size,
        square_size,
        RED
    );
}

// ################################ safety distence logic ######################################
pub fn add_car(car_vec: &mut Vec<Vehicle>, dir: Direction, route: Route) {
    let safety_distance: f32 = 60.0;

    // Check the last car in the same direction
    if
        let Some(last_car) = car_vec
            .iter()
            .rev()
            .find(|v| v.direction == dir)
    {
        println!("north  {} {}", last_car.y, screen_height() - safety_distance);
        println!("{}", last_car.y < screen_height() - safety_distance);
        let too_close = match dir {
            Direction::North => last_car.y > safety_distance,
            Direction::South => last_car.y < screen_height() - safety_distance,
            Direction::East => last_car.x > safety_distance,
            Direction::West => last_car.x < screen_width() - safety_distance,
        };
        if !too_close {
            return;
        }
    }

    // Create vehicle with proper spawn position
    let new_vehicle = Vehicle::new(dir, route);
    car_vec.push(new_vehicle);
}
