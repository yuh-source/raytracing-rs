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
        let r_sqr = self.r*self.r;
        let mut x: f32 = self.x-self.r;

        while x <= self.x+self.r {
            let mut y: f32 = self.y-self.r;

            while y <= self.y+self.r {

                let d_sqr: f32 = (x-self.x)*(x-self.x) + (y-self.y)*(y-self.y);

                if d_sqr <= r_sqr {
                    draw_rectangle(x, y, 1.0, 1.0, WHITE);
                }

                y += 1.0;
            }

            x += 1.0;
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
