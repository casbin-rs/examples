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

use crate::utils::csv_utils::{load_csv, walk_csv};
use actix_casbin::casbin::Result;
use actix_casbin::casbin::{CachedEnforcer, DefaultModel};
use actix_casbin::{CasbinActor, CasbinCmd};
use actix_casbin_auth::casbin::MgmtApi;
use actix_casbin_auth::CasbinService;
use actix_cors::Cors;
use actix_web::middleware::NormalizePath;
use diesel_adapter::DieselAdapter;

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

    let model = DefaultModel::from_file("casbin.conf").await?;
    let adapter = DieselAdapter::new()?;
    let mut casbin_middleware = CasbinService::new(model, adapter).await;
    casbin_middleware.write().await;

    let share_enforcer = casbin_middleware.get_enforcer().await;
    let clone_enforcer = share_enforcer.clone();
    let casbin_actor =
        CasbinActor::<CachedEnforcer>::set_enforcer(share_enforcer).await?;

    let preset_rules = load_csv(walk_csv("."));
    for mut policy in preset_rules {
        let ptype = policy.remove(0);
        if ptype.starts_with('p') {
            // match casbin_actor.send(CasbinCmd::AddPolicy(policy)).await {
            //     Ok(_) => info!("p policies has been added in database"),
            //     Err(err) => debug!("{}", err),
            //}
            clone_enforcer.write().await.add_policy(policy).await;
            continue;
        } else if ptype.starts_with("g") {
            // match casbin_actor
            //     .send(CasbinCmd::AddNamedGroupingPolicy(ptype.to_string(), policy))
            //     .await
            // {
            //     Ok(_) => info!("g policies has been added in database"),
            //     Err(err) => debug!("{}", err),
            // }
            clone_enforcer
                .write()
                .await
                .add_named_grouping_policy(&ptype, policy)
                .await;
            continue;
        } else {
            unreachable!()
        }
    }

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(casbin_actor.clone())
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
            .wrap(casbin_middleware.clone())
            .configure(routers::routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}
