use std::default;

use eframe::{egui::{self, Color32, Stroke, Vec2}, epaint::Shadow};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Status{
    Fazer,
    Fazendo,
    Concluido
}

#[derive(Debug, Clone)]
struct Tarefa{
    id: usize,
    titulo: String,
    status: Status,
    selecionado: bool,
    comentarios: Vec<String>,
    descricoes: String,
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
                    status: Status::Fazer,
                    selecionado: false,
                    comentarios: Vec::new(),
                    descricoes: String::new()
                },
                Tarefa {
                    id: 2,
                    titulo: "Desenhar as 3 colunas do quadro".to_string(),
                    status: Status::Fazer,
                    selecionado: false,
                    comentarios: Vec::new(),
                    descricoes: String::new()
                },
                Tarefa {
                    id: 3,
                    titulo: "Criar a lógica de mover os cards".to_string(),
                    status: Status::Fazer,
                    selecionado: false,
                    comentarios: Vec::new(),
                    descricoes: String::new()
                },
                Tarefa {
                    id: 4,
                    titulo: "Polimentos".to_string(),
                    status: Status::Fazer,
                    selecionado: false,
                    comentarios: Vec::new(),
                    descricoes: String::new()
                },
            ], input_nova_tarefa: String::new(), proximo_id: 5 }
    }
}

impl eframe::App for MyApp{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {

        let frame_topo = egui::Frame::default()
            .fill(Color32::from_rgb(35, 45, 65))
            .inner_margin(10.0)
            .outer_margin(5.0);

        egui::TopBottomPanel::top("frame_identificacao_window").frame(frame_topo).show(ctx, |top_frame|{
            top_frame.heading("Rust + EGUI Development");
        });

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
                        status: Status::Fazer,
                        selecionado: false,
                        comentarios: Vec::new(),
                        descricoes: String::new()
                    });
                self.proximo_id += 1;
                self.input_nova_tarefa = String::new();
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui|{

            let frame_cards = egui::Frame::default()
            .fill(Color32::from_rgb(30, 35, 45))
            .rounding(8.0)
            .stroke(Stroke::new(1.0, Color32::from_rgb(50, 55, 65)))
            .inner_margin(12.0)
            .shadow(Shadow 
            { 
                offset: Vec2::new(0.0, 4.0), 
                blur: 10.0, 
                spread: 0.0, 
                color: Color32::from_black_alpha(120) 
            })
            .outer_margin(6.0);

            egui::ScrollArea::both().hscroll(false).id_source("scroll_topo").show(ui, |scroll|{
                scroll.columns(3, |col|{

                    col[0].vertical_centered(|col0|{
                        col0.heading(egui::RichText::new("FAZER").color(Color32::RED).strong());
                        
                        for i in self.tarefas.iter_mut(){
                            if i.status == Status::Fazer{
                                frame_cards
                                .show(col0, |card0|{
                                    if card0.selectable_label(i.selecionado,&i.titulo).clicked() {
                                        if !i.selecionado {
                                            i.selecionado = true;
                                        }
                                    }
                                });
                            }
                        }
                    });

                    col[1].vertical_centered(|col1|{
                        col1.heading(egui::RichText::new("FAZENDO").color(Color32::DARK_GREEN).strong());
                        for i in self.tarefas.iter_mut(){
                            if i.status == Status::Fazendo{
                                frame_cards.show(col1, |card1|{
                                    if card1.selectable_label(i.selecionado,&i.titulo).clicked() {
                                        if !i.selecionado {
                                            i.selecionado = true;
                                        }
                                    }
                                });
                            }
                        }
                    });

                    col[2].vertical_centered(|col2|{
                        col2.heading(egui::RichText::new("FEITO").color(Color32::BLUE).strong());
                        for i in self.tarefas.iter_mut(){
                            if i.status == Status::Concluido{
                                frame_cards.show(col2, |card2|{
                                    if card2.selectable_label(i.selecionado,&i.titulo).clicked() {
                                        if !i.selecionado {
                                            i.selecionado = true;
                                        }
                                    }
                                });
                            }
                        }
                    });
                })
            });

            if let Some(x) = self.tarefas.iter_mut().find(|f| f.selecionado == true){
                egui::Window::new("Card").open(&mut x.selecionado).default_width(450.0).show(ctx, |fr|{

                    fr.horizontal(|horizontal_card|{
                            horizontal_card.label(egui::RichText::new("Card selecionado:").size(16.0));
                            horizontal_card.label(egui::RichText::new(&x.titulo).size(16.0));
                    });

                    fr.separator();

                    fr.text_edit_singleline(&mut x.descricoes);

                    fr.columns(2, |card_col|{

                        card_col[0].horizontal(|cl0|{
                            if cl0.button("adicionar a lista de anotações").clicked() {
                                x.comentarios.push(x.descricoes.clone());
                                x.descricoes = String::new();
                            }
                        });

                        card_col[1].horizontal(|cl1|{
                            if x.status == Status::Fazer && !x.comentarios.is_empty() {
                                if cl1.button("Alterar para Fazendo").clicked() {
                                    x.status = Status::Fazendo;
                                }
                            }

                            if x.status == Status::Fazendo {
                                if cl1.button("Alterar para Feito").clicked() {
                                    x.status = Status::Concluido;
                                }
                            }
                        });

                    });

                    fr.separator();

                    if x.comentarios.is_empty(){

                        fr.vertical_centered(|vertical_card|{
                            vertical_card.label(
                                egui::RichText::new("Não há comentários lançados para este card")
                            .color(Color32::RED)
                            .strong()
                            .size(22.0));
                        });

                    }
                    else{

                        fr.vertical_centered(|vertical_card|{
                            
                            vertical_card.horizontal(|hor_sect|{
                                hor_sect.label(egui::RichText::new("Comentarios").color(Color32::RED).size(16.0).strong());
                            });

                            for i in &x.comentarios{
                                vertical_card.label(i);
                            }

                        });

                    }

                });
            }
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