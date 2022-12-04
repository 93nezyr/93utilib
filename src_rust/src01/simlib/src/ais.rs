mod env;
mod agent;
use env::Env;
use agent::Agent;

pub struct Thinker {
    env: Env,
    agent: Agent,
}

impl Thinker {
    pub fn new() -> Thinker {
        Thinker {
            env: Env::new(),
            agent: Agent::new(),
        }
    }
}

