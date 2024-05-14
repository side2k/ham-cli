use chrono::{Datelike, Days, NaiveDate};
use markdown::mdast::{Link, Node, Text};
use std::time::Duration;

pub fn week_start(date: NaiveDate) -> NaiveDate {
    date.checked_sub_days(Days::new(date.weekday().num_days_from_monday() as u64))
        .unwrap()
}

pub trait DurationFormatting {
    fn duration_minutes(&self) -> u64;
    fn as_hhmm(&self) -> String {
        let minutes_total = self.duration_minutes();
        let hours = minutes_total / 60;
        let minutes = minutes_total % 60;

        format!("{hh}:{mm:02}", hh = hours, mm = minutes)
    }
}

impl DurationFormatting for Duration {
    fn duration_minutes(&self) -> u64 {
        self.as_secs() / 60
    }
}

pub trait MarkdownProcessing {
    fn flatten_tree(&self) -> Vec<&Node>;
    fn links(&self) -> Vec<&Link> {
        self.flatten_tree()
            .into_iter()
            .filter_map(|node| match node {
                Node::Link(link) => Some(link),
                _ => None,
            })
            .collect()
    }
    fn texts(&self) -> Vec<&Text> {
        self.flatten_tree()
            .into_iter()
            .filter_map(|node| match node {
                Node::Text(text) => Some(text),
                _ => None,
            })
            .collect()
    }
}

impl MarkdownProcessing for Node {
    fn flatten_tree(&self) -> Vec<&Node> {
        match self.children() {
            None => vec![self],
            Some(children) => vec![self]
                .into_iter()
                .chain(children.iter().flat_map(|child| child.flatten_tree()))
                .collect(),
        }
    }
}

pub trait LinkText {
    fn text(&self) -> String;
}

impl LinkText for Link {
    fn text(&self) -> String {
        self.children
            .iter()
            .fold(String::new(), |acc, node| match node {
                Node::Text(text) => acc + &String::from(&text.value),
                _ => String::new(),
            })
    }
}

pub fn unique_lines(lines: Vec<String>) -> Vec<String> {
    let mut unique = vec![];
    let mut by_text: Vec<(usize, String)> = lines.into_iter().enumerate().collect();
    by_text.sort_by_key(|(_, line)| line.clone());

    let mut last_line = String::from("");

    for (line_index, line) in by_text {
        if line != last_line {
            unique.push((line_index, line.clone()));
        }
        last_line = line;
    }

    unique.sort_by_key(|(index, _)| index.clone());
    unique.into_iter().map(|(_, line)| line).collect()
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use chrono::NaiveDate;
    use markdown::ParseOptions;

    use crate::utils::{DurationFormatting, MarkdownProcessing};

    use super::week_start;

    #[test]
    fn week_start_works() {
        assert_eq!(
            week_start(NaiveDate::from_ymd_opt(2024, 04, 26).unwrap()),
            NaiveDate::from_ymd_opt(2024, 04, 22).unwrap()
        )
    }

    #[test]
    fn duration_as_hhmm_works() {
        assert_eq!(Duration::new(0, 0).as_hhmm(), String::from("0:00"));
        assert_eq!(Duration::new(60, 0).as_hhmm(), String::from("0:01"));
        assert_eq!(Duration::new(65, 0).as_hhmm(), String::from("0:01"));
        assert_eq!(Duration::new(600, 0).as_hhmm(), String::from("0:10"));
        assert_eq!(Duration::new(4000, 0).as_hhmm(), String::from("1:06"));
        assert_eq!(Duration::new(3600 * 10, 0).as_hhmm(), String::from("10:00"));
        assert_eq!(
            Duration::new(3600 * 100, 1).as_hhmm(),
            String::from("100:00")
        );
    }

    #[test]
    fn texts_extracted_correctly() {
        let markdown = String::from(
            "- first item\n\
            - second item\n - item 2.1\n    - item 2.1.1\n  - item 2.2\n\
            - item 3",
        );
        let root_node = markdown::to_mdast(&markdown, &ParseOptions::default()).unwrap();
        assert_eq!(
            root_node
                .texts()
                .into_iter()
                .map(|text| text.value.clone())
                .collect::<Vec<String>>(),
            vec![
                "first item",
                "second item",
                "item 2.1",
                "item 2.1.1",
                "item 2.2",
                "item 3"
            ]
        );
    }
}
