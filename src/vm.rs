use core::panic;
use std::collections::HashMap;

use crate::instuction::*;
use crate::values::*;


#[derive(Debug)]
pub struct VM {
    stack: Vec<Value>,
    instructions: Vec<Instruction>,
    labels: HashMap<String, usize>,
    return_ptrs: Vec<usize>,

    pc: usize
}


impl VM {

    pub fn new(instructions: Vec<Instruction>, labels: HashMap<String, usize>) -> VM {
        VM {
            stack: Vec::new(),
            return_ptrs: Vec::new(),

            pc: 0,

            instructions,
            labels,

        }
    }

    pub fn run(&mut self) {
        self.run_label("main");
    }

    pub fn run_label(&mut self, name: &str) {
        self.pc = *self.labels.get_key_value(name).expect("Couln't find label: {*name}").1; 

        while self.instructions[self.pc] != Instruction::RETURN  {
            match &self.instructions[self.pc] {

                Instruction::PUSH(val) => self.stack.push(val.clone()),
                Instruction::POP => { let _ = self.stack.pop(); },

                Instruction::DUP => {
                    let val = self.stack.pop().unwrap();
                    self.stack.push(val.clone());
                    self.stack.push(val.clone());
                }
                Instruction::SWAP => {
                    let len = self.stack.len();
                    self.stack.swap(len - 1, len - 2);
                }

                

                Instruction::INC => {
                    let tmp = self.stack.pop().unwrap().add(&Value::Int(1));
                    self.stack.push(tmp);             
                }
                Instruction::DEC => {
                    let tmp = self.stack.pop().unwrap().sub(&Value::Int(1));
                    self.stack.push(tmp);             
                }
                Instruction::ADD => {
                    let tmp = self.stack.pop().unwrap().add(&self.stack.pop().unwrap());
                    self.stack.push(tmp);  
                }
                Instruction::SUB => {
                    let tmp = self.stack.pop().unwrap().sub(&self.stack.pop().unwrap());
                    self.stack.push(tmp); 
                }
                Instruction::MUL => {
                    let tmp = self.stack.pop().unwrap().mul(&self.stack.pop().unwrap());
                    self.stack.push(tmp);  
                }
                Instruction::DIV => {
                    let tmp = self.stack.pop().unwrap().div(&self.stack.pop().unwrap());
                    self.stack.push(tmp);  
                }



                Instruction::JMP(s) => {
                    self.pc = *self.labels.get(s).unwrap();
                    continue;
                }
                Instruction::JE(s) => {
                    if self.stack.pop() == self.stack.pop() {
                        self.pc = *self.labels.get(s).unwrap();
                        continue;
                    }
                }
                Instruction::JNE(s) => {
                    if self.stack.pop() != self.stack.pop() {
                        self.pc = *self.labels.get(s).unwrap();
                        continue;
                    }
                }
                Instruction::JG(s) => {
                    if self.stack.pop().unwrap().greater(&self.stack.pop().unwrap()) {
                        self.pc = *self.labels.get(s).unwrap();
                        continue;
                    }
                }
                Instruction::JGE(s) => {
                    if self.stack.pop().unwrap().greater_equal(&self.stack.pop().unwrap()) {
                        self.pc = *self.labels.get(s).unwrap();
                        continue;
                    }
                }
                Instruction::JL(s) => {
                    if self.stack.pop().unwrap().less(&self.stack.pop().unwrap()) {
                        self.pc = *self.labels.get(s).unwrap();
                        continue;
                    }
                }
                Instruction::JLE(s) => {
                    if self.stack.pop().unwrap().less_equal(&self.stack.pop().unwrap()) {
                        self.pc = *self.labels.get(s).unwrap();
                        continue;
                    }
                }


                Instruction::CALL(s) => {
                    self.return_ptrs.push(self.pc.clone());
                    self.pc = *self.labels.get(s).unwrap();
                    continue;
                }

                Instruction::RETURN => {
                    if !self.return_ptrs.is_empty() {
                        self.pc = self.return_ptrs.pop().unwrap()
                    }else {
                        break;
                    }
                }
               

                Instruction::PRINT => {
                    println!("{:?}", self.stack.pop().unwrap()); 
                }

                _ => panic!("Not yet implemented for some reason")
            }

            self.pc += 1;

        } 
        
    }

}
