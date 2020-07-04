use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json;
use uuid::Uuid;

use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::BufReader;

#[derive(Deserialize, Serialize)]
pub struct Ballot {
    event: Option<Event>,
    intro: String,
    items: Vec<Item>,
    signature: Option<Signature>,
}

impl Ballot {
    fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let ballot = serde_json::from_reader(reader)?;

        Ok(ballot)
    }
}

impl fmt::Display for Ballot {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.event {
            Some(e) => writeln!(f, "{}", e)?,
            None => ()
        }

        writeln!(f, "{}", &self.intro)?;

        for item in &self.items {
            writeln!(f, "{}", item)?
        }

        match &self.signature {
            Some(sig) => writeln!(f, "{}", sig),
            None      => writeln!(f, "Unsigned"),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct Event {
    id: Uuid,
    name: String,
    date: NaiveDate,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", &self.name)?;
        writeln!(f, "Date: {}", &self.date)
    }
}

#[derive(Deserialize, Serialize)]
pub struct Item {
    description: String,
    choices: Choices,
}

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", &self.description)?;
        writeln!(f, "{}", &self.choices)
    }
}

#[derive(Deserialize, Serialize)]
enum Choices {
    Ranked(RankedChoice),
    Single(SingleChoice),
    Tally(TallyChoice),
}

#[derive(Deserialize, Serialize)]
struct RankedChoice {
    options: Vec<String>,
}

#[derive(Deserialize, Serialize)]
struct SingleChoice {
    options: Vec<String>,
    selected: Option<String>,
}

#[derive(Deserialize, Serialize)]
struct TallyChoice {
    count: u64
}

impl fmt::Display for Choices {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Choices::Ranked(r) => r.fmt(f),
            Choices::Single(s) => s.fmt(f),
            Choices::Tally(t) => t.fmt(f),
        }
    }
}

impl fmt::Display for RankedChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let opt_count = self.options.len();

        for i in 1..opt_count {
            writeln!(f, "{}: {}", i, self.options[i - 1])?;
        }

        writeln!(f, "{}: {}", opt_count, self.options[opt_count])
    }
}

impl fmt::Display for SingleChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.options.len() {
            writeln!(f, "{}", self.options[i])?;
        }

        match &self.selected {
            None => writeln!(f, "No selection"),
            Some(opt) => writeln!(f, "{}", &opt),
        }
    }
}

impl fmt::Display for TallyChoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Total: {}", &self.count)
    }
}

#[derive(Deserialize, Serialize)]
pub struct Signature {
    name: Option<String>,
    signature: Option<Vec<u8>>,
}

impl fmt::Display for Signature {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.name {
            Some(name) => writeln!(f, "{}", name),
            None       => writeln!(f, "Anonymous"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ranked_choice_display() {
        let list = RankedChoice {
            options: vec!["one".to_string(), "two".to_string()],
        };
        let choices = Choices::Ranked(list);

        assert_eq!(choices.fmt(), "1: one\n2: two");
    }
}
