use actix_web::{delete, get, HttpResponse, post, put, Responder, web};
use serde_json::json;
use crate::user::User;

#[get("/users")]
async fn find_all() -> impl Responder {
    HttpResponse::Ok().json(
        vec![
      User { id: 1, email: "test@email.com".to_string()},
      User {id: 2, email: "test2@email.com".to_string()},
    ]
    )
}

#[get("/users/{id}")]
async fn find() -> impl Responder {
    HttpResponse::Ok().json(
        User { id: 1, email: "test@email.com".to_string() }
    )
}

#[post("/users")]
async fn create(user: web::Json<User>) -> impl Responder {
    HttpResponse::Created().json(user.into_inner())
}

#[put("/users/{id}")]
async fn update(user: web::Json<User>) -> impl Responder {
    HttpResponse::Ok().json(user.into_inner())
}

#[delete("/users/{id}")]
async fn delete() -> impl Responder {
  HttpResponse::Ok().json(json!({"message": "Deleted"}))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
  cfg.service(find_all);
  cfg.service(find);
  cfg.service(create);
  cfg.service(update);
  cfg.service(delete);
}