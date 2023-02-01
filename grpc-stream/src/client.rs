pub mod sample {
    tonic::include_proto!("sample");
}

use sample::{sample_service_client::SampleServiceClient, ImageStream};
use tonic::Request;
use tokio::time;
use std::time::Duration;

use std::sync::mpsc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let (tx, rx) = mpsc::channel();

    ctrlc::set_handler(move || {
        tx.send(())
            .expect("sending ctrlc failed");
    })
    .expect("Listening for ctrl+c failed");
    
    let mut client = SampleServiceClient::connect("http://127.0.0.1:10000").await?;
    let start = time::Instant::now();
    
    let outbound = async_stream::stream! {
        let mut interval = time::interval(Duration::from_secs(1));

        loop {

            if let Ok(()) = rx.try_recv() {
                break;
            };

            let mut image_bytes: Vec<u8> = Vec::new();
            image_bytes.push(1);
            image_bytes.push(2);
            image_bytes.push(3);

            let time = interval.tick().await; //delay            
            let elapsed = time.duration_since(start);

            let i = ImageStream {
                image: image_bytes            };
            println!("Time passed {:?}", elapsed);

            yield i;
        }
    };

    let resp =  client.send_stream(Request::new(outbound)).await?;    
    let inbound = resp.into_inner();

    println!("Server answered {}", inbound.message);

    Ok(())
}