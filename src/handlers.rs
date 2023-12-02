use crate::database::DataBase;
use crate::models::Post;

use iron::headers::ContentType;
use iron::{status, AfterMiddleware, Handler, IronResult, Request, Response};
use router::Router;
use std::error::Error;
use std::io::Read;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

macro_rules! try_handler {
    ($e:expr) => {
        match $e {
            Ok(x) => x,
            Err(e) => return Ok(Response::with((status::InternalServerError, e.to_string()))),
        }
    };
    ($e:expr, $error:expr) => {
        match $e {
            Ok(x) => x,
            Err(e) => return Ok(Response::with(($error, e.to_string()))),
        }
    };
}

macro_rules! lock {
    ($e:expr) => {
        $e.lock().unwrap()
    };
}
macro_rules! get_http_param {
    ($r:expr, $e:expr) => {
        match $r.extensions.get<Router()> {
            Some(router) => {
                match router.find($e) {
                    Some(v) => v,
                    None => return Ok(Response::with((status::BadRequest))),
                }
            }
            None => return Ok(Response::with((status::InternalServerError))),
        }
    }
}

pub struct Handlers {
    pub post_feed: PostFeedHandler,
    pub post_post: PostPostHandler,
    // pub get_post: GetPostHandler,
}

impl Handlers {
    pub fn new(db: DataBase) -> Handlers {
        let database = Arc::new(Mutex::new(db));

        Handlers {
            post_feed: PostFeedHandler::new(database.clone()),
            post_post: PostPostHandler::new(database.clone()),
            // get_post: GetPostHandler::new(database.clone()),
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

pub struct JsonAfterMiddleware;

impl AfterMiddleware for JsonAfterMiddleware {
    fn after(&self, _: &mut Request, mut res: Response) -> IronResult<Response> {
        res.headers.set(ContentType::json());
        Ok(res)
    }
}
