use actix_web::{get, post, web, App, HttpServer, HttpResponse, Responder};
use actix_web::web::Path;

async fn saludar() -> impl Responder {
    HttpResponse::Ok().body("Hola!")
}

#[get("/tweets")]
async fn get_tweets() -> HttpResponse {
    let tweets = ["tweet 1: Hola", "tweet 2: chao"];

    HttpResponse::Ok().content_type("application/json").json(tweets)
}

#[post("/tweet")]
async fn create_tweet() -> HttpResponse {
    let tweet = "This is a new tweet";
    HttpResponse::Created().content_type("application/json").json(tweet)
}

#[get("/tweets/{id}")]
async fn get_tweet_by_id(path: Path<(String,)>) -> HttpResponse {
    let tweet = format!("este es el tweet {:?}", path.0);

    HttpResponse::Ok().content_type("application/json").json(tweet)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hola", web::get().to(saludar))
            .service(get_tweets)
            .service(create_tweet)
            .service(get_tweet_by_id)
    }).bind("127.0.0.1:8000")?
        .run().await
}
