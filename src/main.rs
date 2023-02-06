use actix_web::{
    get, route,
    web::{Data, Json},
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
    pool: Data<db::Pool>,
    schema: Data<schema::root::Schema>,
    data: Json<GraphQLRequest>,
) -> impl Responder {
    let ctx = context::Context {
        pool: pool.get_ref().to_owned(),
    };

    let res = data.execute(&schema, &ctx).await;
    HttpResponse::Ok().json(res)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let pool = create_pg_pool().await?;

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

async fn create_pg_pool() -> std::io::Result<sqlx::postgres::PgPool> {
    let database_url = std::env::var("DATABASE_URL").map_err(|_| {
        std::io::Error::new(
            std::io::ErrorKind::Other,
            "DATABASE_URL environment variable is not set",
        )
    })?;

    sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
}
