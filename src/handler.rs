use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

pub struct Handler {
    stream: TcpStream,
}

impl Handler {
    /// Create a new instance of the handler
    /// 
    /// This function will create a new instance of the handler with the provided stream.
    /// 
    /// # Arguments
    /// * `stream` - The stream to handle
    /// 
    /// # Return
    /// `Self` - The handler instance
    #[must_use] 
    pub const fn new(stream: TcpStream) -> Self {
        Self {
            stream,
        }
    }

    /// Process the stream
    /// 
    /// This function will process the stream by reading the stream and writing the response.
    /// 
    /// # Return
    /// `Result<()>` - The result of processing the stream
    /// 
    /// # Errors
    /// `Error` - When an error occurs while processing the stream
    pub async fn process_stream(&mut self) -> crate::Result<()> {
        // Read stream
        // message buffer
        let mut buffer = [0; 512];
        // total message
        let mut msg = String::new();
        loop {
            // read bytes into buffer
            let bytes_read = self.stream.read(&mut buffer).await?;
            // If there is no bytes read
            if bytes_read == 0 {
                break;
            }
            // push buffer string between 0 to bytes_read
            msg.push_str(&String::from_utf8_lossy(&buffer[..bytes_read]));
            if msg.contains("PING\r\n") {
                // Write the response to stream
                self.stream.write_all(b"+PONG\r\n").await?;
                // flush the stream
                self.stream.flush().await ?;
                msg.clear();
            }
        }
        Ok(())
    }
}

