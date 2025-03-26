use macroquad::prelude::*;

struct Circle {
    x: f32,
    y: f32,
    r: f32,
}

trait Draw {
    fn draw(&self);
}

impl Draw for Circle {
    fn draw(&self) {
        for x in (self.x - self.r) as i32..=(self.x + self.r) as i32 {
            for y in (self.y - self.r) as i32..=(self.y + self.r) as i32 {
                let dx = self.x - x as f32;
                let dy = self.y - y as f32;
                if dx * dx + dy * dy <= self.r * self.r {
                    draw_rectangle(self.x, self.y, 1.0, 1.0, WHITE);
                    draw
                }
            }
        }
    }
}


#[macroquad::main("raytracing")]
async fn main() {
    loop {
        clear_background(BLACK);

        let pos = mouse_position();

        let circle = Circle {
            x: pos.0,
            y: pos.1,
            r: 50.0,
        };

        if is_mouse_button_down(MouseButton::Left) {
            circle.draw();
        }
        
        next_frame().await
    }
}
