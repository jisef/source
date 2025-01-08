use std::collections::LinkedList;
use std::env;
use std::fs;

#[derive(Debug)]
#[derive(Clone)]
struct Command {
    command: String,
    count: u16
}


impl PartialEq for Command {
    fn eq(&self, other: &Command) -> bool {
        self.command == other.command
    }

}
struct Analyser {
    list: LinkedList<Command>
}

impl Analyser {
    fn analyse_line(&mut self, line: &str) {
        let line = line.split(" ").next().unwrap();
        if line.starts_with(":") {
            return;
        }
        let command = Command {
            command: String::from(line),
            count: 1,
        };
        if !self.list.contains(&command) {

            self.list.push_front(command);
        }
        else {
            let iter = self.list.iter_mut();
            for awd in iter {
                if awd.command == command.command {
                    awd.count += 1;
                }
            }
        }
    }

    fn get_count(&mut self) -> u16 {
        let iter = self.list.iter();
        let mut count:u16 = 0;
        for cmd in iter {
            count += cmd.count;
        }
        count
    }

    fn most_user_commands(&self) -> LinkedList<Command> {
        self.list.clone()
    }

    fn get_most_used_commands(&self) -> Command{
        let ll: LinkedList<Command> = sort_linked_list(self.list.clone());
        let mut biggest:Command = Command {
            command: String::new(),
            count: 0
        };
        for cmd in ll {
            if biggest.count < cmd.count {
                biggest = cmd
            }
        }
        biggest
    }
}



fn main() {
    let file = read_file(get_filepath_through_shell());
    let file = file.lines();
    let mut anal = Analyser {
        list: LinkedList::new(),
    };

    for sigma in file {
        anal.analyse_line(sigma);
    }
    for li in &anal.list {
        println!("Command: {}; count: {}", li.command, li.count);
    }
    println!("Total Used Commands: {}",anal.get_count()); 
    
    sort_linked_list(anal.list.clone());
    println!("Most Used Command: \n {:?}", anal.get_most_used_commands());
    
    
}

fn read_file(filepath: String) -> String {
    let content_of_file = fs::read(filepath).expect("Could not read file");
    let content = String::from_utf8_lossy(&content_of_file).into_owned();
    content
}

fn get_filepath_through_shell() -> String {
    let shell = get_shell();
    let mut filepath = String::new();

    if shell.contains("bash") {
        String::from("/Users/josef/.bash_history")
    } else {
        String::from("/Users/josef/.zsh_history")
    }
}

#[cfg(unix)]
fn get_shell() -> String {
    let sigma = env::var("SHELL").ok();
    match sigma {
        Some(sigma) => { Some(sigma).unwrap()},
        _ =>  sigma.unwrap().as_str().to_string(),
    }
}

fn sort_linked_list(list: LinkedList<Command>) -> LinkedList<Command> {
    println!("---------------------------------------------");
    let mut vec: Vec<Command> = list.to_owned().into_iter().collect();
    vec.sort_by(|a, b| b.count.cmp(&a.count));
    vec = vec[..10].to_vec();
    let skibiditoilet = 0;
    for toiklet in &vec {
        println!("{}: {}", toiklet.command, toiklet.count)
    }
    //vec = vec.to_vec();
    //vec.into_iter().collect()
    vec.to_vec().into_iter().collect()
}
