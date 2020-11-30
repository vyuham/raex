// The Executor is a set of components that execute code on data. Given a coloured image, we intend to generate a black and white image.
use std::fmt::Debug;

#[derive(Debug, PartialEq, Clone)]
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
}

#[derive(Debug, PartialEq, Clone)]
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

    pub fn task(&self) -> &Task { &self.task }
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
            Task::Divide => {
                let mut save: Vec<Vec<Color>> = vec![];
                if let Data::Image(image) = &self.data {
                    for line_no in 0..image.len() {
                        let mut line = Self::new(
                            Task::MakeBW, line_no as u8, Data::Line((*image[line_no]).to_vec())
                        );
                        line.execute();
                        match line.task() {
                            Task::Collate => save.push(line.data().line()),
                            _ => ()
                        }
                    }
                }
                self.data = Data::Image(save);
                self.task = Task::Done;
            }
            _ => ()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn black_white_test() {
        let image = Data::Line(vec![Color::new(163, 200, 255)]);
        let mut exec = Exec::new(Task::MakeBW, 0, image);
        exec.execute();
        let s = 194.95f64 as u8;
        let expect = Exec::new(
            Task::Collate, 0, Data::Line(vec![Color::new(s, s, s)])
        );
        assert_eq!(exec, expect);
    }

    #[test]
    fn divide_image_test() {
        let image = Data::Image(vec![vec![Color::new(163, 200, 255)]]);
        let mut exec = Exec::new(Task::Divide, 0, image);
        exec.execute();
        let s = 194.95f64 as u8;
        let expect = Exec::new(
            Task::Done, 0, Data::Image(vec![vec![Color::new(s, s, s)]])
        );
        assert_eq!(exec, expect);
    }
}