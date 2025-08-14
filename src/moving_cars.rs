// ############################## car turns logic ####################################
use road_intersection::*;
use macroquad::prelude::*;

pub fn moving_cars(vehicles: &mut Vec<Vehicle>) {
    // Mise a jour position des voitures
    for v in vehicles {
        match v.direction {
            Direction::North => {
                if v.route == Route::Left && (v.y - WINDOW_HEIGHT / 2.0).abs() < v.velocity {
                    v.x += v.velocity;
                } else if
                    v.route == Route::Right &&
                    (v.y - (WINDOW_HEIGHT / 2.0 - LANE_WIDTH)).abs() < v.velocity
                {
                    v.x -= v.velocity;
                } else {
                    v.y += v.velocity; // keep going
                }
            }
            Direction::South => {
                if
                    v.route == Route::Left &&
                    (v.y - (WINDOW_HEIGHT / 2.0 - CAR_LENGTH)).abs() < v.velocity // to the west
                {
                    v.x -= v.velocity;
                } else if
                    v.route == Route::Right &&
                    (v.y - WINDOW_HEIGHT / 2.0).abs() < v.velocity // to the east
                {
                    v.x += v.velocity;
                } else {
                    v.y -= v.velocity;
                }
            }
            Direction::East => {
                if
                    v.route == Route::Left &&
                    (v.x - WINDOW_WIDTH / 2.0).abs() < v.velocity // to the south
                {
                    v.y -= v.velocity;
                } else if
                    v.route == Route::Right &&
                    (v.x - (WINDOW_WIDTH / 2.0 - CAR_LENGTH)).abs() < v.velocity // to the north
                {
                    v.y += v.velocity;
                } else {
                    v.x += v.velocity;
                }
            }
            Direction::West => {
                if
                    v.route == Route::Left &&
                    (v.x - (WINDOW_WIDTH / 2.0 - CAR_LENGTH)).abs() < v.velocity // to the north
                {
                    v.y += v.velocity;
                } else if
                    v.route == Route::Right &&
                    (v.x - WINDOW_WIDTH / 2.0).abs() < v.velocity // to the south
                {
                    v.y -= v.velocity;
                } else {
                    v.x -= v.velocity;
                }
            }
        }
    }
}

// ########################## clean up logic ###########################

pub fn clear_cars(cars_vec: &mut Vec<Vehicle>) {
    // todo: refine this later
    cars_vec.retain(|c| {
        c.x > -CAR_LENGTH &&
            c.x < WINDOW_WIDTH + CAR_LENGTH &&
            c.y > -CAR_LENGTH &&
            c.y < WINDOW_HEIGHT + CAR_LENGTH + 3.0
    });
}

// ################################ safety distence logic ######################################
pub fn add_car(car_vec: &mut Vec<Vehicle>, dir: Direction) {
    let safety_distance: f32 = 60.0;

    // Check the last car in the same direction
    if
        let Some(last_car) = car_vec
            .iter()
            .rev()
            .find(|v| v.direction == dir)
    {
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
    let new_vehicle = Vehicle::new(dir);
    car_vec.push(new_vehicle);
}
