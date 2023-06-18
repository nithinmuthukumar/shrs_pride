use std::collections::HashMap;

use lazy_static::lazy_static;

use crossterm::style::{Color, ContentStyle, Stylize};
use shrs::{
    prelude::{styled, LineCtx, LineMode, Prompt, StyledBuf},
    prompt::{top_pwd, username},
};
lazy_static! {
    static ref COLORS: Vec<Color> = vec![
        //red
        Color::Rgb {
            r: 255,
            g: 0,
            b: 24,
        },
        //orange
        Color::Rgb {
            r: 255,
            g: 165,
            b: 44
        },
        //yellow
        Color::Rgb {
            r: 255,
            g: 255,
            b: 65
        },
        //green
        Color::Rgb {
            r: 0,
            g: 180,
            b: 24
        },
        //blue
        Color::Rgb { r: 42, g: 125, b: 255 },
        //violet
        Color::Rgb {
            r: 134,
            g: 49,
            b: 247
        }
    ];
}

pub fn pride_rule(buf: &str) -> HashMap<usize, ContentStyle> {
    let mut c_style: HashMap<usize, ContentStyle> = HashMap::new();

    let mut colors = COLORS.iter().cycle();
    for (i, c) in buf.chars().enumerate() {
        if !c.is_whitespace() {
            c_style.insert(
                i,
                ContentStyle {
                    foreground_color: Some(colors.next().unwrap().clone()),
                    ..Default::default()
                },
            );
        }
    }
    c_style
}
pub struct PridePrompt;

impl Prompt for PridePrompt {
    fn prompt_left(&self, line_ctx: &mut LineCtx) -> StyledBuf {
        let indicator = match line_ctx.mode() {
            LineMode::Insert => String::from(">").cyan(),
            LineMode::Normal => String::from(":").yellow(),
        };
        if !line_ctx.lines.is_empty() {
            return styled! {" ", indicator, " "};
        }

        styled! {"(He/Him)", @(blue)username(), " ", @(white,bold)top_pwd(), " ", indicator, " "}
    }
    fn prompt_right(&self, line_ctx: &mut LineCtx) -> StyledBuf {
        styled! {"🏳️‍🌈 ".repeat(3)}
    }
}
