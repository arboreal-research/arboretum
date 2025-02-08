use std::{collections::HashMap, path::PathBuf, sync::Arc};

use clap::Parser;
use futures::future;
use reify_rs::io::TableBuilders;
use tokio::sync::{mpsc, Mutex};

mod api;
use api::{api_server, ApiServerState};

mod clang_collector;
use clang_collector::{clang_collector, CollectorSessionMessage};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    db_path: String,

    #[arg(long, default_value = "localhost:3000")]
    api_bind_addr: String,

    #[arg(long, default_value = "localhost:3232")]
    collector_bind_addr: String,
}

async fn graph_buffer_loader(
    db_path: PathBuf,
    mut buffer_rx: mpsc::Receiver<CollectorSessionMessage>,
) {
    let mut table_builders = HashMap::new();
    while let Some(buf) = buffer_rx.recv().await {
        match buf {
            CollectorSessionMessage::Open(subgraph_id) => {
                table_builders.insert(
                    subgraph_id,
                    TableBuilders::new(db_path.join(subgraph_id.to_string()), 1000),
                );
            }
            CollectorSessionMessage::Record(subgraph_id, record) => {
                if let Some(builder) = table_builders.get_mut(&subgraph_id) {
                    if let Err(e) = builder.push(record).await {
                        tracing::error!(
                            "Error pushing record to builder for subgraph {}: {}",
                            subgraph_id,
                            e
                        );
                    }
                }
            }
            CollectorSessionMessage::OkClose(subgraph_id) => {
                if let Some(mut builder) = table_builders.remove(&subgraph_id) {
                    if let Err(e) = builder.cancel().await {
                        tracing::error!(
                            "Error cancelling builder for subgraph {}: {}",
                            subgraph_id,
                            e
                        );
                    }
                }
            }
            CollectorSessionMessage::ErrClose(subgraph_id) => {
                if let Some(mut builder) = table_builders.remove(&subgraph_id) {
                    if let Err(e) = builder.cancel().await {
                        tracing::error!(
                            "Error cancelling builder for subgraph {}: {}",
                            subgraph_id,
                            e
                        );
                    }
                }
            }
        }
    }
    for (subgraph_id, mut builders) in table_builders {
        if let Err(e) = builders.cancel().await {
            tracing::error!(
                "Error cancelling remaining builder for subgraph {}: {}",
                subgraph_id,
                e
            );
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // Process command line flags.
    let args = Args::parse();

    // Setup a channel and start recieving graphs.
    let (new_rows_tx, new_rows_rx) = tokio::sync::mpsc::channel(10);

    // Setup a handler which loads graph buffers into the graph.
    tokio::spawn(graph_buffer_loader(
        args.db_path.clone().into(),
        new_rows_rx,
    ));

    // Find the highest existing subgraph ID in the db path
    let mut highest_id = 0;
    if let Ok(mut entries) = tokio::fs::read_dir(&args.db_path).await {
        while let Ok(Some(entry)) = entries.next_entry().await {
            if let Ok(file_name) = entry.file_name().into_string() {
                if let Ok(id) = file_name.parse::<u32>() {
                    highest_id = highest_id.max(id);
                }
            }
        }
    }
    let next_subgraph_id = Arc::new(Mutex::new(highest_id + 1));

    let http_handle = api_server(ApiServerState::new(), args.api_bind_addr);
    let collector_handle = clang_collector(next_subgraph_id, new_rows_tx, args.collector_bind_addr);

    future::try_join(http_handle, collector_handle).await?;

    Ok(())
}
