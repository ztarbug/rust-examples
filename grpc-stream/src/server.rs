pub mod sample {
    tonic::include_proto!("sample");
}

use chrono::Local;

use opencv::imgcodecs::IMREAD_ANYCOLOR;
use opencv::imgcodecs::*;
use opencv::core::Vector;

use sample::{StreamResponse, ImageStream};
use sample::sample_service_server::SampleServiceServer;
use tonic::{transport::Server, Request, Response, Status, Streaming};
use futures::StreamExt;
use opencv::core::Mat;

#[derive(Debug)]
pub struct MyService {
}

#[tonic::async_trait]
impl sample::sample_service_server::SampleService for MyService {
    
    async fn send_stream(
        &self,
        request: Request<Streaming<ImageStream>>,
    ) -> Result<Response<StreamResponse>, Status> {
        println!("receiving stream message");
        let mut stream = request.into_inner();
        let mut stream_response = StreamResponse::default();
        stream_response.message = String::from("test response");

        while let Some(image) = stream.next().await {
            let image = image.unwrap().image;
            let size = image.len();

            let result = imdecode(&opencv::types::VectorOfu8::from_iter(image),IMREAD_ANYCOLOR).unwrap();
            save_image(&result);

            println!("received bytes {} and converted to {:?}", size, &result);
        }

        Ok(Response::new(StreamResponse {message: String::from("test")}))
    }
}


#[tokio::main]
async fn main()  -> Result<(), Box<dyn std::error::Error>> {

    //let addr = "[::1]:10000".parse().unwrap();    
    let addr = "127.0.0.1:10000".parse().unwrap();
    println!("Sample server listening on: {}", addr);

    let sample_service = MyService {};
    let svc = SampleServiceServer::new(sample_service);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}

fn save_image(image: &Mat) {
    let filename = format!("tmp/received_image_{}.jpg", Local::now());
    let params:Vector<i32> = Vector::new();
    imwrite(&filename, &image, &params)
        .expect("couldn't write file");
}