extern crate redis;
use redis::Commands;
use pedis_core::redis_command::RedisCommand;
use std::rc::Rc;
use std::sync::{Arc,  RwLock};
use std::{collections::HashMap, thread, net::TcpStream, net::TcpListener};
use core::time::Duration;
use std::io::{Read, Write};

fn main() -> std::io::Result<()> {
    thread::spawn(|| {
        thread::sleep(Duration::from_millis(1000));
        let mut client = redis::Client::open("redis://127.0.0.1:8379")
            .expect("something went wrong connecting to pedis");
        let result: () = client
            .set("key", "Hello World")
            .expect("something went wrong running set command");
        println!("{:?}", result);
    });
    let listener = TcpListener::bind("127.0.0.1:8379")?;
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) {
    let mut commands: HashMap<String, Box<dyn pedis_core::RedisCommandHandler>> = HashMap::new();
    commands.insert("config".to_string(), Box::new(pedis_core::config_handler::ConfigHandler {}));
    commands.insert("set".to_string(), Box::new(pedis_core::set_handler::SetHandler {}));

    loop {
        let mut buffer = [0; 1024];
        let bytes_count = stream.read(&mut buffer[..]).unwrap();
        let redis_command = std::str::from_utf8(&buffer[0..bytes_count]).unwrap();

        eprintln!(
            "DEBUG: redis_command{:?} payload_size={:?}",
            redis_command, bytes_count
        );
        let elems = parse_command(redis_command);
        for cmd in elems {
            eprintln!("DEBUG: {:?}", cmd);

            if let Some(handler) = commands.get(&cmd.name())  {
                let mut store = pedis_core::redis_store::RedisStore::default();
                let result = handler.exec(Arc::new(RwLock::new(&mut store)), cmd);
                let _ = stream.write(result.as_bytes());
                continue
            }

            let _ = stream.write("-ERR command not found \r\n".as_bytes());
        }
    }
}


fn parse_command(cmd: &str) -> Vec<Rc<RedisCommand>> {
    let re = regex::Regex::new(r"\*\d*").unwrap();
    let elems: Vec<&str> = re.split(cmd).collect();

    if elems.is_empty() {
        return vec![Rc::new(RedisCommand::new(cmd.to_string()))];
    }

    let mut cmds: Vec<Rc<RedisCommand>> = vec![];
    for cmd in elems.clone() {
        if cmd.is_empty() {
            continue
        }
        cmds.push(Rc::new(RedisCommand::new(cmd.to_string())))
    }

    cmds
}
