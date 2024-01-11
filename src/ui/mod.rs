use std::mem::{size_of, zeroed};
use std::thread;
use std::time::Duration;
use windows::Win32::UI::Input::KeyboardAndMouse::{
    GetAsyncKeyState, GetKeyState, SendInput, INPUT, INPUT_0, INPUT_MOUSE, MOUSEEVENTF_MOVE,
    MOUSEINPUT, VIRTUAL_KEY, VK_CAPITAL, VK_LBUTTON, VK_RBUTTON,
};

pub mod font;
pub mod r#macro;

static mut I: INPUT = INPUT {
    r#type: INPUT_MOUSE,
    Anonymous: INPUT_0 {
        mi: MOUSEINPUT {
            dwFlags: MOUSEEVENTF_MOVE,
            ..unsafe { zeroed() }
        },
    },
};

#[inline]
fn mouse_down(horizontal: i32, vertical: i32) {
    unsafe {
        I.Anonymous.mi.dx = horizontal;
        I.Anonymous.mi.dy = vertical;
        SendInput(&[I], size_of::<INPUT>() as i32);
    }
    thread::sleep(Duration::from_millis(10));
}

#[inline]
fn is_pressed(vk: VIRTUAL_KEY) -> bool {
    unsafe { GetAsyncKeyState(vk.0 as i32) as u32 >> 31 == 1 }
}

#[inline]
fn is_enabled() -> bool {
    unsafe { GetKeyState(VK_CAPITAL.0 as i32) & 0x0001 != 0 }
}

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub enum Aim {
    Left,
    Right,
    #[default]
    None,
}

#[derive(Debug, Copy, Clone, Default)]
pub struct State {
    down: bool,
    up: bool,
    left: bool,
    right: bool,
}

#[inline]
pub fn mouse(horizontal: i32, vertical: i32, aim: &mut Aim) {
    if !is_pressed(VK_RBUTTON) && !is_pressed(VK_LBUTTON) {
        *aim = Aim::None;
    }
    match aim {
        Aim::Right => {
            if is_pressed(VK_RBUTTON) && is_pressed(VK_LBUTTON) {
                mouse_down(horizontal, vertical);
            }
        }
        Aim::None => {
            if is_pressed(VK_LBUTTON) {
                *aim = Aim::Left;
            } else if is_pressed(VK_RBUTTON) {
                *aim = Aim::Right
            }
        }
        _ => {}
    }
}
