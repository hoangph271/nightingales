use iced::button;
use iced::{Button, Column, Element, Sandbox, Settings, Text};

struct Counter {
  value: i32,
  increment_button: button::State,
  decrement_button: button::State,
}
impl Sandbox for Counter {
  type Message = Message;

  fn new() -> Self {
    Self {
      value: 0,
      increment_button: button::State::new(),
      decrement_button: button::State::new(),
    }
  }
  fn title(&self) -> String {
    String::from("Counter - Iced")
  }
  fn update(&mut self, message: Message) {
    match message {
      Message::IncrementPressed => self.value += 1,
      Message::DecrementPressed => self.value -= 1,
    }
  }
  fn view(&mut self) -> Element<Message> {
    Column::new()
      .push(
        Button::new(&mut self.increment_button, Text::new("+")).on_press(Message::IncrementPressed),
      )
      .push(Text::new(&self.value.to_string()).size(40))
      .push(
        Button::new(&mut self.decrement_button, Text::new("-")).on_press(Message::DecrementPressed),
      )
      .into()
  }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
  IncrementPressed,
  DecrementPressed,
}

pub fn start_app() {
  Counter::run(Settings::default());
}
