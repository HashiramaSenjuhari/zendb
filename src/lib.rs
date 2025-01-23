use postgres::{
    tls::{MakeTlsConnect, TlsConnect},
    Error, Socket,
};

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
