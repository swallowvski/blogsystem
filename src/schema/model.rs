use async_graphql::*;
use chrono::{DateTime, Local};
use std::{
    fs::{read_dir, remove_file, File},
    io::prelude::*,
    vec,
};

use crate::PAGES_PATH;

#[derive(Clone, SimpleObject)]
pub struct Page {
    title: String,
    text: String,
}

pub struct Query;

#[Object]
impl Query {
    async fn total_pages(&self) -> Vec<Page> {
        let paths = read_dir(PAGES_PATH).unwrap();
        let mut pages = vec![];

        for path in paths {
            let p = path.unwrap().path();
            let splited_path = &p.to_str().unwrap().split('/').collect::<Vec<&str>>();
            let title_extension = splited_path[splited_path.len() - 1]
                .split('.')
                .collect::<Vec<&str>>();
            let title = title_extension[0].to_string();

            let mut file = File::open(p).unwrap();
            let mut text = String::new();
            file.read_to_string(&mut text).unwrap();
            println!("get title {} text {}", &title, &text);
            let page = Page { title, text };
            pages.push(page);
            break;
        }

        pages
    }

    async fn search_pages(&self, title: String) -> Result<Option<Page>> {
        println!("search");
        let paths = read_dir(PAGES_PATH).unwrap();
        let mut page = None;
        for path in paths {
            let p = path.unwrap().path();
            let splited_path = &p.to_str().unwrap().split('/').collect::<Vec<&str>>();
            let title_extension = splited_path[splited_path.len() - 1]
                .split('.')
                .collect::<Vec<&str>>();
            let origin_title = title_extension[0].to_string();
            println!("Search {}, {}", title, origin_title);
            if title == origin_title {
                let mut file = File::open(p).unwrap();
                let mut text = String::new();
                file.read_to_string(&mut text).unwrap();
                println!("get title {} text {}", &title, &text);
                page = Some(Page {
                    title: origin_title,
                    text,
                });
                break;
            }
        }
        Ok(page)
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn post_pages(&self, title: String, text: String) -> Page {
        let pages = PAGES_PATH.to_string();
        let mut file = File::create(format!("{}/{}.txt", pages, title)).unwrap();
        file.write_all(text.as_bytes()).unwrap();
        println!("write title {} text {}", &title, &text);
        let page = Page { title, text };
        page
    }

    async fn delete_page(&self, title: String) -> String {
        let pages = PAGES_PATH.to_string();
        if let Ok(_) = remove_file(format!("{}/{}.txt", pages, title)) {
            format!("Ok, removed {}", title)
        } else {
            format!("Can't remove {}", title)
        }
    }

    async fn update_pages(&self, title: String, text: String) -> String {
        let paths = read_dir(PAGES_PATH).unwrap();
        for path in paths {
            let p = path.unwrap().path();
            let splited_path = &p.to_str().unwrap().split('/').collect::<Vec<&str>>();
            let title_extension = splited_path[splited_path.len() - 1]
                .split('.')
                .collect::<Vec<&str>>();
            let origin_title = title_extension[0].to_string();
            if title == origin_title {
                println!("update {}, {}", title, origin_title);
                let mut file = File::open(p).unwrap();
                file.write_all(text.as_bytes()).unwrap();
                return "Updated Page".to_string();
            }
        }
        "Oh, No such Page".to_string()
    }
}
