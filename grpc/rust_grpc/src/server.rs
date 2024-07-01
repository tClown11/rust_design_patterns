use tokio_util::sync::CancellationToken;
use tonic::{transport::Server, Request, Response, Status};
use crate::greeter::greeter_server::{Greeter, GreeterServer};
use crate::greeter::{HelloReply, HelloRequest};


#[derive(Debug, Default)]
pub struct MyGreeter{}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(
        &self,
        request: Request<HelloRequest>,
    ) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", request);

        let reply = crate::greeter::HelloReply{
            message: format!("Hello {}!", request.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }
}

pub async fn run_server(token: CancellationToken) -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve_with_shutdown(addr, token.cancelled())
        .await?;

    Ok(())
}