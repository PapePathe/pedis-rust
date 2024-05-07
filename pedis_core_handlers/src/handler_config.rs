use pedis_core::{AsyncLockedStore, RedisCommandHandler, RedisCommand};
use std::rc::Rc;

/// Handles the CONFIG command
pub struct ConfigHandler {}
impl RedisCommandHandler for ConfigHandler {
    fn exec(&self, _store: AsyncLockedStore, _cmd: Rc<RedisCommand>) -> String {
        "+OK".to_string()
    }
}

#[cfg(test)]
mod test {
    use pedis_core::{ RedisCommand, Teststore, RedisCommandHandler};
    use std::rc::Rc;
    use std::sync::Arc;
    use std::sync::RwLock;
    use crate::handler_config::ConfigHandler;

    struct TestCase<'a> {
        store_error: bool,
        cmd: Rc<RedisCommand<'a>>,
        result: String,
    }

    #[test]
    fn test_exec() {
        let tests: Vec<Rc<TestCase>> = vec![
            Rc::new(TestCase {
                store_error: false,
                cmd: Rc::new(RedisCommand::new(
                    "*\r\n$6\r\nCLIENT\r\n$7\r\nSETINFO\r\n$8\r\nLIB-NAME\r\n$8\r\nredis-rs\r\n",
                )),
                result: "+OK".to_string(),
            }),
            Rc::new(TestCase {
                store_error: false,
                cmd: Rc::new(RedisCommand::new(
                    "*\r\n$6\r\nCLIENT\r\n$7\r\nSETINFO\r\n$7\r\nLIB-VER\r\n$6\r\n0.25.3\r\n",
                )),
                result: "+OK".to_string(),
            }),
        ];

        for test in tests {
            let h = ConfigHandler {};
            let mut s = Teststore {
                err: test.store_error,
            };
            let result = h.exec(Arc::new(RwLock::new(&mut s)), test.cmd.clone());
            assert_eq!(result, test.result)
        }
    }
}
