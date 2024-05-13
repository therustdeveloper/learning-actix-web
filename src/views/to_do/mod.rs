mod create;
use actix_web::web::{post, scope, ServiceConfig};

pub fn to_do_views_factory(app: &mut ServiceConfig) {
    app.service(scope("v1/item").route("create", post().to(create::create)));
}
