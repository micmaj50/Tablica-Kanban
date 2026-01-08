use eframe::egui;

struct ToDo {
    tresc_zadania: String,
    lista_zadan: Vec<String>,
}

impl Default for ToDo {
    fn default() -> Self {
        Self {
            tresc_zadania: String::new(),
            lista_zadan: Vec::new(),
        }
    }
}

impl eframe::App for ToDo {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Aplikacja To-Do");
            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Dodaj zadanie: ");
                let input = ui.text_edit_singleline(&mut self.tresc_zadania);

                if ui.add(egui::Button::new("Dodaj").min_size(egui::Vec2 { x: 80.0, y: 40.0 })).clicked()
                || (input.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter))) {
                    if !self.tresc_zadania.is_empty() {
                        self.lista_zadan.push(self.tresc_zadania.clone());
                        self.tresc_zadania.clear();
                    }
                }
            });

            let mut index_do_usuniecia: usize = 42;

            egui::ScrollArea::vertical().show(ui, |ui| {
                for (index, zadanie) in self.lista_zadan.iter().enumerate() {
                    ui.label(format!("{}. {}", index, zadanie));

                    if ui.add(egui::Button::new("Usuń").min_size(egui::Vec2 { x: 80.0, y: 40.0 })).clicked() {
                        index_do_usuniecia = index;
                    }
                }
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "ToDo",
        options,
        Box::new(|_cc| Ok(Box::new(ToDo::default()))),
    )
}