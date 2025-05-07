use actix_web::{ HttpResponse , Result };
use tera::{ Tera , Context };
use once_cell::sync::OnceCell;

static CONFIG: OnceCell<Tera> = OnceCell::new();

fn set_config(conf: Tera) -> Result<(), &'static str> {
    CONFIG
        .set(conf)
        .map_err(|_| "Config can only be set once!")
}

fn get_config() -> Tera {
    CONFIG.get().expect("Config not set!")
}

pub async fn render_page(tera: &Tera, temp: &str, context: &Context) -> Result<String> {
    let rendered = tera.render(temp, context).map_err(|e| actix_web::error::ErrorInternalServerError(e))?;
    Ok(rendered)
}