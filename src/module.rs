use crate::interface::TValue;
use crate::signal::Signal;
use crate::simulator::Simulator;

pub trait Module {
    fn route() -> Self;
}


//TODO
//
//parameter
//
//input
// & dyn Channel
//
//output
// & dyn Channel
//
//comb
//  One comb struct (derive new and implements Channel) for each comb signals(= Regs input & output)
//  For the variables, automatically call read function
//
//process
// & dyn Channel -> Reg -> Signals
//
//clk & rst
