use std::collections::HashMap;
use std::io;
use std::io::{stdout, BufWriter, Write};
use std::str::Lines;
use std::time::Instant;
use pad::{Alignment, PadStr};
use colored::{Colorize};
use prettytable::{Cell, Row, Table, color, Attr};
use crate::args;
use crate::csv::CSV;

/// Prints a csv through Lines and
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

/// Prints the csv object with. Offset is the spacing between a column + lenght of longest value.
pub fn print_csv(csv: CSV, offset: usize, alignment: Alignment) -> bool {
    let width = get_max_width_per_column_csv(&csv, offset);
    let ges_width: usize = width.iter().map(|(_, size)| *size).sum::<usize>() + width.len();

    let stdout = stdout();
    let mut handle = io::BufWriter::new(stdout.lock());
    writeln!(handle, "Buffered output").unwrap();

    // TODO: redo reundant loops
    for head in csv.headers.clone() {
        let size = match width.get(&head) {
            Some(size) => size,
            _ => return false,
        };
        write!(handle, "{}|",head.pad_to_width_with_alignment(size.to_owned(), alignment)).unwrap();
        //sb.push_str(&*(head.pad_to_width_with_alignment(size.to_owned(), alignment) + "|"));
    }
    //sb.push_str("\n");
    write!(handle, "\n").unwrap();
    //sb.push_str(&*"-".repeat(ges_width));
    write!(handle, "{}", "-".repeat(ges_width)).unwrap();
    //sb.push_str("\n");
    write!(handle, "\n").unwrap();



    for row in csv.rows.clone() {
        for head in csv.headers.clone() {
            let print = row.get(&head).unwrap();
            let size = width.get(&head).unwrap();
            write!(handle, "{}|",print.pad_to_width_with_alignment(*size, alignment)).unwrap();
            //sb.push_str(&*((print.pad_to_width_with_alignment(*size, alignment)) + "|"));
        }
        //sb.push_str("\n");
        write!(handle, "\n").unwrap();
    }


    //println!("{}", sb);
    //stdout().flush().unwrap();
    handle.flush().unwrap();

    true
}


pub fn print_pretty_csv(csv: &CSV) {
    // TODO: add colors
    //let mut index_color = 0;
    //let colors = init_colors(csv.rows.len() + 1);

    let foreground_colors = init_colors(csv.rows.len());
    let mut table = Table::new();
    table.set_format(*prettytable::format::consts::FORMAT_BOX_CHARS);

    // adding headers
    let mut headers:Vec<Cell> = Vec::new();
    for head in csv.headers.clone() {
        let cell = Cell::new(&head).with_style(Attr::ForegroundColor(color::GREEN));
        headers.push(cell);
    }
    table.set_titles(Row::from(headers));

    // adding rows
    for (row, color) in csv.rows.iter().zip(foreground_colors.iter()) {
        let mut cells: Vec<Cell> = Vec::new();
        for head in csv.headers.clone() {
            let print = match row.get(&head) {
                Some(size) => size,
                _ => continue, //TODO: temp
            };
            cells.push(Cell::new(print).with_style(Attr::ForegroundColor(color::GREEN)));
        }
        table.add_row(Row::from(cells));
    }

    //let watch_bufwriter = Instant::now();
    let mut writer = BufWriter::new(stdout());
    let _ = table.print(&mut writer);
    
    /*let watcher = Instant::now();
    table.printstd();*/
    
    //println!("buf: {:?}", watch_bufwriter.elapsed());     // 166s
    //println!("stdout : {:?}", watcher.elapsed())          // 115s
    
    
    
}

pub fn print_table_with_comfy_table(lines: Lines) {
    let mut table = comfy_table::Table::new();
    for line in lines {
        table.add_row(line.split(',').collect::<Vec<&str>>());
    }
    println!("{}", table);
}


pub fn init_colors<'a>(max_index: usize) -> Vec<&'a str> {

    let possible_colors = vec!["Fr", "Fb", "Fg", "Fy","Fc", "Fm", "Fw", "Fd"];
    let mut index_colors: Vec<&str> = Vec::new();

    for i in 0..max_index {
        // Use modulo to cycle through the default colors
        let color = &possible_colors[i % possible_colors.len()];

        index_colors.push(color);
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

pub fn get_max_width_per_column_csv(csv: &CSV, offset: usize) -> HashMap<String, usize> {
    let mut list:HashMap<String, usize> = HashMap::new();
    for x in csv.headers.clone() {
        let mut size = x.len();
        for row in csv.rows.clone() {
            let sigma = row.get(&x).unwrap().len();
            if sigma > size {
                size = sigma;
            }
        }
        list.insert(x.to_string(), size + offset);
    }
    list
}