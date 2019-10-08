use r2r;
use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

use std::io;

use failure::Error;

use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::layout::Alignment;
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, Text, Widget};
use tui::Terminal;

mod event;
use event::*;

fn count_newlines(s: &str) -> usize {
    s.as_bytes().iter().filter(|&&c| c == b'\n').count()
}

fn main() -> Result<(), Error> {
    let ctx = r2r::Context::create()?;
    let mut node = r2r::Node::create(ctx, "echo", "")?;

    let args: Vec<String> = env::args().collect();
    let topic = args.get(1).expect("provide a topic!");

    // run for a while to populate the topic list
    let mut count = 0;
    let mut nt = HashMap::new();
    while count < 50 {
        thread::sleep(Duration::from_millis(10));
        nt = node.get_topic_names_and_types()?;
        if nt.contains_key(topic) {
            break;
        }
        count += 1;
    }

    let type_name = nt.get(topic).and_then(|types| types.get(0));
    let type_name = match type_name {
        Some(tn) => tn,
        None => {
            eprintln!("Could not determine the type for the passed topic");
            return Ok(());
        }
    };

    let display = Rc::new(RefCell::new(String::new()));
    let display_cb = display.clone();

    let cb = move |msg: r2r::Result<serde_json::Value>| {
        if let Ok(msg) = msg {
            let s = serde_json::to_string_pretty(&msg).unwrap();
            *display_cb.borrow_mut() = s;
        }
    };

    let _subref = node.subscribe_untyped(topic, type_name, Box::new(cb))?;

    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;
    let events = Events::new();

    let mut last_seen = String::new();
    let mut scroll = 0;
    loop {
        node.spin_once(std::time::Duration::from_millis(20));

        terminal.draw(|mut f| {
            let size = f.size();

            let str = display.borrow().to_owned();

            let fg = if last_seen != str {
                last_seen = str.clone();
                Color::Green
            } else {
                Color::Reset
            };

            let text = [Text::raw(&str)];
            let title = format!("topic: {}, type: {}", topic, type_name);

            let block = Block::default()
                .borders(Borders::ALL)
                .title_style(Style::default().modifier(Modifier::BOLD).fg(fg));
            Paragraph::new(text.iter())
                .block(block.clone().title(&title))
                .alignment(Alignment::Left)
                .style(Style::default())
                .scroll(scroll)
                .render(&mut f, size);
        })?;

        let size = terminal.size()?;
        let msg_lines = count_newlines(&last_seen) as u16;

        match events.next()? {
            Event::Input(key) => {
                if key == Key::PageUp {
                    if scroll < size.height - 3 {
                        scroll = 0;
                    } else {
                        scroll -= size.height - 3;
                    }
                }
                if key == Key::PageDown {
                    scroll += size.height + 3;
                    if scroll > (msg_lines - size.height + 3) {
                        scroll = msg_lines - size.height + 3
                    };
                }
                if key == Key::Up {
                    if scroll > 0 {
                        scroll -= 1;
                    }
                }
                if key == Key::Down {
                    if scroll < (msg_lines - size.height + 3) {
                        scroll += 1
                    };
                }
                if key == Key::Char('q') {
                    break;
                }
                if key == Key::Esc {
                    break;
                }
            }
            _ => {}
        }
    }
    Ok(())
}
