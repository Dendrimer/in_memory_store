extern crate tokio_service;
extern crate futures;

use std::collections::HashMap;
use std::cell::RefCell;
use std::io::{Error, ErrorKind};
use self::tokio_service::Service;
use self::futures::{future, Future, BoxFuture};

pub struct Session {
    store: RefCell<HashMap<String, String>>,
}

impl Session {

    pub fn new() -> Session {
        Session {
            store: RefCell::new(HashMap::new()),
        }
    }

    fn set(&self, key: String, val: String) -> String {
        match self.store.borrow_mut().insert(key, val) {
            Some(val) => val.to_string(),
            None => "Insert Successful".to_string(),
        }
    }

    fn get(&self, key: String) -> String {
        match self.store.borrow().get(&key) {
            Some(val) => val.to_string(),
            None => "Element not present".to_string(),
        }
    }

}

impl Service for Session {
    type Request = String;
    type Response = String;
    type Error = Error;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let mut inputs = req.split_whitespace();
        let command : String = inputs.next().unwrap().to_string();
        let mut vec = Vec::new();
        for string in inputs {
            vec.push(string);
        }
        match command.as_ref() {
            "set" => {
                future::ok(Self::set(&self, vec[0].to_string(), vec[1].to_string())).boxed()
            },
            "get" => {
                future::ok(Self::get(&self, vec[0].to_string())).boxed()
            },
            _ => {
                future::err(Error::new(ErrorKind::Other, "whoops")).boxed()
            },
        }
    }
}
