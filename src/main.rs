use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Road Intersection".to_owned(),
        window_width: 1000,
        window_height: 900,
        fullscreen: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut points: Vec<(f32, f32)> = Vec::new();
    loop {
        clear_background(GRAY);
        let screen_width = screen_width();
        let screen_height = screen_height();

        draw_line(0.0, screen_height / 2.0, screen_width, screen_height / 2.0, 120.0, BLACK);
        draw_line(0.0, screen_height / 2.0, screen_width, screen_height / 2.0, 2.0, WHITE);
        draw_line(screen_width / 2.0, 0.0, screen_width / 2.0, screen_height, 120.0, BLACK);
        draw_line(screen_width / 2.0, 0.0, 500.0, 391.0, 2.0, WHITE);
        draw_line(501.0, 510.0, 501.0, 899.0, 2.0, WHITE);
        
        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();
        points.push((x, y));
        println!("==> {:?}", points);
        }
        
        let square_size = 50.0;
        
        draw_rectangle(561.0, 339.0, square_size, square_size, RED);
        draw_rectangle(389.0, 339.0, square_size, square_size, RED);
        draw_rectangle(560.0, 510.0, square_size, square_size, RED);
        draw_rectangle(389.0, 510.0, square_size, square_size, RED);

        next_frame().await;
    }
}