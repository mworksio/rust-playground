#[derive(Clone, Debug)]
pub struct Config {
    pub outbound: outbound::Config,    
    pub inbount: inbound::Config,
    // pub gateway: gateway::Config,

    // pub dns: dns::Config,
    pub identity: identity::Config,
    // pub dst: dst::Config,
    pub admin: admin::Config,
    pub tap: tap::Config,
    pub oc_collector: oc_collector::Config,
}

impl Config {
    pub fn try_from_env() -> Result<Self, new::EnvError> {
        env::Env.try_config()
    }

    pub async fn build<BIn, BOut, BAdmin>(
        self,
        bind_in: BIn,
        bind_out: BOut,
        bind_admin: BAdmin,
        shutdown_tx: mpsc::UnboundedSender<()>,
        log_level: trace::Handle,
    ) -> Result<>App, Error>
    where
        BIn: Bind<ServerConfig> + 'static,
        BIn::Addrs: Param<Remote<ClientAddr>> + Param<Local<ServerAddr>> + Param<OrigDstAddr>,
        BOut: Bind<ServerConfig> + 'static,
        BOut::Addrs: Param<Remote<ClientAddr>> + Param<Local<ServerAddr>> + Param<OrigDstAddr>,
        BAdmin: Bind<ServerConfig> + Clone + 'static,
        BAdmin::Addrs: Param<Remove<ClientAddr>> + Param<Local<ServerAddr>>,
        {

        }
}