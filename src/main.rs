use std::f32::consts::PI;

use macroquad::prelude::*;

const RAY_COUNT: usize = 150;

struct Circle {
    x: f32,
    y: f32,
    r: f32,
}

#[derive(Clone)]
struct Ray {
    x: f32,
    y: f32,
    angle: f32,
}


impl Circle {
    fn update(&mut self, x : f32, y:f32, r: f32) {
        self.x = x;
        self.y = y;
        self.r = r;
    }

    fn follow_pointer(&mut self) {
        if is_mouse_button_down(MouseButton::Left) {
            let pos = mouse_position();
            self.update(pos.0, pos.1, self.r);
        }
    }

    fn draw(&self) {
        let r_sqr: f32 = self.r * self.r;
        for x in ((self.x - self.r).floor() as i32)..=((self.x + self.r).ceil() as i32) {
            for y in ((self.y - self.r).floor() as i32)..=((self.y + self.r).ceil() as i32) {
                let d_sqr: f32 = (x as f32 - self.x).powi(2) + (y as f32 - self.y).powi(2);
                if d_sqr <= r_sqr {
                    draw_rectangle(x as f32, y as f32, 1.0, 1.0, WHITE);
                }
            }
        }
    }

    fn generate_rays (&self) -> Vec<Ray> {
        let mut rays: Vec<Ray> = vec![Ray { x: 0.0, y:0.0, angle:0.0 }; RAY_COUNT];

        for i in 0..RAY_COUNT {
            let angle: f32 = (i as f32 / RAY_COUNT as f32) * 2.0 * PI;
            let ray = Ray {
                x: self.x,
                y: self.y,
                angle: angle,
            };
            rays[i] = ray;
        }

        return rays;
    }

    fn draw_rays(&self, shadow_circle: &Circle) {
        let r_sqr: f32 = shadow_circle.r * shadow_circle.r;
        let rays: Vec<Ray> = self.generate_rays();

        for i in 0..RAY_COUNT {
            let ray: &Ray = &rays[i];

            let mut x: f32 = ray.x;
            let mut y: f32 = ray.y;

            loop {
                x += ray.angle.cos();
                y += ray.angle.sin();

                draw_rectangle(x, y, 1.0, 1.0, WHITE);

                if ( x < 0.0 || x > screen_width()) || (y < 0.0 || y > screen_height()){
                    break;
                }

                let d_sqr: f32 = (x - shadow_circle.x).powi(2) + (y - shadow_circle.y).powi(2);
                if d_sqr < r_sqr {
                    break;
                }
            }
        }
    }

}


#[macroquad::main("raytracing")]
async fn main() {
    let mut light_circle = Circle {
        x: screen_width()/4.0,
        y: screen_height()/2.0,
        r: 50.0,
    };

    let shadow_circle = Circle {
        x: screen_width()*3.0/4.0,
        y: screen_height()/2.0,
        r: 100.0,
    };
    
    loop {
        clear_background(BLACK);
        draw_fps();

        light_circle.draw();
        shadow_circle.draw();

        light_circle.follow_pointer();

        light_circle.draw_rays(&shadow_circle);

        next_frame().await
    }
}
