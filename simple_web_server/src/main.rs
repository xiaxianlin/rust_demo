use async_std::io::{Read, Write};
use async_std::net::TcpListener;
use async_std::prelude::*;
use async_std::task;
use futures::stream::StreamExt;
use std::fs;
use std::marker::Unpin;
use std::time::Duration;

async fn handle_connection(mut stream: impl Read + Write + Unpin) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "./html/hello.html")
    } else if buffer.starts_with(sleep) {
        task::sleep(Duration::from_secs(5)).await;
        ("HTTP/1.1 200 OK\r\n\r\n", "./html/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "./html/404.html")
    };

    let contents = match fs::read_to_string(filename) {
        Ok(res) => res,
        Err(_) => "File not found".to_string(),
    };

    let response = format!("{status_line}{contents}");
    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}

#[async_std::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").await.unwrap();

    listener
        .incoming()
        .for_each_concurrent(None, |tcpstream| async move {
            let tcpstream = tcpstream.unwrap();
            task::spawn(handle_connection(tcpstream));
        })
        .await;
}

#[cfg(test)]
mod tests {
    use std::cmp::min;

    use super::*;

    struct MockTcpStream {
        read_data: Vec<u8>,
        write_data: Vec<u8>,
    }

    impl Read for MockTcpStream {
        fn poll_read(
            self: std::pin::Pin<&mut Self>,
            _: &mut task::Context<'_>,
            buf: &mut [u8],
        ) -> task::Poll<std::io::Result<usize>> {
            let size: usize = min(self.read_data.len(), buf.len());
            buf[..size].copy_from_slice(&self.read_data[..size]);
            task::Poll::Ready(Ok(size))
        }
    }

    impl Write for MockTcpStream {
        fn poll_write(
            mut self: std::pin::Pin<&mut Self>,
            _: &mut task::Context<'_>,
            buf: &[u8],
        ) -> task::Poll<std::io::Result<usize>> {
            self.write_data = Vec::from(buf);
            task::Poll::Ready(Ok(buf.len()))
        }

        fn poll_flush(
            self: std::pin::Pin<&mut Self>,
            _: &mut task::Context<'_>,
        ) -> task::Poll<std::io::Result<()>> {
            task::Poll::Ready(Ok(()))
        }

        fn poll_close(
            self: std::pin::Pin<&mut Self>,
            _: &mut task::Context<'_>,
        ) -> task::Poll<std::io::Result<()>> {
            task::Poll::Ready(Ok(()))
        }
    }

    impl Unpin for MockTcpStream {}

    #[async_std::test]
    async fn test_handle_connection() {
        let input_bytes = b"GET / HTTP/1.1\r\n";
        let mut contents = vec![0u8; 1024];
        contents[..input_bytes.len()].clone_from_slice(input_bytes);
        let mut stream = MockTcpStream {
            read_data: contents,
            write_data: Vec::new(),
        };

        handle_connection(&mut stream).await;
        let mut buf = [0u8; 1024];
        stream.read(&mut buf).await.unwrap();

        let expected_contents = match fs::read_to_string("./html/hello.html") {
            Ok(res) => res,
            Err(_) => "File not found".to_string(),
        };
        let expected_response = format!("HTTP/1.1 200 OK\r\n\r\n{}", expected_contents);
        assert!(stream.write_data.starts_with(expected_response.as_bytes()));
    }
}
