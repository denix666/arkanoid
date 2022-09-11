use macroquad::prelude::*;

const BULLET_SPEED: f32 = 300.0;

pub struct Bullet {
    pub x: f32,
    pub y: f32,
    texture: Texture2D,
    pub actual: bool,
    pub rect: Rect,
}

impl Bullet {
    pub async fn new(x:f32, y:f32) -> Self {
        Self {
            x,
            y,
            texture: load_texture("assets/images/bullet.png").await.unwrap(),
            actual: true,
            rect: Rect::new(x, y, 6.0, 11.0),
        }
    }

    pub fn update_position(&mut self, dt: f32) {
        self.y -= dt * BULLET_SPEED;
    }

    pub fn update(&mut self) {
        self.rect.x = self.x;
        self.rect.y = self.y;
    }

    pub fn draw(&mut self) {
        if self.actual {
            self.update_position(get_frame_time());
            self.update();
            draw_texture(self.texture, self.x, self.y, WHITE);
        }
    }
}