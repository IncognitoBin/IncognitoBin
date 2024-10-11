use actix_web::{web};
use crate::handlers::auth_handlers::{new_user, user_login};
use crate::handlers::paste_handlers::{create_paste, delete_paste, get_paste, get_user_pastes};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            // Paste endpoints
            .service(get_paste)       // GET /paste/{paste_id}
            .service(get_user_pastes) // GET /paste
            .service(create_paste)    // POST /paste
            .service(delete_paste)    // DELETE /paste/{paste_id}
            // User endpoints
            .service(new_user)        // GET /user
            .service(user_login)      // POST /user
    );
}
