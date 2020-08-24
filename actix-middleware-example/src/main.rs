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

use actix_web::{App, HttpServer};

// use crate::utils::csv_utils::load_csv;
// use actix_casbin::casbin::{CachedEnforcer, DefaultModel, FileAdapter};
// use actix_casbin::{CasbinActor, CasbinCmd, CasbinResult};
// use actix_casbin_auth::CasbinService;
// use diesel_adapter::DieselAdapter;
// use std::path::Path;
use actix_casbin::casbin::Result;
use actix_cors::Cors;
use actix_web::middleware::NormalizePath;

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
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = config::db::migrate_and_config_db(&database_url);

    // let model = DefaultModel::from_file("casbin.conf").await?;
    // let adapter = DieselAdapter::new()?;
    //
    // let addr = CasbinActor::<CachedEnforcer>::new(model, adapter).await?;
    // let share_enforcer = addr.get_enforcer().await;
    // let casbin_middleware = CasbinService::set_enforcer(share_enforcer).await;
    //
    // let csv_path = vec![Path::new("preset_policy.csv")];
    // let preset_rules = load_csv(csv_path);
    // let add_policies = addr
    //     .send(CasbinCmd::AddPolicies(
    //         preset_rules.iter().map(|s| (*s).to_string()).collect(),
    //     ))
    //     .await;
    // let result_add_policies = match add_policies {
    //     Ok(Ok(CasbinResult::AddPolicies(result))) => result,
    //     _ => panic!("Actor Error"),
    // };

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            //.data(addr.clone())
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
            .wrap(NormalizePath)
            .wrap(actix_web::middleware::Logger::default())
            .configure(routers::routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}
