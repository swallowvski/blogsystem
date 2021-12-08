use async_graphql::*;
use chrono::{DateTime, Local};
use once_cell::sync::Lazy;
use std::{sync::Mutex, vec};

#[derive(Clone)]
pub struct Page {
    title: String,
    datetime: String,
    text: String,
}

#[Object]
impl Page {
    async fn title(&self) -> String {
        self.title.clone()
    }

    async fn datetime(&self) -> String {
        self.datetime.clone()
    }

    async fn text(&self) -> String {
        self.text.clone()
    }
}

pub static PAGES: Lazy<Mutex<Vec<Page>>> = Lazy::new(|| Mutex::new(vec![]));

pub struct Query;

#[Object]
impl Query {
    async fn total_pages(&self) -> usize {
        PAGES.lock().unwrap().len()
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn post_pages(&self, title: String, text: String) -> bool {
        let local_datetime: DateTime<Local> = Local::now();
        let datetime = local_datetime.format("%Y/%m/%d %H%M%S %Z").to_string();
        let page = Page {
            title,
            datetime,
            text,
        };
        PAGES.lock().unwrap().push(page);
        true
    }
}
