// Copyright (C) 2026  Antonio-Miguel Corbi Bellot
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

// -- Uses: ---------------------------------------------------------------
use egui::{RichText, Style};
//use delegate::delegate;
use egui::{
    emath::{self, RectTransform},
    epaint::Hsva,
    pos2, Color32, CornerRadius, Frame, PointerButton, Pos2, Rect, Sense, Stroke, Ui, Vec2,
};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }

    fn create_drawing_widget(&mut self, ui: &mut Ui) {
        const CANVAS_W: f32 = 800.0;
        const CANVAS_H: f32 = 600.0;
        let height_for_widgets = 40.0; // Espacio que necesitas abajo
        let scroll_height = ui.available_height() - height_for_widgets;

        egui::ScrollArea::both() // Habilita scroll horizontal y vertical
            .auto_shrink([false; 2]) // Evita que el área se colapse si hay poco contenido
            .max_height(scroll_height) // Limitamos la altura del scroll
            .show(ui, |ui| {
                // 1. Definimos el tamaño total de nuestro "papel" o lienzo
                let canvas_size = egui::vec2(CANVAS_W, CANVAS_H);
                Frame::canvas(ui.style())
                    // .corner_radius(5.0)
                    .fill(Color32::from_rgb(20, 60, 100)) // Fondo azul
                    // .stroke(Stroke::new(1.5, Color32::LIGHT_RED)) // Borde negro
                    .show(ui, |ui| {
                        // self.ui_canvas(ui); // Llamamos a la lógica de dibujo

                        let (response, painter) = ui.allocate_painter(
                            // ui.available_size() - [2.0, 45.0].into(),
                            canvas_size,
                            Sense::drag() | Sense::click(), // ¡Importante! Detectar arrastre (clic + movimiento)
                                                            //Sense::DRAG | Sense::CLICK,
                        );

                        // If there are no worlds object defined,
                        // define it, else update screen_rect
                        //self.init_worlds(response.rect);

                        // Draw the GRID
                        // if self.grid {
                        //     self.draw_grid(&painter, response.rect);
                        // }

                        // 1. Comprobamos el click izquierdo
                        if response.secondary_clicked() {
                            //println!("¡Click derecho detectado en el Painter!");
                            if let Some(pos) = response.interact_pointer_pos() {
                                // println!("Añadir Repeller!: Click en la posición: {:?}", pos);
                                // let wpos = self.worlds.as_ref().unwrap().pos2_to_world(pos);
                                // let wx = wpos.x;
                                // let wy = wpos.y;
                                // let nrepellers_old = self.repellers.len();
                                // self.remove_repellers_at_point(wx, wy);
                                // let nrepellers_new = self.repellers.len();
                                // // dbg!(nrepellers_old, nrepellers_new);
                                // if nrepellers_new == nrepellers_old {
                                //     // No repeller under (wx,wy) removed so create a new one
                                //     let repeller = Repeller::new(
                                //         wx,
                                //         wy,
                                //         constants::REP_POWER,
                                //         constants::REP_SIZE,
                                //     );
                                //     self.repellers.push(repeller);
                                // }
                            }
                        }

                        // 2. Comprobamos el click intermedio
                        if response.middle_clicked() {
                            //println!("¡Click central detectado en el Painter!");
                            if let Some(pos) = response.interact_pointer_pos() {
                                println!("Click en la posición: {:?}", pos);
                            }
                            //self.show_data();
                        }

                        // 3. Comprobamos el click izquierdo
                        if response.clicked() {
                            //println!("¡Click izquierdo detectado en el Painter!");
                            if let Some(pos) = response.interact_pointer_pos() {
                                // let wpos = self.worlds.as_ref().unwrap().pos2_to_world(pos);
                                // let wx = wpos.x;
                                // let wy = wpos.y;

                                // self.set_status_text(
                                //     &format!(
                                //         //"sx: {} , sy: {} , wx: {} , wy: {}",
                                //         "particle system @ [{:.2},{:.2}]",
                                //         wx, wy
                                //     ),
                                //     egui::Color32::LIGHT_RED,
                                // );

                                // let wr = Rect::from_min_max(
                                //     constants::WR_MIN.into(),
                                //     constants::WR_MAX.into(),
                                //);
                                // let ps = ParticleSystem::new(
                                //     constants::NPARTICLES,
                                //     wx,
                                //     wy,
                                //     wr,
                                //     self.particle_size,
                                //     self.particle_mass,
                                // );
                                // self.psystems.push(ps);
                            }
                        }

                        if response.dragged_by(PointerButton::Primary) {
                            // Obtenemos la posición actual del puntero
                            if let Some(pos) = response.interact_pointer_pos() {
                                // let wpos = self.worlds.as_ref().unwrap().pos2_to_world(pos);
                                // let wx = wpos.x;
                                // let wy = wpos.y;
                                let ctx = ui.ctx();
                                ctx.send_viewport_cmd(egui::ViewportCommand::CursorVisible(false));

                                // Dibujamos un círculo donde esté el ratón mientras arrastramos
                                //  painter.circle_filled(pos, 2.0, Color32::LIGHT_RED);

                                // También puedes obtener cuánto se ha movido desde el frame anterior
                                // let delta = response.drag_delta();
                                // println!("Moviendo: {:?}", delta);

                                // if self.psystems.len() == 0 {
                                //     // First psystem, create it
                                //     let wr = Rect::from_min_max(
                                //         constants::WR_MIN.into(),
                                //         constants::WR_MAX.into(),
                                //     );
                                //     let ps = ParticleSystem::new(
                                //         constants::NPARTICLES,
                                //         wx,
                                //         wy,
                                //         wr,
                                //         self.particle_size,
                                //         self.particle_mass,
                                //     );
                                //     self.psystems.push(ps);
                                // } else {
                                //     // Add new particle to last ParticleSystem
                                //     let p = Particle::new(wx, wy, self.particle_size);
                                //     //dbg!(p);
                                //     let ps = self.psystems.last_mut().unwrap();
                                //     ps.add_particle(&p);
                                // }
                            }
                        }

                        if response.drag_stopped_by(PointerButton::Primary) {
                            let ctx = ui.ctx();
                            // Útil si el usuario estaba arrastrando y soltó el botón
                            ctx.send_viewport_cmd(egui::ViewportCommand::CursorVisible(true));
                        }

                        // // Update particles status
                        // self.run();
                        //
                        // // Draw repeller
                        // for r in &self.repellers {
                        //     self.draw_repeller(r, &painter);
                        // }

                        //if self.repeller.is_some() {
                        //    self.draw_repeller(self.repeller.as_ref().unwrap(), &painter);
                        //}

                        // Draw particles
                        // println!("N-psystems: {}", self.psystems.len());
                        // for ps in &self.psystems {
                        //     //println!("Drawing psystem");
                        //     self.draw_particle_system(ps, &painter);
                        // }
                    });
            });

        // let (response, painter) = ui.allocate_painter(
        //     Vec2::new(ui.available_width(), ui.available_height() - 50.0),
        //     Sense::DRAG | Sense::CLICK,
        // );
    }
}

impl eframe::App for TemplateApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::Panel::top("top_panel").show_inside(ui, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::MenuBar::new().ui(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ui.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("eframe template");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.label);
            });

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }

            ui.separator();

            // ui.add(egui::github_link_file!(
            //     "https://github.com/emilk/eframe_template/blob/main/",
            //     "Source code."
            // ));

            self.create_drawing_widget(ui);

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
