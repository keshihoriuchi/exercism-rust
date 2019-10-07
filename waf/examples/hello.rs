extern crate waf;
use serde::{Deserialize, Serialize};
use tinytemplate::TinyTemplate;
use std::sync::{Arc, Mutex};


#[derive(Serialize, Deserialize, Debug)]
struct Hello {
    message: String,
}

#[derive(Serialize)]
struct Context {
    count: u32,
}

static TEMPLATE: &'static str = "<!DOCTYPE html>
<html lang=\"ja\">
  <head>
    <meta charset=\"utf-8\" />
    <title>†ほりうちの館†</title>
  </head>
  <body>
    <h1>†ほりうちの館†</h1>
    <p>☆☆☆☆☆あなたは{count}人目のお客様です☆☆☆☆☆</p>
  </body>
</html>";

fn main() {
    let count = Arc::new(Mutex::new(0));
    let mut app = waf::App::default();
    let mut router = waf::Router::default();
    router.get(
        "/".to_string(),
        Box::new(|_req, res| {
            res.return_text("Hello".to_string());
        }),
    );
    router.get(
        "/fuga".to_string(),
        Box::new(|_req, res| {
            res.return_text("Fuga".to_string());
        }),
    );
    router.get(
        "/index.html".to_string(),
        Box::new(|_req, res| {
            let mut tt = TinyTemplate::new();
            tt.add_template("hello", TEMPLATE).unwrap();

            let context = Context {
                count: 0,
            };

            let rendered = tt.render("hello", &context).unwrap();
            res.return_html(rendered.to_string());
        }),
    );
    app.add_router(router);
    app.add_middleware(Box::new(|_req, res| {
        res.return_json(Hello {
            message: "Hello, World!".to_string(),
        });
        true
    }));
    app.listen();
}
