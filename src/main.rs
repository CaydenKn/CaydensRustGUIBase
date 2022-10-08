use iced::{
    button, Button, Column, Container, Element, Length, Sandbox,
    Settings, Text,
};

pub fn main() -> iced::Result {
    Exit::run(Settings::default())
}

#[derive(Default)]
struct Exit {
    ex_btn: button::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Example,
}

impl Sandbox for Exit {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Cayden's Rust Environment Base")
    }

    fn update(&mut self, _message: Message) {
        
    }

    fn view(&mut self) -> Element<Message> {
        let content = Column::new()
                .spacing(10)
                .push(Text::new("Hello World!"))
                .push(
                    Button::new(&mut self.ex_btn, Text::new("Start"))
                        .padding([10, 20])
                        .on_press(Message::Example),
                 );

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .into()
    }
}