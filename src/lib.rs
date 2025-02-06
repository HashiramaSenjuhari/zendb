pub use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
pub use postgres::types::ToSql;
pub use postgres::{
    tls::{MakeTlsConnect, TlsConnect},
    Error, NoTls, Socket,
};
pub use rand;
pub use rand::Rng;
pub use serde_json;
pub use uuid::Uuid;

pub use rand::distributions::Alphanumeric;
pub use serde::{Deserialize, Serialize};

#[cfg(feature = "async")]
pub use tokio;

#[cfg(feature = "async")]
use tokio_postgres::Connection;

#[cfg(feature = "async")]
pub use tokio::io;

#[cfg(feature = "async")]
pub use tokio_postgres::types::ToSql as AsyncToSql;

#[cfg(feature = "async")]
pub use tokio_postgres::{
    tls::{MakeTlsConnect as AsyncMakeTlsConnect, TlsConnect as AsyncTlsConnect},
    Error as AsyncError, NoTls as AsyncNoTls, Socket as AsyncSocket,
};

#[cfg(feature = "async")]
pub use tokio::fs::File;

#[cfg(feature = "async")]
pub use tokio::io::AsyncWriteExt;

#[cfg(feature = "async")]
pub use tokio_postgres::GenericClient;

#[cfg(feature = "async")]
pub use tokio::fs::DirBuilder;

#[cfg(feature = "async")]
pub use tokio_postgres::Client;

#[macro_use]
pub mod method;

#[cfg(not(feature = "async"))]
pub struct Client;

#[cfg(not(feature = "async"))]
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

#[cfg(feature = "async")]
pub struct TokioClient;

#[cfg(feature = "async")]
impl TokioClient {
    pub async fn connect<T>(client: &str, tls_mode: T) -> Result<tokio_postgres::Client, Error>
    where
        T: AsyncMakeTlsConnect<Socket> + 'static + Send,
        T::TlsConnect: Send,
        T::Stream: Send,
        <T::TlsConnect as TlsConnect<Socket>>::Future: Send,
    {
        let client = tokio_postgres::connect(client, tls_mode);
        match client.await {
            Ok((client, connection)) => {
                tokio::spawn(async move {
                    if let Err(err) = connection.await {
                        eprintln!("{}", err);
                    }
                });
                Ok(client)
            }
            Err(err) => Err(err),
        }
    }
}
