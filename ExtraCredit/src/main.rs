//main.rs for the convert32 assignment

use std::env;

// STRUCT -------------------------------------------------------------
struct InputObject
{
    m_value: i32,
    m_type: InputType
}
// -------------------------------------------------------------------



// ENUMS -------------------------------------------------------------
// enum to classify input
enum InputType
{
    BIN,
    DEC,
    FLT,
    INT
}
// -------------------------------------------------------------------



// FUNCS -------------------------------------------------------------
// function to parse input
//fn ParseInput
// -------------------------------------------------------------------
fn main() 
{
    // grab input
    let input = std::env::args().nth(0).unwrap();
    
    println!("{}",input);

}
