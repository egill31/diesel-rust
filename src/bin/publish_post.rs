use diesel_rust::models::Post;
use diesel::prelude::*;
use diesel_rust::*;
use std::env::args;

fn main() {
    use diesel_rust::schema::posts::dsl::{posts, published};

    let id = args()
        .nth(1)
        .expect("publishpost requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = &mut establish_connection();

    let post = diesel::update(posts.find(id))
        .set(published.eq(true))
        .get_result::<Post>(connection)
        .unwrap();
    println!("Published post {}", post.title);

}