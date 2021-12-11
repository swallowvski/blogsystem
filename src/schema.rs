pub mod model;
pub mod setup;

table! {
    pages (id) {
        id -> Nullable<Integer>,
        posttime -> Text,
        title -> Text,
        body -> Text,
    }
}
