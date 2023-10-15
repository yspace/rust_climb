use super::handlers::*;
use actix_web::web;


pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}


// The expression service(web::scope("/courses")) creates a new resource scope called courses, under which all APIs related to courses can be added. A scope is a set of resources with a common root path. A set of routes can be registered under a scope. Application state can be shared among routes within the same scope.
pub fn course_routes(cfg: &mut web::ServiceConfig) { cfg
    .service(web::scope("/courses")
    .route("/{tutor_id}", web::get().to(get_courses_for_tutor))
    .route("/{tutor_id}/{course_id}", web::get().to(get_course_detail))
    .route("/", web::post().to(new_course)));

}
    