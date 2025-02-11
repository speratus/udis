use std::hash::Hash;
use serde::{Deserialize, Serialize};
use crate::Value;

pub enum CommandType {
    Create(Value),
    Retrieve,
    Update(Value),
    Delete
}

pub struct Command<K: Eq + Hash> {
    key: K,
    command: CommandType,
}

pub trait CommandPacket<'de>: Serialize + Deserialize<'de> {
    
    fn get_command<K: Eq + Hash>(&self) -> &Command<K>;
    
}