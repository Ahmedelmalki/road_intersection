use road_intersection::*;
use macroquad::prelude::*;
mod moving_cars;
use moving_cars::*;

#[allow(deprecated)]
use ::rand::{ thread_rng, Rng };

#[macroquad::main(window_conf)]
async fn main() {
    let mut vehicles: Vec<Vehicle> = Vec::new();
    let mut traffic_controller = TrafficLightController::new();

    loop {
        clear_background(GRAY);
        clear_cars(&mut vehicles);
        let delta_time = get_frame_time();
        traffic_controller.update(delta_time, &vehicles);
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        let mut direction_to_spawn = None;
        if is_key_pressed(KeyCode::Up) {
            direction_to_spawn = Some(Direction::South);
        }
        if is_key_pressed(KeyCode::Down) {
            direction_to_spawn = Some(Direction::North);
        }
        if is_key_pressed(KeyCode::Right) {
            direction_to_spawn = Some(Direction::West);
        }
        if is_key_pressed(KeyCode::Left) {
            direction_to_spawn = Some(Direction::East);
        }
        if is_key_pressed(KeyCode::R) {
            #[allow(deprecated)]
            let mut rng = thread_rng();
            #[allow(deprecated)]
            let random_dir = match rng.gen_range(0..4) {
                0 => Direction::North,
                1 => Direction::South,
                2 => Direction::East,
                _ => Direction::West,
            };
            direction_to_spawn = Some(random_dir);
        }

        if let Some(dir) = direction_to_spawn {
            add_car(&mut vehicles, dir);
        }

        moving_cars(&mut vehicles, &traffic_controller);

        // Routes
        let screen_width = screen_width();
        let screen_height = screen_height();
        render_route(screen_width, screen_height);
        render_traffic_lights(&traffic_controller);

        // voitures
        for v in &vehicles {
            draw_rectangle(v.x, v.y, CAR_WIDTH, CAR_LENGTH, v.color);
        }

        next_frame().await;
    }
}
