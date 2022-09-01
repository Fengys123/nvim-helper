mod error;
mod trailing_space;

use neovim_lib::{Neovim, NeovimApi, Session};
use tracing::info;
use tracing_appender::non_blocking;
use tracing_appender::{non_blocking::WorkerGuard, rolling};
use trailing_space::trailing_space_statistics;

enum Message {
    TrailSpace,
    Clippy,
    Unknown(String),
}

impl From<String> for Message {
    fn from(event: String) -> Self {
        match event.as_str() {
            "clippy" => Message::Clippy,
            "trail_space" => Message::TrailSpace,
            _ => Message::Unknown(event),
        }
    }
}

struct EventHandler {
    nvim: Neovim,
}

impl EventHandler {
    fn new() -> EventHandler {
        let session = Session::new_parent().unwrap();
        let nvim = Neovim::new(session);

        EventHandler { nvim }
    }

    fn recv(&mut self) {
        let receiver = self.nvim.session.start_event_loop_channel();

        for (event, _values) in receiver {
            info!("recv an event: {:?}", event);
            match Message::from(event) {
                Message::Clippy => {
                    self.nvim.command("vsplit | terminal").unwrap();
                    let command = ":call jobsend(b:terminal_job_id, \"cargo clippy --workspace --all-targets -- -D warnings -D clippy::print_stdout -D clippy::print_stderr\\n\")";
                    self.nvim.command(command).unwrap();
                }
                Message::TrailSpace => {
                    let buffer = self.nvim.get_current_buf().unwrap();
                    let line_count = buffer.line_count(&mut self.nvim).unwrap();
                    let text = buffer
                        .get_lines(&mut self.nvim, 0, line_count, false)
                        .unwrap();
                    self.nvim
                        .command(&format!(
                            "echo \"trailing space line: {:?}\"",
                            trailing_space_statistics(text)
                        ))
                        .unwrap();
                }
                Message::Unknown(event) => {
                    self.nvim
                        .command(&format!("echo \"Unknown command: {}\"", event))
                        .unwrap();
                }
            }
        }
    }
}

fn init_log() -> WorkerGuard {
    let (appender, guard) = non_blocking(rolling::never("/tmp/nvim/", "nvim-helper.log"));
    tracing::subscriber::set_global_default(
        tracing_subscriber::FmtSubscriber::builder()
            .with_writer(appender)
            .finish(),
    )
    .unwrap();
    guard
}

fn main() {
    let _guard = init_log();
    let mut event_handler = EventHandler::new();
    event_handler.recv();
}
