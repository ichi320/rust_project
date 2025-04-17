use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

#[actix_web::main]
async fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });

    println!("Serving on http://localhost:3000...");
    server
        .bind("127.0.0.1:3000").expect("error binding server to address")
        .run().await.expect("error running server");
}

async fn get_index() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(
            r#"
            <title>GCD Calculator</title>
            <form action="/gcd" method="post">
            <input type="text" name="m"/>
            <input type="text" name="n"/>
            <button type="submit">Compute GCD</button>
            </form>
            "#,
            )
}

#[derive(Deserialize)]
struct GcdParameters {
    m: u64,
    n: u64,
}

async fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.m == 0 || form.n == 0 {
        return HttpResponse::BadRequest()
            .content_type("text.html")
            .body("Computing the GCD with zero is boring.");
    }

    let response =
        format!("The greatest common divisor of the numbers {} and {} \
        is <b>{}</b>\n",
        form.m, form.n, gcd(form.m, form.n));

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}

fn gcd(mut m: u64, mut n: u64) -> u64 {
    assert!(m != 0 && n != 0);
    while n != 0 {
        if n < m {
            let t = n;
            n = m;
            m = t;
        }
        n = n % m;
    }
    m
}
        

