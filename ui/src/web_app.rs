use anyhow::Result;
use async_trait::async_trait;
use crb::agent::{Agent, AgentSession, Context, DoAsync, Next, Standalone};
use crb::superagent::{Supervisor, SupervisorSession};

pub struct WebApp {}

impl WebApp {
    pub fn new() -> Self {
        Self {}
    }
}

impl Standalone for WebApp {}

impl Supervisor for WebApp {
    type BasedOn = AgentSession<Self>;
    type GroupBy = ();
}

impl Agent for WebApp {
    type Context = SupervisorSession<Self>;

    fn begin(&mut self) -> Next<Self> {
        Next::do_async(Bootstrap)
    }
}

struct Bootstrap;

#[async_trait]
impl DoAsync<Bootstrap> for WebApp {
    async fn handle(&mut self, _: Bootstrap, _ctx: &mut Context<Self>) -> Result<Next<Self>> {
        Ok(Next::events())
    }
}
