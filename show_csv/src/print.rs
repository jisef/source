use std::collections::HashMap;
use std::str::Lines;
use pad::{Alignment, PadStr};
use colored::{Color, Colorize};
use tabled::Table;

pub fn print_lines_default(max_width_per_column: Vec<usize>, lines: Lines, seperator: char, offset: usize) {
    let mut ges_width: usize = max_width_per_column.iter().sum();
    ges_width += (offset + 1) * max_width_per_column.len();
    let mut is_first = true;
    let row_seperator = "-".repeat(ges_width);

    for line in lines {
        let mut index: usize = 0;
        for item in line.split(seperator) {
            let size:usize = max_width_per_column[index] + offset;

            let mut padded_text = item.pad_to_width_with_alignment(size, Alignment::Middle);
            if is_first {
                padded_text = padded_text + '|'.to_string().as_str();
                print!("{}", padded_text.bold());
            } else {
                padded_text = padded_text + '|'.to_string().as_str();
                print!("{}", padded_text);
            }
            index += 1;
        }
        is_first = false;
        println!();
        println!("{}", row_seperator);


    }
}
pub fn print_table_with_tabled(lines: Lines) {
    let lines : Vec<_> = lines.collect();

    let table = Table::new(&lines).to_string();

    println!("{}", table);
}


pub fn print_table_with_comfy_table(lines: Lines) {
    let mut table = comfy_table::Table::new();
    for line in lines {
        table.add_row(line.split(',').collect::<Vec<&str>>());
    }
    println!("{}", table);
}


pub fn init_colors(max_index: usize) -> HashMap<u16, Color> {
    let default_colors = vec![
        Color::Blue,
        Color::Green,
        Color::Red,
        Color::Cyan,
        Color::Magenta,
        Color::Yellow,
    ];

    let mut index_colors: HashMap<u16, Color> = HashMap::new();

    for i in 0..max_index {
        // Use modulo to cycle through the default colors
        let color = &default_colors[i % default_colors.len()];

        index_colors.insert(i as u16, *color);
    }

    index_colors
}

pub fn get_max_width_per_column(text: &String, seperator: char) -> Vec<usize>{
    let lines = text.lines();
    let mut list:Vec<usize> = Vec::new();

    let mut is_first: bool = true;
    for line in lines {
        let line = line.split(seperator);
        let mut index: usize = 0;
        for x in line {
            if is_first {
                list.push(x.len())
            } else {
                let len = x.len();
                if list[index] < len {
                    list[index] = len;
                }
            }
            index = index + 1;
        }
        is_first = false;

    }
    list
}