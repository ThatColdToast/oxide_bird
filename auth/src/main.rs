use std::error::Error;

use async_graphql::*;
use async_graphql_poem::*;
use poem::{*, listener::TcpListener};

struct Query;

#[Object]
impl Query {
  async fn howdy(&self) -> &'static str {
    "partner"
  }
}

async fn main() -> Result<(), Box<dyn Error>> {
  // create the schema
  let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();

  // start the http server
  let app = Route::new().at("/", get(graphiql).post(GraphQL::new(schema)));
  println!("GraphiQL: http://localhost:8080");
  Server::new(TcpListener::bind("0.0.0.0:8080")).run(app).await?;
  Ok(())
}