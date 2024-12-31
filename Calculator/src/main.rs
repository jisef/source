// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_button_clicked({
        move || {
            let ui = ui_handle.unwrap();
            let sigma: Vec<char> = ui.get_operation().chars().collect();
            let op: char = *sigma.first().unwrap();

            let number0: f64 = ui.get_number0().into();
            let number1: f64 = ui.get_number1().into();
            //dbg!(number as f64);



            let result: f64 = calculate_input(number0, number1, op);
            ui.set_result(dbg!(result as f32));
        }
    });

    ui.on_get_input({
        move || {
            
        }
    });



    ui.run()?;

    Ok(())
}

fn calculate_input(x: f64, y: f64, operation: char) -> f64 {
    if operation == '+' {
        x + y 
    } else if operation == '-'  {
        x - y
    } else if operation == 'x' {
        x * y
    } else if operation == '/' {
        x / y
    } else {
        return 0.0;
    }
}
