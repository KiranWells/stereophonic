use color_eyre::Result;
// this cfg statement selects code based on whether we are on the Raspberry Pi
#[cfg(all(target_arch = "arm", target_os = "linux", target_env = "gnu"))]
use spidev::{SpiModeFlags, Spidev, SpidevOptions, SpidevTransfer};
use std::{
    f64::consts::PI,
    sync::mpsc::{channel, Receiver, Sender},
    thread,
    time::{self, Duration, Instant},
};

use crate::types::{AppState, ControllerMessage};

pub struct Spi {
    rx: Receiver<ControllerMessage>,
    state: AppState,
    start_time: Instant,
    #[cfg(all(target_arch = "arm", target_os = "linux", target_env = "gnu"))]
    device: Spidev,
}

impl Spi {
    pub fn new() -> Result<(Self, Sender<ControllerMessage>)> {
        let (tx, rx) = channel();

        #[cfg(all(target_arch = "arm", target_os = "linux", target_env = "gnu"))]
        {
            // initialize the SPI device
            let mut device = Spidev::open("/dev/spidev0.0")?;
            let options = SpidevOptions::new()
                .bits_per_word(8)
                .max_speed_hz(20_000)
                .mode(SpiModeFlags::SPI_MODE_0)
                .build();
            device.configure(&options)?;
        }

        let controller = Self {
            rx,
            #[cfg(all(target_arch = "arm", target_os = "linux", target_env = "gnu"))]
            device,
            start_time: Instant::now(),
            state: AppState::Paused,
        };
        Ok((controller, tx))
    }

    pub fn spawn(mut self) {
        thread::spawn(move || loop {
            // read all pending states
            while let Ok(x) = self.rx.try_recv() {
                match x {
                    ControllerMessage::Change(state) => {
                        self.state = state;
                    }
                }
            }
            // write to the device
            match self.state {
                AppState::Paused => thread::sleep(Duration::from_millis(10)),
                AppState::Constant(val) => {
                    self.set_val(val).unwrap();
                }
                AppState::Circular(val) => {
                    let time_offset = time::Instant::now()
                        .duration_since(self.start_time)
                        .as_secs_f64();
                    self.set_val(((time_offset * 2.0 * PI * val).sin() + 1.0) / 2.0)
                        .unwrap();
                }
            }
        });
    }

    /// Sets the value on the device by sending the correct signal.
    /// Also creates a debug visual in the terminal to track the signal
    /// being sent
    fn set_val(&mut self, val: f64) -> Result<()> {
        let val = (val * 255.0) as u8;

        // terminal visualization
        for _ in u8::MIN..(val / 2) {
            print!("#");
        }
        println!();

        #[cfg(all(target_arch = "arm", target_os = "linux", target_env = "gnu"))]
        self.device.transfer(&mut SpidevTransfer::write(&[val]))?;
        #[cfg(not(all(target_arch = "arm", target_os = "linux", target_env = "gnu")))]
        thread::sleep(time::Duration::from_millis(100));

        Ok(())
    }
}
