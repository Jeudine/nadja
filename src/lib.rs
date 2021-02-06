pub mod signal;
pub mod trace;
mod simulator;

pub use crate::signal::Signal;

#[cfg(test)]
mod tests {
    use crate::signal::Signal;
    #[test]
    fn it_works() {
        let mut s:Signal<u32> = Default::default();
        s.write(&0);
        let x = s.read();
        println!("{}", x);
        assert_eq!(2 + 2, 4);
    }
}
