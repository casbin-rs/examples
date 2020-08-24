#[warn(unused_parens)]
use actix_web::web;

use crate::api::*;
//use actix_casbin_auth::CasbinService;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/auth")
                    .service(web::resource("/login").route(web::post().to(user::login)))
                    .service(
                        web::resource("/signup").route(web::post().to(user::signup)),
                    ),
            )
            .service(
                web::scope("/user")
                    .wrap(crate::middleware::authn::Authentication)
                    .service(
                        web::resource("logout").route((web::post().to(user::logout))),
                    )
                    .service(
                        web::resource("").route(web::delete().to(user::delete_self)),
                    ),
            )
            .service(
                web::scope("/users")
                    .wrap(crate::middleware::authn::Authentication)
                    .service(
                        web::resource("/{id}")
                            .route(web::delete().to(user::delete_user)),
                    ),
            )
            .service(
                web::scope("/post")
                    .wrap(crate::middleware::authn::Authentication)
                    .service(
                        web::resource("{id}")
                            .route(web::get().to(post::find_by_id_admin))
                            .route(web::delete().to(post::delete)),
                    )
                    .service(web::resource("").route(web::post().to(post::insert))),
            )
            .service(
                web::scope("/posts")
                    .service(
                        web::resource("{id}").route(web::get().to(post::find_by_id)),
                    )
                    .service(web::resource("").route(web::get().to(post::find_all))),
            ),
    );
}
