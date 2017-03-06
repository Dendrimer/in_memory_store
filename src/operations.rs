use std::collections::HashMap;
use std::cell::RefCell;

pub struct Session {
    store: RefCell<HashMap<&'static str, &'static str>>,
}

impl Session  {
    fn new() -> Session {
        Session {
            store: RefCell::new(HashMap::new()),
        }
    }
}

pub trait SessionHandler {
    fn set(&self, &'static str, &'static str) -> Option<&'static str>;
    fn get(&self, &str) -> Option<&str>;
}

impl SessionHandler for Session {

    fn set(&self, key: &'static str, val: &'static str) -> Option<&'static str> {
        self.store.borrow_mut().insert(key, val)
    }

    fn get(&self, key: &str) -> Option<&str> {
        match self.store.borrow().get(key) {
            Some(val) => Some(val),
            None => None,
        }
    }

}
