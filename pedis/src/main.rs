use pedis_core::RedisCommand;
use std::io::Read;
use std::io::Write;
use std::rc::Rc;
use std::sync::{Arc, RwLock};
use std::{collections::HashMap, net::TcpListener, net::TcpStream};

struct PedisServer {
    addr: String,
}
impl PedisServer {
    fn new(addr: String) -> Self {
        Self { addr }
    }
    fn start(&self) -> std::io::Result<()> {
        println!("starting pedis server");
        let listener = TcpListener::bind(self.addr.clone())?;
        println!("server started");
        for stream in listener.incoming() {
            self.handle_client(stream?);
        }
        Ok(())
    }
    fn handle_client(&self, mut stream: TcpStream) {
        let mut commands: HashMap<String, Box<dyn pedis_core::RedisCommandHandler>> =
            HashMap::new();
        commands.insert(
            "config".to_string(),
            Box::new(pedis_core_handlers::handler_config::ConfigHandler {}),
        );
        commands.insert(
            "set".to_string(),
            Box::new(pedis_core_handlers::handler_set::SetHandler {}),
        );

        loop {
            let mut buffer = [0; 1024];
            let bytes_count = stream.read(&mut buffer[..]).unwrap();
            let redis_command = std::str::from_utf8(&buffer[0..bytes_count]).unwrap();

            eprintln!(
                "DEBUG: redis_command{:?} payload_size={:?}",
                redis_command, bytes_count
            );
            let elems = self.parse_command(redis_command);
            for cmd in elems {
                eprintln!("DEBUG: {:?}", cmd);

                if let Some(handler) = commands.get(&cmd.name()) {
                    let mut store = pedis_core::RedisStore::default();
                    let result = handler.exec(Arc::new(RwLock::new(&mut store)), cmd);
                    let _ = stream.write(result.as_bytes());
                    continue;
                }

                let _ = stream.write("-ERR command not found \r\n".as_bytes());
            }
        }
    }
    fn parse_command<'a>(&'a self, cmd: &'a str) -> Vec<Rc<RedisCommand>> {
        let re = regex::Regex::new(r"\*\d*").unwrap();
        let elems: Vec<&str> = re.split(cmd).collect();

        if elems.is_empty() {
            return vec![Rc::new(RedisCommand::new(cmd))];
        }

        let mut cmds: Vec<Rc<RedisCommand>> = vec![];
        for cmd in elems.clone() {
            if cmd.is_empty() {
                continue;
            }
            cmds.push(Rc::new(RedisCommand::new(cmd)))
        }

        cmds
    }
}

fn main() -> std::io::Result<()> {
    let server = PedisServer::new("127.0.0.1:8379".to_string());
    server.start()
}
