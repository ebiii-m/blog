use mainpages;
use post;
use actix_web::{ web , HttpResponse };

pub fn blog_conf(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/post")
            .route(post::post)
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    )
    .service(
        web::resource("/blog")
            .route(mainpages::page)
            .route(web::head().to(HttpResponse::MethodNotAllowed)),
    );
}