use crate::hamster::HamsterFact;
use crate::utils::{LinkText, MarkdownProcessing};
use markdown::ParseOptions;
use regex::Regex;

pub struct TaskLink {
    pub link_title: String,
    pub href: String,
    pub task_id: String,
}

pub trait HamsterEnrichedData {
    fn task(&self) -> Option<TaskLink>;
}

impl HamsterEnrichedData for HamsterFact {
    fn task(&self) -> Option<TaskLink> {
        let markdown_root =
            markdown::to_mdast(&self.description, &ParseOptions::default()).unwrap();
        let links = markdown_root.links();

        if links.is_empty() {
            None
        } else {
            let link = links[0];
            Some(TaskLink {
                link_title: link.text(),
                href: link.url.clone(),
                task_id: Regex::new(r"\/(?<task_id>\d+)\/f")
                    .unwrap()
                    .captures(link.url.as_str())
                    .unwrap()["task_id"]
                    .to_string(),
            })
        }
    }
}
