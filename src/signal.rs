use super::trace;

pub struct Signal<T: Copy + PartialEq + Default + std::fmt::Display + trace::Trace> {
    cur_val:T,
    new_val:T
}

impl<T: Copy + PartialEq + Default + std::fmt::Display + trace::Trace> Signal<T> {
    fn read(&self) -> &T {
        &self.cur_val
    }

    fn write(&mut self, v:&T) {
        self.new_val = *v;
    }

    fn update(&mut self) {
        self.cur_val = self.new_val;
    }
}
