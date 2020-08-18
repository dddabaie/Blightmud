
use std::{net::TcpStream, sync::Mutex, io::{Write, Read}};
use native_tls::{TlsConnector, TlsStream};
use crate::model::ConnectionDetails;
use anyhow::bail;

pub struct Connection {
    standard: Option<Mutex<TcpStream>>,
    tls: Option<Mutex<TlsStream<TcpStream>>>,
}

impl Connection {

    pub fn connect(&mut self, details: &ConnectionDetails) -> anyhow::Result<()> {
        let ConnectionDetails {host, port, tls} = details;
        match TcpStream::connect(format!("{}:{}", host, port)) {
            Ok(stream) => {
                self.standard = Some(Mutex::new(stream.try_clone().unwrap()));
                if *tls {
                    let connector = TlsConnector::new().unwrap();
                    match connector.connect(host, stream) {
                        Ok(connection) => { self.tls = Some(Mutex::new(connection)); Ok(()) }
                        Err(err) => bail!(err),
                    }
                } else {
                    Ok(())
                }
            },
            Err(err) => bail!(err),
        }
    }
}

impl Read for Connection {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if let Some(connection) = self.tls.as_ref() {
            connection.lock().unwrap().read(buf)
        } else if let Some(connection) = self.standard.as_ref() {
            connection.lock().unwrap().read(buf)
        } else {
            Ok(0)
        }
    }
}

impl Write for Connection {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if let Some(connection) = self.tls.as_ref() {
            connection.lock().unwrap().write(buf)
        } else if let Some(connection) = self.standard.as_ref() {
            connection.lock().unwrap().write(buf)
        } else {
            Ok(0)
        }
    }
    fn flush(&mut self) -> std::io::Result<()> {
        if let Some(connection) = self.tls.as_ref() {
            connection.lock().unwrap().flush()
        } else if let Some(connection) = self.standard.as_ref() {
            connection.lock().unwrap().flush()
        } else {
            Ok(())
        }
    }
}
