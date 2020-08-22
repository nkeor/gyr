use fuzzy_matcher::{FuzzyMatcher, skim::SkimMatcherV2};

use tui::style::{Color, Style};
use tui::widgets::Text;

use super::apps;

pub struct UI<'a> {
    pub hidden: Vec<apps::Application>,
    pub shown: Vec<apps::Application>,
    pub selected: Option<usize>,
    pub text: Vec<Text<'a>>,
    pub query: String,
    pub log: Vec<Text<'a>>,
    pub verbose: u64,
    #[doc(hidden)]
    matcher: SkimMatcherV2,
}

impl<'a> UI<'a> {
    pub fn new(items: Vec<apps::Application>) -> UI<'a> {
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

    pub fn verbose(&mut self, b: u64) {
        self.verbose = b;
    }

    pub fn update_info(&mut self, color: Color) {
        if let Some(selected) = self.selected {
            self.text = vec![
                Text::styled(
                    format!("{}\n\n", &self.shown[selected].name),
                    Style::default().fg(color),
                ),
                Text::raw(format!("{}\n", &self.shown[selected].description)),
            ];
            if self.verbose > 0 {
                self.text.push(if self.shown[selected].terminal_exec {
                    Text::raw("\nExec (terminal): ")
                } else {
                    Text::raw("\nExec: ")
                });
                self.text.push(Text::styled(
                    self.shown[selected].exec.to_string(),
                    Style::default().fg(Color::DarkGray),
                ));
                if self.verbose > 1 {
                    self.text.push(Text::raw(format!("\nMatching score: {}", self.shown[selected].score)));
                }
            }
        } else {
            self.text.clear();
        }
    }

    pub fn update_filter(&mut self) {
        let mut i = 0;
        while i != self.shown.len() {
            match self.matcher.fuzzy_match(&self.shown[i].name, &self.query) {
                None => {
                    self.shown[i].score = 0;
                    self.hidden.push(self.shown.remove(i));
                },
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

        // NOTE: We're not using Vec::sort(), because it doesn't sort it the way we want
        self.shown.sort_by(|a, b| a.cmp(b));

        if self.shown.is_empty() {
            self.selected = None;
            self.log.push(Text::raw("NO ITEMS!"));
        } else {
            self.selected = Some(0);
        }

        self.log.push(Text::raw("update_filter\n"));
    }
}
