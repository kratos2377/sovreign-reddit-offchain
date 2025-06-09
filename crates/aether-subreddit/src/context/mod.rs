use std::sync::{Arc, Mutex};

use reqwest::Client;




pub type DynContext = Arc<dyn Context>;

pub trait Context: Sync + Send {
    fn get_reqwest_client(&self) -> Client;
}

#[derive(Clone)]
pub struct ContextImpl {
    pub reqwest_client: Client
}

impl ContextImpl {
    pub fn new_dyn_context(
        reqwest_client: Client,
    ) -> DynContext {
        let context = ContextImpl {
            reqwest_client: reqwest_client
        };
        let context: DynContext = Arc::new(context);
        context
    }
}

impl Context for ContextImpl {
 fn get_reqwest_client(&self) -> Client {
       self.reqwest_client.clone()
    }
}