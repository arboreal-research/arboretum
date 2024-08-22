use ::futures::future;
use arboretum_graph::{GraphBuffer, RootGraph};
use clap::Parser;
use tokio::{
    sync::{broadcast, mpsc},
    task::JoinError,
};
use tracing::info;

mod api;
use api::api_server;

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

#[derive(Debug)]
pub enum ArboretumError {
    GraphError(arboretum_graph::Error),
    JoinError(JoinError),
    IoError(std::io::Error),
}
impl From<arboretum_graph::Error> for ArboretumError {
    fn from(e: arboretum_graph::Error) -> Self {
        ArboretumError::GraphError(e)
    }
}
impl From<JoinError> for ArboretumError {
    fn from(e: JoinError) -> Self {
        ArboretumError::JoinError(e)
    }
}
impl From<std::io::Error> for ArboretumError {
    fn from(e: std::io::Error) -> Self {
        ArboretumError::IoError(e)
    }
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
            let (changed_graph_ids, _) = g.add_graph_buffer(buf).await.unwrap();
            for changed_graph_id in changed_graph_ids {
                changed_graph_tx.send(changed_graph_id).unwrap();
            }
        });
    }
}

#[tokio::main]
async fn main() -> Result<(), ArboretumError> {
    tracing_subscriber::fmt::init();

    // Process command line flags.
    let args = Args::parse();

    // Load the root graph.
    let g = RootGraph::from_folder(args.db_dir, Default::default()).await?;
    if g.subgraphs().await.len() == 0 {
        info!("Loading clang data model.");
        let data_model = reify_rs::build_data_model();
        let num_named_nodes: usize = data_model.named_nodes().len();
        let num_edges: usize = data_model.edges().iter().map(|(_, e)| e.len()).sum();
        g.add_graph_buffer(data_model).await.unwrap();
        info!(
            "Data model loaded with {} named nodes and {} edges.",
            num_named_nodes, num_edges
        );
    }
    info!("Subgraphs at startup: {:?}", g.subgraphs().await);

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

    let http_handle = api_server(g.clone(), args.api_bind_addr);
    let collector_handle = clang_collector(graph_buffer_tx, args.collector_bind_addr);

    future::try_join(http_handle, collector_handle).await?;

    Ok(())
}
