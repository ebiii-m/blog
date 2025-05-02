use actix_web::{ HttpResponse , Result };
use tera::{ Tera , Context };

pub async fn render_page(tera: &Tera, temp: &str, context: Context) -> Result<HttpResponse> {
    let rendered = tera.render(temp, context).map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}