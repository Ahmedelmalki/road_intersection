use macroquad::prelude::*;

#[allow(deprecated)]
use ::rand::{ thread_rng, Rng };

pub const WINDOW_WIDTH: f32 = 1337.0;
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
    Conf {
        window_title: "Road Intersection".to_owned(),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        fullscreen: false,
        ..Default::default()
    }
}

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
pub struct TrafficLightController {
    pub current_green_direction: Direction,
    pub timer: f32,
    pub green_duration: f32,
}

impl TrafficLightController {
    pub fn new() -> Self {
        Self {
            current_green_direction: Direction::North,
            timer: 0.0,
            green_duration: 5.0, // 3 seconds green
        }
    }

    // ********************** max capacity logic  ************************
    fn get_lane_capacity(&self) -> usize {
        let lane_length = WINDOW_WIDTH.min(WINDOW_HEIGHT) / 2.0 - THICKNESS / 2.0; // Distance from spawn to intersection
        let capacity = (lane_length / (CAR_LENGTH + SAFETY_GAP)).floor() as usize;
        capacity
    }

    fn count_waiting_cars(&self, vehicles: &[Vehicle], direction: Direction) -> usize {
        vehicles
            .iter()
            .filter(|v| v.direction == direction && self.is_approaching_intersection(v))
            .count()
    }

    fn is_approaching_intersection(&self, vehicle: &Vehicle) -> bool {
        let center_x = WINDOW_WIDTH / 2.0;
        let center_y = WINDOW_HEIGHT / 2.0;
        let boundary = THICKNESS / 2.0;

        match vehicle.direction {
            Direction::North => vehicle.y < center_y - boundary,
            Direction::South => vehicle.y > center_y + boundary,
            Direction::East => vehicle.x < center_x - boundary,
            Direction::West => vehicle.x > center_x + boundary,
        }
    }

    pub fn update(&mut self, delta_time: f32, vehicles: &[Vehicle]) {
        self.timer += delta_time;
        let capacity = self.get_lane_capacity();
        let current_queue = self.count_waiting_cars(vehicles, self.current_green_direction);

        // Extend green time if lane is at/near capacity
        let mut effective_duration = self.green_duration;
        if current_queue >= capacity {
            effective_duration += 2.0; // Add 2 extra seconds
        }
        if self.timer >= effective_duration {
            self.timer = 0.0;
            // Cycle through directions: North -> East -> South -> West
            self.current_green_direction = match self.current_green_direction {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            };
        }
    }

    pub fn is_green(&self, direction: Direction) -> bool {
        self.current_green_direction == direction
    }
}

pub fn render_traffic_lights(controller: &TrafficLightController) {
    let s = 50.0; // square size

    // Define the 4 traffic light positions with their corresponding directions
    let lights = [
        (
            Direction::North,
            WINDOW_WIDTH / 2.0 - THICKNESS / 2.0 - s,
            WINDOW_HEIGHT / 2.0 - THICKNESS / 2.0 - s,
        ),
        (
            Direction::West,
            WINDOW_WIDTH / 2.0 + THICKNESS / 2.0,
            WINDOW_HEIGHT / 2.0 - THICKNESS / 2.0 - s,
        ),
        (
            Direction::South,
            WINDOW_WIDTH / 2.0 + THICKNESS / 2.0,
            WINDOW_HEIGHT / 2.0 + THICKNESS / 2.0,
        ),
        (
            Direction::East,
            WINDOW_WIDTH / 2.0 - THICKNESS / 2.0 - s,
            WINDOW_HEIGHT / 2.0 + THICKNESS / 2.0,
        ),
    ];

    // Draw each traffic light with the appropriate color
    for (direction, x, y) in lights {
        let color = if controller.is_green(direction) { GREEN } else { RED };
        draw_rectangle(x, y, s, s, color);
    }
}
