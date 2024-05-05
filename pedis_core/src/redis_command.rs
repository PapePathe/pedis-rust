#[derive(Debug)]
pub struct RedisCommand {
    cmd: String,
    args: Vec<String>
}

impl RedisCommand {
    pub fn new(cmd: String) -> Self {
        let mut args: Vec<String> = vec![];
        let elems: Vec<&str> = cmd.split("\r\n").collect::<Vec<_>>();
        for (idx, pat) in elems[1..].iter().enumerate() {
            if idx.rem_euclid(2) == 0 {
                continue;
            }
            args.push(pat.to_string())
        }
        Self { cmd , args}
    }
    pub fn params(&self) -> Vec<String> {
        let mut args: Vec<String> = vec![];
        let binding = self.cmd.clone();
        let elems: Vec<&str> = binding.split("\r\n").collect::<Vec<_>>();
        for (idx, pat) in elems[1..].iter().enumerate() {
            if idx.rem_euclid(2) == 0 {
                continue;
            }
            args.push(pat.to_string())
        }
        args.clone()
    }
    pub fn name(&self) -> String {
        self.args[0].clone().to_lowercase()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args() {
        let c =
            RedisCommand::new("*3\r\n$3\r\nSET\r\n$3\r\nkey\r\n$11\r\nHello World\r\n".to_string());
        assert_eq!(vec!["SET", "key", "Hello World"], c.params());
    }

    #[test]
    fn test_name() {
        let c =
            RedisCommand::new("*3\r\n$3\r\nSET\r\n$3\r\nkey\r\n$11\r\nHello World\r\n".to_string());
        assert_eq!("set", c.name());
    }
}
