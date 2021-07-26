mod utils;

use wasm_bindgen::prelude::*;
use diamond_types::list::ListCRDT;
use diamond_types::list::external_txn::{RemoteTxn, VectorClock};
use diamond_types::AgentId;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Doc {
    inner: ListCRDT,
    agent_id: AgentId,
}

#[wasm_bindgen]
impl Doc {
    #[wasm_bindgen(constructor)]
    pub fn new(agent_name: Option<String>) -> Self {

        let mut inner = ListCRDT::new();
        let name_str = agent_name.as_ref().map_or("seph", |s| s.as_str());
        let agent_id = inner.get_or_create_agent_id(name_str);

        Doc { inner, agent_id }
    }

    #[wasm_bindgen]
    pub fn ins(&mut self, pos: usize, content: &str) {
        // let id = self.0.get_or_create_agent_id("seph");
        self.inner.local_insert(self.agent_id, pos, content.into());
    }

    #[wasm_bindgen]
    pub fn del(&mut self, pos: usize, del_span: usize) {
        self.inner.local_delete(self.agent_id, pos, del_span);
    }

    #[wasm_bindgen]
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    #[wasm_bindgen]
    pub fn get(&self) -> String {
        self.inner.to_string()
    }

    #[wasm_bindgen]
    pub fn get_version(&self) -> Result<JsValue, JsValue> {
        serde_wasm_bindgen::to_value(&self.inner.get_vector_clock())
            .map_err(|err| err.into())
    }

    #[wasm_bindgen]
    pub fn get_txn_since(&self, version: JsValue) -> Result<JsValue, JsValue> {
        let clock: VectorClock = serde_wasm_bindgen::from_value(version)?;
        let txns = self.inner.get_all_txns_since(&clock);
        serde_wasm_bindgen::to_value(&txns)
            .map_err(|err| err.into())
    }

    #[wasm_bindgen]
    pub fn merge_remote_txns(&mut self, txns: JsValue) -> Result<(), JsValue> {
        let txns: Vec<RemoteTxn> = serde_wasm_bindgen::from_value(txns)?;
        for txn in txns.iter() {
            self.inner.apply_remote_txn(txn);
        }
        Ok(())
    }
}
