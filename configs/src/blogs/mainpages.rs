use super::super::render::render_page;
use actix_web::{ Responder , HttpResponse , get};
use dbconnect::get_data;

#[get("/{id}/")]
pub async fn page(id: String) -> impl Responder {
    let template = render_page(get_data::get_rec(id))?;
    HttpResponse::Ok().content_type("text/html").body(template)
}