mod executor;
pub use executor::{Color, Exec, Task, Data};

mod scheduler;

#[cfg(test)]
mod tests {
    use crate::*;
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
