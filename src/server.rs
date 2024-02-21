use crate::{http::request::Request, router::Router, LioneError};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

pub struct Server {
    listener: TcpListener,
    pub routes: Router,
}

impl Server {
    pub fn new(listener: TcpListener) -> Self {
        Self {
            listener: listener,
            routes: Router::new(),
        }
    }

    pub async fn run(&self) {
        loop {
            let incoming = self.listener.accept().await;

            match incoming {
                Ok((mut stream, _)) => {
                    let router = self.routes.clone();
                    tokio::spawn(async move { Self::handle_connection(&mut stream, router).await });
                }
                Err(e) => println!("{:?}", e),
            };
        }
    }

    async fn handle_connection(stream: &mut TcpStream, router: Router) -> Result<(), LioneError> {
        loop {
            let mut buf = [0; 1024];
            _ = stream.read(&mut buf).await;
            match Request::try_from(&buf) {
                Ok(r) => {
                    let response = router.handle(r);
                    stream.write_all(response.to_string().as_bytes()).await?;
                    stream.shutdown().await?;
                }
                Err(_) => break
            };
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::response::Response;
    use crate::http::statuscode::StatusCode;

    #[tokio::test]
    async fn run_server() {
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        let mut server = Server::new(listener);
        server
            .routes
            .get("/", |request| Response::new().text("Hello from lione!"))
            .get("/hello", hello)
            .get("/bye", bye);
        server.run().await;
    }

    fn hello(request: Request) -> Response {
        Response::new().text(&format!("You requested: {}", &request.path))
    }

    fn bye(request: Request) -> Response {
        Response::new()
            .header("HX-Redirect", "/app")
            .status(StatusCode::Ok)
    }
}
