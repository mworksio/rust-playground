use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use sqlparser::dialect::MySqlDialect;
use sqlparser::parser::Parser;
use std::str;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                let n = match socket.read(&mut buf).await {
                    Ok(n) if n == 0 => return,
                    Ok(n) => n, 
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                let sql = match str::from_utf8(&buf) {
                    Ok(v) => v, 
                    Err(e) => panic!("invalid {:?}", e),
                };

                let dialect = MySqlDialect {};
                let ast = Parser::parse_sql(&dialect, sql).unwrap();

                let data = serde_json::to_string_pretty(ast).unwrap();

                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    }
}
