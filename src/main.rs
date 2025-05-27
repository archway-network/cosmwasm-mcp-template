pub mod contract;
pub mod execute;
pub mod instruction;
pub mod query;
pub mod server;

use rmcp::transport::sse_server::{SseServer, SseServerConfig};
use rmcp::transport::streamable_http_server::axum::StreamableHttpServer;
use rmcp::{ServiceExt, transport::stdio};
use std::error::Error as StdError;
use tracing_subscriber::{
    EnvFilter,
    layer::SubscriberExt,
    util::SubscriberInitExt,
    {self},
};

use crate::server::*;

/// (Optionally) toggle between MCP server transport modes (stdio, sse, streamable http)
const TRANSPORT_MODE: ServerTransport = ServerTransport::Stdio;
// const TRANSPORT_MODE: ServerTransport = ServerTransport::Sse;
// const TRANSPORT_MODE: ServerTransport = ServerTransport::StreamableHttp;

const BIND_ADDRESS: &str = "127.0.0.1:8000";

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    match TRANSPORT_MODE {
        ServerTransport::Stdio => {
            tracing_subscriber::fmt()
                .with_env_filter(
                    EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()),
                )
                .with_writer(std::io::stderr)
                .with_ansi(false)
                .init();

            let mcp_server = CwMcp::new().serve(stdio()).await.inspect_err(|e| {
                tracing::error!("serving error: {:?}", e);
            })?;

            mcp_server.waiting().await?;
        }
        ServerTransport::Sse => {
            tracing_subscriber::registry()
                .with(
                    tracing_subscriber::EnvFilter::try_from_default_env()
                        .unwrap_or_else(|_| "debug".to_string().into()),
                )
                .with(tracing_subscriber::fmt::layer())
                .init();

            let config = SseServerConfig {
                bind: BIND_ADDRESS.parse()?,
                sse_path: "/".to_string(),
                post_path: "/".to_string(),
                ct: tokio_util::sync::CancellationToken::new(),
                sse_keep_alive: None,
            };

            let (sse_server, router) = SseServer::new(config);
            let listener = tokio::net::TcpListener::bind(sse_server.config.bind).await?;
            let ct = sse_server.config.ct.child_token();
            let server = axum::serve(listener, router).with_graceful_shutdown(async move {
                ct.cancelled().await;
                tracing::info!("sse server cancelled");
            });
            tokio::spawn(async move {
                if let Err(e) = server.await {
                    tracing::error!(error = %e, "sse server shutdown with error");
                }
            });

            let ct = sse_server.with_service(CwMcp::new);
            tokio::signal::ctrl_c().await?;
            ct.cancel();
        }
        ServerTransport::StreamableHttp => {
            tracing_subscriber::registry()
                .with(
                    tracing_subscriber::EnvFilter::try_from_default_env()
                        .unwrap_or_else(|_| "debug".to_string().into()),
                )
                .with(tracing_subscriber::fmt::layer())
                .init();

            let ct = StreamableHttpServer::serve(BIND_ADDRESS.parse()?)
                .await?
                .with_service(CwMcp::new);

            tokio::signal::ctrl_c().await?;
            ct.cancel();
        }
    }

    Ok(())
}
