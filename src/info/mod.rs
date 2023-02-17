use Answer::*;

#[derive(Debug)]
pub struct Categories {
    pub yes: Vec<Entry>,
    pub no: Vec<Entry>,
    pub maybe: Vec<Entry>,
}

#[derive(Debug)]
pub struct Entry {
    pub name: String,
    pub answer: Answer,
    pub source: String,
    pub desc: Option<String>,
    pub review: Option<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
pub enum Answer {
    Yes,
    No,
    Maybe,
}

pub type Rows<'a> = Vec<[Option<&'a Entry>; 3]>;

impl Answer {
    /// Match string slice to `Answer`, using first character
    fn from(s: &str) -> Option<Self> {
        Some(match s.chars().next()? {
            'y' => Yes,
            'n' => No,
            'm' => Maybe,

            _ => return None,
        })
    }
}

impl Categories {
    /// Create empty `Categories` struct
    fn new() -> Self {
        Self {
            yes: Vec::new(),
            no: Vec::new(),
            maybe: Vec::new(),
        }
    }

    /// Parse categories from text
    fn from(text: &str) -> Categories {
        let mut cats = Categories::new();

        for entry in text.split("\n\n") {
            let mut lines = entry.lines();

            let Some(name) = next_line(&mut lines) else {
                continue;
            };
            let name = upper_first(&name);

            let answer = lines.next().expect("Missing answer");
            let answer = Answer::from(answer).expect(&format!("Unknown answer '{}'", answer));

            let source = next_line(&mut lines).expect("Missing source");

            let desc = next_line(&mut lines);

            let tags = match lines.next() {
                Some(line) => split_line(line),
                None => Vec::new(),
            };

            let review = next_line(&mut lines);

            let entry = Entry {
                name,
                answer,
                source,
                desc,
                review,
                tags,
            };

            match &answer {
                Yes => cats.yes.push(entry),
                No => cats.no.push(entry),
                Maybe => cats.maybe.push(entry),
            }
        }

        cats
    }

    /// Convert `self` into vector of rows
    pub fn rows(&self) -> Rows {
        let mut rows = Vec::new();

        for i in 0.. {
            let yes = self.yes.get(i);
            let no = self.no.get(i);
            let maybe = self.maybe.get(i);

            if yes.is_none() && no.is_none() && maybe.is_none() {
                break;
            }

            rows.push([yes, maybe, no]);
        }

        rows
    }
}

/// Get all categories info
pub fn get_info() -> Categories {
    Categories::from(include_str!("data.txt"))
}

/// Get next line of mutable reference as trimmed `String`
///
/// Returns `None` if string is empty
fn next_line(lines: &mut std::str::Lines) -> Option<String> {
    match lines.next() {
        Some(line) => {
            let line = line.trim().to_string();
            if line.is_empty() {
                return None;
            }
            Some(line)
        }

        None => None,
    }
}

/// Split line at comma, return as `String` list
fn split_line(line: &str) -> Vec<String> {
    line.split(',')
        .map(|item| item.trim().to_string())
        .collect()
}

/// Uppercase first letter of string, lowercase rest
pub fn upper_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_string().to_uppercase() + &chars.as_str().to_lowercase(),
        None => String::new(),
    }
}
