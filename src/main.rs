extern crate iced;

use iced::{executor, Theme, Application, Command, Element, Settings,Length};
use iced::widget::{button, text,Button,Container,Column};

#[derive(Debug,Clone)]
enum TimerMessage {
    Start,
    Stop,
    Reset
}

#[derive(Debug, Default)]
struct PomodoroTimer {
    time_left: u32,
    is_running: bool,
    start_button_state: button::State,
    reset_button_state: button::State,
}

impl Application for PomodoroTimer {
    type Executor = executor::Default;
    type Message = TimerMessage;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (PomodoroTimer, Command<Self::Message>) {
        (PomodoroTimer::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Pomodoro Timer")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            TimerMessage::Start => {
                self.start();
            }
            TimerMessage::Stop=> {
                self.stop();
            }
            TimerMessage::Reset => {
                self.reset();
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let start_button = Button::new("Start")
            .on_press(TimerMessage::Start);

        let stop_button = Button::new("Stop")
            .on_press(TimerMessage::Stop);

        let reset_button = Button::new("Reset")
            .on_press(TimerMessage::Reset);

        let timer_display = text::Text::new(format!("{:02}:{:02}", self.time_left / 60, self.time_left % 60));
        
        let content = Column::new()
            .spacing(20)
            .push(timer_display)
            .push(start_button)
            .push(stop_button)
            .push(reset_button);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

fn main() -> iced::Result {
    PomodoroTimer::run(Settings::default())
}

impl PomodoroTimer {
    fn start(&mut self) {
        self.is_running = true;
    }

    fn stop(&mut self) {
        self.is_running = false;
    }

    fn reset(&mut self) {
        self.is_running = false;
        self.time_left = 25 * 60;
    }
}