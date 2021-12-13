use async_graphql::*;
use chrono::Local;

use super::setup::establish_connection;
use crate::schema::pages;
use diesel::prelude::*;

#[derive(Debug, Clone, SimpleObject, Queryable)]
pub struct Page {
    id: Option<i32>,
    posttime: String,
    title: String,
    body: String,
}

#[derive(Insertable)]
#[table_name = "pages"]
struct NewPage<'a> {
    posttime: &'a str,
    title: &'a str,
    body: &'a str,
}

pub struct Query;

#[Object]
impl Query {
    async fn total_pages(&self) -> FieldResult<Vec<Page>> {
        use crate::schema::pages::dsl::*;
        let conn = establish_connection();
        let results = pages.load::<Page>(&conn).expect("Error loading posts");
        if results.len() == 0 {
            return Err(
                FieldError::new("PagesError").extend_with(|_, e| e.set("pages", "No such pages"))
            );
        }
        println!("Displaying {} posts", results.len());
        for post in &results {
            println!("{}", post.title);
            println!("----------\n");
            println!("{}", post.body);
        }
        Ok(results)
    }

    async fn search_pages(&self, post_id: i32) -> FieldResult<Page> {
        use crate::schema::pages::dsl::*;

        let conn = establish_connection();
        println!("Search {}", post_id);

        if let Some(page) = pages
            .filter(id.eq(post_id))
            .load::<Page>(&conn)
            .expect("Error loading posts")
            .clone()
            .get(0)
        {
            Ok(page.clone())
        } else {
            Err(FieldError::new("PagesError").extend_with(|_, e| e.set("pages", "No such page")))
        }
    }
}

pub struct Mutation;

#[Object]
impl Mutation {
    async fn post_pages(&self, title: String, body: String) -> FieldResult<String> {
        let posttime: &str = &Local::now().format("%F/%T/%Z").to_string();
        let conn = establish_connection();

        if let Ok(pages_id) = create_page(&conn, posttime, &title, &body) {
            Ok(format!(
                "Created Page:\nid: {}\ntitle:{}\ntext:\n{}\n{}",
                pages_id, title, body, posttime
            ))
        } else {
            Err(FieldError::new("PagesError")
                .extend_with(|_, e| e.set("pages", "Error saving new page")))
        }
    }

    async fn delete_page(&self, delete_id: i32) -> FieldResult<String> {
        use crate::schema::pages::dsl::*;

        let conn = establish_connection();
        if let Ok(num_deleted) = diesel::delete(pages.filter(id.eq(delete_id))).execute(&conn) {
            Ok(format!("Deleted pages id {}", num_deleted))
        } else {
            Err(FieldError::new("PagesError")
                .extend_with(|_, e| e.set("pages", "Error deleting page")))
        }
    }

    async fn update_pages(
        &self,
        id: i32,
        new_title: String,
        new_body: String,
    ) -> FieldResult<String> {
        use crate::schema::pages::dsl::{body, pages, posttime, title};

        let new_posttime: &str = &chrono::Local::now().format("%F/%T/%Z").to_string();
        let conn = establish_connection();
        if let Ok(updated_id) = diesel::update(pages.find(id))
            .set((
                title.eq(new_title),
                body.eq(new_body),
                posttime.eq(new_posttime),
            ))
            .execute(&conn)
        {
            Ok(format!("Updated pages id {}", updated_id))
        } else {
            Err(FieldError::new("PagesError")
                .extend_with(|_, e| e.set("pages", "Error deleting page")))
        }
    }
}

fn create_page(
    conn: &SqliteConnection,
    posttime: &str,
    title: &str,
    body: &str,
) -> Result<usize, diesel::result::Error> {
    let new_post = NewPage {
        posttime,
        title,
        body,
    };

    diesel::insert_into(pages::table)
        .values(&new_post)
        .execute(conn)
}

#[test]
fn test_post() {
    use crate::schema::pages::dsl::*;

    let conn = establish_connection();

    let results = pages
        .filter(id.eq(1))
        .load::<Page>(&conn)
        .expect("Error loading posts")[0]
        .clone();
    println!("{:#?}", results);
}
