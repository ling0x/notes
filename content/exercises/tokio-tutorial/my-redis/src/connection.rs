use std::io::Cursor;

use bytes::BytesMut;
use mini_redis::{Frame, Result};
use tokio::{io::AsyncReadExt, net::TcpStream};

pub struct Connection {
    stream: TcpStream,
    buffer: Vec<u8>,
    cursor: usize,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Connection {
        Connection {
            stream,
            // Allocate the buffer with 4kb of capacity.
            buffer: vec![0; 4096],
            // When working with byte arrays and read, we must also maintain
            // a cursor tracking how much data has been bufffered
            cursor: 0,
        }
    }

    /// Read a frame from the connection
    ///
    /// Returns `None` if EOF is reached
    pub async fn read_frame(&mut self) -> Result<Option<Frame>> {
        loop {
            // Attempt to parse a frame from the buffered data. If
            // enough data has been buffered, the frame is
            // returned.
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }

            // Ensure the buffer has capacity
            if self.buffer.len() == self.cursor {
                // Grow the buffer
                self.buffer.resize(self.cursor * 2, 0);
            }

            // Read into the buffer, tracking the number of bytes read
            let n = self.stream.read(&mut self.buffer[self.cursor..]).await?;

            // There is not enough buffered data to read a frame.
            // Attempt to read more data from the socket.
            //
            // On success, the number of bytes is returned. `0`
            // indicates "end of stream".
            if 0 == n {
                // The remote closed the connection. For this to be
                // a clean shutdown, there should be no data in the
                // read buffer. If there is, this means that the
                // peer closed the socket while sending a frame.
                if self.cursor == 0 {
                    return Ok(None);
                } else {
                    return Err("connection reset by peer".into());
                }
            } else {
                // Update our cursor
                self.cursor += n;
            }
        }
    }

    fn parse_frame(&mut self) -> Result<Option<Frame>> {
        // Create the `T: Buf` type
        let mut buf = Cursor::new(&self.buffer[..]);

        //
    }

    /// Write a frame to the connection
    pub async fn write_frame(&mut self, frame: &Frame) -> Result<()> {
        todo!()
    }
}
