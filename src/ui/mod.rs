use std::mem::{size_of, zeroed};
use std::thread;
use std::time::Duration;
use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, INPUT, INPUT_0, INPUT_MOUSE, MOUSEEVENTF_MOVE, MOUSEINPUT, SendInput, VIRTUAL_KEY, VK_LBUTTON, VK_RBUTTON};

pub mod r#macro;
pub mod font;

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
fn mouse_down(val: i32) {
    unsafe {
        I.Anonymous.mi.dy = val;
        SendInput(&[I], size_of::<INPUT>() as i32);
    }
    thread::sleep(Duration::from_millis(10));
}

#[inline]
fn is_pressed(vk: VIRTUAL_KEY) -> bool {
    unsafe { GetAsyncKeyState(vk.0 as i32) as u32 >> 31 == 1 }
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
    add: bool,
    sub: bool,
}

#[inline]
pub fn mouse(level: i32, aim: &mut Aim) {
    if !is_pressed(VK_RBUTTON) && !is_pressed(VK_LBUTTON) {
        *aim = Aim::None;
    }
    match aim {
        Aim::Right => {
            if is_pressed(VK_RBUTTON) && is_pressed(VK_LBUTTON) {
                mouse_down(level);
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