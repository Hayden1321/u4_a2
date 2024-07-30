use crate::College;

use colored::Colorize;
use prettytable::{format, row, Row, Table};
use std::io::{self, Stdin, Write};

pub fn collect(stdin: &Stdin) {
    for i in 0..4 {
        let mut confirm = false;

        while !confirm {
            let college_name = &mut String::new();
            let mut members: Vec<String> = Vec::new();

            let mut team_valid = false;

            println!("Team Setup [{}/4]", i + 1);

            while !team_valid {
                college_name.clear();

                print!("{}", "Please enter team name: ".blue());
                io::stdout().flush().unwrap();

                stdin.read_line(college_name).unwrap();

                if college_name.trim().len() > 0 {
                    team_valid = true;
                } else {
                    team_valid = false;
                    println!("{}", "Invalid Entry! Please enter a team name!".red())
                }
            }

            for i in 0..5 {
                let mut valid = false;

                while !valid {
                    let member = &mut String::new();

                    print!(
                        "  {} {}{} ",
                        "Please enter member".green(),
                        (i + 1).to_string().bright_green(),
                        "'s name:".green()
                    );
                    io::stdout().flush().unwrap();

                    stdin.read_line(member).unwrap();

                    if member.trim().chars().all(char::is_alphabetic) {
                        valid = true;

                        members.push(member.trim().to_string().clone());
                    } else {
                        println!(
                            "  {}",
                            "Invalid Entry! Please enter alphabetic characters only.".red()
                        )
                    }
                }
            }
            let mut tbl = Table::new();

            tbl.set_titles(row!["Name", "Members", "Points"]);
            tbl.set_format(*format::consts::FORMAT_NO_BORDER);

            tbl.add_row(Row::from(College {
                name: college_name.clone(),
                members: members.clone(),
                points: 0,
            }));
            println!("{}", tbl);

            let mut valid = false;

            while !valid {
                let input = &mut String::new();

                print!("{}", "Please confirm (y/n): ".yellow());
                io::stdout().flush().unwrap();

                stdin.read_line(input).unwrap();

                match input.trim().to_lowercase().as_str() {
                    "y" => {
                        valid = true;
                        confirm = true;

                        println!("-------------------------\n")
                    }
                    "n" => {
                        valid = true;
                        confirm = false;
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
}
