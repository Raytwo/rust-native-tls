extern crate libc;
extern crate nnsdk;

use std::io;
use std::fmt;
use std::error;

use std::marker::PhantomData;

use self::nnsdk::nn;

use {TlsAcceptorBuilder, TlsConnectorBuilder};

// TODO: Move bindings in nnsdk-rs
mod Connection {
    // Estimate through SDK binary usage of the fields. Might be 0x28?
    #[repr(C)]
    pub struct Connection {
        _x: [u8;0x24],
    }

    impl Connection {
        pub fn new() -> Self {
            Self {
                _x: [0;0x24],
            }
        }
    }

    extern "C" {
        #[link_name = "\u{1}_ZN2nn3ssl10ConnectionC1Ev"]
        pub fn Connection(this: *mut Connection);
    }
    extern "C" {
        #[link_name = "\u{1}_ZN2nn3ssl10Connection6CreateEPNS0_7ContextE"]
        pub fn Create(this: *mut Connection, context: *const u8) -> u32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN2nn3ssl10Connection19SetSocketDescriptorEi"]
        pub fn SetSocketDescriptor(this: *mut Connection, socket_desc: u32) -> u32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN2nn3ssl10Connection4ReadEPcPij"]
        pub fn Read(this: *const Connection, out_buf: *mut u8, buf_len: usize) -> usize;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN2nn3ssl10Connection5WriteEPKcj"]
        pub fn Write(this: *const Connection, buf: *const u8, buf_len: usize) -> usize;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN2nn3ssl10Connection17FlushSessionCacheEv"]
        pub fn FlushSessionCache(this: *const Connection) -> u32;
    }
    extern "C" {
        #[link_name = "\u{1}_ZN2nn3ssl10Connection7DestroyEv"]
        pub fn Destroy(this: *const Connection) -> u32;
    }
}

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
        // The place where the TCP socket needs to be opened (use the domain for the address), connected then provided to the SSL library
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
        // TODO: Make sure nn::ssl::Initialize has been called before any of this
        // TODO: Prepare a nn::ssl::Context
        let mut connection = Box::new(Connection::Connection::new());
        // Initialize the class before doing anything
        unsafe { Connection::Connection(connection.as_mut()) };

        // TODO: Create the connection by providing it the context
        // let result = unsafe { Connection::Create(connection.as_mut(), context idk) };

        // TODO: Protocol 6 is TCP. Make constants in nnsdk-rs to facilitate. Same goes for the libc values.
        let tcp_socket = unsafe { nn::socket::Socket(libc::AF_INET, libc::SOCK_STREAM, 6) };
        // TODO: Connect the socket to the domain
        // Assign the socket to the Connection. After doing so, you musn't use it again or even free it.
        let result = unsafe { Connection::SetSocketDescriptor(connection.as_mut(), tcp_socket as _) };
        unimplemented!()
    }
}

pub struct TlsStream<S> {
    connection: Box<Connection::Connection>,
    _m: PhantomData<S>,
}

impl<S: fmt::Debug> fmt::Debug for TlsStream<S> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        unimplemented!()
    }
}

impl<S> TlsStream<S> {
    pub fn get_ref(&self) -> &S {
        unsafe { & *(self.connection.as_ref() as *const Connection::Connection as *const S) }
    }

    pub fn get_mut(&mut self) -> &mut S {
        unsafe { &mut *(self.connection.as_mut() as *mut Connection::Connection as *mut S) }
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
        let result = unsafe { Connection::Destroy(self.connection.as_ref()) };
        // Should we take care of this for the user directly in the dependencies?
        // unsafe { nnsdk::ssl::Finalize(); }
        Ok(())
    }
}

impl<S: io::Read + io::Write> io::Read for TlsStream<S> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let result = unsafe { Connection::Read(self.connection.as_ref(), buf.as_mut_ptr(), buf.len()) };
        // TODO: If result is < 0, we have an error, deal with that
        Ok(result)
    }
}

impl<S: io::Read + io::Write> io::Write for TlsStream<S> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let result = unsafe { Connection::Write(self.connection.as_ref(), buf.as_ptr(), buf.len()) };
        // TODO: If result is < 0, we have an error, deal with that
        Ok(result)
    }

    fn flush(&mut self) -> io::Result<()> {
        let result = unsafe { Connection::FlushSessionCache(self.connection.as_ref()) };
        Ok(())
    }
}
