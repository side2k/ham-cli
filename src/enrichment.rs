use crate::hamster::HamsterFact;
use crate::utils::{LinkText, MarkdownProcessing};
use markdown::ParseOptions;

pub struct TaskLink {
    pub link_title: String,
    pub href: String,
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
            })
        }
    }
}
