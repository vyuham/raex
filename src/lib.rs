mod executor;
pub use executor::{Color, Exec, Task};

mod scheduler;

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn black_white_test() {
        let image: Vec<Color> = vec![Color::new(163, 200, 255)];
        let expect: Vec<u8> = vec![194.95f64 as u8];
        let exec = Exec {
            task: Task::MakeBW,
            hash: 0,
            data: Some(image)
        };
        assert_eq!(exec.execute(), expect);
    }
}
