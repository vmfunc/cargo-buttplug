mod vibrator;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // get all args 
    let args: Vec<String> = std::env::args().collect();
    println!("args: {:?}", args);

    // run cargo with args and interate over the exit code
    let code = std::process::Command::new("cargo")
        .args(&args[2..])
        .status()
        .expect("failed to execute process") //yeah ik
        .code()
        .unwrap();


    if code == 0 {
        println!("you deserve a reward~ 💖");
        vibrator::vibrate().await.unwrap();
    } else {
        println!("do better next time~~");
    }

    // exit
    std::process::exit(code);
}