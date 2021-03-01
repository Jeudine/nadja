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
    use std::cell::RefCell;
    use std::cell::Cell;

    fn process(sim: &mut Simulator) {

    }

    #[derive(Default)]
    struct Module {
        pub s: RefCell<Signal<u32>>,
        pub t: RefCell<Signal<u32>>,
    }

    impl Module {
        fn process<'a>(&'a self, sim: &mut Simulator<'a>) {
            //self.s.write(0, sim);
        }

        fn func(& self, x:u32) {
            println!("{}", x);
        }

    }

    #[derive(Default)]
    struct Sig {
        pub fut: Cell<u32>,
        pub cur: Cell<u32>,
    }

    #[test]
    fn it_works() {
        let mut sim: Simulator = Default::default();
        //let sim = &mut sim;
        let mut m: Module = Default::default();
        //m.s.get_mut().write(0, &mut sim);
        //m.t.write(0, &mut sim);
        let p = |x, y| {Module::process(x, y);};
        //p(& m, &mut sim);
        //let p: Always = Always::new(|x, y| {Module::process(x, y);});
        //let k = &mut m;
        //let p: Always = Always::new(|x| m.process(x));
        let sig: Sig = Default::default();
        let fut = sig.fut.set(7);
        assert_eq!(2 + 2, 4);
    }
}
