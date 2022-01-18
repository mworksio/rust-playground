use tracing::info;
use tracing_subscriber;

#[path = "yak_shave/yak_shave.rs"]
mod yak_shave;

fn main() {
    // use tracing_subscriber::{fmt, EnvFilter};

    // let collector = fmt::Collector::builder()
    //     .with_env_filter(EnvFilter::from_default_env()) 
    //     .finish();

    // tracing::collect::with_default(collector, || {
    //     let number_of_yaks = 3;
    //     tracing::debug!("prepare to shave {:?} yaks", number_of_yaks);
       
    //     let number_shaved = yak_shave::shave_all(number_of_yaks);
    //     tracing::debug!(
    //         message = "yak shaving completed.",
    //         all_yaks_shaved = number_shaved == number_of_yaks,
    //     );
    // });

    tracing_subscriber::fmt::init(); 
    let yak_numbers = 3;
    // info!("prepare to shave {:?}", yak_numbers)
    info!(yak_numbers, "prepare to shave");

    let number_shaved = yak_shave::shave_all(yak_numbers);
    info!(
        all_yaks_shaved = number_shaved == yak_numbers,
        "yak shaving completed."
    );
}
