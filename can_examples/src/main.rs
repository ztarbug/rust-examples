use socketcan::CANFrame;
use std::thread;
use std::time::Duration;

fn main() {
    let socket = socketcan::CANSocket::open("vcan0").unwrap();
    socket
        .set_nonblocking(true)
        .expect("Couldn't set socket to to non blocking");
    dbg!(&socket);
    loop {
        match socket.read_frame() {
            Ok(frame) => {
                println!("{}", frame_to_string(&frame));
            }
            Err(_) => {}
        }
        thread::sleep(Duration::from_millis(50));
    }
}

fn frame_to_string(f: &CANFrame) -> String {
    let id = &f.id();
    let data_string = f
        .data()
        .iter()
        .fold(String::from(""), |a, b| format!("{} {:02x}", a, b));
    format!("{:08X}  [{}]", id, data_string)
}
