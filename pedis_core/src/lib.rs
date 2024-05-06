//! This crate defines the behaviour of pedis key value store.
#![warn(missing_docs)]
use std::{rc::Rc, sync::Arc, sync::RwLock};

/// Import the config handler
pub mod handler_config;
/// Import the set handler
pub mod handler_set;
/// Import the command module
pub mod redis_command;
/// Import the store module
pub mod redis_store;

/// Defines the behaviour of the redis command handlers
///
/// Creating a command handler is as simple as creating
/// a stateless struct and generating the boilerplate to
/// implement the interface.
///
/// # Errors
///
/// # Examples
///
/// ```
/// use pedis_core::RedisCommandHandler;
/// use pedis_core::redis_store::IStore;
/// use pedis_core::redis_command::RedisCommand;
/// ```
///
/// # Todo!
///
/// - [x] Add basic handler that has access to a store with only get and set methods.
/// - [ ] Add a handler that provides a store with the ability to use `del` method.
/// - [ ] Return a `Vec<u8>` instead of `String`
/// - [ ] Return a `Result<Vec<u8>, CommandEror>` so that call may decide pot processing
///       the error and sending a resp error to the client.
/// - [ ] Define which endpoints are authenticated
/// - [ ] Allow each handler to document itself
/// - [ ]
///
pub trait RedisCommandHandler {
    /// Executes a single redis command using a read write
    /// locked store if necessary.
    ///
    fn exec(&self, _: AsyncLockedStore, _: Rc<redis_command::RedisCommand>) -> String;
}

type AsyncLockedStore<'a> = Arc<RwLock<&'a mut (dyn redis_store::IStore + Send + Sync)>>;
