use macroquad::prelude::*;

const PLAYER_SPEED: f32 = 700.0;

pub enum Kind {
    Normal,
    Catch,
    Expanded,
    Laser,
}

pub struct Paddle {
    pub x: f32,
    pub y: f32, 
    paddle_normal: Texture2D,
    paddle_catch: Texture2D,
    paddle_laser: Texture2D,
    paddle_expanded: Texture2D,
    pub kind: Kind,
}

impl Paddle {
    pub async fn new() -> Self {
        Self {
            x: screen_width()/2.0, 
            y: screen_height() - 100.0, 
            paddle_normal: load_texture("assets/images/paddle.png").await.unwrap(),
            paddle_catch: load_texture("assets/images/paddle_catch.png").await.unwrap(),
            paddle_expanded: load_texture("assets/images/paddle_expanded.png").await.unwrap(),
            paddle_laser: load_texture("assets/images/paddle_laser.png").await.unwrap(),
            kind: Kind::Normal,
        }
    }

    pub fn center_x(&mut self) -> f32 {
        self.x + self.paddle_normal.width() / 2.0
    }

    pub fn width(&mut self) -> f32 {
        match self.kind {
            Kind::Normal => {
                self.paddle_normal.width()
            },
            Kind::Catch => {
                self.paddle_catch.width()
            },
            Kind::Expanded => {
                self.paddle_expanded.width()
            },
            Kind::Laser => {
                self.paddle_laser.width()
            },
        }
    }

    pub fn update(&mut self, dt: f32) {
        let mut x_move = 0.0;

        if is_key_down(KeyCode::Left) {
            x_move -= 1.0;
        }

        if is_key_down(KeyCode::Right) {
            x_move += 1.0;
        }

        self.x += x_move * dt * PLAYER_SPEED;

        if self.x < 16.0 {
            self.x = 16.0;
        }

        match self.kind {
            Kind::Normal => {
                if self.x > screen_width() - 16.0 - self.paddle_normal.width() {
                    self.x = screen_width() - 16.0 - self.paddle_normal.width();
                }
            },
            Kind::Catch => {
                if self.x > screen_width() - 16.0 - self.paddle_catch.width() {
                    self.x = screen_width() - 16.0 - self.paddle_catch.width();
                }
            },
            Kind::Expanded => {
                if self.x > screen_width() - 16.0 - self.paddle_expanded.width() {
                    self.x = screen_width() - 16.0 - self.paddle_expanded.width();
                }
            },
            Kind::Laser => {
                if self.x > screen_width() - 16.0 - self.paddle_laser.width() {
                    self.x = screen_width() - 16.0 - self.paddle_laser.width();
                }
            },
        }
    }

    pub fn draw(&self) {
        match self.kind {
            Kind::Normal => {
                draw_texture(self.paddle_normal, self.x, self.y, WHITE);
            },
            Kind::Catch => {
                draw_texture(self.paddle_catch, self.x, self.y, WHITE);
            },
            Kind::Expanded => {
                draw_texture(self.paddle_expanded, self.x, self.y, WHITE);
            },
            Kind::Laser => {
                draw_texture(self.paddle_laser, self.x, self.y, WHITE);
            },
        }
    }
}
