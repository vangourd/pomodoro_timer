extern crate iced;

use iced::{executor, Theme, Application, Command, Element, Settings};

#[derive(Debug, Default)]
struct PomodoroTimer {

}

impl Application for PomodoroTimer {
    type Executor = executor::Default;
    type Message = ();
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (PomodoroTimer, Command<Self::Message>) {
        (PomodoroTimer::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Pomodoro Timer")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        "Pomodoro Timer".into()
    }
}

fn main() -> iced::Result {
    PomodoroTimer::run(Settings::default())
}
