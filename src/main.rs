use macroquad::prelude::*;

struct Circle {
    x: f32,
    y: f32,
    r: f32,
}

impl Circle {
    fn draw(&self) {
        let r_sqr = self.r * self.r;
        for x in ((self.x - self.r).floor() as i32)..=((self.x + self.r).ceil() as i32) {
            for y in ((self.y - self.r).floor() as i32)..=((self.y + self.r).ceil() as i32) {
                let d_sqr = (x as f32 - self.x).powi(2) + (y as f32 - self.y).powi(2);
                if d_sqr <= r_sqr {
                    draw_rectangle(x as f32, y as f32, 1.0, 1.0, WHITE);
                }
            }
        }
    }

    fn update(&mut self, x : f32, y:f32, r: f32) {
        self.x = x;
        self.y = y;
        self.r = r;
    }
}

fn circle_follow_pointer(circle: &mut Circle) {
    if is_mouse_button_down(MouseButton::Left) {
        let pos = mouse_position();
        circle.update(pos.0, pos.1, circle.r);
    }
    
    circle.draw();
}


#[macroquad::main("raytracing")]
async fn main() {
    let sheight = screen_height();
    let swidth = screen_width();

    let mut light_circle = Circle {
        x: swidth/4.0,
        y: sheight/2.0,
        r: 50.0,
    };

    let shadow_circle = Circle {
        x: swidth*3.0/4.0,
        y: sheight/2.0,
        r: 100.0,
    };
    
    loop {
        clear_background(BLACK);
        
        draw_fps();

        shadow_circle.draw();
        
        circle_follow_pointer(&mut light_circle);
        
        next_frame().await
    }
}
