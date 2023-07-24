// use warp::Filter;
// use std::sync::Arc;
// use juniper::http::graphiql::graphiql_source;
// use juniper::RootNode;

// struct QueryRoot;
// struct MutationRoot;

// #[juniper::graphql_object(Context = Context)]
// impl QueryRoot {}

// // #[juniper::graphql_object(Context = Context)]
// // impl MutationRoot {}

// type Schema = RootNode<'static, QueryRoot, MutationRoot>;

// struct Context {
//     client: Client,
// }

// impl juniper::Context for Context {}

// async fn graphql(
//     schema: Arc<Schema>,
//     ctx: Arc<Context>,
//     req: GraphQLRequest,
// ) -> Result<impl warp::Reply, Infallible> {
//     let res = req.execute_async(&schema, &ctx).await;
//     let json = serde_json::to_string(&res).expect("Invalid JSON response");
//     Ok(json)
// }

// #[tokio::main]
// async fn main () {
//     let schema = Arc::new(Schema::new(QueryRoot, MutationRoot));
//     // Create a warp filter for the schema
//     let schema = warp::any().map(move || Arc::clone(&schema));

//     // let ctx = Arc::new(Context { client });
//     // Create a warp filter for the context
//     // let ctx = warp::any().map(move || Arc::clone(&ctx));

//     let graphql_route = warp::post()
//         .and(warp::path!("graphql"))
//         .and(schema.clone())
//         // .and(ctx.clone())
//         .and(warp::body::json())
//         .and_then(graphql);

//     let graphiql_route = warp::get()
//         .and(warp::path!("graphiql"))
//         .map(|| warp::reply::html(graphiql_source("graphql")));

//     let routes = graphql_route.or(graphiql_route);

//     warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
// }

// // use warp::Filter;

// // #[tokio::main]
// // async fn main() {
// //     // GET /hello/warp => 200 OK with body "Hello, warp!"
// //     let hello = warp::path!("hello" / String)
// //         .map(|name| format!("Hello, {}!", name));

// //     println!("Waiting on port 8080");

// //     warp::serve(hello)
// //         .run(([127, 0, 0, 1], 8080))
// //         .await;
// // }