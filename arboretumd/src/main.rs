use std::sync::Arc;

use ::futures::future;
use arboretum_core::GraphBuffer;
use arboretum_graph::RootGraph;
use clap::Parser;
use tokio::sync::{broadcast, mpsc};
use tracing::info;

mod api;
use api::{api_server, ApiServerState};

mod clang_collector;
use clang_collector::clang_collector;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    db_dir: String,

    #[arg(long, default_value = "localhost:3000")]
    api_bind_addr: String,

    #[arg(long, default_value = "localhost:3232")]
    collector_bind_addr: String,
}

async fn canonical_view_extraction(
    _root_graph: RootGraph,
    mut graph_change_rx: broadcast::Receiver<u32>,
) {
    while let Ok(_change) = graph_change_rx.recv().await {
        // Do nothing
    }

    //todo!()
    // let decl = root_graph
    //     .get_named_node("clang::Decl")
    //     .await
    //     .unwrap()
    //     .unwrap();

    // for (s, p, o, _) in query! {root_graph, decl -?-> ?}.await.unwrap() {
    //     let s = root_graph
    //         .get_node_name(s)
    //         .await
    //         .unwrap()
    //         .unwrap_or_else(|| s.to_string());
    // }
}

async fn graph_buffer_loader(
    g: RootGraph,
    mut buffer_rx: mpsc::Receiver<GraphBuffer>,
    changed_graph_tx: broadcast::Sender<u32>,
) {
    while let Some(buf) = buffer_rx.recv().await {
        let g = g.clone();
        let changed_graph_tx = changed_graph_tx.clone();
        tokio::spawn(async move {
            tracing::info!("Adding graph buffer to root graph: {:?}", buf);
            let (changed_graph_ids, _) = g.add_graph_buffer(buf).unwrap();
            for changed_graph_id in changed_graph_ids {
                changed_graph_tx.send(changed_graph_id).unwrap();
            }
        });
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    // Process command line flags.
    let args = Args::parse();

    // Load the root graph.
    let g = RootGraph::from_folder(args.db_dir, Default::default())?;
    if g.subgraphs()?.len() == 0 {
        info!("Loading clang data model.");
        let data_model = reify_rs::build_data_model();
        let num_named_nodes: usize = data_model.named_nodes().len();
        let num_edges: usize = data_model.edges().iter().map(|(_, e)| e.len()).sum();
        g.add_graph_buffer(data_model).unwrap();
        info!(
            "Data model loaded with {} named nodes and {} edges.",
            num_named_nodes, num_edges
        );
    }
    info!("Subgraphs at startup: {:?}", g.subgraphs());

    // Setup a channel and start recieving graphs.
    let (graph_buffer_tx, graph_buffer_rx) = tokio::sync::mpsc::channel(10);
    let (graph_change_broadcast_tx, graph_change_broadcast_rx) =
        tokio::sync::broadcast::channel(100);

    // Setup handlers which listen for new graphs
    tokio::spawn(canonical_view_extraction(
        g.clone(),
        graph_change_broadcast_rx,
    ));

    // Setup a handler which loads graph buffers into the graph.
    tokio::spawn(graph_buffer_loader(
        g.clone(),
        graph_buffer_rx,
        graph_change_broadcast_tx.clone(),
    ));

    let query_executor = ApiServerState::new(Arc::new(
        arboretum_query::local::LocalGraphQueryExecutor::new(g.clone()),
    ));

    let http_handle = api_server(query_executor, args.api_bind_addr);
    let collector_handle = clang_collector(graph_buffer_tx, args.collector_bind_addr);

    future::try_join(http_handle, collector_handle).await?;

    Ok(())
}
