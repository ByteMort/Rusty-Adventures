use gloo::storage::{LocalStorage, Storage};
use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(Store, Default, PartialEq, Clone, Serialize, Deserialize)]
// #[store(storage="local", storage_tab_sync)]
#[store(listener(AuthPersister))]
pub struct AuthStore{
    pub username: Option<String>,
    pub password: Option<String>,
    pub is_authenticated: bool
}

pub struct AuthPersister;

impl Listener for AuthPersister{
    type Store = AuthStore;

    fn on_change(&self, _cx: &yewdux::Context, state: std::rc::Rc<Self::Store>) {
        let _ = LocalStorage::set("my_app_s", &*state);
    }
}