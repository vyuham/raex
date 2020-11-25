// The Executor is a set of components that execute code on data. Given a coloured image, we intend to generate a black and white image.
use std::fmt::Debug;

pub enum Task {
    /// Divides up the image into equal sized chunks of u8 matrices.
    Divide,
    /// Convert color u8 values into black and white.
    MakeBW,
    /// Collate and generate final image.
    Collate,
    /// Fully processed, no further execution.
    Done
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
        let shade = ((0.3 * self.r as f64) + (0.59 * self.g as f64) + (0.11 * self.b as f64)) as u8;
        Self {r:shade, g:shade, b:shade}
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Data {
    Image(Vec<Vec<Color>>),
    Line(Vec<Color>),
}

pub struct Exec {
    task: Task,
    hash: u8,
    data: Data,
}

impl Exec {
    pub fn new(
        task: Task,
        hash: u8,
        data: Data,
    ) -> Self {
        Self { task, hash, data }
    }

    pub fn task(self) -> Task { self.task }
    pub fn hash(self) -> u8 { self.hash }
    pub fn data(self) -> Data { self.data }

    pub fn execute(&mut self) {
        match self.task {
            Task::MakeBW => {
                let mut bw_image: Vec<Color> = vec![];
                if let Data::Line(line) = &self.data {
                    for unit in line { bw_image.push(unit.black_white_shade()); }
                }
                self.data = Data::Line(bw_image);
                self.task = Task::Collate;
            },
            _ => todo!()
        }
    }
}