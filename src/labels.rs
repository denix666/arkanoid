use macroquad::prelude::*;

///
/// Show at bottom of the screen game values as level, score and lives
///

pub struct Label {
    text: String,
    font: Font,
    x: f32,
    y: f32,
    font_size: u16,
}

impl Label {
    pub async fn new(text: String, x: f32, y: f32, font_size: u16) -> Self {
        Self {
            text,
            x,
            y,
            font_size,
            font: load_ttf_font("assets/fonts/iomanoid.ttf").await.unwrap(),
        }
    }

    pub fn draw_label(&self, value: i32) {
        let mut lbl_text = self.text.to_string();
        lbl_text.push_str(&value.to_string());
        draw_text_ex(
            &lbl_text,
            self.x,
            self.y,
            TextParams {
                font: self.font,
                font_size: self.font_size,
                color: WHITE,
                ..Default::default()
            },
        );
    }
}
