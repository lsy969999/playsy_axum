use std::{collections::HashMap, sync::Mutex, thread};

use once_cell::sync::Lazy;
use serde::Deserialize;
use serde_json::{json, Value};
/// Use tracing crates for application-level tracing output.
use tracing_subscriber::{
    layer::SubscriberExt, util::SubscriberInitExt
};

#[derive(Debug, Deserialize, Clone, Eq, Hash, PartialEq)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
}

impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} by {}", self.title, self.author)
    }
}

/// global variable with Lazy, Mutex
pub static DATA: Lazy<Mutex<HashMap<u32, Book>>> = Lazy::new(|| Mutex::new(
    HashMap::from([
        (1, Book {
            id: 1,
            title: "Antigone".into(),
            author: "Sophocles".into()
        }),
        (2, Book {
            id: 2,
            title: "Beloved".into(),
            author: "Toni Morrison".into()
        }),
        (3, Book {
            id: 3,
            title: "Candide".into(),
            author: "Voltaire".into()
        })
    ])
));

#[tokio::main]
async fn main() {
    print_data().await;
    // tracing init
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = axum::Router::new()
    .route(
        "/",
        axum::routing::get(hello)
    )
    .route(
        "/demo.html",
        axum::routing::get(get_demo_html)
    )
    .route(
        "/hello.html",
        axum::routing::get(hello_html)
    )
    .route(
        "/demo-status",
        axum::routing::get(demo_status)
    )
    .route(
        "/demo-uri",
        axum::routing::get(demo_uri)
    )
    .route(
        "/demo.png",
        axum::routing::get(get_demo_png)
    )
    .route(
        "/foo",
        axum::routing::get(|| async { "GET foo" })
            .put(|| async { "PUT foo" })
            .patch(|| async { "PATCH foo" })
            .post(|| async { "POST foo" })
            .delete(|| async { "DELETE foo" })
    )
    .route(
        "/items/:id",
        axum::routing::get(get_items_id)
    )
    .route(
        "/items",
        axum::routing::get(get_items)
    )
    .route(
        "/demo.json",
        axum::routing::get(get_demo_json)
            .put(put_demo_json)
    )
    .route(
        "/books",
        axum::routing::get(get_books)
            .put(put_books)
    )
    .route(
        "/books/:id",
        axum::routing::get(get_books_id)
            .delete(delete_books_id)
    )
    .route("/books/:id/form",
        axum::routing::get(get_books_id_form)
        .post(post_books_id_form)
    )
    .fallback(fallback);

    let listener = tokio::net::TcpListener::bind(
        "0.0.0.0:3000"
    ).await.unwrap();

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

pub async fn hello() -> String {
    "Hello, World2".into()
}

pub async fn get_demo_html() -> axum::response::Html<&'static str> {
    "<h1>Hello</h1>".into()
}

/// include_str 매크로는 컴파일타임에 현재 main.rs 기준,
/// 적은 경로에 파일을 &'static str 타입으로 변환
pub async fn hello_html() -> axum::response::Html<&'static str> {
    include_str!("../assets/html/hello.html").into()
}

pub async fn demo_status() -> (axum::http::StatusCode, String) {
    (axum::http::StatusCode::OK, "Everything is OK".to_string())
}

pub async fn demo_uri(uri: axum::http::Uri) -> String {
    format!("The URI is: {:?}", uri)
}

/// 헤더에 image/png 와 디코드된 이미지 데이터를 리턴한다.
pub async fn get_demo_png() -> impl axum::response::IntoResponse {
    use base64::Engine;
    let png = concat!(
        "iVBORw0KGgoAAAANSUhEUgAAAAEAAAAB",
        "CAYAAAAfFcSJAAAADUlEQVR42mPk+89Q",
        "DwADvgGOSHzRgAAAAABJRU5ErkJggg=="
    );
    (
        axum::response::AppendHeaders([
            (axum::http::header::CONTENT_TYPE, "image/png"),
        ]),
        base64::engine::general_purpose::STANDARD.decode(png).unwrap(),
    )
}

/// 익스트랙터 사용 예시, Path 추출
pub async fn get_items_id(
    axum::extract::Path(id): axum::extract::Path<String>
) -> String {
    format!("Get items with path id: {:?}", id)
}

/// 익스트랙터 사용 예시, Query 추출
pub async fn get_items(
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>
) -> String {
    format!("Get items with query params: {:?}", params)
}

pub async fn get_demo_json() -> axum::extract::Json<Value> {
    json!({"a": "b"}).into()
}

pub async fn put_demo_json(
    axum::extract::Json(data): axum::extract::Json<serde_json::Value>
) -> String {
    format!("Put demo JSON data: {:?}", data)
}

pub async fn print_data() {
    thread::spawn(move || {
        let data = DATA.lock().unwrap();
        println!("data: {:?}", data);
    }).join().unwrap()
}

/// sort 하기위해 clone 해야한다.
pub async fn get_books() -> axum::response::Html<String> {
    thread::spawn(move || {
        let data = DATA.lock().unwrap();
        let mut books = data.values().collect::<Vec<_>>().clone();
        books.sort_by(|a, b| a.title.cmp(&b.title));
        books.iter()
        .map(|&book| format!("<p>{}</p>\n", &book))
        .collect::<String>()
    }).join().unwrap().into()
}

pub async fn get_books_id(
    axum::extract::Path(id): axum::extract::Path<u32>
) -> axum::response::Html<String> {
    
    thread::spawn(move || {
        let data = DATA.lock().unwrap();
        match data.get(&id) {
            Some(book) => format!("<pt>{}</p> \n", &book),
            None => format!("<p>Book id {} not found</p>", id)
        }
    }).join().unwrap().into()
}

pub async fn put_books(
    axum::extract::Json(book): axum::extract::Json<Book>
) -> axum::response::Html<String> {
    thread::spawn(move|| {
        let mut data = DATA.lock().unwrap();
        data.insert(book.id, book.clone());
        format!("Put book: {}", &book)
    }).join().unwrap().into()
}

/// axum handler for "GET /books/:id/form" which responds with a form.
/// This demo shows how to write a typical HTML form with input fields.
pub async fn get_books_id_form(
    axum::extract::Path(id): axum::extract::Path<u32>
) -> axum::response::Html<String> {
    thread::spawn(move || {
        let data = DATA.lock().unwrap();
        match data.get(&id) {
            Some(book) => format!(
                concat!(
                    "<form method=\"post\" action=\"/books/{}/form\">\n",
                    "<input type=\"hidden\" name=\"id\" value=\"{}\">\n",
                    "<p><input name=\"title\" value=\"{}\"></p>\n",
                    "<p><input name=\"author\" value=\"{}\"></p>\n",
                    "<input type=\"submit\" value=\"Save\">\n",
                    "</form>\n"
                ),
                &book.id,
                &book.id,
                &book.title,
                &book.author
            ),
            None => format!("<p>Book id {} not found</p>", id),
        }
    }).join().unwrap().into()
}

/// axum handler for "POST /books/:id/form" which submits an HTML form.
/// This demo shows how to do a form submission then update a resource.
pub async fn post_books_id_form(
    form: axum::extract::Form<Book>
) -> axum::response::Html<String> {
    let new_book: Book = form.0;
    thread::spawn(move || {
        let mut data = DATA.lock().unwrap();
        if data.contains_key(&new_book.id) {
            data.insert(new_book.id, new_book.clone());
            format!("Post book: {}", &new_book)
        } else {
            format!("Book id not found: {}", &new_book.id)
        }
    }).join().unwrap().into()
}

/// axum handler for "DELETE /books/:id" which destroys a resource.
/// This demo extracts an id, then mutates the book in the DATA store.
pub async fn delete_books_id(
    axum::extract::Path(id): axum::extract::Path<u32>
) -> axum::response::Html<String> {
    thread::spawn(move || {
        let mut data = DATA.lock().unwrap();
        if data.contains_key(&id) {
            data.remove(&id);
            format!("Delete book id: {}", &id)
        } else {
            format!("Book id not found: {}", &id)
        }
    }).join().unwrap().into()
}

/// 아무 핸든러도 찾지못하면
/// route 의 fallback 에의해
/// 이 fallback 이 실행된다.
pub async fn fallback(
    uri: axum::http::Uri
) -> impl axum::response::IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, format!("No route {uri}"))
}

/// 그레이스풀 셧다운
/// https://github.com/tokio-rs/axum/blob/main/examples/graceful-shutdown/src/main.rs
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
    };
    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
        .expect("failed to install signal handler")
        .recv()
        .await;
    };
    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();
    tokio::select! {
        _ = ctrl_c => {
            println!("shoutdown ctrl_c")
        },
        _ = terminate => {
            println!("shoutdown terminate")
        },
    }
}