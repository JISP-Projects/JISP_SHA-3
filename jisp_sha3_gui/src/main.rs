use std::{thread, fmt};
use std::sync::mpsc::{self, Sender, Receiver};

use eframe::egui;
use eframe::epaint::FontId;
use jisp_sha3 as sha;
use sha::sha3::*;
use sha::printer::{print_bytes_be, print_bytes_le};

fn main() {
    let mut native_options = eframe::NativeOptions::default();
    let _ = native_options.viewport.inner_size.insert((660., 480.).into());
    let _ = eframe::run_native("SHA-3", native_options, Box::new(|cc| Box::new(ShaGUI::new(cc))))
        .expect("Unexpected Error");
}

#[derive(Clone, Copy, PartialEq)]
enum Algorithm {
    Sha3_256,
    Sha3_224,
    Sha3_512,
    Sha3_384,
    Shake128,
    Shake256,
    Shake512
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Encoding {
    BigEndian,
    LittleEndian
}

const ALG_ITER:[Algorithm;7] = [
    Algorithm::Sha3_256,
    Algorithm::Sha3_224,
    Algorithm::Sha3_512,
    Algorithm::Sha3_384,
    Algorithm::Shake128,
    Algorithm::Shake256,
    Algorithm::Shake512];

impl fmt::Display for Algorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Algorithm::Sha3_256 => write!(f, "SHA3-256"),
            Algorithm::Sha3_512 => write!(f, "SHA3-512"),
            Algorithm::Sha3_224 => write!(f, "SHA3-224"),
            Algorithm::Sha3_384 => write!(f, "SHA3-384"),
            Algorithm::Shake128 => write!(f, "SHAKE128"),
            Algorithm::Shake256 => write!(f, "SHAKE256"),
            Algorithm::Shake512 => write!(f, "SHAKE512")
        }
    }
}

enum Message {
    Hex(String),
    Hash(String)
}

#[derive(Clone, Copy)]
struct AlgInfo {
    alg:Algorithm,
    encoding:Encoding,
    digest:usize,
}


struct ShaGUI {
    input:String,
    hex:String,
    hash:String,
    thread_active:bool,
    alg_info:AlgInfo,

    tx:Sender<(AlgInfo, String)>,
    rx:Receiver<Message>

}

fn hashing_thread(tx:Sender<Message>, rx:Receiver<(AlgInfo, String)>) {
    for (a, s) in rx.iter() {
        let bytes = match a.encoding {
            Encoding::BigEndian => sha::preprocessing::be_encoding(&s),
            Encoding::LittleEndian => sha::preprocessing::le_encoding(&s)
        };
        let print = match a.encoding {
            Encoding::BigEndian => print_bytes_be,
            Encoding::LittleEndian => print_bytes_le,
        };
        tx.send(Message::Hex(print(&bytes))).unwrap();

        let hash = match a.alg {
            Algorithm::Sha3_224 => sha3_224(&bytes),
            Algorithm::Sha3_256 => sha3_256(&bytes),
            Algorithm::Sha3_384 => sha3_384(&bytes),
            Algorithm::Sha3_512 => sha3_512(&bytes),
            Algorithm::Shake128 => shake128(&bytes, a.digest),
            Algorithm::Shake256 => shake256(&bytes, a.digest),
            Algorithm::Shake512 => unofficial_sha::shake512(&bytes, a.digest)
        };
        tx.send(Message::Hash(print(&hash))).unwrap();
    }
}

impl ShaGUI {
    fn new(cc:&eframe::CreationContext<'_>) -> Self {
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        let (tx1, rx1) = mpsc::channel();
        let (tx2, rx2) = mpsc::channel::<Message>();
        thread::spawn(move|| hashing_thread(tx2, rx1));
        Self {
            tx: tx1, rx: rx2,
            thread_active:false,
            input: "".to_owned(),
            hex: "".to_owned(),
            hash: "".to_owned(),
            alg_info: AlgInfo { alg: Algorithm::Sha3_256, encoding: Encoding::BigEndian, digest: 256 },
        }
    }

    fn update_logic(&mut self) {
        if self.thread_active {
            match self.rx.try_recv() {
                Ok(Message::Hex(text)) => self.hex = text,
                Ok(Message::Hash(text)) => {
                    self.hash = text;
                    self.thread_active = false;
                },
                Err(_) => (), 
            }
        }
    }
}

impl eframe::App for ShaGUI {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.update_logic();
            ui.horizontal(|ui| {
                egui::ComboBox::new(1,"")
                    .selected_text(format!("{}", &self.alg_info.alg))
                    .show_ui(ui, |ui| {
                        for alg in ALG_ITER.iter() {
                            ui.selectable_value(&mut self.alg_info.alg, *alg, alg.to_string());
                        }
                });

                ui.label("\tEncoding:");
                egui::ComboBox::new(2,"")
                    .selected_text(format!("{:?}", &self.alg_info.encoding))
                    .show_ui(ui, |ui| {
                        for enc in [Encoding::BigEndian, Encoding::LittleEndian].iter() {
                            ui.selectable_value(&mut self.alg_info.encoding, *enc, format!("{:?}", enc));
                        }
                });

                if ALG_ITER[4..].contains(&self.alg_info.alg) {
                    ui.label("\tDigest:");
                    if ui.add(egui::DragValue::new(&mut self.alg_info.digest).clamp_range(64..=usize::MAX).speed(16)).changed() {
                        self.alg_info.digest = (self.alg_info.digest / 64)*64
                    }
                }
            });
            

            ui.group(|ui| {
                egui::Grid::new("stuff").show(ui, |ui| {
                    ui.label("[IN]:");
                    ui.add_sized((520., 20.), egui::TextEdit::singleline(&mut self.input).hint_text("Input Text..."));
                    if ui.button("Submit").clicked() && !self.thread_active {
                        let s = self.input.trim().to_owned();
                        self.tx.send((self.alg_info, s)).unwrap();
                        self.thread_active = true;
                        self.hex = "[Loading...]".to_owned();
                        self.hash = "[Loading...]".to_owned();
                    }
                    ui.end_row();
                    ui.end_row();
                    ui.label("[HEX]:"); 
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {

                        //ui.style_mut().visuals.override_text_color = Some(egui::Color32::WHITE);
                        ui.style_mut().override_font_id = Some(FontId::monospace(12.));
                        ui.add(egui::Label::new(&self.hex).selectable(true).wrap(true));
                    });

                    ui.end_row();
                    ui.end_row();
                    ui.label("[OUT]:"); 
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {

                        //ui.style_mut().visuals.override_text_color = Some(egui::Color32::WHITE);
                        ui.style_mut().override_font_id = Some(FontId::monospace(12.));
                        ui.add(egui::Label::new(&self.hash).selectable(true).wrap(true));
                    })
                    
                    
                });
                
            })
            

        });
    }
}
