pub mod nj_signal;
pub mod nj_trace;
mod nj_simulator;

#[cfg(test)]
mod tests {
    use crate::nj_signal::Signal;

    #[test]
    fn it_works() {
        let mut s:Signal<u32> = Default::default();
        s.write(&0);
        let x = s.read();
        println!("{}", x);
        assert_eq!(2 + 2, 4);
    }
}
