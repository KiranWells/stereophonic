use std::sync::mpsc::Sender;

use crate::types;
use color_eyre::Result;

pub struct Ui {
    tx: Sender<types::ControllerMessage>,
}

impl Ui {
    pub fn new(tx: Sender<types::ControllerMessage>) -> Result<Self> {
        Ok(Self { tx })
    }
}
