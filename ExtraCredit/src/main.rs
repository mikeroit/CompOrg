//main.rs for the convert32 assignment

use std::env;

// STRUCT -------------------------------------------------------------
struct input_obj
{
    m_value: i32,
    m_type: input_type,
    m_src: String
}
impl input_obj
{
}
// -------------------------------------------------------------------



// ENUMS -------------------------------------------------------------
// enum to classify input
#[derive(Debug)]
enum input_type
{
    ERR,
    BIN,
    HEX,
    FLT,
    INT
}
// -------------------------------------------------------------------



// FUNCS -------------------------------------------------------------
// function to parse input
fn parse_input(input: String) -> input_obj
{
    let mut head_0: char = '#';
    let mut head_1: char = '#';
    let mut needToResetSrc: bool = false;

    //set up a return value
    let mut res: input_obj = input_obj
    {
        m_value: -1,
        m_type: input_type::ERR,
        m_src: String::new()
    };

    for n in 0 .. input.len()
    {
        let c: char= input.chars().nth(n).unwrap();

        if(n == 0) // first char is part of header
        {
            head_0 = c;
        }

        else if(n == 1) // second char is part of header
        {
            head_1 = c;
        }

        else // char is part of the value
        {
            res.m_src.push(c);
        }
    }

    // classify
    match(head_0, head_1)
    {
        // binary
        ('0', 'b') => 
        {
            res.m_type = input_type::BIN;
        },

        // hex
        ('0', 'x') => 
        {
            res.m_type = input_type::HEX;
        }

        // other
        _ => 
        {
            needToResetSrc = true;
            // floating point
            if(input.contains('.'))
            {
                res.m_type = input_type::FLT;
            }
            else 
            {res.m_type = input_type::INT;}
        }
    }

    if(needToResetSrc)
    {
        res.m_src.clear();
        for n in 0..input.len() 
        {res.m_src.push(input.chars().nth(n).unwrap());}
    }

    println!("{}", res.m_src);
    println!("{:?}", (res.m_type));

    res
}
// -------------------------------------------------------------------



// MAIN  -------------------------------------------------------------
fn main() 
{
    // grab input
    let input = std::env::args().nth(1).unwrap();

    parse_input(input);
}
// -------------------------------------------------------------------



// TESTS -------------------------------------------------------------
#[test]
fn test_inputs()
{
    assert_eq!(1, 1);
}
// -------------------------------------------------------------------
