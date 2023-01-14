use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;


#[derive(Debug, PartialEq)]
enum CommandType {
    First,
    Second,
    Stop,
    None,
}

impl CommandType {
    pub fn parse_command_string(s: &str) -> CommandType {
        match s {
            "first_command" => CommandType::First,
            "second_command" => CommandType::Second,
            "stop_command" => CommandType::Stop,
            _ => CommandType::None,
        }
    }
}

#[derive(Debug)]
struct Command {
    my_type: CommandType,
    param_01: f64,
    param_02: String,
}

fn main() {
    let lines = read_lines("./commands").unwrap();
    let (tx_command, rx_command) = mpsc::channel();

    let command_reader_handle = thread::spawn(move || {
        let mut run: bool = true;
        for line in lines {
            if let Ok(command_line) = line {
                let c = parse_command(command_line);
                if CommandType::Stop == c.my_type {
                    run = false;
                }
                tx_command.send(c).unwrap();
            }
            if run {
                thread::sleep(Duration::from_millis(100));
            } else {
                break;
            }
        }
    });

    let command_executor_handle = thread::spawn(move || {
        let mut run = true;
        loop {
            match rx_command.try_recv() {
                Ok(c) => {
                    if CommandType::Stop == c.my_type {
                        run = false;
                    } else {
                        execute_command(&c);
                    }
                }
                Err(_e) => {}
            }
            if run {
                thread::sleep(Duration::from_millis(100));
            } else {
                break;
            }
            thread::sleep(Duration::from_millis(50));
        }
    });
    println!("All threads created");
    command_reader_handle.join().unwrap();
    command_executor_handle.join().unwrap();
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_command(command: String) -> Command {
    let trimmed = command.trim();
    let parts = trimmed.split(',');
    let vec: Vec<&str> = parts.collect();
    println!("command parts {},{},{}", vec[0], vec[1], vec[2]);
    let c = CommandType::parse_command_string(vec[0]);
    let param_01 = match vec[1].parse::<f64>() {
        Ok(param_01) => param_01,
        Err(_e) => 0.0,
    };
    Command {
        my_type: c,
        param_01: param_01,
        param_02: vec[2].to_string(),
    }
}

fn execute_command(c: &Command) {
    match c.my_type {
        CommandType::First => {
            println!(
                "Execution of first command will be done here. My parameters are {},{}",
                c.param_01, c.param_02
            );
        }
        CommandType::Second => {
            println!(
                "Execution of second command will be done here. My parameters are {},{}",
                c.param_01, c.param_02
            );
        }
        CommandType::Stop => todo!(),
        CommandType::None => {}
    }
}
