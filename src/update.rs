use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;

// key press events to update the UI
pub fn update(app: &mut App, key_event: KeyEvent) {
    match key_event.code {
        KeyCode::Esc | KeyCode::Char('q') => app.quit(),
        KeyCode::Char('c') | KeyCode::Char('C') => {
            if key_event.modifiers == KeyModifiers::CONTROL {
                app.quit()
            }
        },
        KeyCode::Right | KeyCode::Char('k') => {
            app.user_choose_previous();
        }
        KeyCode::Left | KeyCode::Char('j') => {
            app.user_choose_next();
        }
        _ => {},
    };
}
