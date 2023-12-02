use crate::database::DataBase;
use crate::models::Post;

use iron::headers::ContentType;
use iron::{status, AfterMiddleWare, Handler, IronResult, Request, Response};
use router::Router;
use std::error::Error;
use std::io::Read;
use std::sync::{Arc, Mutex};
use uuid::Uuid;
