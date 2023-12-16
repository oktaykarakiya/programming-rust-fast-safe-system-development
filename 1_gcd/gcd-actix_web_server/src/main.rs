// You can find the documentation for the things i did here: https://actix.rs/docs/url-dispatch

use actix_web::{web, App, HttpResponse, HttpServer};
use serde::{Deserialize};

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

fn gcd(mut n: u64, mut m: u64) -> u64 {                                     // The GCD calculation is a CPU-bound operation and doesn't benefit from being async
    assert!(n != 0 && m != 0);
    while m != 0 {                                                          // This algorithmus does % until it is no longer possible doing so taking always the biggest number to the left, this ensures it is the gratest, and on the next iteration if the result from the x%y was 0 BOOM - GCD magic done!
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

async fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {         // Make it asyn here
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with zero is boring.");
    }
    let response = format!( "The greatest common divisor of the numbers {} and {} is <b>{}</b>\n", form.n, form.m, gcd(form.n, form.m) );
    HttpResponse::Ok().content_type("text/html").body(response)
}

async fn get_index() -> HttpResponse {                                      // Make it asyn here
    HttpResponse::Ok().content_type("text/html").body(
        r#"

            <title>GCD Calculator</title>

            <form action="/gcd" method="post">
                <input type="text" name="n"/>
                <input type="text" name="m"/>
                <button type="submit">Compute GCD</button>
            </form>

        "#,
    )
}

#[actix_web::main]                                                          // I adepted this entire function based on the documentation that i linked on the top
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
