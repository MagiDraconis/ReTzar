//! One finger as the left mouse button.
//!
//! The front end reads the mouse only: `Window::cursor_position`, the
//! `WindowEvent` messages and `ButtonInput<MouseButton>`. This plugin turns the
//! first finger into the same three, so the menus work by touch. Orders,
//! camera panning and zooming need real gestures and come later.
use bevy::{
    input::{
        ButtonState, InputSystems,
        mouse::MouseButtonInput,
        touch::{TouchInput, TouchPhase},
    },
    prelude::*,
    window::{CursorMoved, PrimaryWindow, WindowEvent},
};

pub struct TouchAsMousePlugin;

impl Plugin for TouchAsMousePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Finger>()
            .add_systems(PreUpdate, touch_as_mouse.before(InputSystems));
    }
}

/// The finger that acts as the mouse until it lifts.
#[derive(Resource, Default)]
struct Finger(Option<u64>);

fn touch_as_mouse(
    mut touches: MessageReader<TouchInput>,
    mut finger: ResMut<Finger>,
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    mut moved: MessageWriter<CursorMoved>,
    mut buttons: MessageWriter<MouseButtonInput>,
    mut events: MessageWriter<WindowEvent>,
) {
    for touch in touches.read() {
        let Ok(mut window) = windows.get_mut(touch.window) else {
            continue;
        };
        let mine = finger.0 == Some(touch.id);
        match touch.phase {
            TouchPhase::Started if finger.0.is_none() => {
                finger.0 = Some(touch.id);
                point(touch, &mut window, &mut moved, &mut events);
                press(touch, ButtonState::Pressed, &mut buttons, &mut events);
            }
            TouchPhase::Moved if mine => point(touch, &mut window, &mut moved, &mut events),
            TouchPhase::Ended | TouchPhase::Canceled if mine => {
                point(touch, &mut window, &mut moved, &mut events);
                press(touch, ButtonState::Released, &mut buttons, &mut events);
                finger.0 = None;
            }
            _ => {}
        }
    }
}

fn point(
    touch: &TouchInput,
    window: &mut Window,
    moved: &mut MessageWriter<CursorMoved>,
    events: &mut MessageWriter<WindowEvent>,
) {
    let delta = window.cursor_position().map(|old| touch.position - old);
    window.set_cursor_position(Some(touch.position));
    let event = CursorMoved {
        window: touch.window,
        position: touch.position,
        delta,
    };
    events.write(WindowEvent::from(event.clone()));
    moved.write(event);
}

fn press(
    touch: &TouchInput,
    state: ButtonState,
    buttons: &mut MessageWriter<MouseButtonInput>,
    events: &mut MessageWriter<WindowEvent>,
) {
    let event = MouseButtonInput {
        button: MouseButton::Left,
        state,
        window: touch.window,
    };
    events.write(WindowEvent::from(event.clone()));
    buttons.write(event);
}
