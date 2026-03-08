/* ============================================================= */
/* RTM: Rust Timer (Desktop Stopwatch)                          */
/* ============================================================= */
use iced::theme;
use iced::widget::{button, column, row, text};
use iced::{
    executor, keyboard, time, Alignment, Application, Command, Element, Event, Font, Length,
    Settings, Theme,
};
use std::time::{Duration, Instant};

pub fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.window.size = (400, 180);
    settings.window.resizable = false;
    settings.default_font = Font::with_name("PixelMplus12-Regular");

    GUI::run(settings)
}

struct GUI {
    tick_state: TickState,
    duration: Duration,
    last_tick: Instant,
}

#[derive(Debug, Clone)]
pub enum Message {
    Start,
    Stop,
    Toggle, // [s] キー用
    Reset,
    Tick(Instant),
    Minimize, // [space] キー用
    Exit,     // [Esc] キー用
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TickState {
    Stopped,
    Ticking,
}

impl Application for GUI {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (GUI, Command<Self::Message>) {
        (
            GUI {
                tick_state: TickState::Stopped,
                duration: Duration::default(),
                last_tick: Instant::now(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("RTM - Rust Timer")
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        let tick = match self.tick_state {
            TickState::Ticking => time::every(Duration::from_millis(10)).map(Message::Tick),
            TickState::Stopped => iced::Subscription::none(),
        };

        let events = iced::subscription::events_with(|event, _status| match event {
            Event::Keyboard(keyboard::Event::KeyPressed { key_code, .. }) => match key_code {
                keyboard::KeyCode::S => Some(Message::Toggle),
                keyboard::KeyCode::R => Some(Message::Reset),
                keyboard::KeyCode::Space => Some(Message::Minimize),
                keyboard::KeyCode::Escape => Some(Message::Exit),
                _ => None,
            },
            _ => None,
        });

        iced::Subscription::batch(vec![tick, events])
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Start => {
                self.tick_state = TickState::Ticking;
                self.last_tick = Instant::now();
            }
            Message::Stop => {
                self.tick_state = TickState::Stopped;
            }
            Message::Toggle => {
                if self.tick_state == TickState::Ticking {
                    return self.update(Message::Stop);
                } else {
                    return self.update(Message::Start);
                }
            }
            Message::Reset => {
                self.duration = Duration::default();
            }
            Message::Tick(now) => {
                if self.tick_state == TickState::Ticking {
                    self.duration += now - self.last_tick;
                    self.last_tick = now;
                }
            }
            Message::Minimize => {
                return iced::window::minimize(true);
            }
            Message::Exit => {
                return iced::window::close();
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let total_seconds = self.duration.as_secs();
        let hours = total_seconds / 3600;
        let minutes = (total_seconds / 60) % 60;
        let seconds = total_seconds % 60;
        let centiseconds = self.duration.subsec_millis() / 10;

        let duration_text = format!(
            "{:02}:{:02}:{:02}:{:02}",
            hours, minutes, seconds, centiseconds
        );

        let pixel_font = Font::with_name("PixelMplus12-Regular");

        let (label, message, btn_style) = match self.tick_state {
            TickState::Stopped => ("Start [S]", Message::Start, theme::Button::Secondary),
            TickState::Ticking => ("Stop [S]", Message::Stop, theme::Button::Destructive),
        };

        let start_stop_button = button(
            text(label)
                .font(pixel_font)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .width(Length::Fill),
        )
        .on_press(message)
        .style(btn_style)
        .width(120);

        let reset_button = button(
            text("Reset [R]")
                .font(pixel_font)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .width(Length::Fill),
        )
        .on_press(Message::Reset)
        .style(theme::Button::Secondary)
        .width(120);

        column![
            text(duration_text).font(pixel_font).size(60),
            row![start_stop_button, reset_button].spacing(20)
        ]
        .spacing(10)
        .padding(10)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_items(Alignment::Center)
        .into()
    }
}

/*
   =============================================================
   COLOR CHART (Theme::Dark / Iced 0.10)
   =============================================================

   [ Background / Surface ]
   - Window Background : #202225 (濃いグレー/黒)
   - Surface/Card      : #2F3136 (やや明るいグレー)

   [ Button Styles (.style) ]
   - theme::Button::Positive    : [  Start  ] -> 背景: #43B581 (緑) / 文字: White
   - theme::Button::Destructive : [  Stop   ] -> 背景: #F04747 (赤) / 文字: White
   - theme::Button::Primary     : [  Reset  ] -> 背景: #5865F2 (青) / 文字: White
   - theme::Button::Secondary   : [  Other  ] -> 背景: #4F545C (灰) / 文字: White
   - theme::Button::Text        : [  Link   ] -> 背景: 透明      / 文字: #00AFF4 (水色)

   [ Text Colors ]
   - Default Text : #FFFFFF (白)
   - Soft Text    : #B9BBBE (薄いグレー)
   =============================================================
*/
/*
   =============================================================
   BUILD & RUN
   =============================================================
   ビルドと実行 (開発モード)
      $ cargo run

   リリースビルドと実行
      $ cargo run --release

   Windows向けインストーラーの作成 (cargo-wixが必要)
       - cargo-wixのインストール
         $ cargo install cargo-wix

       - インストーラーのビルド
         $ cargo wix -v
*/
