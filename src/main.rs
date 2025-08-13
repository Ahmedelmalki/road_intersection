use road_intersection::*;
use macroquad::prelude::*;
#[allow(deprecated)]
use ::rand::{ thread_rng, Rng };

#[macroquad::main(window_conf)]
async fn main() {
    let mut vehicles: Vec<Vehicle> = Vec::new();

    loop {
        clear_background(GRAY);

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
            let route = Route::Straight;
            vehicles.push(Vehicle::new(Direction::North, route));
            vehicles.push(Vehicle::new(dir, route));
        }

        // Mise a jour position des voitures
        for v in &mut vehicles {
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

        // Routes
        let screen_width = screen_width();
        let screen_height = screen_height();
        render_route(screen_width, screen_height);
        render_trafic_lights(); 

        // voitures
        for v in &vehicles {
            draw_rectangle(v.x, v.y, CAR_WIDTH, CAR_LENGTH, v.color);
        }

        next_frame().await;
    }
}

fn render_route(w: f32, h: f32) {
    draw_line(0.0, h / 2.0, w, h / 2.0, 120.0, BLACK);
    draw_line(0.0, h / 2.0, w, h / 2.0, 2.0, WHITE);
    draw_line(w / 2.0, 0.0, w / 2.0, h, 120.0, BLACK);
    draw_line(w / 2.0, 0.0, 500.0, 391.0, 2.0, WHITE);
    draw_line(501.0, 510.0, 501.0, 899.0, 2.0, WHITE);
}

fn render_trafic_lights() {
    let square_size = 50.0;
    draw_rectangle(561.0, 339.0, square_size, square_size, RED);
    draw_rectangle(389.0, 339.0, square_size, square_size, RED);
    draw_rectangle(560.0, 510.0, square_size, square_size, RED);
    draw_rectangle(389.0, 510.0, square_size, square_size, RED);
}
