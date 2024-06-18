use crate::error::Error;
use crate::handler::Handler;
use crate::Result;
use std::net::{TcpListener, ToSocketAddrs};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

pub struct App {
    listener: TcpListener,
    tx: mpsc::Sender<Error>,
    rx: Arc<Mutex<mpsc::Receiver<Error>>>,
}

impl App {
    /// Create a new instance of the application
    ///
    /// This function will create a new instance of the application with the provided address and buffer size.
    ///
    /// # Arguments
    /// * `address` - The address to bind the application to
    /// * `buffer` - The buffer size for the application
    ///
    /// # Return
    /// `Result<Self>` - The application instance if successful, otherwise an error
    ///
    /// # Errors
    /// `Error` - When an error occurs while creating the application
    pub fn new<A: ToSocketAddrs>(address: A, buffer: usize) -> Result<Self> {
        let listener = TcpListener::bind(address)?;
        let (tx, rx) = mpsc::channel(buffer);
        Ok(Self {
            listener,
            tx,
            rx: Arc::new(Mutex::new(rx)),
        })
    }

    /// Get port number
    ///
    /// This function will return the port number the application is bound to.
    ///
    /// # Return
    /// `Result<u16>` - The port number if successful, otherwise an error
    ///
    /// # Errors
    /// `Error` - When an error occurs while getting the port number
    pub fn port(&self) -> Result<u16> {
        Ok(self.listener.local_addr()?.port())
    }

    /// Get address
    ///
    /// This function will return the address the application is bound to.
    ///
    /// # Return
    /// `Result<String>` - The address if successful, otherwise an error
    ///
    /// # Errors
    /// `Error` - When an error occurs while getting the address
    pub fn address(&self) -> Result<String> {
        Ok(self.listener.local_addr()?.ip().to_string())
    }
    /// Run the application
    ///
    /// This function will run the application and spawn a new tokio task for each incoming connection.
    ///
    /// # Return
    /// `Result<()>` - Ok if the application runs successfully, otherwise an error
    ///
    /// # Errors
    /// `Error` - When an error occurs while running the application
    pub async fn run(&mut self) -> Result<()> {
        let rx = self.rx.clone();
        let tx = self.tx.clone();
        let listener = self.listener.try_clone()?;
        let server_task = Self::run_server(&listener, &tx);
        let rx_task = tokio::spawn(async move {
            let mut rx = rx.lock().await;
            let mut errors = Vec::new();
            while let Some(e) = rx.recv().await {
                errors.push(e);
            }
            if !errors.is_empty() {
                return Err(Error::Multiple(errors));
            }
            Ok(())
        });
        
        tokio::select! {
            res = server_task => res ,
            res = rx_task => res?,
        }
    }

    /// Run the server
    ///
    /// This function will run the server and spawn a new tokio task for each incoming connection.
    ///
    /// # Arguments
    /// * `listener` - The listener to accept incoming connections
    /// * `tx` - The sender to send errors to
    ///
    /// # Return
    /// `Result<()>` - Ok if the server runs successfully, otherwise an error
    ///
    /// # Errors
    /// `Error` - When an error occurs while running the server
    #[allow(clippy::unused_async)]
    pub async fn run_server(listener: &TcpListener, tx: &mpsc::Sender<Error>) -> Result<()> {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let clone_tx = tx.clone();
                    let mut handler = Handler::new(stream);
                    tokio::spawn(async move {
                        match handler.process_stream() {
                            Ok(()) => {}
                            Err(e) => {
                                if (clone_tx.send(e).await).is_err() {
                                    println!("receiver has been drop, unable to send messages.");
                                }
                            }
                        }
                    });
                }
                Err(e) => {
                    println!("error: {e}");
                }
            }
        }
        Ok(())
    }
}
