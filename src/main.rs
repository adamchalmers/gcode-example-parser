use anyhow::Error;

fn main() {
    let mut program = Program::new();
    program.add_comment("ADAM".to_owned());
    program.add_empty_line();
    program.add_command(vec!["N10".to_owned(), "10111".to_owned()]);
    program.add_command(vec!["M20".to_owned(), "10111".to_owned()]);
    program.add_empty_line();
    program.add_comment("KEVIN".to_owned());
    let gcode = program.to_gcode_string();
    println!("{}", gcode);

    let parsed = Program::from_gcode_string(&gcode).unwrap();
    dbg!(&parsed);
}

/// Basically a line in a GCode program.
#[derive(Debug)]
enum Node {
    Empty,
    Comment(String),
    Command(Vec<String>),
}

/// Represents a GCode program.
#[derive(Default, Debug)]
pub struct Program {
    nodes: Vec<Node>,
}

impl Program {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    pub fn add_empty_line(&mut self) {
        self.nodes.push(Node::Empty);
    }

    pub fn add_comment(&mut self, comment: String) {
        self.nodes.push(Node::Comment(comment));
    }

    pub fn add_command(&mut self, args: Vec<String>) {
        self.nodes.push(Node::Command(args))
    }

    pub fn from_gcode_string(gcode: &str) -> Result<Self, Error> {
        let mut program = Program::new();
        for line in gcode.lines() {
            if line.is_empty() {
                program.add_empty_line();
            } else if let Some(comment) = line.strip_circumfix('(', ')') {
                program.add_comment(comment.to_string());
            } else {
                program.add_command(line.split(' ').map(|arg| arg.to_string()).collect());
            }
        }
        Ok(program)
    }

    pub fn to_gcode_string(&self) -> String {
        let mut output = String::new();
        for node in &self.nodes {
            match node {
                Node::Empty => {}
                Node::Comment(s) => output.push_str(&format!("({s})")),
                Node::Command(items) => output.push_str(&items.join(" ")),
            }
            output.push('\n');
        }
        output
    }
}
