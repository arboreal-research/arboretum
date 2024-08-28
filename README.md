
<p align="center">
  <img src="./media/arboretum2.webp" width="700" alt="An arboretum in decopunk style">
</p>
<p align="center">
  Analysis of C/C++ Abstract Syntax Trees at Scale
</p>

---

Arboretum is a framework designed for analyzing C/C++ syntax trees at scale. It is composed of several key components that work together to create, store, and query code property graphs efficiently. Below is an overview of the primary components.

### [**reificator**](./reificator/)

The `reificator` is a Clang plugin that serves two primary functions:

- **Schema Generation**: It analyzes Clang itself to generate a comprehensive knowledge graph schema. This schema is tailored to simplify the extraction of code properties from C/C++ projects.
  
- **Extractor Plugin**: This plugin simplifies the process of creating code property graphs by providing streamlined access to the properties of C/C++ code. The extractor plugin is highly configurable and designed to work seamlessly with various Clang-based workflows.

### [**arboretumd**](./arboretumd/)

The `arboretumd` is a daemon process that acts as the central server for the Arboretum framework. It provides two main functions:

- **Graph Storage**: It accepts connections from extractor processes (like those created with `reificator`) and stores the resulting graph data. 

- **Query API**: `arboretumd` implements an HTTP/JSON API, enabling remote clients to query the stored graph data efficiently. This API is designed to support both simple and complex queries, making it easy to integrate Arboretum into various analysis pipelines.

### [**arboretum-graph**](./arboretum-graph/)

The `arboretum-graph` component provides the underlying storage layer for graph data within Arboretum. It supports two formats optimized for different use cases:

- **rkyv Serialized Memory Mapped Files**: This format is used for read-only subgraphs, enabling highly efficient querying and access to static graph data. The memory-mapped approach ensures that even large datasets can be queried quickly without the overhead of traditional deserialization.

- **sled Backend**: The sled backend is used for read-write subgraphs, providing a flexible and high-performance storage solution for dynamic graph data that may need to be updated or modified over time.

### [**arboretum-query**](./arboretum-query/

`arboretum-query` provides an abstract query interface which supports local, http/json with reqwest and http/json with reqwasm. 

### [**arboretum-py**](./arboretum-py/)

`arboretum-py` is a Python module that exposes the core functionality of Arboretum, making it accessible to Python developers. It can operate in two modes:

- **Client Mode**: In this mode, `arboretum-py` acts as a client for a remote `arboretumd` server, allowing users to perform queries and analyze graph data stored on the server.

- **Local Mode**: Alternatively, `arboretum-py` can interact directly with a local directory using the `arboretum-graph` storage engine. This mode is ideal for cases where data needs to be analyzed locally or when working offline.
