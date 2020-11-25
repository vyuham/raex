mod executor;
pub use executor::{Color, Exec, Task};

mod scheduler;

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn black_white_test() {
        let image: Vec<Color> = vec![Color::new(163, 200, 255)];
        let s = 194.95f64 as u8;
        let expect: Vec<Color> = vec![Color::new(s, s, s)];
        let mut exec = Exec::new(Task::MakeBW, 0, image);
        exec.execute();
        assert_eq!(exec.data(), expect);
    }
}
