use macroquad::prelude::*;

use ::rand::{thread_rng, Rng};



const WINDOW_WIDTH: f32 = 1000.0;
const WINDOW_HEIGHT: f32 = 900.0;

const LANE_WIDTH: f32 = 60.0;
const CAR_LENGTH: f32 = 40.0;
const CAR_WIDTH: f32 = 20.0;
const SAFETY_GAP: f32 = 20.0;
const CAR_VELOCITY: f32 = 1.5;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Route {
    Straight,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Vehicle {
    x: f32,
    y: f32,
    direction: Direction,
    route: Route,
    velocity: f32,
    color: Color,
}

impl Vehicle {
    fn new(direction: Direction, route: Route) -> Self {
        let (x, y) = match direction {
            Direction::North => (WINDOW_WIDTH / 2.0 + (LANE_WIDTH / 4.0), 0.0 - CAR_LENGTH),
            Direction::South => (WINDOW_WIDTH / 2.0 - (LANE_WIDTH / 4.0), WINDOW_HEIGHT + CAR_LENGTH),
            Direction::East => (0.0 - CAR_LENGTH, WINDOW_HEIGHT / 2.0 - (LANE_WIDTH / 4.0)),
            Direction::West => (WINDOW_WIDTH + CAR_LENGTH, WINDOW_HEIGHT / 2.0 + (LANE_WIDTH / 4.0)),
        };
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


fn window_conf() -> Conf {
    Conf {
        window_title: "Road Intersection".to_owned(),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut vehicles: Vec<Vehicle> = Vec::new();
    
    loop {
        clear_background(GRAY);

        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        
        let mut direction_to_spawn = None;
        if is_key_pressed(KeyCode::Up) { direction_to_spawn = Some(Direction::South); }
        if is_key_pressed(KeyCode::Down) { direction_to_spawn = Some(Direction::North); }
        if is_key_pressed(KeyCode::Right) { direction_to_spawn = Some(Direction::West); }
        if is_key_pressed(KeyCode::Left) { direction_to_spawn = Some(Direction::East); }
        if is_key_pressed(KeyCode::R) {
            let mut rng = thread_rng();
            let random_dir = match rng.gen_range(0..4) {
                0 => Direction::North,
                1 => Direction::South,
                2 => Direction::East,
                _ => Direction::West,
            };
            direction_to_spawn = Some(random_dir);
        }

        
        if let Some(dir) = direction_to_spawn {
            let route = Route::Straight; 
            vehicles.push(Vehicle::new(dir, route));
        }

        // Mise a jour position des voitures
        for v in &mut vehicles {
            match v.direction {
                Direction::North => v.y -= v.velocity,
                Direction::South => v.y += v.velocity,
                Direction::East  => v.x += v.velocity,
                Direction::West  => v.x -= v.velocity,
            }
        }

        // Routes
        let screen_width = screen_width();
        let screen_height = screen_height();
        
        draw_line(0.0, screen_height / 2.0, screen_width, screen_height / 2.0, 120.0, BLACK);
        draw_line(0.0, screen_height / 2.0, screen_width, screen_height / 2.0, 2.0, WHITE);
        draw_line(screen_width / 2.0, 0.0, screen_width / 2.0, screen_height, 120.0, BLACK);
        draw_line(screen_width / 2.0, 0.0, 500.0, 391.0, 2.0, WHITE);
        draw_line(501.0, 510.0, 501.0, 899.0, 2.0, WHITE);

        let square_size = 50.0;
        draw_rectangle(561.0, 339.0, square_size, square_size, RED);
        draw_rectangle(389.0, 339.0, square_size, square_size, RED);
        draw_rectangle(560.0, 510.0, square_size, square_size, RED);
        draw_rectangle(389.0, 510.0, square_size, square_size, RED);

        // voitures
        for v in &vehicles {
            draw_rectangle(v.x, v.y, CAR_WIDTH, CAR_LENGTH, v.color);
        }

        next_frame().await
    }
}
