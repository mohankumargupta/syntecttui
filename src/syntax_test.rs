use std::{ops::Range, path::Path};
use syntect::{
    easy::HighlightLines,
    highlighting::{
        FontStyle, HighlightState, Highlighter, RangedHighlightIterator, Style, ThemeSet,
    },
    parsing::{ParseState, ScopeStack, SyntaxSet, SyntaxSetBuilder},
    util::LinesWithEndings,
};
use tui::text::{Span, Spans};

/*
struct SyntaxLine {
    items: Vec<(Style, usize, Range<usize>)>,
}

pub struct SyntaxText {
    text: String,
    lines: Vec<SyntaxLine>,
}
*/

#[derive(Clone)]
pub struct SyntaxLine<'a> {
    pub items: Vec<(Style, &'a str)>,
}

pub struct SyntaxText<'a> {
    pub text: String,
    pub lines: Vec<SyntaxLine<'a>>,
}

impl<'a> SyntaxText<'a> {
    pub fn new(text: String) -> Self {
        //let syntax_set: SyntaxSet = SyntaxSet::load_defaults_nonewlines();
        //let syntax_set: SyntaxSet = SyntaxSet::load_from_folder("src/resources").unwrap();
        let mut builder = SyntaxSetBuilder::new();
        builder.add_from_folder("src/resources", false).unwrap();
        let syntax_set: SyntaxSet = builder.build();

        //let boo: Vec<_> = theme_set.themes.keys().cloned().collect();
        //let mut state = ParseState::new(syntax_set.find_syntax_by_extension("ini").unwrap());
        //let theme = ThemeSet::get_theme("src/resources/monokai/monokai.tmTheme").unwrap();
        let theme_set: ThemeSet = ThemeSet::load_defaults();
        //let highlighter = Highlighter::new(&theme_set.themes["Solarized (dark)"]);
        //let highlighter = Highlighter::new(&theme);

        let syntax = syntax_set.find_syntax_by_extension("ini").unwrap();
        let mut h = HighlightLines::new(syntax, &theme_set.themes["Solarized (dark)"]);

        let syntax_lines: &mut Vec<SyntaxLine<'a>> = &mut Vec::new();

        let mut textcopy = text.clone();

        for line in LinesWithEndings::from(Self::string_to_static_str(text)) {
            let ranges: Vec<(Style, &str)> = h.highlight_line(line, &syntax_set).unwrap();
            syntax_lines.push(SyntaxLine { items: ranges });
        }

        Self {
            text: textcopy,
            lines: syntax_lines.clone(),
        }

        /*
        let mut syntax_lines: Vec<SyntaxLine> = Vec::new();
        let mut highlight_state = HighlightState::new(&highlighter, ScopeStack::new());

        for (number, line) in text.lines().enumerate() {
            let ops = state.parse_line(line, &syntax_set).unwrap();
            //let ops = state.parse_line(line, &syntax_set);

            let iter =
                RangedHighlightIterator::new(&mut highlight_state, &ops[..], line, &highlighter);

            syntax_lines.push(SyntaxLine {
                items: iter
                    .map(|(style, _, range)| (style, number, range))
                    .collect(),
            });
        }

        Self {
            text,
            lines: syntax_lines,
        }
        */
    }

    fn string_to_static_str(s: String) -> &'static str {
        Box::leak(s.into_boxed_str())
    }

    pub fn convert(&self) -> tui::text::Text<'_> {
        let mut result_lines: Vec<Spans> = Vec::with_capacity(self.lines.len());

        for (syntax_line, line_content) in self.lines.iter().zip(self.text.lines()) {
            let mut line_span = Spans(Vec::with_capacity(syntax_line.items.len()));

            for (style, item_content) in &syntax_line.items {
                //let item_content = &line_content[range.clone()];
                let item_style = syntact_style_to_tui(style);

                line_span.0.push(Span::styled(*item_content, item_style));
            }

            result_lines.push(line_span);
        }

        result_lines.into()
    }
}

impl<'a> From<SyntaxText<'a>> for tui::text::Text<'a> {
    fn from(v: SyntaxText<'a>) -> Self {
        let mut result_lines: Vec<Spans> = Vec::with_capacity(v.lines.len());

        for (syntax_line, line_content) in v.lines.iter().zip(v.text.lines()) {
            let mut line_span = Spans(Vec::with_capacity(syntax_line.items.len()));

            for (style, item_content) in &syntax_line.items {
                //let item_content = &line_content[range.clone()];
                let item_style = syntact_style_to_tui(style);
                line_span.0.push(Span::styled(*item_content, item_style));
            }

            result_lines.push(line_span);
        }

        result_lines.into()
    }
}

fn syntact_style_to_tui(style: &Style) -> tui::style::Style {
    let mut res = tui::style::Style::default().fg(tui::style::Color::Rgb(
        style.foreground.r,
        style.foreground.g,
        style.foreground.b,
    ));

    if style.font_style.contains(FontStyle::BOLD) {
        res = res.add_modifier(tui::style::Modifier::BOLD);
    }
    if style.font_style.contains(FontStyle::ITALIC) {
        res = res.add_modifier(tui::style::Modifier::ITALIC);
    }
    if style.font_style.contains(FontStyle::UNDERLINE) {
        res = res.add_modifier(tui::style::Modifier::UNDERLINED);
    }

    res
}
