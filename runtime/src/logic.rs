use crate::{debug, error};
use std::collections::HashMap;

type UnitFunc = fn(&Logic, Vec<u8>) -> Result<u8, error::Error>;
pub struct Unit {
    input_len: usize,
    func: UnitFunc,
}
pub struct Logic {
    pub units: HashMap<String, Unit>,
}
impl Logic {
    pub fn new() -> Self {
        let mut units = HashMap::new();
        units.insert(
            String::from("nand"),
            Unit {
                input_len: 2,
                func: |_, input: Vec<u8>| -> Result<u8, error::Error> {
                    return Ok(!(input[0] == 1 && input[1] == 1) as u8);
                },
            },
        );
        Logic { units }
    }
    pub fn insert(
        &mut self,
        name: &str,
        input_len: usize,
        func: UnitFunc,
    ) -> Result<(), error::Error> {
        if self.units.contains_key(name) {
            return Err(error::Error::LogicError(format!(
                "Unit with this name {:#?} already exists",
                name
            )));
        }
        self.units
            .insert(name.to_string(), Unit { input_len, func });
        Ok(())
    }
    pub fn output<'a>(&'a self, name: &'a str, input: Vec<u8>) -> Result<u8, error::Error> {
        let unit = self.units.get(name).ok_or_else(|| {
            error::Error::LogicError(format!(
                "logic unit with the name {:#?} does not exist",
                name
            ))
        })?;
        if unit.input_len != input.len() {
            return Err(error::Error::LogicError(format!(
                "The number of provided inputs does not match the requested registered function"
            )));
        }
        return (unit.func)(self, input);
    }
}

pub fn test() -> Result<(), error::Error> {
    let mut logic = Logic::new();
    logic.insert("not", 1, |logic, input| {
        return logic.output("nand", vec![input[0], input[0]]);
    })?;
    logic.insert("and", 2, |logic, input| {
        let nand_output = logic.output("nand", vec![input[0], input[1]])?;
        return logic.output("not", vec![nand_output]);
    })?;
    logic.insert("or", 2, |logic, input| {
        let not_output_a = logic.output("not", vec![input[0]])?;
        let not_output_b = logic.output("not", vec![input[1]])?;
        return logic.output("nand", vec![not_output_a, not_output_b]);
    })?;

    debug::info(&logic.output("or", vec![0, 0])?.to_string());
    debug::info(&logic.output("or", vec![0, 1])?.to_string());
    debug::info(&logic.output("or", vec![1, 0])?.to_string());
    debug::info(&logic.output("or", vec![1, 1])?.to_string());

    Ok(())
}
