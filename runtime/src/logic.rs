use crate::{debug, error};

// pub struct Unit {}
// impl Unit {
//     pub fn input(&mut self, ) -> &mut Self {
//         return self;
//     }
// }
//
// pub struct Logic {}
// impl Logic {
//     pub fn nand() -> Unit {
//     }
// }

pub fn logic_test() -> Result<(), error::Error> {
    let not_gate = Logic::nand()
        .input(Logic::input("a"))
        .input(Logic::input("a"));
    let and_gate = not_gate.input(
        Logic::nand()
            .input(Logic::input("a"))
            .input(Logic::input("b")),
    );
    let or_gate = Logic::nand()
        .input(not_gate.input(Logic::input("a")))
        .input(not_gate.input(Logic::input("b")));

    debug::info("testing logic...");
    Ok(())
}
