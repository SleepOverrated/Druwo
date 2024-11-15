use crate::values::Value;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    PUSH(Value),
    POP,

    DUP,
    SWAP,
    SWAPxxxxxx,

    INC,
    DEC,
    ADD,
    SUB,
    MUL,
    DIV,


    //TODO: get a good idea for jump targets
    JMP(String),
    JE(String),
    JNE(String),
    JG(String),
    JGE(String),
    JL(String),
    JLE(String),


    CALL(String),
    RETURN,


    PRINT,
    STOP,
    NOP,
}

