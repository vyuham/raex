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
    /// Waiting for all sub-processes created to be completed.
    Wait
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
    /// The type of processing to be applied on data.
    task: Task,
    /// UID used to refer to an execution unit.
    /// TODO: Write code to generate the same.
    hash: u32,
    /// Data values to be processed.
    data: Data,
}

impl Exec {
    pub fn new(
        task: Task,
        hash: u32,
        data: Data,
    ) -> Self {
        Self { task, hash, data }
    }

    pub fn task(&self) -> &Task { &self.task }
    pub fn hash(self) -> u32 { self.hash }
    pub fn data(self) -> Data { self.data }

    pub fn execute(&self) -> Vec<Self> {
        match self.task {
            Task::Divide => {
                let mut lines: Vec<Self> = vec![]; 
                if let Data::Image(image) = &self.data {
                    for line_no in 0..image.len() {
                        // TODO: Store the lines of pixels in a global-datastrucutre that can later process it within the cluster.
                        lines.push(Self::new(
                            Task::MakeBW, line_no as u32, Data::Line((*image[line_no]).to_vec())
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
                vec![Self { task: Task::Collate, hash: 1000, data: Data::Line(bw_line) }]
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
        let exec = Exec::new(Task::MakeBW, 0, image);
        let s = 194.95f64 as u8;
        let expect = vec![Exec::new(
            Task::Collate, 1000, Data::Line(vec![Color::new(s, s, s)])
        )];
        assert_eq!(exec.execute(), expect);
    }

    #[test]
    fn divide_image_test() {
        let pixel = Color::new(163, 200, 255);
        let image = Data::Image(vec![vec![pixel]]);
        let exec = Exec::new(Task::Divide, 0, image);
        let expect = vec![Exec::new(
            Task::MakeBW, 0, Data::Line(vec![pixel])
        )];
        assert_eq!(exec.execute(), expect);
    }
}