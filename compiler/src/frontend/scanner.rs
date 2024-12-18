use crate::frontend::token;
use std::fs::File;
use std::io::{self, Read};

use super::parser;

struct FileParser {
    buffer: Vec<char>, // Holds all characters in the file
    position: usize,   // Tracks the current position
}

impl FileParser {
    // Initialize the parser by reading the file into a buffer
    pub fn new(file_path: &str) -> io::Result<Self> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        Ok(FileParser {
            buffer: contents.chars().collect(), // Convert the file contents to a Vec<char>
            position: 0,
        })
    }

    // Get the current character without advancing
    pub fn current_char(&self) -> Option<char> {
        self.buffer.get(self.position).copied()
    }

    // Move forward and get the next character
    pub fn next_char(&mut self) -> Option<char> {
        if self.position < self.buffer.len() {
            self.position += 1;
        }
        self.current_char()
    }

    // Move backward and get the previous character
    pub fn prev_char(&mut self) -> Option<char> {
        if self.position > 0 {
            self.position -= 1;
        }
        self.current_char()
    }

    // Peek forward without advancing
    pub fn peek_next(&self) -> Option<char> {
        self.buffer.get(self.position + 1).copied()
    }

    // Peek backward without moving
    pub fn peek_prev(&self) -> Option<char> {
        if self.position == 0 {
            None
        } else {
            self.buffer.get(self.position - 1).copied()
        }
    }
}
pub fn Scanner(filePath: &str) {
    let mut fparser = FileParser::new(filePath);
    while let Some(c) = fparser.current_char() {
        println!("Current Character:{}", c);
    }
}
