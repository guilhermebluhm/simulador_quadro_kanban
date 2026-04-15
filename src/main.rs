use eframe::{egui};

#[derive(Debug)]
enum Status{
    Fazer,
    Fazendo,
    Concluido
}

#[derive(Debug)]
struct Tarefa{
    id: usize,
    titulo: String,
    status: Status
}

#[derive(Debug)]
struct MyApp{
    tarefas: Vec<Tarefa>,
    input_nova_tarefa: String,
    proximo_id: usize
}

impl Default for MyApp{
    fn default() -> Self {
        Self { tarefas: vec![
                Tarefa {
                    id: 1,
                    titulo: "Configurar o eframe e egui".to_string(),
                    status: Status::Concluido,
                },
                Tarefa {
                    id: 2,
                    titulo: "Desenhar as 3 colunas do quadro".to_string(),
                    status: Status::Fazendo,
                },
                Tarefa {
                    id: 3,
                    titulo: "Criar a lógica de mover os cards".to_string(),
                    status: Status::Fazer,
                },
                Tarefa {
                    id: 4,
                    titulo: "Polimentos".to_string(),
                    status: Status::Fazer, // Podemos ter mais de uma na mesma coluna
                },
            ], input_nova_tarefa: String::new(), proximo_id: 5 }
    }
}

impl eframe::App for MyApp{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        egui::TopBottomPanel::top("adicao_nova_tarefa").show(ctx, |top|{
            top.separator();
            top.label(egui::RichText::new("Descrição nova tarefa").strong());
            top.horizontal_centered(|ui|{
                ui.text_edit_singleline(&mut self.input_nova_tarefa);
                if ui.button("Adicionar").clicked() {
                    self.tarefas.push(Tarefa
                    {
                        id: self.proximo_id, 
                        titulo: self.input_nova_tarefa.clone(), 
                        status: Status::Fazer
                    });
                self.proximo_id += 1;
                self.input_nova_tarefa = String::new();
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui|{
            egui::ScrollArea::both().id_source("scroll_topo").show(ui, |scroll|{
                scroll.columns(3, |col|{

                    col[0].vertical_centered(|col0|{
                        col0.heading("FAZER");
                    });

                    col[1].vertical_centered(|col1|{
                        col1.heading("FAZENDO");
                    });

                    col[2].vertical_centered(|col2|{
                        col2.heading("FEITO");
                    });

                })
            })
        });
    }
}

fn main() -> eframe::Result<()> {

    let options = eframe::NativeOptions{
        viewport: egui::ViewportBuilder::default().with_inner_size([1000.0,600.0]), // w - h
        ..Default::default()
    };

    eframe::run_native("Kanbam", options, Box::new(|_cc| Box::<MyApp>::default()))

}