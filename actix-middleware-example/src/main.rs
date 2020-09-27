#![allow(proc_macro_derive_resolution_fallback)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use crate::utils::csv_utils::{load_csv, walk_csv};
use actix::Supervisor;
use actix_casbin::casbin::{
    function_map::key_match2, CachedEnforcer, CoreApi, DefaultModel, MgmtApi, Result,
};
use actix_casbin::CasbinActor;
use actix_casbin_auth::CasbinService;
use actix_cors::Cors;
//use actix_web::middleware::NormalizePath;
use actix_web::{App, HttpServer};
use sqlx_adapter::SqlxAdapter;
use std::env;

mod api;
mod config;
mod constants;
mod errors;
mod middleware;
mod models;
mod routers;
mod schema;
mod services;
mod utils;

#[actix_rt::main]
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

    let pool = config::db::migrate_and_config_db(&database_url, pool_size);

    let model = DefaultModel::from_file("casbin.conf").await?;
    let adapter = SqlxAdapter::new(database_url, pool_size).await?;
    let mut casbin_middleware = CasbinService::new(model, adapter).await.unwrap();
    casbin_middleware
        .write()
        .await
        .get_role_manager()
        .write()
        .unwrap()
        .matching_fn(Some(key_match2), None);

    let share_enforcer = casbin_middleware.get_enforcer();
    let clone_enforcer = share_enforcer.clone();
    let casbin_actor = CasbinActor::<CachedEnforcer>::set_enforcer(share_enforcer)?;
    let started_actor = Supervisor::start(|_| casbin_actor);

    let preset_rules = load_csv(walk_csv("."));
    for mut policy in preset_rules {
        let ptype = policy.remove(0);
        if ptype.starts_with('p') {
            match clone_enforcer.write().await.add_policy(policy).await {
                Ok(_) => info!("Preset policies(p) add successfully"),
                Err(err) => error!("Preset policies(p) add error: {}", err.to_string()),
            };
            continue;
        } else if ptype.starts_with('g') {
            match clone_enforcer
                .write()
                .await
                .add_named_grouping_policy(&ptype, policy)
                .await
            {
                Ok(_) => info!("Preset policies(p) add successfully"),
                Err(err) => error!("Preset policies(g) add error: {}", err.to_string()),
            };
            continue;
        } else {
            unreachable!()
        }
    }

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(started_actor.clone())
            .wrap(
                Cors::new()
                    .send_wildcard()
                    .allowed_methods(vec!["GET", "POST", "DELETE"])
                    .allowed_headers(vec![
                        http::header::AUTHORIZATION,
                        http::header::ACCEPT,
                    ])
                    .allowed_header(http::header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish(),
            )
            //.wrap(NormalizePath)
            .wrap(actix_web::middleware::Logger::default())
            .wrap(casbin_middleware.clone())
            .wrap(crate::middleware::authn::Authentication)
            .configure(routers::routes)
    })
    .bind(&app_url)?
    .run()
    .await?;

    Ok(())
}
