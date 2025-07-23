#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use console_display::{
    character_display::StaticCharacterDisplay,
    color::{
        ARGBColor,
        RGBColor,
        TerminalColor,
    },
    display_driver::{
        DisplayDriver,
        UpdateStatus,
    },
    pixel::character_pixel::CharacterPixel,
    widget::{
        single_widget::UvWidget,
        two_widget::OverlayWidget,
    },
};
use crossterm::event::KeyCode;

type Display = OverlayWidget<
    UvWidget<
        StaticCharacterDisplay<CharacterPixel, 29, 11>,
        CharacterPixel,
    >,
    UvWidget<
        StaticCharacterDisplay<CharacterPixel, 29, 11>,
        CharacterPixel,
    >,
>;

#[allow(clippy::too_many_lines)]
fn main() {
    const WIDTH: usize = 9 * 3 + 2;
    const HEIGHT: usize = 3 * 3 + 2;

    let foreground = RGBColor::WHITE.into();
    let background = ARGBColor::TRANSPARENT.into();

    let mut cursor_pos = (1, 1);
    let mut symbol = 'X';
    let mut state: [[Option<char>; 3]; 3] = [[None; 3]; 3];

    let mut char_disp = OverlayWidget::new(
        UvWidget::new(StaticCharacterDisplay::<_, WIDTH, HEIGHT>::new(
            CharacterPixel::new::<'.'>(
                ARGBColor::TRANSPARENT.into(),
                ARGBColor::TRANSPARENT.into(),
            ),
        )),
        UvWidget::new(StaticCharacterDisplay::<_, WIDTH, HEIGHT>::new(
            CharacterPixel::new::<' '>(foreground, background),
        )),
    );

    char_disp.0.set_uv_x_min(0.);
    char_disp.0.set_uv_x_max(3.);
    char_disp.0.set_uv_y_min(0.);
    char_disp.0.set_uv_y_max(3.);

    char_disp.1.set_uv_x_min(0.);
    char_disp.1.set_uv_x_max(3.);
    char_disp.1.set_uv_y_min(0.);
    char_disp.1.set_uv_y_max(3.);

    for i in 0..4 {
        char_disp.1.draw_line(
            0.,
            i as f32,
            3.,
            i as f32,
            CharacterPixel::new::<'━'>(foreground, background).into(),
        );
        char_disp.1.draw_line(
            i as f32,
            0.,
            i as f32,
            3.,
            CharacterPixel::new::<'┃'>(foreground, background).into(),
        );
    }

    for x in 1..3 {
        for y in 1..3 {
            let _ = char_disp.1.set_pixel(
                x as f32,
                y as f32,
                CharacterPixel::new::<'╋'>(foreground, background).into(),
            );
        }
    }

    let mut display = DisplayDriver::new(char_disp);

    display
        .0
        .set_pixel(
            1.5,
            1.5,
            CharacterPixel::new::<'X'>(
                RGBColor::GREEN.into(),
                ARGBColor::TRANSPARENT.into(),
            )
            .into(),
        )
        .unwrap();

    display.set_on_update(move |disp, latest_event| {
        if check_winner(disp, state, foreground, background).is_some() {
            return UpdateStatus::Continue;
        }

        #[allow(clippy::cast_sign_loss)]
        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_possible_wrap)]
        if let Some(key_event) = latest_event {
            if key_event.code == KeyCode::Enter &&
                state[cursor_pos.1][cursor_pos.0].is_none()
            {
                state[cursor_pos.1][cursor_pos.0] = Some(symbol);
                disp.0
                    .set_pixel(
                        cursor_pos.0 as f32 + 0.5,
                        cursor_pos.1 as f32 + 0.5,
                        CharacterPixel::build(
                            symbol,
                            RGBColor::WHITE.into(),
                            ARGBColor::TRANSPARENT.into(),
                        )
                        .unwrap()
                        .into(),
                    )
                    .unwrap();
                symbol = if symbol == 'X' { 'O' } else { 'X' };
                return UpdateStatus::Continue;
            }

            let mut direction: (i32, i32) = (0, 0);
            if let KeyCode::Char(key) = key_event.code {
                match key {
                    'a' | 'A' => direction = (-1, 0),
                    'd' | 'D' => direction = (1, 0),
                    'w' | 'W' => direction = (0, -1),
                    's' | 'S' => direction = (0, 1),
                    _ => (),
                }
            }
            else {
                match key_event.code {
                    KeyCode::Left => direction = (-1, 0),
                    KeyCode::Right => direction = (1, 0),
                    KeyCode::Up => direction = (0, -1),
                    KeyCode::Down => direction = (0, 1),
                    _ => (),
                }
            }

            if state[cursor_pos.1][cursor_pos.0].is_none() {
                disp.0
                    .set_pixel(
                        cursor_pos.0 as f32 + 0.5,
                        cursor_pos.1 as f32 + 0.5,
                        CharacterPixel::new::<' '>(
                            ARGBColor::TRANSPARENT.into(),
                            ARGBColor::TRANSPARENT.into(),
                        )
                        .into(),
                    )
                    .unwrap();
            }
            else {
                disp.0
                    .set_pixel(
                        cursor_pos.0 as f32 + 0.5,
                        cursor_pos.1 as f32 + 0.5,
                        CharacterPixel::build(
                            state[cursor_pos.1][cursor_pos.0].unwrap(),
                            foreground,
                            background,
                        )
                        .unwrap()
                        .into(),
                    )
                    .unwrap();
            }

            cursor_pos.0 =
                (cursor_pos.0 as i32 + direction.0).clamp(0, 2) as usize;
            cursor_pos.1 =
                (cursor_pos.1 as i32 + direction.1).clamp(0, 2) as usize;

            disp.0
                .set_pixel(
                    cursor_pos.0 as f32 + 0.5,
                    cursor_pos.1 as f32 + 0.5,
                    CharacterPixel::build(
                        symbol,
                        if state[cursor_pos.1][cursor_pos.0].is_some() {
                            RGBColor::RED.into()
                        }
                        else {
                            RGBColor::GREEN.into()
                        },
                        ARGBColor::TRANSPARENT.into(),
                    )
                    .unwrap()
                    .into(),
                )
                .unwrap();
        }
        UpdateStatus::Continue
    });

    display.initialize().expect("Could not initialize display.");
    display.update();
}

#[allow(clippy::too_many_lines)]
fn check_winner(
    disp: &mut Display,
    state: [[Option<char>; 3]; 3],
    foreground: TerminalColor,
    background: TerminalColor,
) -> Option<char> {
    let mut winner = None;
    let mut line;

    for (row_index, row) in state.iter().enumerate() {
        line = false;
        if row.iter().all(|x| *x == Some('X')) {
            winner = Some('X');
            line = true;
        }
        if row.iter().all(|x| *x == Some('O')) {
            winner = Some('O');
            line = true;
        }
        if line {
            disp.1.draw_line(
                0.5,
                row_index as f32 + 0.5,
                2.5,
                row_index as f32 + 0.5,
                CharacterPixel::new::<'─'>(
                    RGBColor::RED.into(),
                    background,
                )
                .into(),
            );
            for column_index in 1..3 {
                disp.1
                    .set_pixel(
                        column_index as f32,
                        row_index as f32 + 0.5,
                        CharacterPixel::new::<'╂'>(
                            foreground, background,
                        )
                        .into(),
                    )
                    .unwrap();
            }
        }
    }

    for column_index in 0..3 {
        line = false;
        if state.iter().all(|x| x[column_index] == Some('X')) {
            winner = Some('X');
            line = true;
        }
        if state.iter().all(|x| x[column_index] == Some('O')) {
            winner = Some('O');
            line = true;
        }
        if line {
            disp.1.draw_line(
                column_index as f32 + 0.5,
                0.5,
                column_index as f32 + 0.5,
                2.5,
                CharacterPixel::new::<'│'>(
                    RGBColor::RED.into(),
                    background,
                )
                .into(),
            );
            for row_index in 1..3 {
                disp.1
                    .set_pixel(
                        column_index as f32 + 0.5,
                        row_index as f32,
                        CharacterPixel::new::<'┿'>(
                            foreground, background,
                        )
                        .into(),
                    )
                    .unwrap();
            }
        }
    }

    let diagonal1 = [state[0][0], state[1][1], state[2][2]];
    let diagonal2 = [state[0][2], state[1][1], state[2][0]];

    line = false;
    if diagonal1.iter().all(|x| *x == Some('X')) {
        line = true;
        winner = Some('X');
    }
    if diagonal1.iter().all(|x| *x == Some('O')) {
        line = true;
        winner = Some('O');
    }
    if line {
        disp.1.draw_line(
            0.5,
            0.5,
            2.5,
            2.5,
            CharacterPixel::new::<'*'>(RGBColor::RED.into(), background)
                .into(),
        );
    }

    line = false;
    if diagonal2.iter().all(|x| *x == Some('X')) {
        line = true;
        winner = Some('X');
    }
    if diagonal2.iter().all(|x| *x == Some('O')) {
        line = true;
        winner = Some('O');
    }
    if line {
        disp.1.draw_line(
            0.5,
            2.5,
            2.5,
            0.5,
            CharacterPixel::new::<'*'>(RGBColor::RED.into(), background)
                .into(),
        );
    }

    winner
}
