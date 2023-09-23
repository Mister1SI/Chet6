use iced::widget::{Button, Column, Container, Text, TextInput};
use iced::{Sandbox, Length};


#[derive(Debug, Clone)]
pub enum Chet6Message {
    TextUpdated(String),
    Connect,
    SendMessage,
}

pub struct Chet6 {
    address: String,
    message: String,
}

impl Sandbox for Chet6 {
    type Message = Chet6Message;

    fn new() -> Self {
        Chet6 {
            address: String::from("address:port"),
            message: String::from(""),
        }
    }

    fn title(&self) -> String {
        String::from("Chet6")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Chet6Message::TextUpdated(t) => {
                println!("Text: {}", t);
                self.address = t;
                println!("Address: {}", self.address);
            }
            Chet6Message::Connect => {
                println!("Attempting connection...")
            }
            Chet6Message::SendMessage => {

            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let addr_box = TextInput::new("ip:port", self.address.as_str())
            .on_input(Chet6Message::TextUpdated)
            .padding(10);

        let conn_button = Button::new("Connect").on_press(Chet6Message::Connect).width(Length::Fill)
            .height(Length::FillPortion(1));

        let msg_log = Text::new("1\n2\n3\n4\n5\n6\n7\n8\n9").height(Length::FillPortion(19));

        let msg_box: TextInput<'_, Chet6Message> = TextInput::new("Send a message", self.message.as_str())
            .on_submit(Chet6Message::SendMessage).padding(10);

        let col = Column::new().push(addr_box).push(conn_button).push(msg_log).push(msg_box);
        Container::new(col).into()
    }
}
