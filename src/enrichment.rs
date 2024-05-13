use crate::hamster::HamsterFact;
use crate::utils::{LinkText, MarkdownProcessing};
use markdown::mdast::Node;
use markdown::ParseOptions;
use regex::Regex;

pub struct TaskLink {
    pub link_title: String,
    pub href: String,
    pub task_id: String,
}

pub trait HamsterEnrichedData {
    fn task(&self) -> Option<TaskLink>;
    /// Extracts comments
    fn comments(&self) -> Vec<String>;
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

    /// extracts comments, but with some catches
    fn comments(&self) -> Vec<String> {
        let markdown_root =
            markdown::to_mdast(&self.description, &ParseOptions::default()).unwrap();
        markdown_root
            .children()
            .unwrap()
            .into_iter()
            .filter_map(|node| match node {
                Node::List(_) => Some(node.texts()),
                _ => None,
            })
            .flatten()
            .map(|text| text.value.clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime, TimeDelta};

    use crate::hamster::HamsterFact;

    use super::HamsterEnrichedData;

    fn get_fact_with_descr(description: String) -> HamsterFact {
        let timezone = Local::now().timezone();
        let start_time = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 5, 12).unwrap(),
            NaiveTime::from_hms_opt(10, 33, 0).unwrap(),
        )
        .and_local_timezone(timezone)
        .unwrap();
        HamsterFact {
            id: 1,
            start_time: start_time,
            end_time: Some(start_time + TimeDelta::new(3600, 0).unwrap()),
            description: description,
            activity: String::from("running and jumping"),
            category: String::from("Sports"),
        }
    }

    #[test]
    fn ensure_task_extracted_correctly() {
        let fact = get_fact_with_descr(String::from(
            "[Some task](https://example.com/task/123456/f)",
        ));

        let extracted_task = fact.task().unwrap();

        assert_eq!(extracted_task.link_title, String::from("Some task"));
        assert_eq!(
            extracted_task.href,
            String::from("https://example.com/task/123456/f")
        );
        assert_eq!(extracted_task.task_id, String::from("123456"));
    }

    #[test]
    fn simple_comments_extracted_correctly() {
        let fact = get_fact_with_descr(String::from(
            "[Some task](https://example.com/task/123456/f)\n\
            + some running\n\
            - some jumping",
        ));
        let comments = fact.comments();
        assert_eq!(comments, ["some running", "some jumping"]);
    }
}
