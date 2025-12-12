use actix_web::HttpResponse;

pub async fn home() -> HttpResponse {
    HttpResponse::Ok().body(include_str!("./home/home.html"))
}
