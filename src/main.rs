use tokio;
use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::accept_async;
use futures_util::{SinkExt, StreamExt};
use serde_json;
use tracing::{info, error};
use tracing_subscriber;
use config::{Config, File};
use std::error::Error;
use std::sync::Arc;
use tokio::signal;
use tokio_tungstenite::tungstenite::Message;
//use tokio::task;
use tokio::sync::Semaphore;

mod pow;
use pow::{PowRequest, perform_pow};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Load configuration
    let config = Arc::new(Config::builder()
        .add_source(File::with_name("config/default"))
        .build()?);

    // Set up logging with tracing
    tracing_subscriber::fmt::init();

    // Get server configuration
    let host = config.get_string("server.host").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = config.get_int("server.port").unwrap_or(8080) as u16;
    let addr = format!("{}:{}", host, port);

    // Set up the TCP listener
    let listener = TcpListener::bind(&addr).await?;
    info!("Listening on: {}", addr);

    // Set up the semaphore for limiting connections
    let max_connections = config.get_int("server.max_connections").unwrap_or(100) as usize;
    let semaphore = Arc::new(Semaphore::new(max_connections));

    let server = async move {
        loop {
            match listener.accept().await {
                Ok((stream, _)) => {
                    let permit = match semaphore.clone().acquire_owned().await {
                        Ok(permit) => permit,
                        Err(_) => {
                            error!("Semaphore closed");
                            break;
                        }
                    };

                    let config_clone = Arc::clone(&config);
                    tokio::spawn(async move {
                        let _permit = permit;

                        if let Err(e) = handle_connection(stream, config_clone).await {
                            error!("Error handling connection: {}", e);
                        }
                    });
                }
                Err(e) => {
                    error!("Accept error: {}", e);
                }
            }
        }
    };

    // Handle graceful shutdown
    tokio::select! {
        _ = server => {},
        _ = signal::ctrl_c() => {
            info!("Shutdown signal received");
        },
    }

    info!("Server is shutting down");

    Ok(())
}

// Handle connection
async fn handle_connection(
    stream: TcpStream,
    config: Arc<Config>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let ws_stream = accept_async(stream).await?;
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    let default_difficulty = config
        .get_int("pow.default_difficulty")
        .unwrap_or(0) as u32;
    let max_difficulty = config
        .get_int("pow.max_difficulty")
        .unwrap_or(u32::MAX as i64) as u32;

    while let Some(msg_result) = ws_receiver.next().await {
        let msg = match msg_result {
            Ok(msg) => msg,
            Err(e) => {
                error!("Error receiving message: {}", e);
                break;
            }
        };

        match msg {
            Message::Text(text) => {
                let mut pow_request: PowRequest = match serde_json::from_str(&text) {
                    Ok(req) => req,
                    Err(e) => {
                        error!("Error parsing PowRequest: {}", e);
                        continue;
                    }
                };

                // Use default difficulty if not specified
                if pow_request.target_pow == 0 {
                    pow_request.target_pow = default_difficulty;
                }
                // Enforce maximum difficulty
                pow_request.target_pow = pow_request.target_pow.min(max_difficulty);

                // Offload the POW calculation
                let pow_result = match tokio::task::spawn_blocking(move || perform_pow(pow_request)).await {
                    Ok(pow_result) => pow_result,
                    Err(e) => {
                        error!("Join error: {}", e);
                        continue;
                    }
                };

                let response = serde_json::to_string(&pow_result)?;
                ws_sender.send(Message::Text(response)).await?;
            }
            Message::Ping(ping) => {
                ws_sender.send(Message::Pong(ping)).await?;
            }
            Message::Close(_) => {
                info!("Client initiated close");
                break;
            }
            _ => {
                // Optionally handle other message types
            }
        }
    }

    info!("Connection closed");
    Ok(())
}
