use crate::pubsub::PubSub;
use crate::pubsub::publisher::agent::PubAgent;
use crate::pubsub::subscriber::agent::SubAgent;
use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Address, Agent, Context, DoAsync, Next, OnEvent, RunAgent};
use crb::runtime::{InteractiveRuntime, Runtime};
use crb::superagent::{EventBridge, StreamSession, Supervisor, SupervisorSession};
use std::sync::LazyLock;
use std::sync::OnceLock;

static REGISTRAR: OnceLock<RegistrarLink> = OnceLock::new();

static BRIDGE: LazyLock<EventBridge<Control>> = LazyLock::new(|| EventBridge::new());

pub struct RegistrarLink {
    address: Address<Registrar>,
}

impl RegistrarLink {
    pub fn spawn_pub<T: PubSub>(&self, agent: PubAgent<T>) -> Address<PubAgent<T>> {
        let runtime = RunAgent::new(agent);
        let address = runtime.address();
        let control = Control {
            runtime: Box::new(runtime),
            group: Group::Publisher,
        };
        BRIDGE.send(control);
        address
    }

    pub fn spawn_sub<T: PubSub>(&self, agent: SubAgent<T>) -> Address<SubAgent<T>> {
        let runtime = RunAgent::new(agent);
        let address = runtime.address();
        let control = Control {
            runtime: Box::new(runtime),
            group: Group::Subscriber,
        };
        BRIDGE.send(control);
        address
    }
}

pub struct Registrar {}

impl Registrar {
    pub fn link() -> &'static RegistrarLink {
        REGISTRAR.get().unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Group {
    Publisher,
    Subscriber,
}

impl Supervisor for Registrar {
    type BasedOn = StreamSession<Self>;
    type GroupBy = Group;
}

impl Agent for Registrar {
    type Context = SupervisorSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Initialize)
    }
}

struct Initialize;

#[async_trait]
impl DoAsync<Initialize> for Registrar {
    async fn handle(&mut self, _: Initialize, ctx: &mut Context<Self>) -> Result<Next<Self>> {
        let events = BRIDGE.events().await?;
        ctx.consume(events);
        Ok(Next::events())
    }
}

struct Control {
    runtime: Box<dyn Runtime>,
    group: Group,
}

#[async_trait]
impl OnEvent<Control> for Registrar {
    async fn handle(&mut self, event: Control, ctx: &mut Context<Self>) -> Result<()> {
        let rel = ctx.spawn_trackable(event.runtime, event.group);
        Ok(())
    }
}
