//! Module that implements traites so that the interface works & typechecks

use rocket::response::Responder;
use rocket::Response;
use rocket::http::Status;
use rocket::request::Request;

pub struct ListWrapper(Vec<String>);

impl ListWrapper {
    pub fn new(list: Vec<String>) -> ListWrapper {
        ListWrapper(list)
    }
}

impl<'a> Responder<'a> for ListWrapper {
    fn respond_to(self, request: &Request) -> Result<Response<'a>, Status> {
        unimplemented!();
    }
}
