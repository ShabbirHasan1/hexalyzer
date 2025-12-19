use eframe::egui;

#[derive(Default, Clone, Copy)]
pub(crate) struct EventState {
    pub(crate) last_key_released: Option<egui::Key>,
    pub(crate) last_hex_char_released: Option<char>,
    pub(crate) pointer_down: bool,
    pub(crate) pointer_hover: Option<egui::Pos2>,
    pub(crate) escape_pressed: bool,
}

/// Helper for mapping keys to hex chars
const fn key_to_hex_char(key: egui::Key) -> Option<char> {
    use egui::Key::*;
    Some(match key {
        Num0 => '0',
        Num1 => '1',
        Num2 => '2',
        Num3 => '3',
        Num4 => '4',
        Num5 => '5',
        Num6 => '6',
        Num7 => '7',
        Num8 => '8',
        Num9 => '9',
        A => 'A',
        B => 'B',
        C => 'C',
        D => 'D',
        E => 'E',
        F => 'F',
        _ => return None,
    })
}

/// Collect events once per frame and return aggregated state
pub(crate) fn collect_ui_events(ui: &egui::Ui) -> EventState {
    ui.input(|i| {
        let mut state = EventState::default();

        // Pointer state
        state.pointer_down = i.pointer.primary_down();
        state.pointer_hover = i.pointer.hover_pos();

        // Key press events (only consider key releases)
        for event in &i.events {
            if let egui::Event::Key { key, pressed: false, .. } = event {
                state.last_key_released = Some(*key);
                if let Some(ch) = key_to_hex_char(*key) {
                    state.last_hex_char_released = Some(ch);
                }
            }
        }

        // direct query for Escape pressed this frame (not release)
        state.escape_pressed = i.key_pressed(egui::Key::Escape);

        state
    })
}
