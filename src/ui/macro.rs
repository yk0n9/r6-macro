use crate::ui::font::set_style;
use crate::ui::{is_enabled, is_pressed, mouse, Aim, State};
use crate::weapon::{ToStr, Weapon};
use clap::ValueEnum;
use eframe::egui::Context;
use eframe::egui::{CentralPanel, ComboBox, DragValue, TextEdit};
use eframe::CreationContext;
use eframe::Frame;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::fs::write;
use windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY;

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
    pub map: HashMap<Weapon, (i32, i32)>,
    pub favorites: BTreeMap<String, (Weapon, (i32, i32))>,
}

pub struct Macro {
    pub hand: Hand,
    pub hand_state: HandState,
    pub weapon: Weapon,
    pub horizontal: i32,
    pub vertical: i32,
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
            horizontal: 0,
            vertical: 1,
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
                        if ui
                            .selectable_value(&mut self.weapon.name, *name, name.to_str())
                            .changed()
                        {
                            if let Some(value) = self.config.map.get(&self.weapon) {
                                self.horizontal = value.0;
                                self.vertical = value.1;
                            }
                        }
                    }
                });
            ComboBox::from_label("瞄准镜")
                .selected_text(self.weapon.sight.to_str())
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    for sight in crate::weapon::Sight::value_variants() {
                        if ui
                            .selectable_value(&mut self.weapon.sight, *sight, sight.to_str())
                            .changed()
                        {
                            if let Some(value) = self.config.map.get(&self.weapon) {
                                self.horizontal = value.0;
                                self.vertical = value.1;
                            }
                        }
                    }
                });
            ComboBox::from_label("枪管")
                .selected_text(self.weapon.barrel.to_str())
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    for barrel in crate::weapon::Barrel::value_variants() {
                        if ui
                            .selectable_value(&mut self.weapon.barrel, *barrel, barrel.to_str())
                            .changed()
                        {
                            if let Some(value) = self.config.map.get(&self.weapon) {
                                self.horizontal = value.0;
                                self.vertical = value.1;
                            }
                        }
                    }
                });
            ComboBox::from_label("握把")
                .selected_text(self.weapon.grip.to_str())
                .show_ui(ui, |ui| {
                    ui.style_mut().wrap = Some(false);
                    for grip in crate::weapon::Grip::value_variants() {
                        if ui
                            .selectable_value(&mut self.weapon.grip, *grip, grip.to_str())
                            .changed()
                        {
                            if let Some(value) = self.config.map.get(&self.weapon) {
                                self.horizontal = value.0;
                                self.vertical = value.1;
                            }
                        }
                    }
                });
            ui.horizontal(|ui| {
                let horizontal = ui.add(DragValue::new(&mut self.horizontal));
                let left = is_pressed(VIRTUAL_KEY(219));
                if !left {
                    self.state.left = false
                }
                if left != self.state.left {
                    self.state.left = left;
                    self.horizontal -= 1;
                }
                let right = is_pressed(VIRTUAL_KEY(221));
                if !right {
                    self.state.right = false
                }
                if right != self.state.right {
                    self.state.right = right;
                    self.horizontal += 1;
                }
                if horizontal.changed() {
                    self.config
                        .map
                        .insert(self.weapon, (self.horizontal, self.vertical));
                }
                ui.label("水平方向强度");
            });
            ui.horizontal(|ui| {
                let vertical = ui.add(DragValue::new(&mut self.vertical));
                let up = is_pressed(VIRTUAL_KEY(189)) || is_pressed(VIRTUAL_KEY(109));
                if !up {
                    self.state.up = false
                }
                if up != self.state.up {
                    self.state.up = up;
                    self.vertical -= 1;
                }
                let down = is_pressed(VIRTUAL_KEY(187)) || is_pressed(VIRTUAL_KEY(107));
                if !down {
                    self.state.down = false
                }
                if down != self.state.down {
                    self.state.down = down;
                    self.vertical += 1;
                }
                if vertical.changed() {
                    self.config
                        .map
                        .insert(self.weapon, (self.horizontal, self.vertical));
                }
                ui.label("垂直方向强度");
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label("名称");
                ui.add(TextEdit::singleline(&mut self.editing_name).desired_width(100.0));
                if ui
                    .button(if self.current_name.eq(&self.editing_name) {
                        "覆盖收藏"
                    } else {
                        "添加收藏"
                    })
                    .clicked()
                {
                    if !self.editing_name.is_empty() {
                        if let None = self.config.favorites.values().find(|value| {
                            value.eq(&&(self.weapon, (self.horizontal, self.vertical)))
                        }) {
                            self.current_name = self.editing_name.clone();
                            self.config.favorites.insert(
                                self.editing_name.clone(),
                                (self.weapon, (self.horizontal, self.vertical)),
                            );
                        }
                    }
                }
            });
            ui.horizontal(|ui| {
                ComboBox::from_label("收藏列表")
                    .selected_text(self.current_name.as_str())
                    .show_ui(ui, |ui| {
                        for favorite in self.config.favorites.iter() {
                            if ui
                                .selectable_value(
                                    &mut self.weapon,
                                    favorite.1 .0,
                                    favorite.0.as_str(),
                                )
                                .changed()
                            {
                                self.current_name = favorite.0.to_string();
                                self.editing_name = favorite.0.to_string();
                                self.horizontal = favorite.1 .1 .0;
                                self.vertical = favorite.1 .1 .1;
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
            ui.label("[ / ] 调整水平方向值");
            ui.label("+ / - 调整垂直方向值");
            ui.label("1 / 2 启用/禁用宏");
            ui.label("先按住右键再按住左键开始下压");
            ui.label("请把常用的配置添加到收藏");
        });

        let main = is_pressed(VIRTUAL_KEY(49));
        if !main {
            self.hand_state.main = false;
        }
        if main != self.hand_state.main {
            self.hand_state.main = main;
            self.hand = Hand::Main;
        }

        let deputy = is_pressed(VIRTUAL_KEY(50));
        if !deputy {
            self.hand_state.deputy = false;
        }
        if deputy != self.hand_state.deputy {
            self.hand_state.deputy = deputy;
            self.hand = Hand::Deputy;
        }

        if is_enabled() {
            if let Hand::Main = self.hand {
                mouse(self.horizontal, self.vertical, &mut self.aim);
            }
        }
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        let config = ron::to_string(&self.config).unwrap();
        write("r6-config.txt", &config).ok();
    }
}
