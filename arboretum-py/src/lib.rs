use arboretum_graph::{GraphOptions, RootGraph, SubgraphCacheStrategy};
use arboretum_query::http_reqwest::HttpGraphQueryExecutor;
use arboretum_query::{local::LocalGraphQueryExecutor, GraphQueryExecutor};
use pyo3::exceptions::PyException;
use pyo3::prelude::*;

#[pyclass]
pub struct Graph(Box<dyn GraphQueryExecutor>);

#[pyfunction]
pub fn open_local_graph(path: String) -> PyResult<Graph> {
    let root_graph = RootGraph::from_folder(
        path,
        GraphOptions {
            cache_strategy: SubgraphCacheStrategy::LRU {
                max_usage_bytes: 4 * 1024 * 1024 * 1024,
            },
        },
    )
    .map_err(|e| PyException::new_err(e.to_string()))?;

    let executor = LocalGraphQueryExecutor::new(root_graph);

    Ok(Graph(Box::new(executor)))
}

#[pyfunction]
pub fn open_remote_graph(endpoint: String) -> PyResult<Graph> {
    let executor = HttpGraphQueryExecutor::new(endpoint);
    Ok(Graph(Box::new(executor)))
}

#[pymodule]
fn arboretum_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Graph>()?;
    m.add_function(wrap_pyfunction!(open_local_graph, m)?)?;
    m.add_function(wrap_pyfunction!(open_remote_graph, m)?)?;
    Ok(())
}
