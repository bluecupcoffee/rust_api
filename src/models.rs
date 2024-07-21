use std::error::Error;
use std::fs::File;
use std::{fmt, io};
use std::io::BufRead;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use rand::prelude::*;
use crate::models;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Person {
    pub id: u64,
    pub name: String,
    pub color: Colors
}

impl Person {
    pub fn new(n: String, c: Colors) -> Person {
        Person {
            id: 0,
            name: n,
            color: c
        }
    }
}
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Colors {
    red,
    blue,
    yellow,
}

impl fmt::Display for Colors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Colors::yellow => write!(f, "yellow"),
            Colors::red => write!(f, "red"),
            Colors::blue => write!(f, "blue")
        }
    }
 }

pub struct PersonGenerator {
    rng: ThreadRng,
    population: u64,
    name_file: PathBuf,
    name_list: Option<Vec<String>>
}

impl PersonGenerator {
    pub fn new(name_file: String, pop: u64) -> PersonGenerator {
        PersonGenerator {
            rng: rand::thread_rng(),
            population: pop,
            name_file: PathBuf::from(name_file),
            name_list: None,
        }
    }

    pub fn load_names(&mut self) -> Result<(), Box<dyn Error>> {
        if !self.name_file.is_file() {
            let err: Box<dyn Error> = "No file to load".into();
            return Err(err)
        }
        let file = File::open(&self.name_file);
        let mut v: Vec<String> = Vec::new();
        match file {
            Ok(f) => {
                let reader = io::BufReader::new(f);
                for line in reader.lines() {
                    if let Ok(l) = line {
                        v.push(l);
                    }
                }
                self.name_list = Some(v);
                Ok(())
            },
            Err(e) => {
                let err: Box<dyn Error> = e.into();
                Err(err)
            }
        }
    }

    pub fn generate_person(&mut self) -> Option<Person> {
        if let Some(s) = self.name_list.as_ref() {
            let rn = self.rng.gen_range(0..s.len());
            let rc = self.rng.gen_range(0..3);
            let temp_name = s[rn].clone();
            let temp_color = match rc {
                0 => Colors::blue,
                1 => Colors::red,
                2 => Colors::yellow,
                _ => Colors::blue
            };

            Some(Person::new(temp_name, temp_color))
            //Person::new()
        } else {
            None
        }
    }

    pub fn generate_population(&mut self) -> Vec<Person> {
        let mut v: Vec<Person> = Vec::new();
        for i in 0..self.population {
            let p = self.generate_person();
            match p {
                Some(person) => {
                    v.push(person);
                },
                None => {
                    eprintln!("Error pushing person on to population vector");
                    std::process::exit(1);
                }
            }
        }
        v
    }
}