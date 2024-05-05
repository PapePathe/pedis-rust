use crate::redis_command::RedisCommand;
use crate::redis_store::{IStore, Value};
use crate::RedisCommandHandler;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::RwLock;

pub struct SetHandler {}
impl RedisCommandHandler for SetHandler {
    fn exec(
        &self,
        ss: Arc<RwLock<&mut (dyn IStore + Send + Sync)>>,
        cmd: Rc<RedisCommand>,
    ) -> String {
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
    use crate::{redis_command::RedisCommand, redis_store::IStore, RedisCommandHandler};

    use super::SetHandler;
    use std::{sync::Arc, sync::RwLock};

    #[test]
    fn test_set_handler_exec() {
        let h = SetHandler {};
        let mut s = Teststore { err: false };
        let cmd =
            RedisCommand::new("*3\r\n$3\r\nSET\r\n$3\r\nkey\r\n$11\r\nHello World\r\n".to_string());

        let result = h.exec(Arc::new(RwLock::new(&mut s)), cmd.into());
        assert_eq!(result, "+OK".to_string())
    }

    #[test]
    fn test_set_handler_exec_with_store_error() {
        let h = SetHandler {};
        let mut s = Teststore { err: true };
        let cmd =
            RedisCommand::new("*3\r\n$3\r\nSET\r\n$3\r\nkey\r\n$11\r\nHello World\r\n".to_string());

        let result = h.exec(Arc::new(RwLock::new(&mut s)), cmd.into());
        assert_eq!(result, "-ERR key not found".to_string())
    }

    struct Teststore {
        err: bool,
    }
    impl IStore for Teststore {
        fn set(
            &mut self,
            _: String,
            _: crate::redis_store::Value,
        ) -> Result<(), crate::redis_store::StoreError> {
            if self.err {
                return Err(crate::redis_store::StoreError::KeyNotFoundError);
            }
            Ok(())
        }
        fn get(
            &self,
            _: String,
            _: crate::redis_store::ValueKind,
        ) -> Result<&crate::redis_store::Value, crate::redis_store::StoreError> {
            todo!()
        }
    }
}
