use super::super::render::render_page;
use actix_web::{ Responder , HttpResponse , get};
use dbconnect::get_data::get_rec;
use tera::Context;

#[get("/{id}/")]
pub async fn post(id: String) -> impl Responder {
    let data = get_rec(id);
    let template = render_page();
    HttpResponse::Ok().content_type("text/html").body(template)
}