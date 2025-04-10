use std::{
    collections::HashMap,
    error::Error,
    io::Write,
    net::{TcpListener, TcpStream, ToSocketAddrs},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use crate::{
    models::{
        encoding::EncodingType,
        method::Method,
        request::{ReqError, Request},
        response::Response,
        status::Status,
    },
    router::Router,
    thread_pool::ThreadPool,
};

#[derive(Debug)]
pub struct App {
    listener: TcpListener,
    routes: HashMap<String, MethodHandlerMap>,
    pool: ThreadPool,
    encoding_types: Vec<EncodingType>,
    shutdown_flag: Arc<AtomicBool>,
}

impl App {
    pub fn new<T: ToSocketAddrs>(addr: T) -> Self {
        Self {
            listener: TcpListener::bind(addr).expect("Invalid bind address."),
            routes: HashMap::new(),
            pool: ThreadPool::new(5),
            encoding_types: vec![EncodingType::Gzip],
            shutdown_flag: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn get(self, route: impl Into<String>, handler: RequestHandler) -> Self {
        self.add_route(Method::Get, route, handler)
    }

    #[allow(dead_code)]
    pub fn post(self, route: impl Into<String>, handler: RequestHandler) -> Self {
        self.add_route(Method::Post, route, handler)
    }

    #[allow(dead_code)]
    pub fn patch(self, route: impl Into<String>, handler: RequestHandler) -> Self {
        self.add_route(Method::Patch, route, handler)
    }

    #[allow(dead_code)]
    pub fn put(self, route: impl Into<String>, handler: RequestHandler) -> Self {
        self.add_route(Method::Put, route, handler)
    }

    #[allow(dead_code)]
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

    pub fn build(self) -> Arc<Self> {
        Arc::new(self)
    }

    pub fn run(self: Arc<Self>) {
        println!("Routes: {:#?}", self.routes);

        for stream in self.listener.incoming() {
            if self.shutdown_flag.load(std::sync::atomic::Ordering::SeqCst) {
                println!("Shutdown flag set. Exiting server loop.");
                break;
            }
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
        let Ok(req) = Request::try_from(&mut stream) else {
            return Ok(());
        };

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

    fn send_404(_: &Request, stream: &mut TcpStream) {
        let res = Response::default().status(Status::NotFound).to_bytes();

        if let Err(e) = stream.write_all(&res) {
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

    pub fn shutdown(&self) {
        self.shutdown_flag.store(true, Ordering::SeqCst);

        // Send a dummy request to unblock listener
        let _ = TcpStream::connect(self.listener.local_addr().unwrap());
    }
}

pub type MethodHandlerMap = HashMap<Method, RequestHandler>;

pub type RequestHandler = fn(&Request, Response) -> Result<Response, Box<dyn Error>>;

pub type ServerResponse = Result<Response, Box<dyn Error>>;
