mod vibrator;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    vibrator::vibrate().await.unwrap();
}