use std::env;

use lazy_static::lazy_static;
use tera::Tera;

pub mod auth;
pub mod http;
pub mod state;

lazy_static! {
    pub static ref TEMPLATE: Tera = {
        let template_dir_path = env::var("FOWOO_TEMPLATE_DIR_PATH")
            .unwrap_or("/workspace/templates/**/*.html".to_string());
        Tera::new(&template_dir_path).unwrap()
    };
}
