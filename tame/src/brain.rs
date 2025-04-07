use crb::agent::{Agent, AgentSession};

pub struct Brain {}

impl Agent for Brain {
    type Context = AgentSession<Self>;
}
