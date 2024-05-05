use crate::redis_command::RedisCommand;
use crate::redis_store::IStore;
use crate::RedisCommandHandler;
use std::rc::Rc;
use std::{sync::Arc, sync::RwLock};

pub struct ConfigHandler {}
impl RedisCommandHandler for ConfigHandler {
    fn exec(&self, _: Arc<RwLock<&mut (dyn IStore + Send + Sync)>>, _: Rc<RedisCommand>) -> String {
        "+OK".to_string()
    }
}
