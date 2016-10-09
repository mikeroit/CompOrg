//main.rs for the convert32 assignment
// STRUCT -------------------------------------------------------------
#[derive(Debug)]
struct InputObj
{
    m_type: InputType,
    m_src: String
}

// -------------------------------------------------------------------



// ENUMS -------------------------------------------------------------
// enum to classify input
#[derive(Debug)]
enum InputType
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
#[allow(dead_code)]
fn parse_input(input: String) -> InputObj
{
    let mut head_0: char = '#';
    let mut head_1: char = '#';
    let mut need_to_reset: bool = false;

    //set up a return value
    let mut res: InputObj = InputObj
    {
        m_type: InputType::ERR,
        m_src: String::new()
    };

    for n in 0 .. input.len()
    {
        let c: char= input.chars().nth(n).unwrap();

        if n == 0 // first char is part of header
        {
            head_0 = c;
        }

        else if n == 1 // second char is part of header
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
            res.m_type = InputType::BIN;
        },

        // hex
        ('0', 'x') => 
        {
            res.m_type = InputType::HEX;
        }

        // other
        _ => 
        {
            need_to_reset = true;
            // floating point
            if input.contains('.')
            {
                res.m_type = InputType::FLT;
            }
            else 
            {
                res.m_type = InputType::INT;
            }
        }
    }

    if need_to_reset
    {
        res.m_src.clear();

        if head_0 != 'n' 
        {
            for n in 0..input.len() 
            {res.m_src.push(input.chars().nth(n).unwrap());}
        }
        else
        {
            res.m_src.push('-');
            for n in 1..input.len() 
            {res.m_src.push(input.chars().nth(n).unwrap());}
        }
    }

    // return
    res
}

// funtion to take a parsed input object and read the value to memory
#[allow(dead_code)]
fn extract_value(input: &InputObj) -> f32
{
    let mut res: f32 = 0.0;
    let chars_slice: &str  = &input.m_src;
    let mut chars = chars_slice.chars().rev();
    println!("{:?}", chars);


    let mut count = 0;
    match input.m_type 
    {
        InputType::BIN => 
        {

            while let Some(c) = chars.next()
            {
                //println!("{}", c);
                //println!("{}", count);
                match c
                {
                    '1' => 
                    {
                        res += exponentiate(2, count);
                    }

                    '0' => 
                    {
                        // do nothing
                    }

                    _ => 
                    {
                        println!("error --> character: {} is invalid", count);
                        println!("{}", c);
                    }

                    
                }
                println!("res: {}", res);
                count += 1;
            }

        }
        InputType::HEX => 
        {
            while let Some(c) = chars.next()
            {
                match c 
                {
                    'a' => 
                    {
                        res += 10.0 * exponentiate(16, count);
                    }
                    'b' => 
                    {
                        res += 11.0 * exponentiate(16, count);
                    }
                    'c' => 
                    {
                        res += 12.0 * exponentiate(16, count);
                    }
                    'd' => 
                    {
                        res += 13.0 * exponentiate(16, count);

                    }
                    'e' => 
                    {
                        res += 14.0 * exponentiate(16, count);

                    }
                    'f' => 
                    {
                        res += 15.0 * exponentiate(16, count);

                    }
                    '0' => 
                    {
                        res += 0.0 * exponentiate(16, count);
                    }
                    '1' => 
                    {
                        res += 1.0 * exponentiate(16, count);
                    }
                    '2' => 
                    {
                        res += 2.0 * exponentiate(16, count);
                    }
                    '3' => 
                    {
                        res += 3.0 * exponentiate(16, count);
                    }
                    '4' => 
                    {
                        res += 4.0 * exponentiate(16, count);
                    }
                    '5' => 
                    {
                        res += 5.0 * exponentiate(16, count);
                    }
                    '6' => 
                    {
                        res += 6.0 * exponentiate(16, count);
                    }
                    '7' => 
                    {
                        res += 7.0 * exponentiate(16, count);
                    }
                    '8' => 
                    {
                        res += 8.0 * exponentiate(16, count);
                    }
                    '9' => 
                    {
                        res += 9.0 * exponentiate(16, count);
                    }

                    _ => 
                    {
                        println!("Do not use caps for hex");
                        std::process::exit(0);
                    }
                }
                
                count = count + 1;
            }
        }

        InputType::FLT => 
        {
            res = chars_slice.parse().unwrap();
        }

        InputType::INT => 
        {
            res = chars_slice.parse().unwrap();
        }

        _ => 
        {
            println!("Not a valid InputType");

        }
    }
    res
}

//function for exponentiation
#[allow(dead_code)]
fn exponentiate(base: i32, power: i32) -> f32
{
    if power == 0 
    {
        return 1 as f32;
    }
    let mut res = base;
    for _ in 0..power-1
    {
        res *= base;
    }
    res as f32
}

// function to calculate how many bits we need to print without any leading zeros
fn calculate_number_bits_needed(num: i32) -> i32
{
    let mut res: i32 = 0;
    for i in 0..num
    {
        if (exponentiate(2, i) as i32) > num 
        {
            res = i;
            break;
        }
    }
    res
}

#[allow(dead_code)]
fn calculate_number_bits_needed_hex(num: i32) -> i32
{
    let mut res: i32 = 0;
    for i in 0..num
    {
        if (exponentiate(16, i) as i32) > num 
        {
            res = i;
            break;
        }
    }
    res
}

// function to send a string converting int to bin
#[allow(dead_code)]
fn int_to_bin(num: &i32) -> String
{
    // set a return val
    let mut res: String = String::from("0B");
    // set remainder to the starting value
    let mut remainder = *num;

    // find out how many bits we will need
    let num_bits = calculate_number_bits_needed(remainder);

    for i in 0..num_bits
    {
        let current_bit_val = exponentiate(2, (num_bits - i - 1));
        if current_bit_val <= remainder as f32
        {
            remainder -= current_bit_val as i32;
            res.push('1');
        }
        else
        {
            res.push('0');
        }
    }
    
    // return
    res
}

//function to send a string converting int ti hex
#[allow(dead_code)]
fn int_to_hex(num: &i32) -> String
{
    let hex_map = |n: i32|
    {
        match n
        {
            0 => return Some('0'),
            1 => return Some('1'),
            2 => return Some('2'),
            3 => return Some('3'),
            4 => return Some('4'),
            5 => return Some('5'),
            6 => return Some('6'),
            7 => return Some('7'),
            8 => return Some('8'),
            9 => return Some('9'),
            10 => return Some('A'),
            11 => return Some('B'),
            12 => return Some('C'),
            13 => return Some('D'),
            14 => return Some('E'),
            15 => return Some('F'),
            _  => return None
        }
    };

    // set a return val
    let mut res: String = String::from("0X");

    // set remainder to the starting value
    let mut remainder = *num;

    // find out how many bits we will need
    let num_bits = calculate_number_bits_needed_hex(remainder);

    for i in 0..num_bits
    {
        let current_bit_val = exponentiate(16, (num_bits - i - 1));

        // we will set this iteratively
        let mut val_to_push: i32 = 0;

        while current_bit_val <= remainder as f32
        {
            val_to_push += 1;
            remainder -= current_bit_val as i32;
        }

        res.push(hex_map(val_to_push).unwrap());
    }
    
    // return
    res

}

// function to decode a number int a string representation 
// in single precision floating point notation
fn float_to_single_precision(num: &f32) -> String
{

    // this will be needed for the exponent
    let offset = 127;

    // calculate what the exponent will need to be
    let mut fp_exponent: i32 = 0;
    loop
    {
        if exponentiate(2, fp_exponent) > *num
        {
            break;
        }
        fp_exponent += 1
    }
    fp_exponent -= 1;

    //calculate what the mantisa is
    let mut fp_mantisa: String = String::new();
    let mut temp_mant = *num/exponentiate(2, fp_exponent);
    assert!(temp_mant >= 1.0 && temp_mant < 2.0);
    temp_mant -= 1.0;

    for i in 0..24
    {
        temp_mant *= 2.0;
        if temp_mant > 0.0 
        {
            fp_mantisa.push('1');
            temp_mant -= 1.0;
        }
        else
        {
            fp_mantisa.push('0');
        }
    }

    let mut fp_sign: String = String::new();

    if *num >= 0.0
    {
        fp_sign.push('0');
    }
    else
    {
        fp_sign.push('1');
    }


    //return
    let real_exp: String = int_to_bin(&((&fp_exponent) + offset));

    fp_sign + &real_exp[2..] + &fp_mantisa
}

// function to set bit by index and value
fn set_number_sign(number: &i32, index: i32, value: i32) 
{
    //make sure the user didn't try to pass the mask by mistake
    assert!((value == 1) || (value == 0));

    //make a mask
    let mask: i32 = 0 | ((value) << index);

    //modify target
    let temp = (*number) | mask;
    let number = temp | mask;
}

// -------------------------------------------------------------------




// MAIN  -------------------------------------------------------------
#[allow(dead_code)]
fn main() 
{
    let input: InputObj = parse_input(std::env::args().nth(1).unwrap());
    let temp_val = extract_value(&input);

    match input.m_type 
    {
        InputType::BIN => 
        {
            println!("Recieved binary input\n");

            println!("Binary format: ");
            println!("{}\n", int_to_bin(&(temp_val as i32)));

            println!("Hexadecimal format: ");
            println!("{}\n", int_to_hex(&(temp_val as i32)));

            println!("Floating point format: ");
            println!("{}\n", float_to_single_precision(&(temp_val)));

            println!("Integer format: ");
            println!("{}\n", temp_val);
  
        }

        InputType::HEX => 
        {
            println!("Recieved hexadecimal input\n");

            println!("Binary format: ");
            println!("{}\n", int_to_bin(&(temp_val as i32)));

            println!("Hexadecimal format: ");
            println!("{}\n", int_to_hex(&(temp_val as i32)));

            println!("Floating point format: ");
            println!("{}\n", float_to_single_precision(&(temp_val)));

            println!("Integer format: ");
            println!("{}\n", temp_val);
        }

        InputType::FLT => 
        {
            println!("Recieved float input");

            println!("Binary format (rounded integer): ");
            println!("{}\n", int_to_bin(&(temp_val as i32)));

            println!("Hexadecimal format (rounded integer): ");
            println!("{}\n", int_to_hex(&(temp_val as i32)));

            println!("Floating point format: ");
            println!("{}\n", float_to_single_precision(&(temp_val)));

            println!("Integer format (rounded integer): ");
            println!("{}\n", temp_val as i32);

        }

        InputType::INT => 
        {
            println!("Recieved signed integer input");

            println!("Binary format: ");
            println!("{}\n", int_to_bin(&(temp_val as i32)));

            println!("Hexadecimal format: ");
            println!("{}\n", int_to_hex(&(temp_val as i32)));

            println!("Floating point format: ");
            println!("{}\n", float_to_single_precision(&(temp_val)));


        }
        _ => 
        {

        }
    }
}
// -------------------------------------------------------------------



// TESTS -------------------------------------------------------------
#[test]
#[allow(dead_code)]
fn test_exponentiate()
{
    let base = 10;
    let power = 0;

    assert_eq!(exponentiate(base, power), 1 as f32);
}
// -------------------------------------------------------------------
