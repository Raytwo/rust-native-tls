use std::io;
use std::fmt;
use std::error;

use {TlsAcceptorBuilder, TlsConnectorBuilder};


pub struct Error(io::Error);

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        unimplemented!()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Error {
        unimplemented!()
    }
}

#[derive(Clone)]
pub struct Identity;

impl Identity {
    pub fn from_pkcs12(buf: &[u8], pass: &str) -> Result<Identity, Error> {
        unimplemented!()
    }

    pub fn from_pkcs8(pem: &[u8], key: &[u8]) -> Result<Identity, Error> {
        unimplemented!()
    }
}

#[derive(Clone)]
pub struct Certificate;

impl Certificate {
    pub fn from_der(buf: &[u8]) -> Result<Certificate, Error> {
        unimplemented!()

    }

    pub fn from_pem(buf: &[u8]) -> Result<Certificate, Error> {
        unimplemented!()

    }

    pub fn to_der(&self) -> Result<Vec<u8>, Error> {
        unimplemented!()
    }
}

pub struct MidHandshakeTlsStream<S>(S);

impl<S> fmt::Debug for MidHandshakeTlsStream<S>
where
    S: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl<S> MidHandshakeTlsStream<S> {
    pub fn get_ref(&self) -> &S {
        unimplemented!()
    }

    pub fn get_mut(&mut self) -> &mut S {
        unimplemented!()
    }
}

impl<S> MidHandshakeTlsStream<S>
where
    S: io::Read + io::Write,
{
    pub fn handshake(self) -> Result<TlsStream<S>, HandshakeError<S>> {
        unimplemented!()
    }
}

pub enum HandshakeError<S> {
    Failure(Error),
    WouldBlock(MidHandshakeTlsStream<S>),
}

impl<S> From<io::Error> for HandshakeError<S> {
    fn from(e: io::Error) -> HandshakeError<S> {
        unimplemented!()
    }
}

#[derive(Clone, Debug)]
pub struct TlsConnector;

impl TlsConnector {
    pub fn new(builder: &TlsConnectorBuilder) -> Result<TlsConnector, Error> {
        unimplemented!()
    }

    pub fn connect<S>(&self, domain: &str, stream: S) -> Result<TlsStream<S>, HandshakeError<S>>
    where
        S: io::Read + io::Write,
    {
        unimplemented!()
    }
}

#[derive(Clone)]
pub struct TlsAcceptor;

impl TlsAcceptor {
    pub fn new(builder: &TlsAcceptorBuilder) -> Result<TlsAcceptor, Error> {
        unimplemented!()
    }

    pub fn accept<S>(&self, stream: S) -> Result<TlsStream<S>, HandshakeError<S>>
    where
        S: io::Read + io::Write,
    {
        unimplemented!()
    }
}

pub struct TlsStream<S>(S);

impl<S: fmt::Debug> fmt::Debug for TlsStream<S> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl<S> TlsStream<S> {
    pub fn get_ref(&self) -> &S {
        unimplemented!()
    }

    pub fn get_mut(&mut self) -> &mut S {
        unimplemented!()
    }
}

impl<S: io::Read + io::Write> TlsStream<S> {
    pub fn buffered_read_size(&self) -> Result<usize, Error> {
        unimplemented!()
    }

    pub fn peer_certificate(&self) -> Result<Option<Certificate>, Error> {
        unimplemented!()
    }

    pub fn tls_server_end_point(&self) -> Result<Option<Vec<u8>>, Error> {
        unimplemented!()
    }

    pub fn shutdown(&mut self) -> io::Result<()> {
        unimplemented!()
    }
}

impl<S: io::Read + io::Write> io::Read for TlsStream<S> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        unimplemented!()
    }
}

impl<S: io::Read + io::Write> io::Write for TlsStream<S> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        unimplemented!()
    }

    fn flush(&mut self) -> io::Result<()> {
        unimplemented!()
    }
}
