use macroquad::prelude::*;
pub struct Brick {
    pub x: f32,
    pub y: f32, 
    pub destroyed: bool,
    texture: Vec<Texture2D>,
    pub rect: Rect,
    pub power: usize,
    pub brick_with_bonus: bool,
}

impl Brick {
    pub async fn new(x: f32, y: f32, brick_type: &str) -> Self {
        let mut sprites:Vec<Texture2D> = Vec::new();
        let brick_power: usize;
        
        if brick_type == "a" {
            brick_power = 3;
            for i in 0..=3 {
                let path = format!("assets/images/bricks/brick_a{}.png", i);
                sprites.push(load_texture(&path).await.unwrap());
            }
        } else {
            brick_power = 0;
            let path = format!("assets/images/bricks/brick_{}.png", brick_type);
            sprites.push(load_texture(&path).await.unwrap());
        }
        Self {
            x,
            y,
            destroyed: false,
            texture: sprites,
            rect: Rect::new(0.0, 0.0, 50.0,20.0),
            power: brick_power,
            brick_with_bonus: false,
        }
    }

    pub fn draw(&mut self) {
        draw_texture(self.texture[self.power], self.x, self.y, WHITE);

        // define rect
        self.rect.x = self.x;
        self.rect.y = self.y;
    }
}