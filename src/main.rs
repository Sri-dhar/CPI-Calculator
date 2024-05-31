mod functions;
mod semdata;
use std::sync::Mutex;
use eframe::egui;
use egui::TopBottomPanel;
use egui::ViewportCommand;
use std::process::Command;
use egui::ScrollArea;

// use egui_extras;
// use egui::{Button, ViewportCommand};

extern crate lazy_static;

lazy_static::lazy_static! {
    static ref SCALE: Mutex<f32> = Mutex::new(1.5);
    static ref GRADEINPUT: Mutex<Vec<String>> = Mutex::new(Vec::new());
    static ref SHOW_SPI: Mutex<bool> = Mutex::new(false);
    static ref SHOWGRADETABLE : Mutex<bool> = Mutex::new(false);
    static ref SHOWACTIONBAR : Mutex<bool> = Mutex::new(true);
}

fn main() -> Result<(), eframe::Error> {

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([900.0, 1200.0]),
        ..Default::default()
    };
    eframe::run_native(
        "CPI/SPI Calculator",
        options,
        Box::new(|_cc| {
            Box::new(MyApp::default())
        }),
    )
}

struct MyApp {

    show_scale_window: bool,
    scale_input: String,
    option_cpi_spi: i32,
    sem_no: i32,
    sem_no_f32 : f32,
    sem_no_str : String,
    sem_option : i32,
    done_1: bool,
    sem_info : semdata::Semester,
    grades: Vec<f32>,
    grade_input: String,
    calc_cpi_option: i32,
    cpi_op1_var1: f32,
    cpi_op1_var1_str: String,
    cpi_op2_var1_str: String,
    cpi_op2_var1: f32,
    cpi_op2_var2_str: String,
    cpi_op2_var2: f32,    
}

// impl MyApp {
//     fn add(&self) -> i32 {
//         self.var1 + self.var2
//     }
// }

impl Default for MyApp {
    fn default() -> Self {
        Self {
            show_scale_window: false,
            scale_input: "1.2".to_string(),
            option_cpi_spi: -1,
            sem_no: 0,
            sem_no_f32: 0.0,
            sem_no_str: "0".to_string(),
            sem_option: 0,
            done_1: false,
            sem_info : semdata::Semester {
                sem_no: 0.0,
                course_code: vec![],
                course_name: vec![],
                course_credit: vec![],
                total_credit: 0.0,
                total_credit_till_sem: 0.0,
            },
            grades: Vec::new(),
            grade_input: "0".to_string(),
            calc_cpi_option : 0,
            cpi_op1_var1: 0.0,
            cpi_op1_var1_str: "".to_string(),
            cpi_op2_var1_str: "".to_string(),
            cpi_op2_var1: 0.0,
            cpi_op2_var2_str: "".to_string(),
            cpi_op2_var2: 0.0,


        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // let scale = *SCALE.lock().unwrap();
        let scale = functions::read_data();
        *SCALE.lock().unwrap() = scale;
        ctx.set_pixels_per_point(scale);

        TopBottomPanel::top("Top Panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button(" Menu", |ui| {
                    if ui.button("Change Scale").clicked() {
                        if self.show_scale_window {
                            self.show_scale_window = false;
                        } else {
                            self.show_scale_window = true;
                        }
                    }
                    if ui.button(
                        if *SHOWACTIONBAR.lock().unwrap() == true {
                            "Hide Action Bar"
                        } else {
                            "Show Action Bar"
                        },
                    ).clicked() {
                        if *SHOWACTIONBAR.lock().unwrap() == true {
                            *SHOWACTIONBAR.lock().unwrap() = false;
                        } else {
                            *SHOWACTIONBAR.lock().unwrap() = true;
                        }
                    }
                });
            });
        });


        if self.show_scale_window {
            egui::Window::new("Change Scale").show(ctx, |ui| {
                ui.label("Enter the scale between 0.5 and 2.5");

                if ui.text_edit_singleline(&mut self.scale_input).lost_focus() {
                    if let Ok(new_scale) = self.scale_input.parse::<f32>() {
                        if new_scale >= 0.5 && new_scale <= 2.5 {
                            *SCALE.lock().unwrap() = new_scale;
                            functions::write_data(new_scale);

                        }
                    }
                }

                if ui.button("Close").clicked() {
                    self.show_scale_window = false;
                }
            });
        }

        if *SHOWGRADETABLE.lock().unwrap() == true
        {
            egui::Window::new("Grade Table").show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.label("Grades      Points");
                    egui::Grid::new("grade_table")
                        .striped(true)
                        .min_col_width(60.0)
                        .show(ui, |ui| {
                            ui.label("AA"); ui.label("10");
                            ui.end_row();

                            ui.label("AB"); ui.label("9");
                            ui.end_row();

                            ui.label("BB"); ui.label("8");
                            ui.end_row();

                            ui.label("BC"); ui.label("7");
                            ui.end_row();

                            ui.label("CC"); ui.label("6");
                            ui.end_row();

                            ui.label("CD"); ui.label("5");
                            ui.end_row();

                            ui.label("DD"); ui.label("4");
                            ui.end_row();

                            ui.label("FF"); ui.label("0");
                            ui.end_row();
                        });
                });
            });
        }


        egui::CentralPanel::default().show(ctx, |ui| {
            let scroll_area = ScrollArea::vertical().max_height(1000.0); // Adjust the max_height as needed
            scroll_area.show(ui, |ui| {



                ui.heading("CPI / SPI Calculator");
            ui.separator();

            egui::ComboBox::from_label("Select Calculation")
                .selected_text(match self.option_cpi_spi {
                    0 => "Calculate SPI",
                    1 => "Calculate CPI",
                    _ => "Select",
                })
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.option_cpi_spi, 0, "Calculate SPI");
                    ui.selectable_value(&mut self.option_cpi_spi, 1, "Calculate CPI");
                });
            
            if self.option_cpi_spi == 0 || self.option_cpi_spi == 1
            {
                ui.horizontal(
                    |ui| {
                        ui.label("Select the semester :");
                        ui.add(egui::Slider::new(&mut self.sem_no, 1..=8).text("Semester"));                         
                    }
                );

                if self.sem_no == 7 || self.sem_no == 8
                {
                    ui.horizontal(
                        |ui| {
                            if self.sem_no == 7 {
                                ui.label("Select an option");
                                egui::ComboBox::from_label("")
                                    .selected_text(match self.sem_option {
                                        1 => "Option 1",
                                        2 => "Option 2",
                                        _ => "Select",
                                    })
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(&mut self.sem_option, 1, "Option 1");
                                        ui.selectable_value(&mut self.sem_option, 2, "Option 2");
                                    });
                            }
                            else if self.sem_no == 8 {
                                ui.label("Select an option");
                                egui::ComboBox::from_label("")
                                    .selected_text(match self.sem_option {
                                        1 => "Option 1",
                                        2 => "Option 2",
                                        3 => "Option 3",
                                        _ => "Select",
                                    })
                                    .show_ui(ui, |ui| {
                                        ui.selectable_value(&mut self.sem_option, 1, "Option 1");
                                        ui.selectable_value(&mut self.sem_option, 2, "Option 2");
                                        ui.selectable_value(&mut self.sem_option, 3, "Option 3");
                                    });
                            }
                        }
                    ); 
                }
                
                ui.horizontal(|ui| {
                    self.sem_no_f32 = self.sem_no as f32 ;
                    ui.label(format!("Semester : {}", self.sem_no).to_string());
                    if self.sem_no == 7 || self.sem_no == 8
                    {
                        self.sem_no_f32 = self.sem_no_f32 + 0.1 * self.sem_option as f32;
                        ui.label(format!("Option : {}", self.sem_option).to_string());
                    }
                });
            }
            if ui.button("Done").clicked() {
                self.done_1 = true;
            }
            ui.separator();

            // FOR SPI CALCULATION
            if (self.done_1 && self.option_cpi_spi == 0 && self.sem_no != 0) ||
            (self.done_1 && self.option_cpi_spi == 1 && self.sem_no != 0 && self.sem_no == 1)
            {

                self.sem_info = semdata::get_semesters(self.sem_no_f32).unwrap();
                GRADEINPUT.lock().unwrap().resize(self.sem_info.course_code.len(), String::new());
                if  GRADEINPUT.lock().unwrap().len() == 0{
                    update_grade_info_size(self.sem_info.course_code.len());
                }
                let mut spi: f32 = 0.0;
                for i in 0..self.sem_info.course_code.len()
                {
                    ui.add_space(2.0);
                    ui.horizontal(|ui| {
                        ui.label(format!("{} ", self.sem_info.course_code[i]).to_string());
                        ui.label(format!("{} ", self.sem_info.course_name[i]).to_string());
                        ui.label(format!(" Credit: {} ", self.sem_info.course_credit[i]).to_string()); 
                    });

                    //get greades in global vector of string
                    ui.horizontal(|ui| {
                        ui.label("Enter the grade : ");
                        self.grades.push(0.0);
                        let mut grade_int: i32 = self.grades[i] as i32;
                        ui.add(egui::Slider::new(&mut grade_int, 0..=10).text("Grade"));
                        self.grades[i] = grade_int as f32;
                        GRADEINPUT.lock().unwrap()[i] = grade_int.to_string();
                    });
                    //print the entered grades
                    // println!("{:?}", GRADEINPUT.lock().unwrap());
                    if GRADEINPUT.lock().unwrap().iter().all(|grade| !grade.is_empty()) {
                        spi = functions::calc_spi(self.sem_no_f32, GRADEINPUT.lock().unwrap().clone());
                    }
                }
                if SHOW_SPI.lock().unwrap().to_owned()
                {
                    ui.add_space(10.0);
                    // ui.monospace(format!("SPI of Semester {} is {:.3}", self.sem_no_f32, spi));
                    if self.sem_no == 1{
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(format!("SPI and CPI of Semester {} is {:.3}", self.sem_no_f32, spi))
                                    .heading(), // Use heading() method to set the text style to Heading
                            ),
                        );
                    }
                    else {
                        {
                            ui.add(
                                egui::Label::new(
                                    egui::RichText::new(format!("SPI of Semester {} is {:.3}", self.sem_no_f32, spi))
                                        .heading(), // Use heading() method to set the text style to Heading
                                ),
                            );
                        }
                    }
                    
                }
                ui.add_space(100.0); //for scrolling ease
            }
            
            //FOR CPI CALCULATION
            else if self.done_1 && self.option_cpi_spi == 1 && self.sem_no != 0 && self.sem_no != 1{
                ui.label("Calculating CPI");
                let option1_label = format!("You have the CPI till sem {} and want to calculate the CPI of sem {}", self.sem_no -1, self.sem_no);
                ui.radio_value(&mut self.calc_cpi_option, 1, &option1_label);
                
                let option2_label = format!("You have the SPI of sem {} and the CPI till sem {} And want to calculate the CPI of sem {}", self.sem_no , self.sem_no-1, self.sem_no);
                ui.radio_value(&mut self.calc_cpi_option, 2, &option2_label);

                ui.separator();

                // OPTION 1
                if self.calc_cpi_option == 1
                {
                    ui.horizontal(|ui| {
                        
                        let uilabelstring1 = format!("Enter the value of CPI of sem {}", self.sem_no - 1);
                        ui.label(&uilabelstring1);
                        ui.text_edit_singleline(&mut self.cpi_op1_var1_str);
                        if self.cpi_op1_var1_str.len() > 0
                        {
                            self.cpi_op1_var1 = self.cpi_op1_var1_str.parse::<f32>().unwrap();
                            // println!("CPI of sem {} : {}", self.sem_no - 1,self.cpi_op1_var1);  
                            // let mut tempStr = format!("The CPI of Sem {} is {} ", self.sem_no, self.cpi_op1_var1);
                            // ui.label(&tempStr);
                        }

                        

                        // self.cpi_op1_var1 = self.cpi_op1_var1_str.parse::<f32>().unwrap();

                        // println!("Enter the value of CPI of sem {} : ", self.sem_no - 1);                        
                        
                        // ui.add(egui::Slider::new(&mut self.sem_no_f32, 1.0..=8.0).text("Semester"));
                    });
                    ui.add_space(5.0);

                    //getting SPI
                    let uplabelstring2: String = format!("Enter the Grades of Sem {} :",self.sem_no);
                    ui.label(&uplabelstring2);
                    ui.add_space(2.0);

                    self.sem_info = semdata::get_semesters(self.sem_no_f32).unwrap();
                    GRADEINPUT.lock().unwrap().resize(self.sem_info.course_code.len(), String::new());
                    if  GRADEINPUT.lock().unwrap().len() == 0{
                        update_grade_info_size(self.sem_info.course_code.len());
                    }
                    let mut spi: f32 = 0.0;
                    for i in 0..self.sem_info.course_code.len() {
                        ui.add_space(2.0);
                        ui.horizontal(|ui| {
                            ui.label(format!("{} ", self.sem_info.course_code[i]).to_string());
                            ui.label(format!("{} ", self.sem_info.course_name[i]).to_string());
                            ui.label(format!(" Credit: {} ", self.sem_info.course_credit[i]).to_string()); 
                        });
                    
                        //get grades in global vector of string
                        ui.horizontal(|ui| {
                            ui.label("Enter the grade : ");
                            self.grades.push(0.0);
                            let mut grade_int: i32 = self.grades[i] as i32;
                            ui.add(egui::Slider::new(&mut grade_int, 0..=10).text("Grade"));
                            self.grades[i] = grade_int as f32;
                            GRADEINPUT.lock().unwrap()[i] = grade_int.to_string();
                        });
                        //print the entered grades
                        // println!("{:?}", GRADEINPUT.lock().unwrap());
                        if GRADEINPUT.lock().unwrap().iter().all(|grade| !grade.is_empty()) {
                            spi = functions::calc_spi(self.sem_no_f32, GRADEINPUT.lock().unwrap().clone());
                        }
                    }
                    ui.add_space(5.0);
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(format!("SPI of Semester {} is {:.3}", self.sem_no_f32, spi))
                                .heading(),
                        ),
                    );
                    ui.add_space(3.0);
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(format!("CPI of Semester {} is {:.3}", self.sem_no_f32, functions::calculate_cpi_option3(self.sem_no_f32, self.cpi_op1_var1, spi)))
                                .heading(),
                        ),
                    );
                    
                }

                else if self.calc_cpi_option  == 2{
                    let uilabelstring2 = format!("Enter the value of SPI of sem {}", self.sem_no);

                    let uilabelstring3 = format!("Enter the value of CPI of sem {}", self.sem_no - 1);
                    
                    ui.label(&uilabelstring3);
                    ui.horizontal(|ui|{
                        ui.text_edit_singleline(&mut self.cpi_op2_var1_str);
                        if self.cpi_op2_var1_str.len() > 0
                        {
                            self.cpi_op2_var1 = self.cpi_op2_var1_str.parse::<f32>().unwrap();
                        }
                    });

                    ui.label(&uilabelstring2);
                    ui.horizontal(|ui|{
                        ui.text_edit_singleline(&mut self.cpi_op2_var2_str);
                        if self.cpi_op2_var2_str.len() > 0
                        {
                            self.cpi_op2_var2 = self.cpi_op2_var2_str.parse::<f32>().unwrap();
                        }
                    });

                    ui.add_space(5.0);
                    let uilabelstring4 = format!("CPI of Semester {} is {:.3}", self.sem_no, functions::calculate_cpi_option3(self.sem_no_f32, self.cpi_op2_var1, self.cpi_op2_var2)); 
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(uilabelstring4)
                                .heading(),
                        ),
                    );
                }
                ui.add_space(100.0); 

            }


            });
            


            

        });

        if *SHOWACTIONBAR.lock().unwrap() == true
        {
            
            egui::TopBottomPanel::bottom("Bottom Panel").show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.add_space(5.0);
                    ui.horizontal(|ui|(

                        if ui.button(if *SHOWGRADETABLE.lock().unwrap() == true {"Hide Grade Table"} else {"Show Grade Table"})
                        .clicked() {
                            if *SHOWGRADETABLE.lock().unwrap() == true{
                                *SHOWGRADETABLE.lock().unwrap() = false;
                            }
                            else{
                                *SHOWGRADETABLE.lock().unwrap() = true;
                            }
                        },
                        if ui.button("Reset").clicked() {
                            self.option_cpi_spi = -1;
                            self.sem_no = 0;
                            self.sem_no_f32 = 0.0;
                            self.sem_no_str = "0".to_string();
                            self.sem_option = 0;
                            self.done_1 = false;
                            self.sem_info = semdata::Semester {
                                sem_no: 0.0,
                                course_code: vec![],
                                course_name: vec![],
                                course_credit: vec![],
                                total_credit: 0.0,
                                total_credit_till_sem: 0.0,
                            };
                            self.grades = Vec::new();
                            self.grade_input = "0".to_string();
                        },
                        if ui.button("Quit").clicked() {
                            std::process::exit(0);
                        },
                        if ui.button("Quit and Rerun").clicked() {
                            ctx.send_viewport_cmd(ViewportCommand::Close);
                            Command::new("cargo")
                            .arg("run")
                            .spawn()
                            .expect("Failed to execute command");
                        },
                    ));
                    ui.add_space(2.0);
                });
            });
        }
    }
}

fn update_grade_info_size(size: usize)
{
    let mut grade_input = GRADEINPUT.lock().unwrap();
    grade_input.clear();
    for _i in 0..size
    {
        grade_input.push("".to_string());
    }
}