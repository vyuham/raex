// The Executor is a set of components that execute code on data. Given a coloured image, we intend to generate a 


pub enum Task {
    /// Divides up the image into equal sized chunks of u8 matrices.
    Divide,
    /// Convert color u8 values into black and white.
    MakeBW,
    /// Collate and generate final image.
    Collate,
}

pub struct Color{
    r: u8, 
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r:u8, g:u8, b:u8) -> Self {
        Self { r, g, b }
    }

    pub fn black_white(self) -> u8 {
        ((0.3 * self.r as f64) + (0.59 * self.g as f64) + (0.11 * self.b as f64)) as u8
    }
}

/// TODO: Remove public interface when necessary implements made.
pub struct Exec {
    task: Task,
    hash: u8,
    data: Vec<Color>,
}

impl Exec {
    pub fn new(
        task: Task,
        hash: u8,
        data: Vec<Color>,
    ) -> Self {
        Self { task, hash, data }
    }

    pub fn execute(self) -> Vec<u8> {
        match self.task {
            Task::MakeBW => {
                let mut bw_vec: Vec<u8> = vec![];
                for unit in self.data {
                    bw_vec.push(unit.black_white());
                }
                bw_vec
            },
            _ => todo!()
        }
    }
}