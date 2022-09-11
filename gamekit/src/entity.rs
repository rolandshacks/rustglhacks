//
// Entity
//

use crate::graphics::{sprite::{Sprite, SpriteData}, types::{Rectangle, Color, Vector2}, api::Api};

defaults!();

#[derive(Copy, Clone, Debug)]
pub struct Entity {
    pub sprite: SpriteData,
    target: Vector2,
    velocity: Vector2,
    time_to_live: f32
}

impl Sprite for Entity {
    fn get_sprite_data(&self) -> &SpriteData {
        return &self.sprite;
    }
}

impl Entity {
    pub fn new() -> Entity {

        let sprite = SpriteData {
            position: Vector2::new(0.0, 0.0),
            size: Vector2::new(16.0, 16.0),
            color: Color::new(1.0, 1.0, 1.0, 0.5),
            texture_coords: Rectangle::one(),
        };

        Entity {
            sprite,
            target: Vector2::new(0.0, 0.0),
            velocity: Vector2::new(0.0, 0.0),
            time_to_live: 0.0
        }
    }

    pub fn initialize(&mut self, api: &mut dyn Api) {

        let metrics = api.get_metrics();

        self.time_to_live = api.get_random_range(2.0, 5.0);

        let x = api.get_random_range(100.0, metrics.width as  f32 - 200.0);
        let y = api.get_random_range(100.0, metrics.height as  f32 - 200.0);

        if metrics.frame_counter == 0 {

            let hue = api.get_random_range(0.0, 360.0);
            self.sprite.color = hsv_to_rgb(hue, 100.0, 50.0);
        }

        self.target = Vector2::new(x, y);
    }

    pub fn update(&mut self, api: &mut dyn Api, delta_time: f32) {

        let mut distance = vector_distance(&self.sprite.position, &self.target);
        let len = vector_len(&distance);
        if len > 0.0 {
            distance.x /= len;
            distance.y /= len;
        }

        if self.time_to_live <= 0.0 || len < 100.0 {
            self.initialize(api);
            return;
        }

        if self.time_to_live > 0.0 {
            self.time_to_live -= delta_time;
        }

        self.velocity.x += distance.x * 5.0 * delta_time;
        self.velocity.y += distance.y * 5.0 * delta_time;

        vector_normalize(&mut self.velocity);

        let speed = 500.0;

        self.sprite.position.x += self.velocity.x * speed * delta_time;
        self.sprite.position.y += self.velocity.y * speed * delta_time;

    }
}

fn vector_distance(v1: &Vector2, v2: &Vector2) -> Vector2 {
    Vector2::new(v2.x-v1.x, v2.y-v1.y)
}

fn vector_len(v: &Vector2) -> f32 {
    (v.x*v.x + v.y*v.y).sqrt()
}

fn vector_normalize(v: &mut Vector2) {
    let l = vector_len(v);
    if l > 0.0 {
        v.x /= l;
        v.y /= l;
    }
}

#[allow(dead_code)]
fn vector_normalize_fast(v: &mut Vector2) {
    let ax = v.x.abs();
    let ay = v.y.abs();

    let m = ax.max(ay);
    if m == 0.0 { return; }

    let mut ratio = 1.0 / ax.max(ay);
    ratio = ratio * (1.29289 - (ax + ay) * ratio * 0.29289);

    v.x = v.x * ratio;
    v.y = v.y * ratio;
}


fn hsv_to_rgb(h: f32, s: f32, v: f32) -> Color {

    let hp = h / 60.0;
    let c = s * v;
    let x = c * (1.0 - (hp % 2.0 - 1.0).abs());
    let m = v - c;

    let mut r = 0.0;
    let mut g = 0.0;
    let mut b = 0.0;

    if hp <= 1.0 {
        r = c;
        g = x;
    } else if hp <= 2.0 {
        r = x;
        g = c;
    } else if hp <= 3.0 {
        g = c;
        b = x;
    } else if hp <= 4.0 {
        g = x;
        b = c;
    } else if hp <= 5.0 {
        r = x;
        b = c;
    } else {
        r = c;
        b = x;
    }

    r += m;
    g += m;
    b += m;

    Color::new(r, g, b, 1.0)

}
