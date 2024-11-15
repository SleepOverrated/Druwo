use std::{collections::HashMap, usize};

use crate::instuction::Instruction;
use crate::values::*;
use crate::vm::*;

pub fn run_tests() {

    let mut test1_map: HashMap<String, usize> = HashMap::new();
    test1_map.insert("main".to_string(), 0);
    let _test1 = 
    [
        Instruction::PUSH(Value::Int(1)),
        Instruction::PUSH(Value::Int(2)),
        Instruction::ADD,
        Instruction::PUSH(Value::Str("Result: ".to_string())),
        Instruction::ADD,
        Instruction::PRINT,
        Instruction::RETURN
    ].to_vec();



    let mut test2_map: HashMap<String, usize> = HashMap::new();
    test2_map.insert("main".to_string(), 0);
    test2_map.insert("start".to_string(), 1);
    test2_map.insert("end".to_string(), 10);
    let _test2 = [

        Instruction::PUSH(Value::Int(10)),
        Instruction::DUP,
        Instruction::PUSH(Value::Int(0)),
        Instruction::JE("end".to_string()),
        Instruction::DUP,
        Instruction::PUSH(Value::Str("Current: ".to_string())),
        Instruction::ADD,
        Instruction::PRINT,
        Instruction::DEC,
        Instruction::JMP("start".to_string()),
        Instruction::PUSH(Value::Str("Done with loop!".to_string())),
        Instruction::PRINT,
        Instruction::RETURN

    ].to_vec();


    let mut vm = VM::new(
        _test2,
        test2_map
    );

    vm.run();

}
