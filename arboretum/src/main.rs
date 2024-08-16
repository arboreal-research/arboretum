use std::sync::Arc;

use arboretum_graph::RootGraph;
use clap::Parser;

mod tcp_server;
use tcp_server::tcp_server_bind;
use tracing::{info, trace};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    db_dir: String,

    #[arg(long, default_value = "localhost:3232")]
    bind_addr: String,
}

#[derive(Debug)]
enum ArboretumError {
    GraphError(arboretum_graph::Error),
}
impl From<arboretum_graph::Error> for ArboretumError {
    fn from(e: arboretum_graph::Error) -> Self {
        ArboretumError::GraphError(e)
    }
}

#[actix::main]
async fn main() -> Result<(), ArboretumError> {
    tracing_subscriber::fmt::init();

    // Process command line flags.
    let args = Args::parse();

    // Load the root graph.
    let root_graph = Arc::new(RootGraph::from_folder(args.db_dir, Default::default()).await?);
    if root_graph.union_graph().subgraphs().await.len() == 0 {
        tracing::info!("Loading clang data model.");
        let data_model = reify_rs::build_data_model();
        let num_named_nodes = data_model.named_nodes().len();
        root_graph.add_graph_buffer(data_model).await.unwrap();
        tracing::info!("Data model loaded with {} named nodes.", num_named_nodes);
    }

    info!("Subgraphs at startup: {:?}", root_graph.union_graph().subgraphs().await);

    // Setup a channel and start recieving graphs.
    let (graph_buffer_tx, mut graph_buffer_rx) = tokio::sync::mpsc::channel(10);

    // Setup a handler which loads buffers into the root.
    tokio::spawn(async move {
        while let Some(buf) = graph_buffer_rx.recv().await {
            let root_graph_1 = root_graph.clone();
            tokio::spawn(async move {
                tracing::info!("Adding graph buffer to root graph: {:?}", buf);
                root_graph_1.add_graph_buffer(buf).await.unwrap();
            });
        }
    });

    tcp_server_bind(graph_buffer_tx, args.bind_addr)
        .await
        .unwrap();

    Ok(())
}
