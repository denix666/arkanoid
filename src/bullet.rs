use macroquad::prelude::*;

const BULLET_SPEED: f32 = 300.0;

pub struct Bullet {
    pub x: f32,
    pub y: f32,
    texture: Texture2D,
    pub destroyed: bool,
    pub rect: Rect,
}

impl Bullet {
    pub async fn new(x:f32, y:f32) -> Self {
        Self {
            x,
            y,
            texture: load_texture("assets/images/bullet.png").await.unwrap(),
            destroyed: false,
            rect: Rect::new(x, y, 6.0, 11.0),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.y -= dt * BULLET_SPEED;
        if self.y < 0.0 + 11.0 {
            self.destroyed = true;
        }
        self.rect.x = self.x;
        self.rect.y = self.y;
    }

    pub fn draw(&mut self) {
        draw_texture(self.texture, self.x, self.y, WHITE);
    }
}