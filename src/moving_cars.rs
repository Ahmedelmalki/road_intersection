// ############################## car turns logic ####################################
use road_intersection::*;
use macroquad::prelude::*;

pub fn moving_cars(vehicles: &mut Vec<Vehicle>, controller: &TrafficLightController) {
    // Mise a jour position des voitures

    for i in 0..vehicles.len() {
        let should_stop =
            should_stop_at_light(&vehicles[i], controller) || stop_before(&vehicles[i], vehicles);

        if should_stop {
            continue;
        }

        let v = &mut vehicles[i];
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

fn should_stop_at_light(vehicle: &Vehicle, controller: &TrafficLightController) -> bool {
    if controller.is_green(vehicle.direction) {
        return false;
    }

    // Check if the vehicle is close to the intersection
    let intersection_center_x = WINDOW_WIDTH / 2.0;
    let intersection_center_y = WINDOW_HEIGHT / 2.0;
    let stop_distance = CAR_LENGTH; // Distance before intersection to stop

    match vehicle.direction {
        Direction::North => {
            // Coming from south, stopping before intersection
            vehicle.y >= intersection_center_y - THICKNESS / 2.0 - stop_distance &&
                vehicle.y <= intersection_center_y - THICKNESS / 2.0
        }
        Direction::South => {
            // Coming from north, stopping before intersection
            vehicle.y <= intersection_center_y + THICKNESS / 2.0 + stop_distance &&
                vehicle.y >= intersection_center_y + THICKNESS / 2.0
        }
        Direction::East => {
            // Coming from west, stopping before intersection
            vehicle.x >= intersection_center_x - THICKNESS / 2.0 - stop_distance &&
                vehicle.x <= intersection_center_x - THICKNESS / 2.0
        }
        Direction::West => {
            // Coming from east, stopping before intersection
            vehicle.x <= intersection_center_x + THICKNESS / 2.0 + stop_distance &&
                vehicle.x >= intersection_center_x + THICKNESS / 2.0
        }
    }
}

fn stop_before(v: &Vehicle, car_vec: &Vec<Vehicle>) -> bool {
    let safety_distance = SAFETY_GAP; // or whatever value you want

    // Find cars in the same lane (same direction) that are ahead of this vehicle
    for other_car in car_vec {
        // Skip if it's the same car or different direction
        if (other_car as *const _) == (v as *const _) || other_car.direction != v.direction {
            continue;
        }

        // Check if the other car is ahead and calculate distance
        let (is_ahead, distance) = match v.direction {
            Direction::North => {
                // Moving up (y decreasing), so "ahead" means smaller y
                if other_car.y < v.y {
                    (true, v.y - other_car.y - CAR_LENGTH)
                } else {
                    (false, 0.0)
                }
            }
            Direction::South => {
                // Moving down (y increasing), so "ahead" means larger y
                if other_car.y > v.y {
                    (true, other_car.y - v.y - CAR_LENGTH)
                } else {
                    (false, 0.0)
                }
            }
            Direction::East => {
                // Moving right (x increasing), so "ahead" means larger x
                if other_car.x > v.x {
                    (true, other_car.x - v.x - CAR_LENGTH)
                } else {
                    (false, 0.0)
                }
            }
            Direction::West => {
                // Moving left (x decreasing), so "ahead" means smaller x
                if other_car.x < v.x {
                    (true, v.x - other_car.x - CAR_LENGTH)
                } else {
                    (false, 0.0)
                }
            }
        };

        // If there's a car ahead and it's too close, stop
        if is_ahead && distance < safety_distance {
            return true;
        }
    }

    false // No obstacles, safe to move
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
