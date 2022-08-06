use actix_web::{http::header::ContentType, HttpRequest, HttpResponse, Responder};

pub async fn index_handler(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().content_type(ContentType::html()).body(
        include_str!("../index.html")
            .lines()
            .collect::<Vec<_>>()
            .join("")
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join(" "),
    )
}
