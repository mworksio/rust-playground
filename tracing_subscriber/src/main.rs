use tracing::info;
use tracing_subscriber;

fn main() {
   tracing_subscriber::fmt::init(); 

    let yak_numbers = 3;
    // info!("prepare to shave {:?}", yak_numbers)
    info!(yak_numbers, "prepare to shave");

    let number_shaved = yak_shave::shave_all(yak_numbers);
    info!(
        all_yaks_shaved = number_shaved == yak_numbers,
        "yak shaving completed."
    );


    println!("Hello, world!");
}
