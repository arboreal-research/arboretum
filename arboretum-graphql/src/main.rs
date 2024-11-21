use juniper::{EmptyMutation, EmptySubscription, RootNode};

pub mod model;

fn main() {
    // Define our schema in Rust.
    let schema = RootNode::new(
        model::Query,
        EmptyMutation::<()>::new(),
        EmptySubscription::<()>::new(),
    );

    // Convert the Rust schema into the GraphQL SDL schema.
    println!("{}", schema.as_sdl());
}
