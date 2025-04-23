use super::Page;
use super::super::render::render_post;
use actix_web::{ Responder , HttpResponse , get};
use dbconnect::get_data;
use super::config_server::Template;

#[get("/{id}/")]
pub async fn post(id: String) -> Responder {
    let template = render_post(get_data(id))?;
    HttpResponse::Ok().content_type("text/html").body(template)
}