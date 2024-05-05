use std::{rc::Rc, sync::Arc, sync::RwLock};

pub mod config_handler;
pub mod redis_command;
pub mod redis_store;
pub mod set_handler;

// Defines the behaviour of the redis command handlers
pub trait RedisCommandHandler {
    fn exec(
        &self,
        s: Arc<RwLock<&mut dyn redis_store::IStore>>,
        cmd: Rc<redis_command::RedisCommand>,
    ) -> String;
}
