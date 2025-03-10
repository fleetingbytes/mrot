use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct MrotConfig {
    pub what: What,
    pub show: Show,
    pub ignore: Ignore,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct What {
    pub number: usize,
}

impl Default for What {
    fn default() -> Self {
        Self { number: 3 }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Show {
    pub range: String,
}

impl Default for Show {
    fn default() -> Self {
        Self {
            range: "from three days before today through tomorrow".to_string(),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Ignore(Vec<String>);

impl Ignore {
    pub fn add(&mut self, s: &str) {
        self.0.push(s.to_string())
    }
    pub fn remove(&mut self, s: &str) {
        self.0.retain(|i| i != s)
    }
    pub fn clear(&mut self) {
        self.0.clear()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl fmt::Display for Ignore {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        let separator = ", ";
        self.0
            .iter()
            .take(self.0.len().checked_sub(1).unwrap_or(0))
            .for_each(|i| {
                out.push_str(&i);
                out.push_str(separator);
            });
        out.push_str(self.0.iter().last().unwrap_or(&"".to_string()));
        fmt::Display::fmt(&out, f)
    }
}
