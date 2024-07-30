use colored::Colorize;
use prettytable::{row, Row};
use std::{fmt::Display, io, str::FromStr};

// Include other files
mod collect_events;
mod collect_teams;

// Create structures
pub struct College {
    name: String,
    members: Vec<String>,
    points: u16,
}
pub struct Event {
    name: String,
    event_type: EventType,
    scoring: Vec<College>,
}
pub enum EventType {
    Sporting,
    Academic,
}

// Allow enum to be printed
impl Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            EventType::Sporting => f.write_str("Sporting"),
            EventType::Academic => f.write_str("Academic"),
        }
    }
}

// Convert struct to a table row
impl From<College> for Row {
    fn from(v: College) -> Self {
        row![v.name, v.members.join("\n").to_string(), v.points]
    }
}

// Convert struct to a table row
impl From<Event> for Row {
    fn from(v: Event) -> Self {
        let mut c = 0;

        let list: String = match v.scoring.len() {
            0 => {
                format!("1st:\n2nd:\n3rd:")
            }
            _ => {
                let (arr, _) = v.scoring.split_at(3);

                arr.iter()
                    .map(|x| {
                        c += 1;
                        let word: &str;

                        match c {
                            1 => word = "1st",
                            2 => word = "2nd",
                            3 => word = "3rd",
                            _ => word = "??nd",
                        }
                        format!("{}: {}", word, x.name)
                    })
                    .collect::<String>()
            }
        };

        row![v.name, v.event_type, list]
    }
}

// Convert a string to an event type
impl FromStr for EventType {
    type Err = ();

    fn from_str(v: &str) -> Result<Self, Self::Err> {
        match v.trim().to_lowercase().as_str() {
            "academic" => Ok(EventType::Academic),
            "a" => Ok(EventType::Academic),
            "sporting" => Ok(EventType::Sporting),
            "sport" => Ok(EventType::Sporting),
            "s" => Ok(EventType::Sporting),
            _ => Err(()),
        }
    }
}

fn main() {
    let stdin = io::stdin();

    collect_teams::collect(&stdin);
    collect_events::collect(&stdin);

    println!("-------------------------");
    println!("{}", "The setup has now been completed!".green().bold());
}
