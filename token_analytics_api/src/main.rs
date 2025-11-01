// In src/main.rs
mod config;
mod cache;
mod models;
mod rpc;
mod graphql;

use actix_web::{web, App, HttpServer};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use dotenv::dotenv;
use tracing::info;

use crate::{
    cache::RedisPool,
    config::Config,
    graphql::schema::QueryRoot,
};

async fn init_redis_client(url: &str) -> Result<RedisPool, Box<dyn std::error::Error>> {
    let client = redis::Client::open(url)?;
    let mut conn = client.get_async_connection().await?;
    let _: String = redis::cmd("PING").query_async(&mut conn).await?;
    Ok(client)
}

async fn graphql_handler(
    schema: web::Data<Schema<QueryRoot, EmptyMutation, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize environment variables and logger
    dotenv().ok();
    env_logger::init();
    
    info!("🚀 Starting Token Analytics API...");
    
    // Load configuration
    let config = Config::from_env().expect("Failed to load configuration");
    
    // Initialize Redis client
    let redis_pool = init_redis_client(&config.redis_url)
        .await
        .expect("Failed to initialize Redis client");
    
    // Create GraphQL schema
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(redis_pool)
        .finish();
    
    // Start the HTTP server
    info!("🌐 Server running at http://{}", config.server_addr);
    
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(schema.clone()))
            .route("/graphql", web::post().to(graphql_handler))
    })
    .bind(&config.server_addr)?
    .run()
    .await
}