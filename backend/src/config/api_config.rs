use actix_web::web::{self, ServiceConfig};

use crate::api::auth_api;

pub fn config_services(config: &mut ServiceConfig) {
    config.service(
        web::scope("/api").service(
            web::scope("/auth")
                .service(auth_api::signup)
                .service(auth_api::login),
        ),
    );
}
