pub mod sample {
    tonic::include_proto!("sample");
}

use sample::{StreamResponse, ImageStream};
use sample::sample_service_server::SampleServiceServer;
use tonic::{transport::Server, Request, Response, Status, Streaming};
use futures::StreamExt;

#[derive(Debug)]pub struct MyService {}
#[tonic::async_trait]impl sample::sample_service_server::SampleService for MyService {
    async fn send_stream(
        &self,
        request: Request<Streaming<ImageStream>>,
    ) -> Result<Response<StreamResponse>, Status> {
        println!("receiving stream message");
        let mut stream = request.into_inner();
        let mut stream_response = StreamResponse::default();
        stream_response.message = String::from("test response");
        while let Some(image) = stream.next().await {
            let image = image.unwrap();
            println!("received bytes {}", image.image.len());
        } 

        Ok(Response::new(StreamResponse {message: String::from("test")}))
    }
}

#[tokio::main]async fn main()  -> Result<(), Box<dyn std::error::Error>> {

    //let addr = "[::1]:10000".parse().unwrap();    
    let addr = "127.0.0.1:10000".parse().unwrap();
    println!("Sample server listening on: {}", addr);

    let sample_service = MyService {};
    let svc = SampleServiceServer::new(sample_service);

    Server::builder().add_service(svc).serve(addr).await?;
    Ok(())
}