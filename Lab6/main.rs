use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
trait Command {
    fn get_name(&self) -> &str;
    fn exec(&mut self, args: &[String]);
}
struct PingCommand;
struct CountCommand;
struct TimesCommand {
    count: usize,
}

impl Command for PingCommand {
    fn get_name(&self) -> &str {
        "ping"
    }
    fn exec(&mut self, _args: &[String]) {
        println!("pong!");
    }
}
impl Command for CountCommand {
    fn get_name(&self) -> &str {
        "count"
    }
    fn exec(&mut self, args: &[String]) {
        println!("{} argumente", args.len());
    }
}
impl Command for TimesCommand {
    fn get_name(&self) -> &str {
        "times"
    }
    fn exec(&mut self, _args: &[String]) {
        self.count += 1;
        println!("Comanda folosita de {} ori.", self.count);
    }
}
struct Terminal {
    commands: Vec<Box<dyn Command>>,
}
impl Terminal {
    fn new() -> Self {
        Self {
            commands: Vec::new(),
        }
    }
    fn register(&mut self, command: Box<dyn Command>) {
        self.commands.push(command);
    }
    fn run(&mut self) {
        let filep = "cmds.txt";
        if !Path::new(filep).exists() {
            println!("No cmds file");
            return;
        }
        let file = File::open(filep).expect("Eroare la deschidere fisier");
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line.unwrap();
            let line = line.trim();

            if line == "stop" {
                break;
            }
            let mut parts = line.split_whitespace();
            if let Some(command_name) = parts.next() {
                let args: Vec<String> = parts.map(|s| s.to_string()).collect();
                let mut found = false;
                for command in &mut self.commands {
                    if command.get_name() == command_name {
                        command.exec(&args);
                        found = true;
                        break;
                    }
                }
                if found == false {
                    println!("Comanda gresita {}", command_name);
                }
            }
        }
    }
}

fn main() {
    let mut terminal = Terminal::new();

    terminal.register(Box::new(PingCommand {}));
    terminal.register(Box::new(CountCommand {}));
    terminal.register(Box::new(TimesCommand { count: 0 }));

    terminal.run();
}
