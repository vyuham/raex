// The Executor is a set of components that execute code on data. Given a coloured image, we intend to generate a black and white image.
use std::fmt::Debug;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hasher, Hash};

#[derive(Debug, PartialEq, Clone)]
pub enum Task {
    /// Divides up the image into equal sized chunks of u8 matrices.
    Divide,
    /// Convert color u8 values into black and white.
    MakeBW,
    /// Collate and generate final image.
    Collate,
    /// Waiting for all sub-processes created to be completed.
    Wait
}

impl Task {
    pub fn description(&self) -> String {
        match self {
            Divide => "Divides input Image".to_string(),
            MakeBW => "Black and White converter".to_string(),
            Collate => "Final collector and assembler".to_string(),
            Wait => "Suspended task".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Color{
    r: u8, 
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r:u8, g:u8, b:u8) -> Self {
        Self { r, g, b }
    }

    pub fn black_white_shade(self) -> Self {
        let shade = (0.3 * self.r as f64 + 0.59 * self.g as f64 + 0.11 * self.b as f64) as u8;
        Self {r:shade, g:shade, b:shade}
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Data {
    Image(Vec<Vec<Color>>),
    Line(Vec<Color>),
}

impl Data {
    pub fn line(self) -> Vec<Color> {
        if let Data::Line(inner) = self {
            return inner;
        } else {
            return vec![];
        }
    }

    pub fn description(&self) -> String {
        match self {
            Image => "Image Matrix".to_string(),
            Line => "Pixel Array".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Exec {
    /// The type of processing to be applied on data.
    task: Task,
    /// UID used to refer to an execution unit.
    /// TODO: Write code to generate the same.
    hash: u64,
    /// Data values to be processed.
    data: Data,
}

impl Exec {
    pub fn new(task: Task, data: Data) -> Self {
        let mut hasher = DefaultHasher::new();
        task.description().hash(&mut hasher);
        data.description().hash(&mut hasher);
        Self { task, hash: hasher.finish(), data }
    }

    pub fn task(&self) -> &Task { &self.task }
    pub fn hash(self) -> u64 { self.hash }
    pub fn data(self) -> Data { self.data }

    pub fn in_words(&self) -> String {
        format!("{}: {} on {}", self.hash, self.task.description(), self.data.description())
    }

    pub fn execute(&self) -> Vec<Self> {
        match self.task {
            Task::Divide => {
                let mut lines: Vec<Self> = vec![]; 
                if let Data::Image(image) = &self.data {
                    for line_no in 0..image.len() {
                        // TODO: Store the lines of pixels in a global-datastrucutre that can later process it within the cluster.
                        lines.push(Self::new(
                            Task::MakeBW, Data::Line((*image[line_no]).to_vec())
                        ));
                    }
                }
                lines
            },
            Task::MakeBW => {
                let mut bw_line: Vec<Color> = vec![];
                if let Data::Line(line) = &self.data {
                    for pixel in line { bw_line.push(pixel.black_white_shade()); }
                }
                // TODO: Store the processed values in a global-datastrucutre that can later collate it into the desired output.
                vec![Self::new(Task::Collate, Data::Line(bw_line))]
            },
            _ => vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn black_white_test() {
        let image = Data::Line(vec![Color::new(163, 200, 255)]);
        let exec = Exec::new(Task::MakeBW, image);
        let s = 194.95f64 as u8;
        let expect = vec![Exec::new(
            Task::Collate, Data::Line(vec![Color::new(s, s, s)])
        )];
        assert_eq!(exec.execute(), expect);
    }

    #[test]
    fn divide_image_test() {
        let pixel = Color::new(163, 200, 255);
        let image = Data::Image(vec![vec![pixel]]);
        let exec = Exec::new(Task::Divide, image);
        let expect = vec![Exec::new(
            Task::MakeBW, Data::Line(vec![pixel])
        )];
        assert_eq!(exec.execute(), expect);
    }

    #[test]
    fn exec_hash() {
        let line = Data::Line(vec![Color::new(100,100,100)]);
        let exec = Exec::new(Task::Collate, line);
        assert_eq!(exec.hash(), 10041778673549373737);
    }
}