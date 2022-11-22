use iced::alignment;
use iced::theme;
use iced::widget::{
    checkbox, column, container, horizontal_space, image, radio, row, scrollable, slider, text,
    text_input, toggler, vertical_space,
};
use iced::widget::{Button, Column, Container, Slider};
use iced::{Color, Element, Length, Renderer, Sandbox, Settings};

fn main() -> iced::Result {
    app::State::run(Settings::default())
}

mod app {
    use super::*;

    /// Global (application) state.
    struct State {
        page: page::State,
    }

    impl Sandbox for State {
        type Message = app::Message;

        fn new() -> Self {
            Self {
                page: app::page::State::new(),
            }
        }

        fn title(&self) -> String {
            "Application".to_string()
        }

        fn update(&mut self, message: Self::Message) {
            todo!()
        }

        fn view(&self) -> Element<'_, Self::Message> {
            // let Self { page, .. } = self;

            // let content: Element<_> = column![

            // ]
            todo!();
        }
    }

    /// Globally relevant messages.
    #[derive(Debug, Clone)]
    pub enum Message {
        BackPressed,
        NextPressed,
        PageMessage(page::Message),
    }

    /// Modularized pages.
    pub mod page {
        // Per page state(s).
        pub struct State {
            pages: Vec<Type>,
            current: usize,
        }

        impl State {
            pub fn new() -> Self {
                Self {
                    pages: vec![
                        Type::Start(start::State::new()),
                        Type::End(end::State::new()),
                    ],
                    current: 0,
                }
            }
        }

        #[derive(Debug, Clone)]
        pub enum Message {
            Start(start::Message),
            End(end::Message),
        }

        pub enum Type {
            Start(start::State),
            End(end::State),
        }

        pub mod start {
            pub struct State {
                value: usize,
            }

            impl State {
                pub fn new() -> Self {
                    Self { value: 0 }
                }
            }

            #[derive(Debug, Clone)]
            pub enum Message {
                Event(u8),
            }
        }

        pub mod end {
            pub struct State {
                value: usize,
            }

            impl State {
                pub fn new() -> Self {
                    Self { value: 0 }
                }
            }

            #[derive(Debug, Clone)]
            pub enum Message {
                Event(u8),
            }
        }
    }
}
