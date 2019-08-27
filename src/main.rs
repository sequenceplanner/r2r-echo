use r2r;
use std::thread;
use std::env;
use std::collections::HashMap;
use std::time::Duration;
use std::cell::RefCell;
use std::rc::Rc;

use std::io;

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

fn main() -> Result<(), ()> {
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
        if nt.contains_key(topic) { break; }
        count += 1;
    }

    let type_name = nt.get(topic).and_then(|types|types.get(0));
    let type_name = match type_name {
        Some(tn) => tn,
        None => {
            eprintln!("Could not determine the type for the passed topic");
            return Err(());
        },
    };

    let display = Rc::new(RefCell::new(String::new()));
    let display_cb = display.clone();

    let cb = move |msg: serde_json::Value | {
        let s = serde_json::to_string_pretty(&msg).unwrap();
        *display_cb.borrow_mut() = s;
    };

    let _subref = node.subscribe_untyped(topic, type_name, Box::new(cb))?;

    let stdout = io::stdout().into_raw_mode().map_err(|_|())?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend).map_err(|_|())?;
    terminal.hide_cursor().map_err(|_|())?;
    let events = Events::new();

    let mut last_seen = String::new();
    loop {
        node.spin_once(std::time::Duration::from_millis(20));

        terminal.draw(|mut f| {
            let size = f.size();

            let str = display.borrow().to_owned();

            let bg = if last_seen != str {
                last_seen = str.clone();
                Color::Green
            } else { Color::White };

            let text = [ Text::raw(&str), ];
            let title = format!("topic: {}, type: {}", topic, type_name);

            let block = Block::default()
                .borders(Borders::ALL)
                .title_style(Style::default().modifier(Modifier::BOLD));
            Paragraph::new(text.iter())
                .block(block.clone().title(&title))
                .alignment(Alignment::Left)
                .style(Style::default().bg(bg))
                .render(&mut f, size);
        }).map_err(|_|())?;

        match events.next().map_err(|_|())? {
            Event::Input(key) => {
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
