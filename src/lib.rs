pub mod clock;
pub mod process;
pub mod signal;
pub mod simulator;
pub mod trace;

pub use crate::signal::Signal;
pub use crate::simulator::Update;

#[cfg(test)]
mod tests {
    use crate::signal::Signal;
    use crate::simulator::Simulator;

    #[test]
    fn it_works() {
        let mut sim: Simulator = Default::default();
        let s1: Signal<bool> = Default::default();
        let s2: Signal<bool> = Default::default();
        let s3: Signal<bool> = Default::default();
    }
}
