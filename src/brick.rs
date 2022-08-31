use macroquad::prelude::*;

pub struct Brick {
    texture: Texture2D,
    x: f32,
    y: f32,
    pub rect: Rect,
    pub destroyed: bool,
}

impl Brick {
    pub async fn new(x:f32, y:f32, brick_type: &str) -> Self {
        let path = format!("assets/bricks/brick_{}.png",brick_type);
        Self {
            texture: load_texture(&path).await.unwrap(),
            x,
            y,
            rect: Rect::new(x, y, 50.0, 20.0),
            destroyed: false,
        }
    }

    pub fn draw(&self) {
        if !self.destroyed {
            draw_texture(self.texture, self.x, self.y, WHITE);
        }
    }
}
