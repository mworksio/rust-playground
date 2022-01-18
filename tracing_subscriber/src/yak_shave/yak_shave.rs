use snafu::{ResultExt, Snafu};
use std::error::Error;
use thiserror::Error;
use tracing::{debug, error, info, span, trace, warn, Level};

// #[tracing::Instrument]
pub fn shave(yak: usize) -> Result<(), Box<dyn Error + 'static>> {
    trace!(excitement = "yay!", "hello! I'm gonna shave a yak");
    if yak == 3 {
        warn!("could not locate yak");
        return OutOfCash
            .fail()
            .map_err(|source| MissingYakError::OutOfSpace { source })
            .context(MissingYak)
            .map_err(|err| err.into());
    } else {
        trace!("yak shaved successfully!");
    }
    Ok(())
}

pub fn shave_all(yaks: usize) -> usize {
    let span = span!(Level::INFO, "shaving_yaks", yaks);
    let _enter = span.enter();

    info!("shaving yask");

    let mut yaks_shaved = 0;
    for yak in 1..=yaks {
        let res = shave(yak);
        debug!(target: "yak_events", yak, shaved = res.is_ok());

        if let Err(ref error) = res {
            error!(yak, error = error.as_ref(), "failed to shave yak");
        } else {
            yaks_shaved += 1;
        }
        trace!(yaks_shaved);
    }

    yaks_shaved
}

#[derive(Debug, Snafu)]
enum OutOfSpaceError {
    #[snafu(display("out of cash"))]
    OutOfCash,
}

#[derive(Debug, Error)]
enum MissingYakError {
    #[error("out of space")]
    OutOfSpace { source: OutOfSpaceError },
}

#[derive(Debug, Snafu)]
enum YakError {
    #[snafu(display("missing yak"))]
    MissingYak { source: MissingYakError },
}