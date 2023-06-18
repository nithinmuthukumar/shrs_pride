use crossterm::style::{Color, Colors, ContentStyle};
use shrs::{
    prelude::{LineBuilder, SyntaxHighlighter, SyntaxTheme},
    ShellBuilder,
};
use shrs_pride::{pride_rule, PridePrompt};

fn main() {
    let syntax_theme = SyntaxTheme::new(
        ContentStyle {
            foreground_color: Some(Color::White),
            ..Default::default()
        },
        vec![pride_rule],
    );
    let prompt = PridePrompt;

    let readline = LineBuilder::default()
        .with_prompt(prompt)
        .with_highlighter(SyntaxHighlighter::new(syntax_theme))
        .build()
        .expect("Could not construct readline");

    let myshell = ShellBuilder::default()
        .with_readline(readline)
        .build()
        .unwrap();

    myshell.run();
}
