pub mod instuction;
pub mod compiler;
pub mod values;
pub mod test;
pub mod vm;

use test::*;

fn main() {

    test::run_tests();

}
