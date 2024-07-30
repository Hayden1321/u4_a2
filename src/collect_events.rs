use crate::{Event, EventType};

use colored::Colorize;
use prettytable::{format, row, Row, Table};
use std::{
    io::{self, Stdin, Write},
    str::FromStr,
};

pub fn collect(stdin: &Stdin) {
    let mut events_confirm = false;
    let mut events_valid = false;

    while !events_confirm {
        let mut tbl = Table::new();

        for i in 0..5 {
            let event_name = &mut String::new();

            println!("Event Setup [{}/5]", i + 1);

            print!("{}", "Please enter event name: ".blue());
            io::stdout().flush().unwrap();

            stdin.read_line(event_name).unwrap();

            let mut event_valid = false;

            while !event_valid {
                let event_type = &mut String::new();

                print!("{}", "Please enter event type (Sporting/Academic): ".blue());
                io::stdout().flush().unwrap();

                stdin.read_line(event_type).unwrap();

                if let Ok(event_t) = EventType::from_str(&event_type) {
                    event_valid = true;

                    tbl.add_row(Row::from(Event {
                        name: event_name.clone(),
                        event_type: event_t,
                        scoring: vec![],
                    }));
                } else {
                    event_valid = false;

                    println!(
                        "{}",
                        "Invalid Entry! Please enter 'Sporting' or 'Academic'".red()
                    );
                }
            }
        }

        tbl.set_titles(row!["Name", "Type", "Positions"]);
        tbl.set_format(*format::consts::FORMAT_NO_BORDER);

        println!("{}", tbl);

        while !events_valid {
            let input = &mut String::new();

            print!("{}", "Please confirm (y/n): ".yellow());
            io::stdout().flush().unwrap();

            stdin.read_line(input).unwrap();

            match input.trim().to_lowercase().as_str() {
                "y" => {
                    events_valid = true;
                    events_confirm = true;
                }
                "n" => {
                    events_valid = true;
                    events_confirm = false;
                }
                _ => {
                    println!(
                        "{}",
                        "Invalid Entry! Please use 'y' or 'n' to choose an option.".red()
                    )
                }
            }
        }
    }
}
