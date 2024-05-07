use pedis_core::RedisCommand;
use pedis_core::Value;
use pedis_core::{AsyncLockedStore, RedisCommandHandler};
use std::rc::Rc;

/// Handles the SET command
pub struct SetHandler {}
impl RedisCommandHandler for SetHandler {
    fn exec(&self, ss: AsyncLockedStore, cmd: Rc<RedisCommand>) -> String {
        let params = cmd.params().clone();
        let value = Value::new_string(params[2].as_bytes().to_vec());
        let mut s = ss.write().unwrap();
        match s.set(params[1].to_string(), value) {
            Result::Ok(_) => "+OK".to_string(),
            Result::Err(e) => e.to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use pedis_core::{IStore, RedisCommand, RedisCommandHandler, StoreError, Value, ValueKind};

    use super::SetHandler;
    use std::{rc::Rc, sync::Arc, sync::RwLock};

    struct SetTestCase<'a> {
        store_error: bool,
        cmd: Rc<RedisCommand<'a>>,
        result: String,
    }

    #[test]
    fn test_set_handler_exec() {
        let tests: Vec<Rc<SetTestCase>> = vec![
            Rc::new(SetTestCase {
                store_error: false,
                cmd: Rc::new(RedisCommand::new(
                    "*3\r\n$3\r\nSET\r\n$3\r\nkey\r\n$11\r\nHello World\r\n",
                )),
                result: "+OK".to_string(),
            }),
            Rc::new(SetTestCase {
                store_error: true,
                cmd: Rc::new(RedisCommand::new(
                    "*3\r\n$3\r\nSET\r\n$3\r\nkey\r\n$11\r\nHello World\r\n",
                )),
                result: "-ERR key not found".to_string(),
            }),
        ];

        for test in tests {
            let h = SetHandler {};
            let mut s = Teststore {
                err: test.store_error,
            };
            let result = h.exec(Arc::new(RwLock::new(&mut s)), test.cmd.clone());
            assert_eq!(result, test.result)
        }
    }

    struct Teststore {
        err: bool,
    }
    impl IStore for Teststore {
        fn set(&mut self, _: String, _: Value) -> Result<(), StoreError> {
            if self.err {
                return Err(StoreError::KeyNotFoundError);
            }
            Ok(())
        }
        fn get(&self, _: String, _: ValueKind) -> Result<&Value, StoreError> {
            todo!()
        }
    }
}
