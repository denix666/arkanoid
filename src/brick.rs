use macroquad::prelude::*;

pub struct Brick {
    texture: Texture2D,
    x: f32,
    y: f32,
}

impl Brick {
    pub async fn new(x:f32, y:f32, brick_type: i32) -> Self {
        let path = format!("assets/bricks/brick_{}.png",brick_type);
        Self {
            texture: load_texture(&path).await.unwrap(),
            x,
            y,
        }
    }

    // pub fn update(&mut self, dt: f32) {
    //     todo!()
    // }

    pub fn draw(&self) {
        draw_texture(self.texture, self.x, self.y, WHITE);
    }
}
