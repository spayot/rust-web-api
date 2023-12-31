use crate::database::DataBase;
use crate::models::Post;

use iron::headers::ContentType;
use iron::{status, AfterMiddleware, Handler, IronResult, Request, Response};
use router::Router;
use std::io::Read;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::{get_http_param, lock, try_handler};

pub struct Handlers {
    pub post_feed: PostFeedHandler,
    pub post_post: PostPostHandler,
    pub get_post: GetPostHandler,
}

impl Handlers {
    pub fn new(db: DataBase) -> Handlers {
        let database = Arc::new(Mutex::new(db));

        Handlers {
            post_feed: PostFeedHandler::new(database.clone()),
            post_post: PostPostHandler::new(database.clone()),
            get_post: GetPostHandler::new(database.clone()),
        }
    }
}

pub struct PostFeedHandler {
    database: Arc<Mutex<DataBase>>,
}

impl PostFeedHandler {
    fn new(database: Arc<Mutex<DataBase>>) -> PostFeedHandler {
        PostFeedHandler { database }
    }
}

impl Handler for PostFeedHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let payload = try_handler!(serde_json::to_string(lock!(self.database).posts()));
        Ok(Response::with((status::Ok, payload)))
    }
}

pub struct PostPostHandler {
    database: Arc<Mutex<DataBase>>,
}

impl PostPostHandler {
    pub fn new(db: Arc<Mutex<DataBase>>) -> Self {
        PostPostHandler { database: db }
    }
}

impl Handler for PostPostHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let mut payload = String::new();
        try_handler!(req.body.read_to_string(&mut payload));
        let post: Post = try_handler!(serde_json::from_str(&payload), status::BadRequest);
        lock!(self.database).add_post(post);
        Ok(Response::with((status::Created, payload)))
    }
}

pub struct GetPostHandler {
    database: Arc<Mutex<DataBase>>,
}

impl GetPostHandler {
    pub fn new(db: Arc<Mutex<DataBase>>) -> Self {
        GetPostHandler { database: db }
    }

    fn find_post(&self, id: &Uuid) -> Option<Post> {
        let locked = lock!(self.database);
        let mut iterator = locked.posts().iter();
        iterator.find(|p| p.uuid() == id).map(|p| p.clone())
    }
}

impl Handler for GetPostHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let ref id = get_http_param!(req, "id");
        let id = try_handler!(Uuid::parse_str(id), status::BadRequest);
        let post = self.find_post(&id);
        match post {
            None => Ok(Response::with(status::NotFound)),
            Some(post) => {
                let payload =
                    try_handler!(serde_json::to_string(&post), status::InternalServerError);
                Ok(Response::with((status::Ok, payload)))
            }
        }
    }
}

pub struct JsonAfterMiddleware;

impl AfterMiddleware for JsonAfterMiddleware {
    fn after(&self, _: &mut Request, mut res: Response) -> IronResult<Response> {
        res.headers.set(ContentType::json());
        Ok(res)
    }
}
