use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use casbin::prelude::*;
use casbin::DefaultModel;
use diesel_adapter::{ConnOptions, DieselAdapter};
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
pub struct Visitor {
    name: String,
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let mut enforcer = get_enforcer().await;
    enforcer
        .add_policy(vec!["casbin", "index", "read"])
        .await
        .unwrap();

    let app = move || {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(index))
    };

    // Start HTTP server
    let bind_address = env::var("BIND_ADDRESS").expect("BIND_ADDRESS is not set");

    HttpServer::new(app)
        .bind(&bind_address)
        .unwrap_or_else(|_| panic!("Cannot bind address to {}", &bind_address))
        .run()
        .await
}

async fn index(me: web::Query<Visitor>) -> impl Responder {
    if grant(&me.name, "index", "read").await.is_err() {
        return HttpResponse::Forbidden().body("Forbidden");
    };

    HttpResponse::Ok().body("OK")
}

async fn grant(sub: &str, obj: &str, act: &str) -> Result<(), ()> {
    let enforcer = get_enforcer().await;

    if let Ok(authorized) = enforcer.enforce(vec![sub, obj, act]) {
        if authorized {
            Ok(())
        } else {
            Err(())
        }
    } else {
        Err(())
    }
}

async fn get_enforcer() -> Enforcer {
    let model = DefaultModel::from_file("model/rbac_model.conf")
        .await
        .unwrap();

    let db_name = env::var("DB_NAME").expect("DB_NAME is not set");
    let username = env::var("DB_USER").expect("DB_USER is not set");
    let password = env::var("DB_PASS").expect("DB_PASS is not set");
    let host = env::var("DB_HOST").expect("DB_HOST is not set");
    let port = env::var("DB_PORT").expect("DB_PORT is not set");

    let mut conn_opts = ConnOptions::default();
    conn_opts
        .set_database(&db_name)
        .set_hostname(&host)
        .set_port(port.parse::<u16>().unwrap())
        .set_auth(&username, &password);

    let adapter = DieselAdapter::new(conn_opts).unwrap();
    Enforcer::new(Box::new(model), Box::new(adapter))
        .await
        .unwrap()
}
