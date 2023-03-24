use color_eyre::Result;
use spidev::{SpiModeFlags, Spidev, SpidevOptions};
use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};

use crate::types::{AppState, ControllerMessage};

pub struct Spi {
    rx: Receiver<ControllerMessage>,
    state: AppState,
    device: Spidev,
}

impl Spi {
    pub fn new() -> Result<(Self, Sender<ControllerMessage>)> {
        let (tx, rx) = channel();

        // initialize the SPI device
        let mut device = Spidev::open("/dev/spidev0.0")?;
        let options = SpidevOptions::new()
            .bits_per_word(8)
            .max_speed_hz(20_000)
            .mode(SpiModeFlags::SPI_MODE_0)
            .build();
        device.configure(&options)?;

        let controller = Self {
            rx,
            device,
            state: AppState::Paused,
        };
        Ok((controller, tx))
    }

    pub fn spawn(&mut self) -> Result<()> {
        thread::spawn(move || loop {
            while let Ok(x) = self.rx.try_recv() {
                match x {
                    ControllerMessage::Change(state) => {
                        self.state = state;
                    }
                }
            }
        });
        Ok(())
    }
}
