use std::fmt;

pub struct Script {
    pub others1: String,
    pub memory: Memory,
    pub others2: String,
}

// name [(attr)] : ORIGIN = origin, LENGTH = len
#[derive(Debug, PartialEq)]
pub struct Block {
    pub name: String,
    pub attr: Option<String>,
    pub origin: u64,
    pub length: u64,
}

pub struct Memory {
    pub blocks: Vec<Block>,
}

impl fmt::Display for Script {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f,"{}",self.others1)?;
        writeln!(f,"{}",self.memory)?;
        writeln!(f,"{}",self.others2)
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}) : ORIGIN = {}, LENGTH = {}",
            self.name,
            self.attr
                .as_ref()
                .map(|attr| " ".to_string() + attr)
                .unwrap_or_else(|| "".to_string()),
            self.origin,
            self.length
        )
    }
}

impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "MEMORY {{")?;
        for block in &self.blocks {
            writeln!(f, "{}", block)?;
        }
        writeln!(f, "}}")
    }
}
