use std::{
    collections::HashMap,
    error::Error,
    io::Write,
    net::{TcpListener, TcpStream, ToSocketAddrs},
    sync::Arc,
};

use codecrafters_http_server::ThreadPool;

use crate::models::{method::Method, request::Request};

#[derive(Debug)]
pub struct App {
    pub listener: TcpListener,
    routes: Arc<HashMap<String, MethodHandlerMap>>,
    pool: ThreadPool,
}

impl App {
    pub fn new<T: ToSocketAddrs>(addr: T) -> Self {
        Self {
            listener: TcpListener::bind(addr).unwrap(),
            routes: Arc::new(HashMap::new()),
            pool: ThreadPool::new(5),
        }
    }

    pub fn get(mut self, route: impl Into<String>, handler: RequestHandler) -> Self {
        let routes = Arc::get_mut(&mut self.routes);
        let routes = routes.unwrap();

        let entry = routes.entry(route.into()).or_default();
        entry.entry(Method::Get).or_insert_with(|| handler);

        Self {
            routes: self.routes,
            ..self
        }
    }

    pub fn build(self) -> Arc<Self> {
        Arc::new(self)
    }

    pub fn run(self: Arc<Self>) {
        for stream in self.listener.incoming() {
            match stream {
                Ok(stream) => {
                    let app = Arc::clone(&self);

                    self.pool.execute(move || {
                        app.handle_connection(stream).unwrap_or_else(|e| {
                            eprintln!("Connection error: {:?}", e);
                        });
                    });
                }
                Err(e) => {
                    println!("error: {}", e);
                }
            }
        }
    }

    fn handle_connection(&self, mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
        let req = Request::try_from(&mut stream).unwrap();

        println!("Req object: {:?}", req);
        //  TODO:  Extract content length and type, then read the body in
        let a: Vec<_> = req.path.split("/").filter(|x| !x.is_empty()).collect::<_>();

        let a = match a.first() {
            Some(s) => String::from(*s),
            None => String::from("/"),
        };

        println!("{:?}", a);

        match self.routes.get_key_value(&a) {
            Some((route, route_handler)) => {
                let entry = route_handler.get_key_value(&req.method);

                match entry {
                    Some((m, handler)) => {
                        println!("Method: {:?}", m);

                        let _ = handler(&req, &mut stream);
                    }
                    None => App::send_404(&req, &mut stream),
                }

                // (route_handler.handler)(&req, &mut stream);
            }
            None => {
                App::send_404(&req, &mut stream);
            }
        }
        Ok(())
    }

    fn send_404(request: &Request, stream: &mut TcpStream) {
        let str = String::from("HTTP/1.1 404 Not Found\r\n\r\n");

        let buf = str.into_bytes();

        println!("sending 404");
        if let Err(e) = stream.write_all(buf.as_slice()) {
            eprintln!("Failed to write response: {:?}", e); // Prevent shutdown on a failed write
        }
    }
}

pub type MethodHandlerMap = HashMap<Method, RequestHandler>;

pub type RequestHandler = fn(&Request, &mut TcpStream) -> Result<(), Box<dyn Error>>;
