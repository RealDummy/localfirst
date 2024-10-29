use core::num;
use std::{env::args, io::{Read, Write}, iter, net::{self, Ipv4Addr}, str::FromStr};
use std::io;

use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind}, layout::{Constraint, Direction, Layout}, style::Stylize, symbols, text::{Line, Text}, widgets::{Block, Borders, Paragraph}, DefaultTerminal
};

use tui_textarea::{Input, Key, TextArea};

use crdt::Crdt;

mod gset;
mod crdt;
mod tester;
mod store;
mod server;

#[test]
fn random_valid_order_test() {
    let mut t = tester::Tester::new("./test");
    let res: Vec<_> = (0..10).map(|_| t.test_random()).collect();
    let equal = res[1..].eq(&res[..res.len() - 1]);
    if !equal {
        panic!("not equal");
    }
}

fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    let mut messages: Vec<String> = vec![];
    let mut messages_displayed = Vec::new();
    let mut textarea = TextArea::default();
    let mut scrollback:usize = 0;
    loop {
        terminal.draw(|frame| {
            let area = Layout::default().direction(Direction::Vertical).constraints([Constraint::Min(0), Constraint::Length(3)]);
            let area_layout = area.split(frame.area());
            let inp_block = Block::bordered();
            let history = Block::new();
            let message_count = area_layout[0].height as usize;
            messages_displayed.clear();
            messages_displayed.resize(message_count, Line::from(""));
            scrollback = if scrollback + message_count > messages.len() {messages.len().saturating_sub(message_count)} else {scrollback};
            for i in (scrollback..message_count + scrollback).rev() {
                if i >= messages.len() {
                    continue;
                }
                let s = &messages[messages.len() - i - 1];
                messages_displayed[message_count - (i - scrollback) - 1] = Line::from(s.clone());
            }
            
            let text = Text::from(messages_displayed.clone());
            let p = Paragraph::new(text).block(history);
           textarea.set_block(inp_block);
            frame.render_widget(&p, area_layout[0]);
            frame.render_widget(&textarea, area_layout[1]);

        })?;

        match event::read()?.into() {
            Input { key: Key::Esc, .. } => return Ok(()),
            Input { key: Key::Enter, .. } => {
                messages.extend(textarea.lines().iter().cloned());
                textarea.select_all();
                textarea.cut();
            }
            Input { key: Key::Up, .. } => {
                scrollback = (messages).len().min(scrollback + 1);
            }
            Input { key: Key::Down, .. } => {
                scrollback = scrollback.saturating_sub(1)
            }
            input => {
                textarea.input(input);
                scrollback = 0;
            }
        }
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let peers: Vec<u16> = args().skip(1).map(|port| port.parse().unwrap()).collect();
    let app_result = run(terminal);
    ratatui::restore();
    app_result
}

