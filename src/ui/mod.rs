use std::sync::mpsc::Sender;

use crate::types::{self, AppState, ControllerMessage};
use color_eyre::Report;
use iced::{
    theme,
    widget::{self, button, container, slider, text, Column, Row},
    Application, Color, Length, Subscription, Theme,
};

pub struct Ui {
    tx: Sender<types::ControllerMessage>,
    state: UiState,
    current_error: Option<Report>,
    value: u16,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiState {
    Paused,
    Constant,
    Circular,
}

#[derive(Debug, Clone)]
pub enum UiMessage {
    ChangeState(UiState),
    SetValue(u16),
    ClearError,
}

const MAX_FREQUENCY: f64 = 10.0;

impl Ui {
    /// Sends the current state to the SPI device thread, handling any relevant errors
    fn send_state(&mut self) {
        let new_state = match self.state {
            UiState::Paused => AppState::Paused,
            UiState::Constant => AppState::Constant(self.value as f64 / u16::MAX as f64),
            UiState::Circular => AppState::Circular(Self::value_to_frequency(self.value)),
        };
        match self.tx.send(ControllerMessage::Change(new_state)) {
            Ok(()) => {}
            Err(e) => self.current_error = Some(e.into()),
        }
    }

    /// Creates a label for the corresponding value slider based on the state
    fn get_value_label(&self) -> String {
        match self.state {
            // we don't have a label in Paused state
            UiState::Paused => unreachable!(),
            UiState::Constant => {
                let percent = (self.value as f64 / u16::MAX as f64 - 0.5) * 2.0;
                format!(
                    "Position: {:.2}% {}",
                    percent.abs() * 100.0,
                    if percent < 0.0 { "left" } else { "right" }
                )
            }
            UiState::Circular => {
                format!("Frequency: {:.2} Hz", Self::value_to_frequency(self.value))
            }
        }
    }

    /// Converts an internal value into a frequency
    fn value_to_frequency(val: u16) -> f64 {
        let of_one = val as f64 / u16::MAX as f64;
        ((of_one - 0.5) * 2.0 * MAX_FREQUENCY.ln()).exp()
    }

    /// create a button from the given state
    fn state_button(
        &self,
        state: UiState,
    ) -> widget::Button<
        '_,
        <Self as Application>::Message,
        iced::Renderer<<Self as Application>::Theme>,
    > {
        button(text(format!("{state:?}")).horizontal_alignment(iced::alignment::Horizontal::Center))
            .on_press(UiMessage::ChangeState(state))
            .style(if self.state == state {
                theme::Button::Primary
            } else {
                theme::Button::Secondary
            })
            .width(Length::Fill)
    }
}

impl Application for Ui {
    type Executor = iced::executor::Default;
    type Message = UiMessage;
    type Flags = (Sender<types::ControllerMessage>,);
    type Theme = Theme;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let (tx,) = flags;
        let ui = Self {
            tx,
            state: UiState::Paused,
            current_error: None,
            value: 0,
        };
        (ui, iced::Command::none())
    }

    fn title(&self) -> String {
        String::from("Signal control")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            UiMessage::ChangeState(new_state) => {
                self.state = new_state;
                self.send_state();
            }
            UiMessage::ClearError => self.current_error = None,
            UiMessage::SetValue(v) => {
                self.value = v;
                self.send_state();
            }
        }
        iced::Command::none()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        Subscription::none()
    }

    fn view(&self) -> iced::Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let title = text("Signal control")
            .size(40)
            .style(Color::from_rgb(0.7, 0.7, 0.7))
            .horizontal_alignment(iced::alignment::Horizontal::Center);

        let mut content = Column::new()
            .align_items(iced::Alignment::Center)
            .spacing(20)
            .push(title);

        if let Some(e) = &self.current_error {
            content = content
                .push(text("Error:").style(Color::from_rgb(0.7, 0.1, 0.1)))
                .push(text(format!("{}", e)))
                .push(
                    button(text("Close"))
                        .on_press(UiMessage::ClearError)
                        .style(theme::Button::Destructive),
                );
        }

        let tabs = Row::with_children(
            [UiState::Paused, UiState::Constant, UiState::Circular]
                .into_iter()
                .map(|s| self.state_button(s).into())
                .collect(),
        )
        .align_items(iced::Alignment::Fill)
        .height(50)
        .width(Length::Fill)
        .spacing(10)
        .padding(10);

        content = content.push(tabs);

        if self.state != UiState::Paused {
            let value_slider = slider(0..=u16::MAX, self.value, UiMessage::SetValue).width(200);
            let row = Row::new()
                .spacing(20)
                .push(text(self.get_value_label()))
                .push(value_slider);
            content = content.push(row);
        }

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
