use anyhow::Result;
use crossterm::{
    cursor::MoveToPreviousLine,
    event::{KeyCode, KeyEvent},
};
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets,
    widgets::{Block, Borders, List, ListItem, ListState, Widget},
};
use tracing::info;

use crate::action::Action;
use crate::action::LineFilter;

use tui_textarea::{CursorMove, TextArea};

use super::{Component, Frame};

#[derive(Default)]
pub struct TextEntry<'a> {
    // state: TuiWidgetState,
    pub textarea: TextArea<'a>,
}

impl TextEntry<'_> {
    pub fn contents(&self) -> &String {
        &self.textarea.lines()[0]
    }

    pub fn clear(&mut self) {
        self.textarea.move_cursor(CursorMove::End);
        self.textarea.delete_line_by_head();
    }

    pub fn pop(&mut self) -> String {
        let returning = self.contents().clone();
        self.clear();
        returning
    }
}

impl<'a> Component for TextEntry<'a> {
    fn init(&mut self) -> Result<()> {
        Ok(())
    }

    fn on_key_event(&self, key: KeyEvent) -> Action {
        match key.code {
            KeyCode::Enter => Action::ConfirmTextEntry,
            KeyCode::Esc => Action::CloseTextEntry,
            keycode => Action::TextEntry(key),
            // _ => { self.textarea.input(key); Action::Tick },
            // _ => { self.textarea.withinput(key); Action::Tick },
            // TODO Send Key Action
        }
    }

    fn dispatch(&mut self, action: Action) -> Option<Action> {
        if let Action::TextEntry(key) = action {
            use ratatui::crossterm::event::{
                KeyCode as RKeyCode, KeyEvent as RKeyEvent, KeyEventKind as RKeyEventKind,
                KeyEventState as RKeyEventState, KeyModifiers as RKeyModifiers,
            };
            let code = match key.code {
                crossterm::event::KeyCode::Backspace => RKeyCode::Backspace,
                crossterm::event::KeyCode::Enter => RKeyCode::Enter,
                crossterm::event::KeyCode::Left => RKeyCode::Left,
                crossterm::event::KeyCode::Right => RKeyCode::Right,
                crossterm::event::KeyCode::Up => RKeyCode::Up,
                crossterm::event::KeyCode::Down => RKeyCode::Down,
                crossterm::event::KeyCode::Home => RKeyCode::Home,
                crossterm::event::KeyCode::End => RKeyCode::End,
                crossterm::event::KeyCode::PageUp => RKeyCode::PageUp,
                crossterm::event::KeyCode::PageDown => RKeyCode::PageDown,
                crossterm::event::KeyCode::Tab => RKeyCode::Tab,
                crossterm::event::KeyCode::BackTab => RKeyCode::BackTab,
                crossterm::event::KeyCode::Delete => RKeyCode::Delete,
                crossterm::event::KeyCode::Insert => RKeyCode::Insert,
                crossterm::event::KeyCode::F(n) => RKeyCode::F(n),
                crossterm::event::KeyCode::Char(c) => RKeyCode::Char(c),
                crossterm::event::KeyCode::Null => RKeyCode::Null,
                crossterm::event::KeyCode::Esc => RKeyCode::Esc,
                // New variants, map to Null
                crossterm::event::KeyCode::CapsLock => RKeyCode::Null,
                crossterm::event::KeyCode::ScrollLock => RKeyCode::Null,
                crossterm::event::KeyCode::NumLock => RKeyCode::Null,
                crossterm::event::KeyCode::PrintScreen => RKeyCode::Null,
                crossterm::event::KeyCode::Pause => RKeyCode::Null,
                crossterm::event::KeyCode::Menu => RKeyCode::Null,
                crossterm::event::KeyCode::KeypadBegin => RKeyCode::Null,
                crossterm::event::KeyCode::Media(_) => RKeyCode::Null,
                crossterm::event::KeyCode::Modifier(_) => RKeyCode::Null,
            };
            let modifiers =
                RKeyModifiers::from_bits(key.modifiers.bits()).unwrap_or(RKeyModifiers::empty());
            let kind = match key.kind {
                crossterm::event::KeyEventKind::Press => RKeyEventKind::Press,
                crossterm::event::KeyEventKind::Repeat => RKeyEventKind::Repeat,
                crossterm::event::KeyEventKind::Release => RKeyEventKind::Release,
            };
            let state = match key.state {
                crossterm::event::KeyEventState::NONE => RKeyEventState::NONE,
                crossterm::event::KeyEventState::CAPS_LOCK => RKeyEventState::NONE,
                crossterm::event::KeyEventState::NUM_LOCK => RKeyEventState::NONE,
                // Map unknown/unsupported states to NONE
                _ => RKeyEventState::NONE,
            };
            let ratatui_key = RKeyEvent {
                code,
                modifiers,
                kind,
                state,
            };
            self.textarea.input(ratatui_key);
            // info!("Received: {:?}", action);
        }
        None
    }

    fn render(&mut self, f: &mut Frame<'_>, rect: Rect) {
        // self.textarea.widget;
        // let tw = ;
        // f.render_widget::<dyn Widget>(tw, rect);
        // let w = ;
        /*let chunks = Layout::default()
          .direction(Direction::Vertical)
          .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
          .split(rect);
        self.filter_screen.render(f, chunks[1]);
        chunks[0]
        */
        /*
        let a: Box<dyn Any> = Box::new(w);
        let _: &tui_textarea::widget::Renderer = match a.downcast_ref::<Renderer>() {
            Some(b) => b,
            None => panic!("&a isn't a B!")
        };
        */
        // f.render_widget(self.textarea.widget(), rect);
        f.render_widget(&self.textarea, rect);
        // f.render_widget(self.textarea.widget(), rect);
        // f.render_stateful_widget(l, rect, &mut self.state);
    }
}
