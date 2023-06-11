use std::collections::HashMap;

use lazy_static::lazy_static;

use crossterm::style::{Color, ContentStyle};
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
            g: 128,
            b: 24
        },
        //blue
        Color::Rgb { r: 0, g: 0, b: 249 },
        //violet
        Color::Rgb {
            r: 134,
            g: 0,
            b: 125
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
