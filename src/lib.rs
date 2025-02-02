pub use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
pub use postgres::types::ToSql;
pub use postgres::{
    tls::{MakeTlsConnect, TlsConnect},
    Error, NoTls, Socket,
};
pub use rand;
pub use rand::Rng;
pub use serde_json;
pub use std::collections::BTreeMap;
pub use std::fs::DirBuilder;
pub use std::fs::File;
pub use std::io;
pub use std::io::Write;
pub use std::panic;
pub use uuid::Uuid;

pub use rand::distributions::Alphanumeric;
pub use serde::{Deserialize, Serialize};

// use core::error::Error;
// use core::fmt::Error;
// use io::Error;
// use rusty_postgres::Error;
// use std::error::Error;
// use std::fmt::Error;
// use std::io::Error;

#[macro_use]
pub mod method;
// pub mod tests;

pub struct Client;

impl Client {
    pub fn connect<T>(client: &str, tls_mode: T) -> Result<postgres::Client, Error>
    where
        T: MakeTlsConnect<Socket> + 'static + Send,
        T::TlsConnect: Send,
        T::Stream: Send,
        <T::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        let client = postgres::Client::connect(client, tls_mode);
        client
    }
}
