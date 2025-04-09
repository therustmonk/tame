use crb::agent::{Agent, AgentSession};
use crb::superagent::{Supervisor, SupervisorSession};

pub struct Registrar {}

impl Supervisor for Registrar {
    type BasedOn = AgentSession<Self>;
    type GroupBy = ();
}

impl Agent for Registrar {
    type Context = SupervisorSession<Self>;
}
