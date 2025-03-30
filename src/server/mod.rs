use std::{
    collections::HashMap,
    error::Error,
    net::{TcpListener, TcpStream, ToSocketAddrs},
};

use crate::models::{method::Method, request::Request};

pub struct App {
    pub listener: TcpListener,
    routes: HashMap<String, MethodHandlerMap>,
}

impl App {
    pub fn new<T: ToSocketAddrs>(addr: T) -> Self {
        Self {
            listener: TcpListener::bind(addr).unwrap(),
            routes: HashMap::new(),
        }
    }

    pub fn get(&mut self, route: String, handler: RequestHandler) {
        // self.routes.entry(route)
    }

    pub fn build(self) -> Self {
        Self {
            listener: self.listener,
            routes: self.routes,
        }
    }
}

pub type MethodHandlerMap = HashMap<Method, RequestHandler>;

pub type RequestHandler = fn(&Request, &mut TcpStream) -> Result<(), Box<dyn Error>>;
