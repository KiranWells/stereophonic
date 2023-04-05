#[derive(Debug, Clone)]
pub enum AppState {
    Paused,
    Constant(f64),
    Circular(f64),
}

#[derive(Debug)]
pub enum ControllerMessage {
    Change(AppState),
}
