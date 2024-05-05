#[derive(Debug, Copy, Clone)]
pub struct RedisCommand<'a> {
    cmd: &'a str,
}

impl<'a> RedisCommand<'a> {
    pub fn new(cmd: &'a str) -> Self {
        Self { cmd }
    }
    pub fn params(&self) -> Vec<String> {
        let mut args: Vec<String> = vec![];
        let binding = self.cmd;
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
        self.params()[0].clone().to_lowercase()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_args() {
        let c = RedisCommand::new("*3\r\n$3\r\nSET\r\n$3\r\nkey\r\n$11\r\nHello World\r\n");
        assert_eq!(vec!["SET", "key", "Hello World"], c.params());
    }

    #[test]
    fn test_name() {
        let c = RedisCommand::new("*3\r\n$3\r\nSET\r\n$3\r\nkey\r\n$11\r\nHello World\r\n");
        assert_eq!("set", c.name());
    }
}
