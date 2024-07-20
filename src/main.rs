use web::play_sy_main;

#[tokio::main]
async fn main() {
    play_sy_main().await;
}

#[warn(dead_code)]
fn deadlock() {
    println!("1");
    let  a = std::sync::Mutex::new(1);
    println!("2");
    let mut g = a.lock().unwrap();
    println!("3");
    *g += 1;
    println!("4");
    // std::mem::drop(g);
    let mut g = a.lock().unwrap();
    println!("5");
    *g += 1;
    println!("6");

}