use iced::widget::{Button, Column, Container, Text, TextInput};
use iced::{Length, Sandbox};

use std::io::{Read, Write};
use std::net::TcpStream;
use std::process;

#[derive(Debug, Clone)]
pub enum Chet6Message {
    TextUpdated(String),
    Connect(Text),
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
                println!("Attempting connection...");
                match Self::connect(&self.address) {
                    Ok(_) => (),
                    Err(e) => {
                        
                    }
                }
            }
            Chet6Message::SendMessage => {}
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let addr_box = TextInput::new("ip:port", self.address.as_str())
            .on_input(Chet6Message::TextUpdated)
            .padding(10);

        

        let msg_log = Text::new("messages go here").height(Length::FillPortion(19));

        let conn_button = Button::new("Connect")
            .on_press(Chet6Message::Connect(msg_log))
            .width(Length::Fill)
            .height(Length::FillPortion(1));
        let msg_box: TextInput<'_, Chet6Message> =
            TextInput::new("Send a message", self.message.as_str())
                .on_submit(Chet6Message::SendMessage)
                .padding(10);

        let col = Column::new()
            .push(addr_box)
            .push(conn_button)
            .push(msg_log)
            .push(msg_box);
        Container::new(col).into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}

impl Chet6 {
    fn connect(address: &String) -> Result<(), std::io::Error> {
        let stream = TcpStream::connect(address)?;
        
        Ok(())
    }
}
