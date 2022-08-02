#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_json;

mod model;
mod schema;
mod utils;
mod errors;
mod service;
mod middleware;
mod constants;
mod api;

use crate::api::user as user_api;
use crate::utils::csv_utils::{load_csv, walk_csv};


use axum_casbin_auth::casbin::MgmtApi;
use std::env;

use axum::{
    routing::{delete, get, post, put},
    extract::Extension,
    Router,
};

use axum_casbin_auth::casbin::{CoreApi, DefaultModel, Result, function_map::key_match2};
use axum_casbin_auth::CasbinAxumLayer;
use diesel_adapter::DieselAdapter;


#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().expect("Failed to read .env file, please add it");
    std::env::set_var("RUST_LOG", "actix_web=debug");
    env_logger::init();

    let app_host = env::var("APP_HOST").expect("APP_HOST must be set.");
    let app_port = env::var("APP_PORT").expect("APP_PORT must be set.");
    let app_url = format!("{}:{}", &app_host, &app_port);
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool_size: u32 = std::env::var("POOL_SIZE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8);
    
    let pool = model::db::migrate_and_config_db(&database_url, pool_size);
    let model = DefaultModel::from_file("rbac_model.conf").await.unwrap();
    let adapter = DieselAdapter::new(database_url, pool_size)?;
    let mut casbin_middleware = CasbinAxumLayer::new(model, adapter).await.unwrap();

    casbin_middleware
    .write()
    .await
    .get_role_manager()
    .write()
    .matching_fn(Some(key_match2), None);

    let share_enforcer = casbin_middleware.get_enforcer();
    let clone_enforcer = share_enforcer.clone();

    let preset_rules = load_csv(walk_csv("."));
    for mut policy in preset_rules {
        let ptype = policy.remove(0);
        if ptype.starts_with('p') {
            match clone_enforcer.write().await.add_policy(policy).await {
                Ok(_) => info!("Present policies(p) added successfully"),
                Err(err) => error!("Present policies(p) add error: {}", err.to_string()),
            };
            continue;
        } else if ptype.starts_with('g') {
            match clone_enforcer
                .write()
                .await
                .add_named_grouping_policy(&ptype, policy)
                .await
                {
                    Ok(_) => info!("Preset policies(g) added successfully"),
                    Err(err) => error!("Preset policies(g) add error: {}", err.to_string()),
                }
                continue;
        } else {
            unreachable!()
        }
    }
    
    let app = Router::new()
    .layer(Extension(pool.clone()))
    .layer(Extension(clone_enforcer))
    .route("/api/auth/register", post(user_api::register))
    .route("/api/auth/signin", post(user_api::signin))
    .route("/api/users", get(user_api::get_all_user))
    .route("/api/user/:id", get(user_api::get_user))
    .route("/api/admin/:id", put(user_api::update_user))
    .route("/api/admin/:id", delete(user_api::delete_user))
    .layer(casbin_middleware.clone())
    .layer(middleware::auth::AuthLayer);

    axum::Server::bind(&app_url.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

        Ok(())


}
