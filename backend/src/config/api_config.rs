use actix_web::web::{self, ServiceConfig};

use crate::api::{auth_api, ping_api};

pub fn config_services(config: &mut ServiceConfig) {
    config.service(
        web::scope("/api")
            .service(
                web::scope("/auth")
                    .service(auth_api::signup)
                    .service(auth_api::login)
                    .service(auth_api::protected),
            )
            .service(ping_api::ping),
    );
}
