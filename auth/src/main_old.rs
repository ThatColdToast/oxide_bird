// use juniper::{RootNode, EmptyMutation, EmptySubscription, GraphQLObject, Context};
// use rocket::{ get, routes, launch, State, Request, Response };

// #[get("/hello")]
// async fn hello() -> String { //_schema: &State<MySchema>
//   "🚀 says hello!".to_string()
// }

// // #[rocket::get("/")]
// // fn graphiql() -> content::RawHtml<String> {
// //     juniper_rocket::graphiql_source("/graphql", None)
// // }

// #[derive(GraphQLObject)]
// struct User {
//   email: String,
//   age: i32
// }

// #[derive(GraphQLObject)]
// struct QueryRoot {
//   users: Vec<User>
// }

// impl QueryRoot {
//   async fn hello(&self, _ctx: &Context<'_>) -> String {
//     "GraphQL says hello!".to_string()
//   }
// }

// type Schema = RootNode<'static, QueryRoot, EmptyMutation, EmptySubscription>;

// #[rocket::post("/graphql", data = "<request>", format = "application/json")]
// async fn graphql_request(schema: &State<Schema>, request: Request) -> Response {
//   request.execute(schema).await
// }

// #[launch]
// fn rocket() -> _ {
//     let schema = Schema::new(QueryRoot { users: vec![]}, EmptyMutation::new(), EmptySubscription::new());

//     rocket::build()
//         // .manage(schema)
//         .mount("/", routes![hello])
// }