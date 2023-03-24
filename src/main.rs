use color_eyre::Result;

mod spi;
mod types;
mod ui;

fn main() -> Result<()> {
    let (mut controller, tx) = spi::Spi::new()?;
    let app = ui::Ui::new(tx)?;

    controller.spawn()?;

    app.run()
}
