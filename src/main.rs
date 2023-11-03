use salvo::prelude::*;
use serde::Serialize;

#[derive(Serialize, Debug)]
struct User {
    name: String,
}

#[handler]
fn hello(res: &mut Response) {
    let user = User{name: "jobs".to_string()};

    res.render(Json(user));
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let router = Router::new().get(hello);
    let acceptor = TcpListener::new("127.0.0.1:5800").bind().await;
    Server::new(acceptor).serve(router).await;
}
