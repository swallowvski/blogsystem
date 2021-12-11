use async_graphql::*;
use chrono::{DateTime, Local};
use diesel::connection;
use std::{
    fs::{read_dir, remove_file, File},
    io::prelude::*,
    vec,
};
use tracing_subscriber::registry::SpanRef;

use super::setup::establish_connection;
use crate::schema::pages;
use diesel::prelude::*;

#[derive(Clone, SimpleObject, Queryable)]
pub struct Page {
    id: Option<i32>,
    posttime: String,
    title: String,
    body: String,
}

#[derive(Insertable)]
#[table_name = "pages"]
struct NewPage<'a> {
    title: &'a str,
    body: &'a str,
}

pub struct Query;

#[Object]
impl Query {
    async fn total_pages(&self) -> Vec<Page> {
        use crate::schema::pages::dsl::*;
        let conn = establish_connection();
        let results = pages.load::<Page>(&conn).expect("Error loading posts");
        println!("Displaying {} posts", results.len());
        for post in &results {
            println!("{}", post.title);
            println!("----------\n");
            println!("{}", post.body);
        }
        results
    }

    async fn search_pages(&self, post_id: i32) -> Result<Option<Page>> {
        use crate::schema::pages::dsl::*;

        let conn = establish_connection();

        let results = pages
            .filter(id.eq(post_id))
            .limit(1)
            .load::<Page>(&conn)
            .expect("Error loading posts")[0]
            .clone();

        Ok(Some(results))
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn post_pages(&self, title: String, body: String) -> String {
        let conn = establish_connection();

        let pages_id = create_page(&conn, &title, &body);

        format!(
            "Created Page:\nid: {}\ntitle:{}\ntext:\n{}",
            pages_id, title, body
        )
    }

    async fn delete_page(&self, delete_id: i32) -> String {
        use crate::schema::pages::dsl::*;

        let conn = establish_connection();
        let num_deleted = diesel::delete(pages.filter(id.eq(delete_id)))
            .execute(&conn)
            .expect("Error deleting pages");
        format!("Deleted pages id {}", num_deleted)
    }

    async fn update_pages(&self, id: i32, new_title: String, new_body: String) -> String {
        use crate::schema::pages::dsl::{body, pages, title};
        let conn = establish_connection();
        let updated_id = diesel::update(pages.find(id))
            .set((title.eq(new_title), body.eq(new_body)))
            .execute(&conn)
            .unwrap_or_else(|_| panic!("Unable to find post"));

        format!("Updated pages id {}", updated_id)
    }
}

fn create_page(conn: &SqliteConnection, title: &str, body: &str) -> usize {
    use crate::schema::pages;
    let new_post = NewPage { title, body };

    diesel::insert_into(pages::table)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post")
}
