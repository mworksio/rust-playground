// use crate::{Command, Connection, Db, DbDropGuard, Shutdown};
use tokio::net::{TcpListener, TcpStream};
use tracing::{info, error, trace, instrument, debug};
use std::future::Future;
use tokio::sync::{broadcast, mpsc};
use crate::{Connection, Command, Db, Shutdown, DbDropGuard};

pub async fn run(listener: TcpListener, shutdown: impl Future) {
    trace!("in run");

    let (notify_shutdown , _) = broadcast::channel(1);
    // NOTE: Using turbofish when we only need to variable declaration. 
    // let (x , _) = broadcast::channel::<broadcast::Sender<()>>(1);
    // let (shutdown_complete_tx, shutdown_complete_rx) = mpsc::channel(1);

    let mut server: Listener = Listener {
        listener,
        notify_shutdown,
        db_holder: DbDropGuard::new(),
        // shutdown_complete_tx,
        // shutdown_complete_rx,
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
            trace!("looping");

            let socket = self.accept().await?;
            let mut handler = Handler {
                connection: Connection::new(socket),
                db: self.db_holder.db(),
                shutdown: Shutdown::new(self.notify_shutdown.subscribe()),
            };

            tokio::spawn(async move {
                if let Err(err) = handler.run().await {
                    error!(cause =?err, "connection error");
                }
            });
        }
    }

    async fn accept(&mut self) -> crate::Result<TcpStream> {
        loop {
            match self.listener.accept().await {
                Ok((socket, _)) => return Ok(socket),
                Err(err) => {
                    return Err(err.into());
                }
            }
        }   
    }
}

struct Listener {
    listener: TcpListener,
    notify_shutdown: broadcast::Sender<()>, 
    db_holder: DbDropGuard,
    // shutdown_complete_tx: mpsc::Sender<()>,
    // shutdown_complete_rx: mpsc::Receiver<()>,
}

// pub fn 
#[derive(Debug)]
struct Handler {
    connection: Connection,
    db: Db,
    shutdown: Shutdown,
}

impl Handler {
    #[instrument(skip(self))]
    async fn run(&mut self) -> crate::Result<()> {
        while true {
            let maybe_frame = tokio::select! {
                res = self.connection.read_frame() => res?,
            };

            let frame = match maybe_frame {
                Some(frame) => frame,
                None => return Ok(()),
            };

            let cmd = Command::from_frame(frame)?;
            debug!(?cmd);
            cmd.apply(&self.db, &mut self.connection, &mut self.shutdown).await?;
        }
        Ok(())
    }
}