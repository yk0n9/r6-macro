use std::collections::{BTreeMap, HashMap};
use std::fs::write;
use clap::ValueEnum;
use eframe::egui::{CentralPanel, ComboBox, DragValue, TextEdit};
use eframe::CreationContext;
use eframe::egui::Context;
use eframe::Frame;
use serde::{Deserialize, Serialize};
use windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY;
use crate::ui::{Aim, is_enabled, is_pressed, mouse, State};
use crate::ui::font::set_style;
use crate::weapon::{ToStr, Weapon};

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
    pub map: HashMap<Weapon, i32>,
    pub favorites: BTreeMap<String, (Weapon, i32)>,
}

pub struct Macro {
    pub hand: Hand,
    pub hand_state: HandState,
    pub weapon: Weapon,
    pub level: i32,
    pub aim: Aim,
    pub state: State,
    pub config: Config,
    pub editing_name: String,
    pub current_name: String,
}

impl Macro {
    pub fn new(cc: &CreationContext) -> Self {
        set_style(cc);
        Self {
            hand: Hand::Main,
            hand_state: Default::default(),
            weapon: Default::default(),
            level: 1,
            aim: Default::default(),
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
        CentralPanel::default().show(ctx, |ui| {
            ui.label("R6 Macro");
            ui.separator();
            ComboBox::from_label("主手武器")
                .selected_text(self.weapon.name.to_str())
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    for name in crate::weapon::Name::value_variants() {
                        if ui.selectable_value(&mut self.weapon.name, *name, name.to_str()).changed() {
                            if let Some(value) = self.config.map.get(&self.weapon) {
                                self.level = *value;
                            }
                        }
                    }
                });
            ComboBox::from_label("瞄准镜")
                .selected_text(self.weapon.sight.to_str())
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    for sight in crate::weapon::Sight::value_variants() {
                        if ui.selectable_value(&mut self.weapon.sight, *sight, sight.to_str()).changed() {
                            if let Some(value) = self.config.map.get(&self.weapon) {
                                self.level = *value;
                            }
                        }
                    }
                });
            ComboBox::from_label("枪管")
                .selected_text(self.weapon.barrel.to_str())
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    for barrel in crate::weapon::Barrel::value_variants() {
                        if ui.selectable_value(&mut self.weapon.barrel, *barrel, barrel.to_str()).changed() {
                            if let Some(value) = self.config.map.get(&self.weapon) {
                                self.level = *value;
                            }
                        }
                    }
                });
            ComboBox::from_label("握把")
                .selected_text(self.weapon.grip.to_str())
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    for grip in crate::weapon::Grip::value_variants() {
                        if ui.selectable_value(&mut self.weapon.grip, *grip, grip.to_str()).changed() {
                            if let Some(value) = self.config.map.get(&self.weapon) {
                                self.level = *value;
                            }
                        }
                    }
                });
            ui.horizontal(|ui| {
                let level = ui.add(DragValue::new(&mut self.level));
                let sub = is_pressed(VIRTUAL_KEY(189)) || is_pressed(VIRTUAL_KEY(109));
                if !sub {
                    self.state.sub = false
                }
                if sub != self.state.sub && self.level > 1 {
                    self.state.sub = sub;
                    self.level -= 1;
                }
                let add = is_pressed(VIRTUAL_KEY(187)) || is_pressed(VIRTUAL_KEY(107));
                if !add {
                    self.state.add = false
                }
                if add != self.state.add {
                    self.state.add = add;
                    self.level += 1;
                }
                if level.changed() {
                    self.config.map.insert(self.weapon, self.level);
                }
                ui.label("下压值");
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("名称");
                ui.add(TextEdit::singleline(&mut self.editing_name).desired_width(100.0));
                if ui.button(if self.current_name.eq(&self.editing_name) { "覆盖收藏" } else { "添加收藏" }).clicked() {
                    if !self.editing_name.is_empty() {
                        if let None = self.config.favorites.values().find(|value| value.eq(&&(self.weapon, self.level))) {
                            self.current_name = self.editing_name.clone();
                            self.config.favorites.insert(self.editing_name.clone(), (self.weapon, self.level));
                        }
                    }
                }
            });
            ui.horizontal(|ui| {
                ComboBox::from_label("收藏列表").selected_text(self.current_name.as_str()).show_ui(ui, |ui| {
                    for favorite in self.config.favorites.iter() {
                        if ui.selectable_value(&mut self.weapon, favorite.1.0, favorite.0.as_str()).changed() {
                            self.current_name = favorite.0.to_string();
                            self.editing_name = favorite.0.to_string();
                            self.level = favorite.1.1;
                        }
                    }
                });
                if ui.button("删除当前收藏").clicked() {
                    self.config.favorites.remove(&self.current_name);
                    self.current_name = String::new();
                }
            });
            ui.separator();
            ui.label(match self.hand {
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
            self.hand = Hand::Main;
        }

        let deputy = is_pressed(VIRTUAL_KEY(50));
        if !deputy { self.hand_state.deputy = false; }
        if deputy != self.hand_state.deputy {
            self.hand_state.deputy = deputy;
            self.hand = Hand::Deputy;
        }

        if is_enabled() {
            if let Hand::Main = self.hand { mouse(self.level, &mut self.aim); }
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        let config = ron::to_string(&self.config).unwrap();
        write("r6-config.txt", &config).ok();
    }
}