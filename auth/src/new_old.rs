// use juniper::{graphql_object, EmptyMutation, EmptySubscription, FieldResult, RootNode};
// use warp::Filter;

// // Define your GraphQL schema
// struct Query;

// #[graphql_object]
// impl Query {
//     fn hello() -> FieldResult<String> {
//         Ok("Hello, GraphQL in Rust!".to_string())
//     }
// }

// // Define the GraphQL schema using the root node
// // type Schema = RootNode<'static, Query, EmptyMutation<Context>>;
// type Schema = RootNode<'static, Query, EmptyMutation, EmptySubscription>;

// // Create a GraphQL filter for warp
// fn graphql_filter(schema: Schema) -> warp::filters::BoxedFilter<(impl warp::Reply,)> {
//     // let schema = warp::any().map(move || schema.clone());
//     let graphql_filter = juniper_warp::make_graphql_filter(schema, warp::reply::json);
//     // let graphql_filter = juniper_warp::make_graphql_filter(schema, warp::any().boxed());
//     warp::post()
//         .and(warp::path("graphql"))
//         .and(graphql_filter)
//         .boxed()
// }

// // Start the Warp server
// #[tokio::main]
// async fn main() {
//     // let schema = Schema::new(Query, EmptyMutation::<Context>::new(), EmptySubscription::new());
//     let schema = Schema::new(Query, EmptyMutation::<()>::new(), EmptySubscription::<()>::new());
//     let graphql_filter = graphql_filter(schema);
//     let routes = warp::any().map(move || warp::reply::html("Hello, Warp server!"));
//     // let routes = routes.or(graphql_filter);

//     warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
// }
