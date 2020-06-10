use actix_web::{middleware, web, App, HttpResponse, HttpServer, Responder};
use diesel_adapter::casbin::prelude::*;
use diesel_adapter::DieselAdapter;
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
        .add_policy(vec!["casbin".to_string(), "index".to_string(), "read".to_string()])
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

async fn grant(sub: &str, obj: &str, act: &str) -> Result<()> {
    let e = get_enforcer().await;

    if let Ok(authorized) = e.enforce(&[sub, obj, act]).await {
        if authorized {
            Ok(())
        } else {
            Err(()).unwrap()
        }
    } else {
        Err(()).unwrap()
    }
}

async fn get_enforcer() -> Enforcer {
    let m = DefaultModel::from_file("model/rbac_model.conf").await.unwrap();
    let a = DieselAdapter::new().unwrap();
    Enforcer::new(m, a).await.unwrap()
}
