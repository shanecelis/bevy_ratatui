use std::time::Duration;

use bevy::{app::AppExit, prelude::*};
use bevy::input::keyboard::KeyboardInput;
use color_eyre::Result;
use crossterm::event::{self, Event::Key, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::layout::Size;
use bevy::window::{PrimaryWindow, WindowCreated, WindowResized};
use crate::event::{KeyEvent, InputSet};

pub struct BevyEventPlugin;

impl Plugin for BevyEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy::input::InputPlugin)
           .add_systems(Startup, setup_window)
           .add_systems(PreUpdate, send_key_events.in_set(InputSet::Post));
    }
}

#[derive(Debug, Deref, DerefMut)]
struct Modifiers(KeyModifiers);

impl Default for Modifiers {
    fn default() -> Self {
        Self(KeyModifiers::empty())
    }
}

fn send_key_events(mut keys: EventReader<KeyEvent>,
                   window: Query<Entity, With<PrimaryWindow>>,
                   mut modifiers: Local<Modifiers>,
                   mut keyboard_input: EventWriter<KeyboardInput>
) {
    let bevy_window = window.single();
    for key_event in keys.read() {
        if let Some((bevy_event, mods)) = key_event_to_bevy(&key_event, bevy_window) {
            // dbg!(mods, *modifiers);
            if mods != **modifiers {
                let delta = mods.symmetric_difference(**modifiers);
                for flag in delta {
                    let state = if mods.contains(flag) {
                        // This flag has been added.
                        bevy::input::ButtonState::Pressed
                    } else { // modifiers.contains(flag)
                        // This flag has been removed.
                        bevy::input::ButtonState::Released
                    };
                    keyboard_input.send(modifier_to_bevy(crossterm_modifier_to_bevy_key(flag), state, bevy_window));
                }
                **modifiers = mods;
            }
            keyboard_input.send(bevy_event);
        }
    }

}

/// This is a dummy window to satisfy the [KeyboardInput] struct.
fn setup_window(mut commands: Commands,
                // mut window_created: EventWriter<WindowCreated>
) {
    // Insert our window entity so that other parts of our app can use them
    let bevy_window = commands.spawn(PrimaryWindow).id();

    // Publish to the app that a terminal window has been created
    // window_created.send(WindowCreated {
    //     window: bevy_window,
    // });
}


fn modifier_to_bevy(modifier: bevy::input::keyboard::Key, state: bevy::input::ButtonState, window: Entity)
                    -> bevy::input::keyboard::KeyboardInput {

    use bevy::input::keyboard::Key as k;
    use bevy::input::keyboard::KeyCode as c;
    let key_code = match modifier {
        k::Control => c::ControlLeft,
        k::Shift => c::ShiftLeft,
        k::Alt => c::AltLeft,
        k::Hyper => c::Hyper,
        k::Meta => c::Meta,
        k::Super => c::SuperLeft,
        x => panic!("No such modifier {x:?}"),
    };
    let logical_key = modifier;
    bevy::input::keyboard::KeyboardInput {
        key_code,
        state,
        window,
        logical_key,
    }
}

fn key_event_to_bevy(
    key_event: &crossterm::event::KeyEvent,
    window: Entity,
) -> Option<(
    bevy::input::keyboard::KeyboardInput,
    crossterm::event::KeyModifiers,
)> {
    let crossterm::event::KeyEvent {
        code,
        modifiers,
        kind,
        state: _state
    } = key_event;
    let state = match kind {
        crossterm::event::KeyEventKind::Press => bevy::input::ButtonState::Pressed,
        crossterm::event::KeyEventKind::Repeat => bevy::input::ButtonState::Pressed,
        crossterm::event::KeyEventKind::Release => bevy::input::ButtonState::Released,
    };
    let key_code = to_bevy_keycode(code);
    let logical_key = to_bevy_key(code);
    key_code
        .zip(logical_key)
        .map(|((key_code, mods), logical_key)| {
            (
                bevy::input::keyboard::KeyboardInput {
                    key_code,
                    state,
                    window,
                    logical_key,
                },
                *modifiers | mods,
            )
        })
}

fn to_bevy_keycode(
    key_code: &crossterm::event::KeyCode,
) -> Option<(
    bevy::input::keyboard::KeyCode,
    crossterm::event::KeyModifiers,
)> {
    use bevy::input::keyboard::KeyCode as b;
    use crossterm::event::KeyCode as c;
    use crossterm::event::KeyModifiers as m;
    let mut mods = crossterm::event::KeyModifiers::empty();
    match key_code {
        c::Backspace => Some(b::Backspace),
        c::Enter => Some(b::Enter),
        c::Left => Some(b::ArrowLeft),
        c::Right => Some(b::ArrowRight),
        c::Up => Some(b::ArrowUp),
        c::Down => Some(b::ArrowDown),
        c::Home => Some(b::Home),
        c::End => Some(b::End),
        c::PageUp => Some(b::PageUp),
        c::PageDown => Some(b::PageDown),
        c::Tab => Some(b::Tab),
        c::BackTab => None,
        c::Delete => Some(b::Delete),
        c::Insert => Some(b::Insert),
        c::F(f) => match f {
            1 => Some(b::F1),
            2 => Some(b::F2),
            3 => Some(b::F3),
            4 => Some(b::F4),
            5 => Some(b::F5),
            6 => Some(b::F6),
            7 => Some(b::F7),
            8 => Some(b::F8),
            9 => Some(b::F9),
            10 => Some(b::F10),
            11 => Some(b::F11),
            12 => Some(b::F12),
            13 => Some(b::F13),
            14 => Some(b::F14),
            15 => Some(b::F15),
            16 => Some(b::F16),
            17 => Some(b::F17),
            18 => Some(b::F18),
            19 => Some(b::F19),
            20 => Some(b::F20),
            31 => Some(b::F31),
            32 => Some(b::F32),
            33 => Some(b::F33),
            34 => Some(b::F34),
            35 => Some(b::F35),
            _ => None,
        },
        c::Char(c) => match c {
            '!' => {
                mods |= m::SHIFT;
                Some(b::Digit1)
            }
            '@' => {
                mods |= m::SHIFT;
                Some(b::Digit2)
            }
            '#' => {
                mods |= m::SHIFT;
                Some(b::Digit3)
            }
            '$' => {
                mods |= m::SHIFT;
                Some(b::Digit4)
            }
            '%' => {
                mods |= m::SHIFT;
                Some(b::Digit5)
            }
            '^' => {
                mods |= m::SHIFT;
                Some(b::Digit6)
            }
            '&' => {
                mods |= m::SHIFT;
                Some(b::Digit7)
            }
            '*' => {
                mods |= m::SHIFT;
                Some(b::Digit8)
            }
            '(' => {
                mods |= m::SHIFT;
                Some(b::Digit9)
            }
            ')' => {
                mods |= m::SHIFT;
                Some(b::Digit0)
            }
            '-' => {
                mods |= m::SHIFT;
                Some(b::Minus)
            }
            '[' => Some(b::BracketLeft),
            ']' => Some(b::BracketRight),
            '{' => {
                mods |= m::SHIFT;
                Some(b::BracketLeft)
            },
            '}' => {
                mods |= m::SHIFT;
                Some(b::BracketRight)
            },
            ',' => Some(b::Comma),
            '=' => Some(b::Equal),
            '<' => {
                mods |= m::SHIFT;
                Some(b::Comma)
            },
            '+' => {
                mods |= m::SHIFT;
                Some(b::Equal)
            },
            '.' => Some(b::Period),
            '>' => {
                mods |= m::SHIFT;
                Some(b::Period)
            },
            '\'' => Some(b::Quote),
            '"' => {
                mods |= m::SHIFT;
                Some(b::Quote)
            },
            ';' => Some(b::Semicolon),
            ':' => {
                mods |= m::SHIFT;
                Some(b::Semicolon)
            },
            '/' => Some(b::Slash),
            '?' => {
                mods |= m::SHIFT;
                Some(b::Slash)
            },
            ' ' => Some(b::Space),
            '1' => Some(b::Digit1),
            '2' => Some(b::Digit2),
            '3' => Some(b::Digit3),
            '4' => Some(b::Digit4),
            '5' => Some(b::Digit5),
            '6' => Some(b::Digit6),
            '7' => Some(b::Digit7),
            '8' => Some(b::Digit8),
            '9' => Some(b::Digit9),
            '0' => Some(b::Digit0),
            'a' => Some(b::KeyA),
            'b' => Some(b::KeyB),
            'c' => Some(b::KeyC),
            'd' => Some(b::KeyD),
            'e' => Some(b::KeyE),
            'f' => Some(b::KeyF),
            'g' => Some(b::KeyG),
            'h' => Some(b::KeyH),
            'i' => Some(b::KeyI),
            'j' => Some(b::KeyJ),
            'k' => Some(b::KeyK),
            'l' => Some(b::KeyL),
            'm' => Some(b::KeyM),
            'n' => Some(b::KeyN),
            'o' => Some(b::KeyO),
            'p' => Some(b::KeyP),
            'q' => Some(b::KeyQ),
            'r' => Some(b::KeyR),
            's' => Some(b::KeyS),
            't' => Some(b::KeyT),
            'u' => Some(b::KeyU),
            'v' => Some(b::KeyV),
            'w' => Some(b::KeyW),
            'x' => Some(b::KeyX),
            'y' => Some(b::KeyY),
            'z' => Some(b::KeyZ),
            'A' => {
                mods |= m::SHIFT;
                Some(b::KeyA)
            },
            'B' => {
                mods |= m::SHIFT;
                Some(b::KeyB)
            },
            'C' => {
                mods |= m::SHIFT;
                Some(b::KeyC)
            },
            'D' => {
                mods |= m::SHIFT;
                Some(b::KeyD)
            },
            'E' => {
                mods |= m::SHIFT;
                Some(b::KeyE)
            },
            'F' => {
                mods |= m::SHIFT;
                Some(b::KeyF)
            },
            'G' => {
                mods |= m::SHIFT;
                Some(b::KeyG)
            },
            'H' => {
                mods |= m::SHIFT;
                Some(b::KeyH)
            },
            'I' => {
                mods |= m::SHIFT;
                Some(b::KeyI)
            },
            'J' => {
                mods |= m::SHIFT;
                Some(b::KeyJ)
            },
            'K' => {
                mods |= m::SHIFT;
                Some(b::KeyK)
            },
            'L' => {
                mods |= m::SHIFT;
                Some(b::KeyL)
            },
            'M' => {
                mods |= m::SHIFT;
                Some(b::KeyM)
            },
            'N' => {
                mods |= m::SHIFT;
                Some(b::KeyN)
            },
            'O' => {
                mods |= m::SHIFT;
                Some(b::KeyO)
            },
            'P' => {
                mods |= m::SHIFT;
                Some(b::KeyP)
            },
            'Q' => {
                mods |= m::SHIFT;
                Some(b::KeyQ)
            },
            'R' => {
                mods |= m::SHIFT;
                Some(b::KeyR)
            },
            'S' => {
                mods |= m::SHIFT;
                Some(b::KeyS)
            },
            'T' => {
                mods |= m::SHIFT;
                Some(b::KeyT)
            },
            'U' => {
                mods |= m::SHIFT;
                Some(b::KeyU)
            },
            'V' => {
                mods |= m::SHIFT;
                Some(b::KeyV)
            },
            'W' => {
                mods |= m::SHIFT;
                Some(b::KeyW)
            },
            'X' => {
                mods |= m::SHIFT;
                Some(b::KeyX)
            },
            'Y' => {
                mods |= m::SHIFT;
                Some(b::KeyY)
            },
            'Z' => {
                mods |= m::SHIFT;
                Some(b::KeyZ)
            },
            _ => None,
        },
        c::Null => None,
        c::Esc => Some(b::Escape),
        c::CapsLock => Some(b::CapsLock),
        c::ScrollLock => Some(b::ScrollLock),
        c::NumLock => Some(b::NumLock),
        c::PrintScreen => Some(b::PrintScreen),
        c::Pause => Some(b::Pause),
        c::Menu => Some(b::ContextMenu),
        c::KeypadBegin => None,
        c::Media(media) => {
            use crossterm::event::MediaKeyCode::*;
            match media {
                Play => Some(b::MediaPlayPause),
                Pause => Some(b::Pause),
                PlayPause => Some(b::MediaPlayPause),
                Reverse => None,
                Stop => Some(b::MediaStop),
                FastForward => Some(b::MediaTrackNext),
                Rewind => Some(b::MediaTrackPrevious),
                TrackNext => Some(b::MediaTrackNext),
                TrackPrevious => Some(b::MediaTrackPrevious),
                Record => None,
                LowerVolume => Some(b::AudioVolumeDown),
                RaiseVolume => Some(b::AudioVolumeUp),
                MuteVolume => Some(b::AudioVolumeMute),
            }
        }
        c::Modifier(modifier) => {
            use crossterm::event::ModifierKeyCode::*;
            match modifier {
                LeftShift => Some(b::ShiftLeft),
                LeftControl => Some(b::ControlLeft),
                LeftAlt => Some(b::AltLeft),
                LeftSuper => Some(b::SuperLeft),
                LeftHyper => Some(b::Hyper),
                LeftMeta => Some(b::Meta),
                RightShift => Some(b::ShiftRight),
                RightControl => Some(b::ControlRight),
                RightAlt => Some(b::AltRight),
                RightSuper => Some(b::SuperRight),
                RightHyper => Some(b::Hyper),
                RightMeta => Some(b::Meta),
                IsoLevel3Shift => None,
                IsoLevel5Shift => None,
            }
        }
    }
    .map(|key_code| (key_code, mods))
}

fn to_bevy_key(key_code: &crossterm::event::KeyCode) -> Option<bevy::input::keyboard::Key> {
    use bevy::input::keyboard::Key as b;
    use crossterm::event::KeyCode as c;
    match key_code {
        c::Backspace => Some(b::Backspace),
        c::Enter => Some(b::Enter),
        c::Left => Some(b::ArrowLeft),
        c::Right => Some(b::ArrowRight),
        c::Up => Some(b::ArrowUp),
        c::Down => Some(b::ArrowDown),
        c::Home => Some(b::Home),
        c::End => Some(b::End),
        c::PageUp => Some(b::PageUp),
        c::PageDown => Some(b::PageDown),
        c::Tab => Some(b::Tab),
        c::BackTab => None,
        c::Delete => Some(b::Delete),
        c::Insert => Some(b::Insert),
        c::F(f) => match f {
            1 => Some(b::F1),
            2 => Some(b::F2),
            3 => Some(b::F3),
            4 => Some(b::F4),
            5 => Some(b::F5),
            6 => Some(b::F6),
            7 => Some(b::F7),
            8 => Some(b::F8),
            9 => Some(b::F9),
            10 => Some(b::F10),
            11 => Some(b::F11),
            12 => Some(b::F12),
            13 => Some(b::F13),
            14 => Some(b::F14),
            15 => Some(b::F15),
            16 => Some(b::F16),
            17 => Some(b::F17),
            18 => Some(b::F18),
            19 => Some(b::F19),
            20 => Some(b::F20),
            31 => Some(b::F31),
            32 => Some(b::F32),
            33 => Some(b::F33),
            34 => Some(b::F34),
            35 => Some(b::F35),
            _ => None,
        },
        c::Char(c) => Some({
            let mut tmp = [0u8; 4];
            let s = c.encode_utf8(&mut tmp);
            b::Character(smol_str::SmolStr::from(s))
        }),
        c::Null => None,
        c::Esc => Some(b::Escape),
        c::CapsLock => Some(b::CapsLock),
        c::ScrollLock => Some(b::ScrollLock),
        c::NumLock => Some(b::NumLock),
        c::PrintScreen => Some(b::PrintScreen),
        c::Pause => Some(b::Pause),
        c::Menu => Some(b::ContextMenu),
        c::KeypadBegin => None,
        c::Media(media) => {
            use crossterm::event::MediaKeyCode::*;
            match media {
                Play => Some(b::MediaPlay),
                Pause => Some(b::Pause),
                PlayPause => Some(b::MediaPlayPause),
                Reverse => None,
                Stop => Some(b::MediaStop),
                FastForward => Some(b::MediaFastForward),
                Rewind => Some(b::MediaRewind),
                TrackNext => Some(b::MediaTrackNext),
                TrackPrevious => Some(b::MediaTrackPrevious),
                Record => Some(b::MediaRecord),
                LowerVolume => Some(b::AudioVolumeDown),
                RaiseVolume => Some(b::AudioVolumeUp),
                MuteVolume => Some(b::AudioVolumeMute),
            }
        }
        c::Modifier(modifier) => {
            use crossterm::event::ModifierKeyCode::*;
            match modifier {
                LeftShift => Some(b::Shift),
                LeftControl => Some(b::Control),
                LeftAlt => Some(b::Alt),
                LeftSuper => Some(b::Super),
                LeftHyper => Some(b::Hyper),
                LeftMeta => Some(b::Meta),
                RightShift => Some(b::Shift),
                RightControl => Some(b::Control),
                RightAlt => Some(b::Alt),
                RightSuper => Some(b::Super),
                RightHyper => Some(b::Hyper),
                RightMeta => Some(b::Meta),
                IsoLevel3Shift => Some(b::AltGraph),
                IsoLevel5Shift => None,
            }
        }
    }
}

fn crossterm_modifier_to_bevy_key(modifier: crossterm::event::KeyModifiers) -> bevy::input::keyboard::Key {
    let mut i = modifier.into_iter();
    let modifier = i.next().expect("mod");
    use crossterm::event::KeyModifiers as c;
    use bevy::input::keyboard::Key as k;
    let result = match modifier {
        c::SHIFT => k::Shift,
        c::CONTROL => k::Control,
        c::ALT => k::Alt,
        c::SUPER => k::Super,
        c::HYPER => k::Hyper,
        c::META => k::Meta,
        x => panic!("Given a modifier of {x:?}"),
    };
    assert!(i.next() == None);
    result
}
