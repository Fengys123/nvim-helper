use neovim_lib::{Neovim, Session, NeovimApi};

enum Message {
    Clippy,
    Unknown(String),
}

impl From<String> for Message {
    fn from(event: String) -> Self {
        match event.as_str() {
            "clippy" => Message::Clippy,
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
            match Message::from(event) {
                Message::Clippy => {
                    self.nvim.command("vsplit | terminal").unwrap();
                    let command = ":call jobsend(b:terminal_job_id, \"cargo clippy --workspace --all-targets -- -D warnings -D clippy::print_stdout -D clippy::print_stderr\\n\")";
                    self.nvim.command(command).unwrap();
                }
                Message::Unknown(event) => {
                    self.nvim
                        .command(&format!("echo \"Unknown command: {}\"", event))
                        .unwrap();
                },
            }
        }
    }
}

fn main() {
    let mut event_handler = EventHandler::new();

    event_handler.recv();
}
