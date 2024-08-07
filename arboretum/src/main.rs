use actix::{Actor, System};

mod clang_annotation;
mod graph;
mod module_layout;
mod networking;
mod rust_annotation;
mod rust_syntax;
mod translation;
mod z3_actor;

#[derive(Debug)]
enum ArboretumError {
    GraphActorError(graph::GraphError),
    IoError(std::io::Error),
}

#[actix::main]
async fn main() -> Result<(), ArboretumError> {
    let system = System::new();

    // Process command line flags.

    // The graph actor is responsible for performing queries and updates to the underlying knowledge graph representation.
    // Additionally, it handles the
    let graph_actor = graph::GraphActor::new()
        .map_err(|e| ArboretumError::GraphActorError(e))?
        .start();

    // The clang annotation actor is responsible for annotating freshly obtained clang data with semantic nodes which tie it into
    // the existing knowledge graph:
    //  - Merges type definitions
    //  - Maintains the canonical decl view.
    let clang_annotation_actor =
        clang_annotation::ClangAnnotationActor::new(graph_actor.clone()).start();

    // let rust_annotation_actor = rust_annotation::RustAnnotationActor::new().start();

    // The module layout actor is responsible for maintaining a tree of modules, and assigning each new decl to a module.
    let module_layout_actor = module_layout::ModuleLayoutActor::new(graph_actor.clone()).start();

    // The z3 actor is responsible for performing constraint solving.
    let z3_actor = z3_actor::Z3Actor::new(graph_actor.clone()).start();

    // The reactive translation actor uses constraint solver values to attempt to locate a translation.
    // Failures are reported back to z3 for continued solving.
    let translation_actor = translation::ReactiveTranslationActor::new(graph_actor.clone()).start();

    // The rust syntax actor is responsible for maintaining a tree of rust syntax based on the graph structure.
    let rust_syn_actor = rust_syntax::RustSyntaxActor::new(graph_actor.clone()).start();

    // The networking actor is responsible for managing client connections.
    let networking_actor = networking::NetworkingActor::new(graph_actor.clone()).start();

    system.run().map_err(|e| ArboretumError::IoError(e))
}
