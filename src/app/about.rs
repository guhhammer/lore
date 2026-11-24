//! The about screen (`+`) and what opens over it: the settings, the palette
//! picker and the choice pickers (Pages in, Favourite).

use std::collections::HashMap;

use crate::configs::FAST_SCROLL_CHOICES;
use crate::interface::palette::Palette;

use super::{App, Choice, Mode, Picker, Setting};

impl App {
    /// Opens the about screen, from the list or a page.
    pub fn open_about(&mut self) {
        if matches!(self.mode, Mode::Browse | Mode::Info) {
            self.about_from = self.mode;
            self.mode = Mode::About;
            self.about_scroll = 0; // from the top
            self.about_follow = false;
        }
    }

    /// True when the about screen is on show: open, or under the picker
    /// opened from it.
    pub fn about_shown(&self) -> bool {
        matches!(self.mode, Mode::About | Mode::Choose) || self.picker_from(Mode::About)
    }

    /// The palette picker is open and was opened from `mode`.
    pub(super) fn picker_from(&self, mode: Mode) -> bool {
        self.mode == Mode::Palette && self.picker.as_ref().is_some_and(|p| p.from == mode)
    }

    /// Chooses the setting `steps` away on the about screen, wrapping round.
    pub fn step_setting(&mut self, steps: isize) {
        self.reset_armed = false; // moving away from Reset all keeps everything
        self.about_follow = true;
        let count = Setting::all().len();
        let forward = steps.rem_euclid(isize::try_from(count).unwrap_or(isize::MAX));
        self.setting = (self.setting + usize::try_from(forward).unwrap_or(0)) % count;
    }

    /// The setting chosen on the about screen.
    pub fn chosen_setting(&self) -> Setting {
        Setting::all()[self.setting % Setting::all().len()]
    }

    /// Turns reading the shell's history on (with what it holds) or off;
    /// the groups and marks follow at once.
    pub fn set_history(&mut self, on: bool, uses: HashMap<String, u32>) {
        self.history = on;
        self.uses = if on { uses } else { HashMap::new() };
        self.rebuild_keeping_selection();
    }

    /// Back from the about screen to where it was opened.
    pub fn close_about(&mut self) {
        self.reset_armed = false;
        if self.mode == Mode::About {
            self.mode = self.about_from;
        }
    }

    /// Opens the palette picker over the list, the page or the about screen.
    pub fn open_palettes(&mut self) {
        if matches!(self.mode, Mode::Browse | Mode::Info | Mode::About) {
            self.picker = Some(Picker {
                from: self.mode,
                before: self.chosen,
                note: None,
            });
            self.mode = Mode::Palette;
        }
    }

    /// The palette to draw with.
    pub fn palette(&self) -> &Palette {
        &self.palettes[self.chosen.min(self.palettes.len().saturating_sub(1))]
    }

    /// Tries the palette `steps` away in `palettes`, wrapping round.
    pub fn step_palette(&mut self, steps: isize) {
        let count = self.palettes.len().max(1);
        // a step back is a step of count - 1 forward
        let forward = steps.rem_euclid(isize::try_from(count).unwrap_or(isize::MAX));
        let forward = usize::try_from(forward).unwrap_or(0);
        self.chosen = (self.chosen + forward) % count;
        if let Some(picker) = &mut self.picker {
            picker.note = None;
        }
    }

    /// Closes the picker, keeping the palette tried (lib.rs has saved it).
    pub fn close_palettes(&mut self) {
        if let Some(picker) = self.picker.take() {
            self.mode = picker.from;
        }
    }

    /// Closes the picker and puts back the palette from before.
    pub fn cancel_palettes(&mut self) {
        if let Some(picker) = self.picker.take() {
            self.chosen = picker.before;
            self.mode = picker.from;
        }
    }

    /// The palette could not be saved: the picker stays open and says why.
    pub fn palette_failed(&mut self, why: &str) {
        if let Some(picker) = &mut self.picker {
            picker.note = Some(why.to_owned());
        }
    }

    /// Opens the picker for `setting` over the about screen, `items` its
    /// choices, the one named `current` chosen (else the first).
    pub fn open_choice(&mut self, setting: Setting, items: Vec<String>, current: &str) {
        let at = items.iter().position(|i| i == current).unwrap_or(0);
        self.choice = Some(Choice { setting, items, at });
        self.mode = Mode::Choose;
    }

    /// The choice `steps` away, stopping at either end.
    pub fn step_choice(&mut self, steps: isize) {
        if let Some(choice) = &mut self.choice {
            let last = choice.items.len().saturating_sub(1);
            choice.at = choice.at.saturating_add_signed(steps).min(last);
        }
    }

    /// The next choice starting with `letter` (any case), round after the
    /// last; none: the choice stays.
    pub fn jump_choice(&mut self, letter: char) {
        if let Some(choice) = &mut self.choice {
            let count = choice.items.len();
            let starts = |i: usize| {
                choice.items[i]
                    .chars()
                    .next()
                    .is_some_and(|c| c.to_lowercase().eq(letter.to_lowercase()))
            };
            if let Some(i) = (1..=count)
                .map(|n| (choice.at + n) % count)
                .find(|&i| starts(i))
            {
                choice.at = i;
            }
        }
    }

    /// Closes the picker, back on the about screen; the setting and the
    /// item chosen when `keep`.
    pub fn close_choice(&mut self, keep: bool) -> Option<(Setting, String)> {
        self.mode = Mode::About;
        let choice = self.choice.take()?;
        let item = choice.items.get(choice.at).cloned()?;
        keep.then_some((choice.setting, item))
    }

    /// Sets the favourite language (None: none), shown first in its topic.
    pub fn set_favourite(&mut self, name: Option<String>) {
        self.favourite = name;
        self.refilter();
    }

    /// The fast scroll's next choice, round after the last (Enter on the
    /// about screen); one not among them goes back to the first.
    pub fn next_fast_scroll(&mut self) {
        let at = FAST_SCROLL_CHOICES
            .iter()
            .position(|&n| n == self.fast_scroll);
        self.fast_scroll = match at {
            Some(i) => FAST_SCROLL_CHOICES[(i + 1) % FAST_SCROLL_CHOICES.len()],
            None => FAST_SCROLL_CHOICES[0],
        };
    }
}
