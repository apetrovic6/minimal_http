use std::{
    collections::HashMap,
    error::Error,
    io::{ErrorKind, Write},
    net::{TcpListener, TcpStream, ToSocketAddrs},
    sync::Arc,
};

use codecrafters_http_server::ThreadPool;

use crate::{
    models::{
        encoding::EncodingType,
        method::Method,
        request::{ReqError, Request},
        response::Response,
    },
    router::Router,
};

#[derive(Debug)]
pub struct App {
    listener: TcpListener,
    routes: HashMap<String, MethodHandlerMap>,
    pool: ThreadPool,
    encoding_types: Vec<EncodingType>,
}

impl App {
    pub fn new<T: ToSocketAddrs>(addr: T) -> Self {
        Self {
            listener: TcpListener::bind(addr).expect("Invalid bind address."),
            routes: HashMap::new(),
            pool: ThreadPool::new(5),
            encoding_types: vec![EncodingType::Gzip],
        }
    }

    pub fn get(self, route: impl Into<String>, handler: RequestHandler) -> Self {
        self.add_route(Method::Get, route, handler)
    }

    pub fn post(self, route: impl Into<String>, handler: RequestHandler) -> Self {
        self.add_route(Method::Post, route, handler)
    }

    pub fn patch(self, route: impl Into<String>, handler: RequestHandler) -> Self {
        self.add_route(Method::Patch, route, handler)
    }

    pub fn put(self, route: impl Into<String>, handler: RequestHandler) -> Self {
        self.add_route(Method::Put, route, handler)
    }

    pub fn delete(self, route: impl Into<String>, handler: RequestHandler) -> Self {
        self.add_route(Method::Delete, route, handler)
    }

    fn add_route(
        mut self,
        method: Method,
        route: impl Into<String>,
        handler: RequestHandler,
    ) -> Self {
        let entry = self.routes.entry(route.into()).or_default();
        entry.entry(method).or_insert_with(|| handler);

        Self {
            routes: self.routes,
            ..self
        }
    }

    pub fn add_router(self) {}

    pub fn build(self) -> Arc<Self> {
        Arc::new(self)
    }

    pub fn run(self: Arc<Self>) {
        println!("Routes: {:#?}", self.routes);

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

        let response = Response::default();

        println!("Ugala {:?}", a);

        let Some((_, route_handler)) = self.routes.get_key_value(&a) else {
            App::send_404(&req, &mut stream);
            return Ok(());
        };

        let Some((_, handler)) = route_handler.get_key_value(&req.method) else {
            App::send_404(&req, &mut stream);
            return Ok(());
        };

        match handler(&req, response) {
            Ok(res) => {
                if let Err(e) = stream.write(&res.to_bytes()) {
                    eprintln!("Failed to write response: {:?}", e);
                }
            }
            Err(err) => return Err(err),
        }

        // match self.routes.get_key_value(&a) {
        //     Some((_, route_handler)) => {
        //         let entry = route_handler.get_key_value(&req.method);
        //
        //         match entry {
        //             Some((_, handler)) => {
        //                 let _ = match handler(&req, response) {
        //                     Ok(res) => {
        //                         if let Err(e) = stream.write(&res.to_bytes()) {
        //                             eprintln!("Failed to write response: {:?}", e);
        //                         }
        //
        //                         Result::Ok(())
        //                     }
        //                     Err(err) => Err(err),
        //                 };
        //             }
        //             None => App::send_404(&req, &mut stream),
        //         }
        //     }
        //     None => {
        //         App::send_404(&req, &mut stream);
        //     }
        // }
        Ok(())
    }

    pub fn with_router(mut self, router: Router) -> Self {
        for (route, handlers) in router.into_routes() {
            let entry = self.routes.entry(route).or_default();
            for (method, handler) in handlers {
                entry.insert(method, handler);
            }
        }

        self
    }

    fn send_404(request: &Request, stream: &mut TcpStream) {
        let str = String::from("HTTP/1.1 404 Not Found\r\n\r\n");

        let buf = str.into_bytes();

        println!("sending 404");
        if let Err(e) = stream.write_all(buf.as_slice()) {
            eprintln!("Failed to write response: {:?}", e); // Prevent shutdown on a failed write
        }
    }

    pub fn get_encoding(&self) -> EncodingType {
        if self.encoding_types.contains(&EncodingType::Gzip) {
            EncodingType::Gzip
        } else {
            EncodingType::None
        }
    }
}

pub type MethodHandlerMap = HashMap<Method, RequestHandler>;

pub type RequestHandler = fn(&Request, Response) -> Result<Response, Box<dyn Error>>;
