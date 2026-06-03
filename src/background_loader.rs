use std::{
    sync::mpsc::{Receiver, channel},
    thread,
};

use crate::gentoo::{package::Package, portage_loader::load_available_packages};

pub enum LoaderMessage {
    Loading,
    Complete(Vec<Package>),
    Error(String),
}

#[derive(Debug)]
pub struct PackageLoader {
    receiver: Receiver<LoaderMessage>,
    thread_handle: thread::JoinHandle<()>,
}

impl PackageLoader {
    pub fn spawn() -> Self {
        let (tx, rx) = channel();
        let thread_handle = thread::spawn(move || {
            let _ = tx.send(LoaderMessage::Loading);

            match load_available_packages() {
                Ok(packages) => {
                    let _ = tx.send(LoaderMessage::Complete(packages));
                }
                Err(e) => {
                    let _ = tx.send(LoaderMessage::Error(e.to_string()));
                }
            }
        });

        PackageLoader {
            receiver: rx,
            thread_handle,
        }
    }

    pub fn try_recv(&self) -> Option<LoaderMessage> {
        self.receiver.try_recv().ok()
    }

    //TODO: join thread back to main thread after packages were loaded
}
