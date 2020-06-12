use casbin::{CoreApi, DefaultModel, Enforcer, FileAdapter};
use std::io;
use std::sync::RwLock;

use ntex::web::{self, middleware, App, HttpRequest, HttpResponse};

/// simple handle
async fn auth(
    enforcer: web::types::Data<RwLock<Enforcer>>,
    req: HttpRequest,
) -> HttpResponse {
    let e = enforcer.write().unwrap();
    println!("{:?}", req);
    let name = req.match_info().get("name").unwrap_or("cat");
    let action = req.match_info().get("action").unwrap_or("meow");

    if let Ok(authorized) = e.enforce(&[name, "data", action]).await {
        if authorized {
            HttpResponse::Ok().body(format!("{} can {} data", &name, &action))
        } else {
            HttpResponse::Ok().body(format!("{} can't {} data", &name, &action))
        }
    } else {
        HttpResponse::Ok().body(format!(
            "Need the right path, like localhost:8080/auth/{}/{}",
            &name, &action
        ))
    }
}

#[ntex::main]
async fn main() -> io::Result<()> {
    dotenv::dotenv().ok();

    loge::init();

    let model = DefaultModel::from_file("acl/acl_model.conf").await.unwrap();
    let adapter = FileAdapter::new("acl/acl_policy.csv");

    let e = Enforcer::new(model, adapter).await.unwrap();
    let e = web::types::Data::new(RwLock::new(e)); // wrap enforcer into actix-state

    //move is necessary to give closure below ownership of counter
    web::server(move || {
        App::new()
            .app_data(e.clone()) // <- create app with shared state
            // enable logger
            .wrap(middleware::Logger::default())
            // register simple handler, handle all methods
            .service((
                web::resource("/auth").to(auth),
                web::resource("/auth/{name}/{action}").to(auth),
            ))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
