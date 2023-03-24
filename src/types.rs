pub enum AppState {
    Paused,
    Constant(f64),
    Circular(f64),
}

pub enum ControllerMessage {
    Change(AppState),
}
