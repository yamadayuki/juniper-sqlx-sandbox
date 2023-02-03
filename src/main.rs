use actix_web::{
    get, route,
    web::{self, Data},
    App, HttpResponse, HttpServer, Responder,
};
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};
use std::sync::Arc;

mod context;
mod db;
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
    pool: web::Data<db::Pool>,
    schema: web::Data<schema::root::Schema>,
    data: web::Json<GraphQLRequest>,
) -> impl Responder {
    let ctx = context::Context {
        pool: pool.get_ref().to_owned(),
    };

    let res = data.execute(&schema, &ctx).await;
    HttpResponse::Ok().json(res)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost:5432/juniper-sqlx")
        .await
        .expect("Failed to connect to database");

    let schema = Arc::new(schema::root::create_schema());

    HttpServer::new(move || {
        App::new()
            .app_data(Data::from(schema.clone()))
            .app_data(Data::new(pool.clone()))
            .service(graphql)
            .service(graphql_playground)
            .service(healthz)
    })
    .bind(("127.0.0.1", 8082))?
    .run()
    .await
}
