use crate::{commander::Error, Commander, LocalnetHandle};
use fehler::throws;
use std::{borrow::Cow, mem};

#[derive(Default)]
pub struct Tester {
    root: Cow<'static, str>,
}

impl Tester {
    pub fn new() -> Self {
        Self {
            root: "../../".into(),
        }
    }

    pub fn with_root(root: impl Into<Cow<'static, str>>) -> Self {
        Self { root: root.into() }
    }

    #[throws]
    pub async fn before(&mut self) -> LocalnetHandle {
        println!("_____________________");
        println!("____ BEFORE TEST ____");
        let commander = Commander::with_root(mem::take(&mut self.root));
        commander.start_localnet().await?
    }

    #[throws]
    pub async fn after(&self, localnet_handle: LocalnetHandle) {
        println!("____ AFTER TEST ____");
        localnet_handle.stop().await?;
        println!("_____________________");
    }
}