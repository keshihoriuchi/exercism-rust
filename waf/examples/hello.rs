extern crate waf;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Hello {
    message: String,
}

fn main() {
    let mut app = waf::App::default();
    app.add_middleware(Box::new(|_req, res| {
        res.return_json(Hello {
            message: "Hello, World!".to_string(),
        });
        true
    }));
    app.listen();
}
