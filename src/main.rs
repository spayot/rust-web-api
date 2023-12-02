mod database;
mod handlers;
mod models;

use database::DataBase;
use handlers::*;
use models::Post;

use iron::{prelude::Chain, Iron};
use logger::Logger;
use router::Router;
use uuid::Uuid;

fn main() {
    env_logger::init();
    let (logger_before, logger_after) = Logger::new(None);
    let mut db = DataBase::new();

    let p1 = Post::new(
        "The First Title",
        "Mr Nobody",
        "this is the content of this post.",
        chrono::offset::Utc::now(),
        Uuid::new_v4(),
    );

    let p2 = Post::new(
        "A Second Title",
        "Mr Everybody",
        "that's barely better than the previous post.",
        chrono::offset::Utc::now(),
        Uuid::new_v4(),
    );

    let p3 = Post::new(
        "Yet a Third Title",
        "Mr Somebody",
        "now, that can't get any better. \ncan it?",
        chrono::offset::Utc::now(),
        Uuid::new_v4(),
    );

    for p in [p1, p2, p3] {
        db.add_post(p);
    }

    let handlers = Handlers::new(db);
    let json_after_middleware = JsonAfterMiddleware;

    let mut router = Router::new();

    // endpoint to get all posts from the database
    router.get("/post_feed", handlers.post_feed, "post_feed");

    // endpoint to post a new post
    router.post("/post_post", handlers.post_post, "post_post");

    let mut chain = Chain::new(router);
    chain.link_before(logger_before);
    chain.link_after(json_after_middleware);
    chain.link_after(logger_after);

    Iron::new(chain).http("localhost:9200").unwrap();
}
