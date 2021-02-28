pub mod module;
pub mod process;
pub mod signal;
pub mod simulator;
pub mod trace;
pub mod clock;

pub use crate::signal::Signal;
pub use crate::simulator::Update;

#[cfg(test)]
mod tests {
    use crate::signal::Signal;
    use crate::simulator::Simulator;
    use crate::process::always::Always;

    fn process(sim: &mut Simulator) {

    }

    #[derive(Default)]
    struct Module {
        s: Signal<u32>,
    }

    impl Module {
        fn process<'a>(&'a mut self, sim: &mut Simulator<'a>) {
            //self.s.write(0, sim);
        }

        fn func(& self, x:u32) {
            println!("{}", x);
        }

    }

    #[test]
    fn it_works() {
        let mut sim: Simulator = Default::default();
        //let sim = &mut sim;
        let mut m: Module = Default::default();
        let p = |x, y| {Module::process(x, y);};
        p(&mut m, &mut sim);
        //p(3333333333);
        //let p: Always = Always::new(|x| m.process(x));
        let mut s: Signal<u32> = Default::default();
        let mut t: Signal<u32> = Default::default();
        let a: u32 = 0;
        let b: u32 = 0;
        s.write(a, &mut sim);
        t.write(b, &mut sim);
        let x = s.read();
        let y = s.read();
        println!("This is a test: {}", x);
        println!("{}", y);
        println!("{}", a);
        assert_eq!(2 + 2, 4);
    }
}
