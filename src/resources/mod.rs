use actix_web::{HttpRequest, HttpResponse};
use actix_web::http::{StatusCode};

pub async fn index_html(req: HttpRequest) -> HttpResponse {
    let index_html = "<!doctype html>
                                <html>
                                <head>
                                    <title>Hello You!</title>
                                </head>
                                <body>
                                <p>This is an example paragraph. Anything in the <strong>body</strong> tag will appear on the page, just like this <strong>p</strong> tag and its contents.</p>
                                </body>
                                </html>";
    println!("REQ: {req:?}");
    HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(index_html)
}