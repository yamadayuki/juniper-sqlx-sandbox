use std::sync::Arc;

use actix_web::{
    get, route,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};

mod schema;

#[get("/healthz")]
async fn healthz() -> impl Responder {
    "OK".to_string()
}

#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    let source = graphiql_source("/graphql", None);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source)
}

#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(
    schema: web::Data<schema::Schema>,
    data: web::Json<GraphQLRequest>,
) -> impl Responder {
    let res = data.execute(&schema, &()).await;
    HttpResponse::Ok().json(res)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let schema = Arc::new(schema::create_schema());

    HttpServer::new(move || {
        App::new()
            .app_data(Data::from(schema.clone()))
            .service(graphql)
            .service(graphql_playground)
            .service(healthz)
    })
    .bind(("127.0.0.1", 8082))?
    .run()
    .await
}
