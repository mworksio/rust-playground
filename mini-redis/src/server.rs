// use crate::{Command, Connection, Db, DbDropGuard, Shutdown};
use tokio::net::{TcpListener};
use tracing::{debug, info, error, instrument};
use std::future::Future;

pub async fn run(listener: TcpListener, shutdown: impl Future) {

    let mut server: Listener = Listener {
        listener,
    };

    tokio::select! {
        res = server.run() => {
            if let Err(err) = res {
                error!(cause = %err, "failed to accept");
            }    
        }
        _ = shutdown => {
            info!("shutdown");
        }
    }
}

/// What is it ?
impl Listener {
    async fn run(&mut self) -> crate::Result<()> {
        info!("accepting inbound connections");
        
        loop {

        }
    }
}

struct Listener {
    listener: TcpListener,
}

// pub fn 

