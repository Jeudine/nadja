pub mod module;
pub mod process;
pub mod signal;
pub mod simulator;
pub mod trace;
//pub mod clock;

pub use crate::signal::Signal;
pub use crate::simulator::Update;

#[cfg(test)]
mod tests {
    use crate::signal::Signal;
    #[test]
    fn it_works() {
        let mut s: Signal<u32> = Default::default();
        let a: u32 = 0;
        s.write(a);
        let x = s.read();
        let y = s.read();
        println!("This is a test: {}", x);
        println!("{}", y);
        println!("{}", a);
        assert_eq!(2 + 2, 4);
    }
}
