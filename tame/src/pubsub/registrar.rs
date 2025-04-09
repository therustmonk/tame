use crate::pubsub::PubSub;
use crate::pubsub::publisher::agent::PubAgent;
use crb::agent::{Address, Agent, AgentSession, RunAgent};
use crb::runtime::InteractiveRuntime;
use crb::superagent::{Supervisor, SupervisorSession};
use std::sync::OnceLock;

static REGISTRAR: OnceLock<RegistrarLink> = OnceLock::new();

pub struct RegistrarLink {
    address: Address<Registrar>,
}

impl RegistrarLink {
    pub fn spawn_pub<T: PubSub>(&self, agent: PubAgent<T>) -> Address<PubAgent<T>> {
        let runtime = RunAgent::new(agent);
        let address = runtime.address();
        // TODO: Spanw the runtime in the registrar
        address
    }
}

pub struct Registrar {}

impl Registrar {
    pub fn link() -> &'static RegistrarLink {
        REGISTRAR.get().unwrap()
    }
}

impl Supervisor for Registrar {
    type BasedOn = AgentSession<Self>;
    type GroupBy = ();
}

impl Agent for Registrar {
    type Context = SupervisorSession<Self>;
}
