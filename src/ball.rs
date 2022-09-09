use macroquad::prelude::*;

const BALL_SPEED: f32 = 60.0;

pub enum HorizontalDir {
    Left,
    Right,
}

pub enum VerticalDir {
    Up,
    Down,
}

pub struct Ball {
    texture: Texture2D,
    pub x: f32,
    pub y: f32,
    pub ball_step_move_x: f32,
    pub ball_step_move_y: f32,
    pub horizontal_dir: HorizontalDir,
    pub vertical_dir: VerticalDir,
    pub rect: Rect,
    pub released: bool,
    pub last_ball_time: f64,
    pub idle_time: f64,
}

impl Ball {
    pub async fn new(x: f32,y: f32) -> Self {
        Self {
            texture: load_texture("assets/images/ball.png").await.unwrap(),
            x,
            y,
            ball_step_move_x: 5.0,
            ball_step_move_y: 5.0,
            horizontal_dir: HorizontalDir::Right,
            vertical_dir: VerticalDir::Up,
            rect: Rect::new(x+4.0, y+4.0, 8.0, 8.0),
            released: false,
            last_ball_time: 0.0,
            idle_time: 3.0,
        }
    }

    pub fn update(&mut self, dt: f32) {
        match self.horizontal_dir {
            HorizontalDir::Left => {
                let x = self.ball_step_move_x;
                self.x -= x * dt * BALL_SPEED;
            },
            HorizontalDir::Right => {
                let x = self.ball_step_move_x;
                self.x += x * dt * BALL_SPEED;
            },
        }

        match self.vertical_dir {
            VerticalDir::Up => {
                let y = self.ball_step_move_x;
                self.y -= y * dt * BALL_SPEED;
            },
            VerticalDir::Down => {
                let y = self.ball_step_move_y;
                self.y += y * dt * BALL_SPEED;
            },
        }
        self.rect.x = self.x+4.0;
        self.rect.y = self.y+4.0;
    }

    pub fn center_x(&mut self) -> f32 {
        self.x + self.texture.width() / 2.0
    }

    pub fn ball_width(&mut self) -> f32 {
        self.texture.width()
    }

    pub fn ball_height(&mut self) -> f32 {
        self.texture.height()
    }

    pub fn draw(&self) {
        draw_texture(self.texture, self.x, self.y, WHITE);
    }
}
