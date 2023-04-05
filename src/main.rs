use color_eyre::Result;
use iced::{Application, Settings};
use ui::Ui;

mod spi;
mod types;
mod ui;

fn main() -> Result<()> {
    let (controller, tx) = spi::Spi::new()?;
    controller.spawn();

    Ui::run(Settings::with_flags((tx,))).map_err(|e| e.into())
}
