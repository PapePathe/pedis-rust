use std::{rc::Rc, sync::Arc, sync::RwLock};

pub mod handler_set;
pub mod handler_config;
pub mod redis_command;
pub mod redis_store;

// Defines the behaviour of the redis command handlers
pub trait RedisCommandHandler {
    fn exec(
        &self,
        _: Arc<RwLock<&mut dyn redis_store::IStore>>,
        _: Rc<redis_command::RedisCommand>,
    ) -> String;
}
