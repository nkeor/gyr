use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};
use tui::style::{Color, Style};
use tui::text::{Span, Spans, Text};

use super::xdg;

pub struct UI<'a> {
    pub hidden: Vec<xdg::App>,
    pub shown: Vec<xdg::App>,
    pub selected: Option<usize>,
    pub text: Vec<Spans<'a>>,
    pub query: String,
    pub log: Vec<Text<'a>>,
    pub verbose: u64,
    #[doc(hidden)]
    matcher: SkimMatcherV2,
}

impl<'a> UI<'a> {
    pub fn new(items: Vec<xdg::App>) -> UI<'a> {
        UI {
            shown: items,
            hidden: vec![],
            selected: Some(0),
            text: vec![],
            query: String::new(),
            log: vec![],
            verbose: 0,
            matcher: SkimMatcherV2::default(),
        }
    }

    pub fn verbosity(&mut self, b: u64) {
        self.verbose = b;
    }

    pub fn info(&mut self, color: Color) {
        if let Some(selected) = self.selected {
            self.text = vec![
                Spans::from(Span::styled(
                    self.shown[selected].name.clone(),
                    Style::default().fg(color),
                )),
                Spans::from(Span::raw(self.shown[selected].description.clone())),
            ];
            if self.verbose > 0 {
                self.text.push(Spans::default());

                let mut text = if self.shown[selected].is_terminal {
                    vec![Span::raw("Exec (terminal): ")]
                } else {
                    vec![Span::raw("Exec: ")]
                };

                text.push(Span::styled(
                    self.shown[selected].command.to_string(),
                    Style::default(),
                ));

                self.text.push(Spans::from(text));

                if self.verbose > 1 {
                    self.text.push(Spans::from(Span::raw(format!(
                        "Times run: {}",
                        &self.shown[selected].history
                    ))));
                    self.text.push(Spans::from(Span::raw(format!(
                        "\nMatching score: {}",
                        self.shown[selected].score
                    ))));
                }
            }
        } else {
            self.text.clear();
        }
    }

    pub fn filter(&mut self) {
        let mut i = 0;
        while i != self.shown.len() {
            match self.matcher.fuzzy_match(&self.shown[i].name, &self.query) {
                None => {
                    self.shown[i].score = 0;
                    self.hidden.push(self.shown.remove(i));
                }
                Some(score) => {
                    self.shown[i].score = score;
                    i += 1;
                }
            }
        }

        i = 0;
        while i != self.hidden.len() {
            if let Some(score) = self.matcher.fuzzy_match(&self.hidden[i].name, &self.query) {
                self.hidden[i].score = score;
                self.shown.push(self.hidden.remove(i));
            } else {
                i += 1;
            }
        }

        // Sort the vector (should use our custom Cmp)
        self.shown.sort();

        if self.shown.is_empty() {
            self.selected = None;
            self.log.push(Text::raw("NO ITEMS!"));
        } else {
            self.selected = Some(0);
        }

        self.log.push(Text::raw("update_filter\n"));
    }
}
