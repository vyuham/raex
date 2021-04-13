#[macro_use]
extern crate async_trait;

use raex::{Consensus, RaEx, RaExConfig, Scheduler};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    sync::Arc,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Task {
    /// Divides up the image into equal sized chunks of u8 matrices.
    Divide,
    /// Convert color u8 values into black and white.
    MakeBW,
    /// Collate and generate final image.
    Collate,
    /// Waiting for all sub-processes created to be completed.
    Wait,
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
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn black_white_shade(self) -> Self {
        let shade = (0.3 * self.r as f64 + 0.59 * self.g as f64 + 0.11 * self.b as f64) as u8;
        Self {
            r: shade,
            g: shade,
            b: shade,
        }
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
pub struct ExecUnit {
    /// The type of processing to be applied on data.
    pub task: Task,
    /// UID used to refer to an execution unit.
    /// TODO: Write code to generate the same.
    pub hash: u64,
    /// Data values to be processed.
    pub data: Data,
}

impl ExecUnit {
    pub fn new(task: Task, data: Data) -> Self {
        let mut hasher = DefaultHasher::new();
        task.description().hash(&mut hasher);
        Self {
            task,
            data,
            hash: hasher.finish(),
        }
    }

    pub fn execute(&self) -> Vec<Self> {
        match self.task {
            Task::Divide => {
                let mut lines: Vec<Self> = vec![];
                if let Data::Image(image) = &self.data {
                    for line_no in 0..image.len() {
                        // TODO: Store the lines of pixels in a global-datastrucutre that can later process it within the cluster.
                        lines.push(Self::new(
                            Task::MakeBW,
                            Data::Line((*image[line_no]).to_vec()),
                        ));
                    }
                }
                lines
            }
            Task::MakeBW => {
                let mut bw_line: Vec<Color> = vec![];
                if let Data::Line(line) = &self.data {
                    for pixel in line {
                        bw_line.push(pixel.black_white_shade());
                    }
                }
                // TODO: Store the processed values in a global-datastrucutre that can later collate it into the desired output.
                vec![Self::new(Task::Collate, Data::Line(bw_line))]
            }
            _ => vec![],
        }
    }
}

struct RenderState<T> {
    consensus: Consensus<T>,
    config: Arc<RaExConfig>,
}

impl<T: Send + Sync + 'static> RenderState<T> {
    async fn new(config: Arc<RaExConfig>) -> Self {
        Self {
            consensus: Consensus::<T>::start(config.local_addr.clone(), config.nodes.clone())
                .await
                .unwrap(),
            config,
        }
    }
}

#[async_trait]
impl<T: Send + Sync + 'static> Scheduler<T> for RenderState<T> {
    async fn add(&mut self, next: T) {
        self.consensus.schedule(next).await;
    }

    async fn next(&mut self) {
        self.consensus.next().await;
    }

    async fn execute(&mut self) {
        unimplemented!()
    }

    async fn current(&mut self) -> T {
        unimplemented!()
    }
}

#[tokio::main]
async fn main() {
    let cfg = Arc::new(RaExConfig::new("examples/raex").unwrap());
    let mut raex =
        RaEx::<ExecUnit>::start(cfg.clone(), Box::new(RenderState::new(cfg).await)).await;

    raex.run().await;
}
