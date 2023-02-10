pub mod sample {
    tonic::include_proto!("sample");
}

use sample::{sample_service_client::SampleServiceClient, ImageStream};
use tonic::Request;
use tokio::time;
use std::time::Duration;

use std::sync::mpsc;

use opencv::{
    prelude::*,
    videoio,
    imgcodecs::*
};

use opencv::core::Vector;


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
        let mut interval = time::interval(Duration::from_millis(150));

        let mut cam = videoio::VideoCapture::new(0, videoio::CAP_ANY).unwrap();
        let mut frame = Mat::default();

        loop {

            if let Ok(()) = rx.try_recv() {
                cam.release()
                    .expect("closing cam didn't work");
                break;
            };

            cam.read(&mut frame).unwrap();
            
            let mut jpg_buf: Vector<u8> = Vector::new();
            let params: Vector<i32> = Vector::new();

            match imencode(".jpg", &frame, &mut jpg_buf, &params) {
                Ok(b) => {
                    println!("Encode result {} with size {}", b, jpg_buf.len());
                }
                Err(e) => {
                    println!("Converting to image didn't work: {e}");
                }
            }

            let time = interval.tick().await; //delay            
            let elapsed = time.duration_since(start);

            let i = ImageStream {
                image: jpg_buf.as_slice().to_owned()
            };
            println!("Time passed {:?}", elapsed);

            yield i;
        }
    };

    let resp =  client.send_stream(Request::new(outbound)).await?;    
    let inbound = resp.into_inner();

    println!("Server answered {}", inbound.message);

    Ok(())
}