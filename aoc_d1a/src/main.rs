use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect(); //Take an the iterator provided by env::args and covert it to a Vector of strings
                                                 
    let file_path: &String = &args[1]; //The first value in the aregs will be the name of the binary so start counting at 1
    
    println!("The file path is {file_path}");

    let contents: String = fs::read_to_string(file_path)                        //reads the file into memory
                    .expect("Should have been able to read the file!"); //Provides an error statement should read to string fail. 

    let mut _all_numbers: Vec<String> = Vec::new(); //declate a mutable vector
    let mut _line_number: String = String::from(""); //declare a mutable string with a blank string to start

    for line in contents.lines() {
        let mut _first_number = ' '; //declare a mutable char 
        let mut _last_number = ' '; //declare a mutable char

        for character in line.chars() { //loop through the line starting at the front
            if character >= '0' && character <= '9'
            {
                _first_number = character; //when you reach the first number save it 
                break; //break the loop
            }
        }

        for character in line.chars().rev(){ //loop through the line start at the back
            if character >= '0' && character <= '9'
            { 
                _last_number = character; //when you reach the first number save it
                break; //break the loop
            }
        }

        _line_number.push(_first_number); //add the first number to the new string
        _line_number.push(_last_number); //add the second number to the new string
        
        println!("Line number: {_line_number}"); //pring the line number
        _all_numbers.push(_line_number); //add the line number to the vector for later use
        _line_number = String::from(""); 
    }

    let mut _total = 0; //declare a mutable integer

    for item in _all_numbers{
        _total += item.parse::<i32>().unwrap() //loop through and all the numbers together
    }

    println!("Total: {_total}") //print the total
}
