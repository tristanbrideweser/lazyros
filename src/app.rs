use std::{io, sync::mpsc, thread, time::Duration};

use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{
    layout::{Constraint, Layout},
    prelude::{Buffer, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::Line,
    widgets::{Block, Gauge, Widget},
    DefaultTerminal, Frame,
};

pub struct App {
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(widget, frame.area())
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where 
        Self: Sized, 
        {
            Line::from("Process overview").bold().render(area, buf);
        }
}