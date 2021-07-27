extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston::{WindowSettings, Size, RenderArgs, EventSettings, Events, RenderEvent, UpdateEvent, UpdateArgs};
use glutin_window::{GlutinWindow as Window, OpenGL};
use opengl_graphics::GlGraphics;
use graphics::{clear, Transformed, circle_arc};
use graphics::color::{WHITE, BLACK};
use rand::{thread_rng, Rng};
use graphics::ellipse::circle;
use std::f64::consts::PI;
use graphics::types::Color;
use vecmath::{Vector2, vec2_len, vec2_dot, vec2_sub};

#[derive(PartialEq)]
struct Planet {
    x: f64,
    y: f64,
    mass: f64,
    color: Color,
    vec: Vector2<f64>
}

struct App {
    backend: GlGraphics,
    planets: Vec<Planet>,
}
impl Planet {
    pub fn radius(&self) -> f64 {
        2.0 * self.mass
    }
    pub fn position(&self) -> Vector2<f64> {
        [self.x, self.y]
    }
    pub fn influence(&mut self, b: &mut Self) {
        let x = b.x - self.x;
        let y = b.y - self.y;
        let resistance = x.hypot(y);
        let acceleration = b.mass / resistance.powf(2.0) / self.mass;
        self.vec[0] += acceleration * x / resistance;
        self.vec[1] += acceleration * y / resistance;
    }
    pub fn distance(&mut self, b: &mut Planet) -> f64 {
        let (distance_x, distance_y) = (self.x - b.x, self.y - b.y);
        distance_x.hypot(distance_y)
    }
    pub fn collision(&mut self, b: &mut Planet) -> bool {
        self.distance(b) <= self.radius() * 2.0 + b.radius() * 2.0
    }
    pub fn movement(&mut self) {
        self.x += self.vec[0];
        self.y += self.vec[1];
    }
}
impl App {
    pub fn render(&mut self, render_args: &RenderArgs) {
        let (x, y) = (render_args.window_size[0] / 2.0, render_args.window_size[1] / 2.0);
        let planets = &self.planets;
        self.backend.draw(render_args.viewport(), |context, backend| {
            clear(WHITE, backend);
            let transform = context.transform.trans(x, y);
            for planet in planets {
                let rect = circle(planet.x, planet.y, planet.radius());
                circle_arc(planet.color, planet.radius(), 0.0, 2.0*PI, rect, transform, backend);
            }
        })
    }
    pub fn mass_update(&mut self) {
        for idx in 0..self.planets.len() {
            let (left, rest) = self.planets.split_at_mut(idx);
            let (a, right) = rest.split_first_mut().unwrap();
            let other_planets = left.iter_mut().chain(right);

            for b in other_planets {
                a.influence(b);
                if a.collision(b) {
                    let x = a.vec[0] - b.vec[0];
                    let y = a.vec[1] - b.vec[1];
                    a.vec[0] = (a.vec[0] * (a.mass - b.mass) + (2.0 * b.mass * b.vec[0])) / (a.mass + b.mass);
                    a.vec[1] = (a.vec[1] * (a.mass - b.mass) + (2.0 * b.mass * b.vec[1])) / (a.mass + b.mass);

                    b.vec[0] = x + a.vec[0];
                    b.vec[1] = y + a.vec[1];
                }
                a.movement()
            }
        }
    }
    pub fn update(&mut self) {
        self.mass_update();
    }
}
fn random_color() -> Color {
    let mut rng = thread_rng();
    [rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0), 1.0]
}
fn main() {
    let version = OpenGL::V4_5;
    let window_size = Size {
        width: 500.0,
        height: 600.0
    };
    let mut window: Window = WindowSettings::new("Planets", window_size)
        .exit_on_esc(false)
        .graphics_api(version)
        .samples(16)
        .build()
        .unwrap();

    let mut app = App {
        backend: GlGraphics::new(version),
        planets: vec![Planet {x: 150.0, y: 15.0, mass: 15.0, color: random_color(), vec: [-0.07, 0.0]},
                      Planet {x: 225.0, y: 233.0, mass: 15.0, color: random_color(), vec: [0.07; 2]},
                      Planet {x: 300.0, y: 50.0, mass: 2.0, color: random_color(), vec: [0.15; 2]}
            ]
    };
    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }
        if let Some(_args) = e.update_args() {
            app.update()
        }
    }
}
