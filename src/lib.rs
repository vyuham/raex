mod executor;
pub use executor::{Color, Exec, Task, Data};

mod scheduler;

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn black_white_test() {
        let image = Data::Line(vec![Color::new(163, 200, 255)]);
        let s = 194.95f64 as u8;
        let expect = Data::Line(vec![Color::new(s, s, s)]);
        let mut exec = Exec::new(Task::MakeBW, 0, image);
        exec.execute();
        assert_eq!(exec.data(), expect);
    }
}
