use std::collections::BTreeMap;
use std::fs::write;

use eframe::CreationContext;
use eframe::egui::{CentralPanel, ComboBox, DragValue, TextEdit};
use eframe::egui::Context;
use eframe::Frame;
use serde::{Deserialize, Serialize};
use windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY;

use crate::ui::{HAND, is_pressed, LEAD_TIME, LEVEL, State};
use crate::ui::font::set_style;

#[derive(Debug, Copy, Clone, Default)]
pub struct HandState {
    pub main: bool,
    pub deputy: bool,
}

#[derive(Debug, Copy, Clone)]
pub enum Hand {
    Main,
    Deputy,
}

#[derive(Debug, Clone, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub favorites: BTreeMap<String, i32>,
    pub lead_time: u64,
}

pub struct Macro {
    pub hand_state: HandState,
    pub state: State,
    pub config: Config,
    pub editing_name: String,
    pub current_name: String,
}

impl Macro {
    pub fn new(cc: &CreationContext) -> Self {
        set_style(cc);
        Self {
            hand_state: Default::default(),
            state: Default::default(),
            config: Default::default(),
            editing_name: String::new(),
            current_name: String::new(),
        }
    }
}

impl eframe::App for Macro {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        ctx.request_repaint();
        CentralPanel::default().show(ctx, |ui| unsafe {
            ui.label("R6 Macro");
            ui.separator();

            ui.horizontal(|ui| {
                ui.add(DragValue::new(&mut LEVEL));
                let sub = is_pressed(VIRTUAL_KEY(189)) || is_pressed(VIRTUAL_KEY(109));
                if !sub {
                    self.state.sub = false
                }
                if sub != self.state.sub {
                    self.state.sub = sub;
                    LEVEL -= 1;
                }
                let add = is_pressed(VIRTUAL_KEY(187)) || is_pressed(VIRTUAL_KEY(107));
                if !add {
                    self.state.add = false
                }
                if add != self.state.add {
                    self.state.add = add;
                    LEVEL += 1;
                }
                ui.label("下压值");
            });
            ui.horizontal(|ui| {
                if ui.add(DragValue::new(&mut LEAD_TIME).suffix("毫秒")).changed() {
                    self.config.lead_time = LEAD_TIME;
                }
                ui.label("下压前置时间");
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("名称");
                ui.add(TextEdit::singleline(&mut self.editing_name).desired_width(100.0));
                if ui.button(if self.current_name.eq(&self.editing_name) { "覆盖收藏" } else { "添加收藏" }).clicked() &&
                    !self.editing_name.is_empty() {
                    self.config.favorites
                        .entry(self.editing_name.clone())
                        .and_modify(|value| *value = LEVEL)
                        .or_insert(LEVEL);
                }
            });
            ui.horizontal(|ui| {
                ComboBox::from_label("收藏列表").selected_text(self.current_name.as_str()).show_ui(ui, |ui| {
                    for favorite in self.config.favorites.iter() {
                        if ui.selectable_value(&mut self.editing_name, favorite.0.to_string(), favorite.0.as_str()).changed() {
                            self.current_name = favorite.0.to_string();
                            LEVEL = *favorite.1;
                        }
                    }
                });
                if ui.button("删除当前收藏").clicked() {
                    self.config.favorites.remove(&self.current_name);
                    self.current_name = String::new();
                }
            });
            ui.separator();
            ui.label(match HAND {
                Hand::Main => "已启用",
                Hand::Deputy => "已禁用",
            });
            ui.separator();
            ui.label("大写锁定键为全局启用");
            ui.label("+/- 调整下压值");
            ui.label("1/2 启用/禁用宏");
            ui.label("先按住右键再按住左键开始下压");
            ui.label("请把常用的配置添加到收藏");
        });

        let main = is_pressed(VIRTUAL_KEY(49));
        if !main { self.hand_state.main = false; }
        if main != self.hand_state.main {
            self.hand_state.main = main;
            unsafe { HAND = Hand::Main; }
        }

        let deputy = is_pressed(VIRTUAL_KEY(50));
        if !deputy { self.hand_state.deputy = false; }
        if deputy != self.hand_state.deputy {
            self.hand_state.deputy = deputy;
            unsafe { HAND = Hand::Deputy; }
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        let config = ron::to_string(&self.config).unwrap();
        write("r6-config.txt", config).ok();
    }
}