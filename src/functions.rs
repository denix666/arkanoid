use macroquad::prelude::*;

pub fn draw_paused_text(font: Font) {
    let header_text = String::from("PAUSED");
    let message_text = String::from("Press 'space' to continue...");

    let header_dims = measure_text(&header_text, Some(font), 70, 1.0);
    let message_dims = measure_text(&message_text, Some(font), 30, 1.0);

    draw_text_ex(
        &header_text,
        screen_width() * 0.5 - header_dims.width * 0.5,
        240.0,
        TextParams {
            font,
            font_size: 70,
            color: BLACK,
            ..Default::default()
        },
    );

    draw_text_ex(
        &message_text,
        screen_width() * 0.5 - message_dims.width * 0.5,
        360.0,
        TextParams {
            font,
            font_size: 30,
            color: BLACK,
            ..Default::default()
        },
    );
}

pub fn draw_level_failed_text(font: Font) {
    let header_text = String::from("Level fail");
    let message_text = String::from("Press 'space' to continue...");

    let header_dims = measure_text(&header_text, Some(font), 70, 1.0);
    let message_dims = measure_text(&message_text, Some(font), 30, 1.0);

    draw_text_ex(
        &header_text,
        screen_width() * 0.5 - header_dims.width * 0.5,
        240.0,
        TextParams {
            font,
            font_size: 70,
            color: BLACK,
            ..Default::default()
        },
    );

    draw_text_ex(
        &message_text,
        screen_width() * 0.5 - message_dims.width * 0.5,
        360.0,
        TextParams {
            font,
            font_size: 30,
            color: BLACK,
            ..Default::default()
        },
    );
}

pub fn draw_level_completed_text(font: Font) {
    let header_text = String::from("Level Completed!");
    let message_text = String::from("Press 'space' to continue...");

    let header_dims = measure_text(&header_text, Some(font), 70, 1.0);
    let message_dims = measure_text(&message_text, Some(font), 30, 1.0);

    draw_text_ex(
        &header_text,
        screen_width() * 0.5 - header_dims.width * 0.5,
        240.0,
        TextParams {
            font,
            font_size: 70,
            color: BLACK,
            ..Default::default()
        },
    );

    draw_text_ex(
        &message_text,
        screen_width() * 0.5 - message_dims.width * 0.5,
        360.0,
        TextParams {
            font,
            font_size: 30,
            color: BLACK,
            ..Default::default()
        },
    );
}

pub fn draw_game_over_text(font: Font) {
    let header_text = String::from("Game over");
    let message_text = String::from("Press 'space' to start new game...");

    let header_dims = measure_text(&header_text, Some(font), 70, 1.0);
    let message_dims = measure_text(&message_text, Some(font), 30, 1.0);

    draw_text_ex(
        &header_text,
        screen_width() * 0.5 - header_dims.width * 0.5,
        240.0,
        TextParams {
            font,
            font_size: 70,
            color: BLACK,
            ..Default::default()
        },
    );

    draw_text_ex(
        &message_text,
        screen_width() * 0.5 - message_dims.width * 0.5,
        360.0,
        TextParams {
            font,
            font_size: 30,
            color: BLACK,
            ..Default::default()
        },
    );
}