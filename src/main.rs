use std::{fs::File, io::{Read, Write}};

use gtk::prelude::*;
use relm4::prelude::*;

#[derive(Debug)]
enum VibeStatus {
     Vibing,
     Gaming,
     RealTalk,
     Art,
     Custom,
}

struct AppModel {
    vibe: String,
}

#[relm4::component]
impl SimpleComponent for AppModel {
    type Input = VibeStatus;
    type Output = ();
    type Init = String;

    view! {
        gtk::Window {
            set_title: Some("VibeMeter"),
            set_default_size: (300,100),

            gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 5,
                set_margin_all: 5,

                gtk::Label {
                    #[watch]
                    set_label: &format!("Current Vibe: {}", model.vibe),
                },

                gtk::Button {
                    set_label: "Vibing",
                    connect_clicked => VibeStatus::Vibing,
                },

                gtk::Button {
                    set_label: "Gaming",
                    connect_clicked => VibeStatus::Gaming,
                },

                gtk::Button {
                    set_label: "Real Talk",
                    connect_clicked => VibeStatus::RealTalk,
                },

                gtk::Button {
                    set_label: "Art",
                    connect_clicked => VibeStatus::Art,
                },

                gtk::Button {
                    set_label: "Custom",
                    connect_clicked => VibeStatus::Custom,
                }
            }
        }
    }

    fn init(
        vibe: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = AppModel {vibe};
    
        let widgets = view_output!();
    
        ComponentParts { model, widgets}    
    }
    
    fn update(&mut self, button: Self::Input, _sender: ComponentSender<Self>) {

        match button {
            VibeStatus::Vibing =>{
                self.vibe = "Vibing".to_string()

            }
    
            VibeStatus::Gaming =>{
                self.vibe = "Gaming".to_string()
            }
    
            VibeStatus::RealTalk =>{
                self.vibe = "Real Talk".to_string()
            }
    
            VibeStatus::Art =>{
                self.vibe = "Art".to_string()
            }
    
            VibeStatus::Custom =>{
                self.vibe = "Custom".to_string()
            }
        }

        let _ = vibe_write(self.vibe.clone());
        
    }
}

fn vibe_write(vib: String) -> std::io::Result<()> {
    let mut f = File::create("Vibe.txt")?;
    write!(f, "{}", vib)?;
    Ok(())
}

fn vibe_read() -> std::io::Result<String>{
    let mut f = File::open("vibe.txt")?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    Ok(contents)
    
}



fn main() {
    let app = RelmApp::new("relm4.test.simple");
    app.run::<AppModel>(vibe_read().unwrap());
}