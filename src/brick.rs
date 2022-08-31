use macroquad::prelude::*;

pub struct Brick {
    texture: Texture2D,
    x: f32,
    y: f32,
    pub left_side: Rect,
    pub right_side: Rect,
    pub up_side: Rect,
    pub down_side: Rect,
    pub destroyed: bool,
}

impl Brick {
    pub async fn new(x:f32, y:f32, brick_type: &str) -> Self {
        let path = format!("assets/bricks/brick_{}.png",brick_type);
        Self {
            texture: load_texture(&path).await.unwrap(),
            x,
            y,
            left_side: Rect::new(x, y, 5.0, 20.0),
            right_side: Rect::new(x+45.0, y, 5.0, 20.0),
            up_side: Rect::new(x, y, 50.0, 5.0),
            down_side: Rect::new(x, y+15.0, 50.0, 5.0),
            destroyed: false,
        }
    }

    pub fn draw(&self) {
        if !self.destroyed {
            draw_texture(self.texture, self.x, self.y, WHITE);
        }
    }
}
