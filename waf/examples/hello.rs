extern crate waf;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Hello {
    message: String,
}

fn main() {
    let mut app = waf::App::default();
    let mut router = waf::Router::default();
    router.get("/".to_string(), Box::new(|_req, res| {
        res.return_text("Hello".to_string());
    }));
    router.get("/fuga".to_string(), Box::new(|_req, res| {
        res.return_text("Fuga".to_string());
    }));
    app.add_router(router);
    app.add_middleware(Box::new(|_req, res| {
        res.return_json(Hello {
            message: "Hello, World!".to_string(),
        });
        true
    }));
    app.listen();
}
