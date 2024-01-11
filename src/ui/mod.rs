use std::mem::{size_of, zeroed};
use std::thread;
use std::time::Duration;

use windows::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, GetKeyState, INPUT, INPUT_0, INPUT_MOUSE, MOUSEEVENTF_MOVE, MOUSEINPUT, SendInput, VIRTUAL_KEY, VK_CAPITAL, VK_LBUTTON, VK_RBUTTON};

use crate::ui::r#macro::Hand;

pub mod r#macro;
pub mod font;

static mut HAND: Hand = Hand::Main;
static mut AIM: Aim = Aim::None;
static mut LEVEL: i32 = 1;
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
    add: bool,
    sub: bool,
}

#[inline]
pub unsafe fn mouse() {
    loop {
        if !is_enabled() {
            continue;
        }
        if let Hand::Deputy = HAND {
            continue;
        }
        if !is_pressed(VK_RBUTTON) && !is_pressed(VK_LBUTTON) {
            AIM = Aim::None;
        }
        match AIM {
            Aim::Right => {
                if is_pressed(VK_RBUTTON) && is_pressed(VK_LBUTTON) {
                    mouse_down(LEVEL);
                }
            }
            Aim::None => {
                if is_pressed(VK_LBUTTON) {
                    AIM = Aim::Left;
                } else if is_pressed(VK_RBUTTON) {
                    AIM = Aim::Right
                }
            }
            _ => {}
        }
    }
}