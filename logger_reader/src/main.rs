use imgui::*;
use std::fs::File;
use std::io::{self, BufRead};
use fuzzy_search;
use regex::Regex;
use std::collections::HashSet;

mod support;

struct LogLine {
    line: i32,
    severity: String,
    category: String,
    message: String,
}

struct LogFile {
    name: String,
    lines: Vec<LogLine>,
    categories: HashSet<String>,
}

fn open_file(file_name: &String) -> LogFile {
    let file = File::open(file_name).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut lines_to_render: Vec<LogLine> = Vec::new();

    let mut categories: HashSet<String> = HashSet::new();
    let re = Regex::new("\\[(.*)\\]\\[(.*)\\]: (.*)").unwrap();

    let mut line_nr = 0;
    for line in lines {
        line_nr += 1;
        let line_str = line.unwrap();
        for cap in re.captures_iter(&line_str) {
            let category = cap[2].to_string();

            if !categories.contains(&category) {
                categories.insert(category.to_string()); // Yes type is string but apparently this is the only way to copy?
            }

            lines_to_render.push(LogLine{
                line: line_nr,
                severity: cap[1].to_string(), 
                category: category, 
                message: cap[3].to_string()
            });
        }
    }

    LogFile{
        name: file_name.to_string(),
        lines: lines_to_render,
        categories: categories,
    }
}

fn main() {
    let mut log_files = Vec::new();
    log_files.push(open_file(&"log.log".to_string()));

    let system = support::init("Logger Reader");

    let mut show_log = true;
    let mut show_warning = true;
    let mut show_error = true;

    let mut current_category = "All".to_string();
    let mut search_text = ImString::new("");

    let font_size = system.font_size;

    system.main_loop(move |_, ui| {
        Window::new(im_str!("Logger Reader"))
            .flags(WindowFlags::NO_TITLE_BAR|WindowFlags::ALWAYS_USE_WINDOW_PADDING|WindowFlags::NO_RESIZE|WindowFlags::NO_MOVE|WindowFlags::NO_COLLAPSE)
            .size([1024.0, 768.0], Condition::FirstUseEver)
            .position([0.0, 0.0], Condition::FirstUseEver)
            .build(ui, || {
                ui.checkbox(im_str!("Show Log"), &mut show_log);
                ui.checkbox(im_str!("Show Warning"), &mut show_warning);
                ui.checkbox(im_str!("Show Error"), &mut show_error);


                InputText::new(ui, im_str!("Search"), &mut search_text).build();

                TabBar::new(im_str!("Open Files")).build(&ui, || {
                    for log_file in &log_files {
                        TabItem::new(&ImString::new(&log_file.name)).build(&ui, || {
                            ComboBox::new(im_str!("Category"))
                                .preview_value(&ImString::new(&current_category))
                                .build(ui, || {
                                    if Selectable::new(&ImString::new("All")).build(ui) {
                                        current_category = "All".to_string();
                                    }

                                    for category in &log_file.categories {
                                        if Selectable::new(&ImString::new(category)).build(ui) {
                                            current_category = category.to_string();
                                        }
                                    }
                                });

                            ChildWindow::new(im_str!("Log Window")).build(ui, || {
                                ui.columns(4, im_str!("LogFile"), true);
                                ui.text(im_str!("Line"));
                                ui.set_current_column_width(font_size * 4.0);

                                ui.next_column();
                                ui.text(im_str!("Severity"));
                                ui.set_current_column_width(font_size * 8.0);

                                ui.next_column();
                                ui.text(im_str!("Category"));
                                ui.set_current_column_width(font_size * 8.0);

                                ui.next_column();
                                ui.text(im_str!("Message"));
                                ui.separator();

                                for line in &log_file.lines {
                                    if !show_log && line.severity == "Log" {
                                        continue;
                                    }

                                    if !show_warning && line.severity == "Warning" {
                                        continue;
                                    }

                                    if !show_error && line.severity == "Error" {
                                        continue;
                                    }

                                    if current_category != "All" && line.category != current_category {
                                        continue;
                                    }

                                    let mut matching = true;
                                    if search_text.to_string() != "" {
                                        matching = fuzzy_search::fuzzy_match(&search_text.to_string(), &line.message).0;
                                    }

                                    if matching {
                                        ui.next_column();
                                        ui.text(format!("{}", line.line));

                                        ui.next_column();
                                        ui.text(&line.severity);

                                        ui.next_column();
                                        ui.text(&line.category);

                                        ui.next_column();
                                        ui.text(&line.message);
                                    }
                                }
                            });
                        });
                    }
                });
            });
    });
}
