use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect(); //Take an the iterator provided by env::args and covert it to a Vector of strings
                                                 
    let file_path: &String = &args[1]; //The first value in the aregs will be the name of the binary so start counting at 1
    
    println!("The file path is {file_path}");

    let file_sting = fs::read_to_string(file_path)                        //reads the file into memory
                     .expect("Should have been able to read the file!"); //Provides an error statement should read to string fail.
    let contents: String = words_to_nums(file_sting);
                    
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

fn words_to_nums(content: String) -> String{
    let mut _new_content= String::from("");

    for line in content.lines(){
        println!("Old Line = {line}");
        let mut _c = 0;
        let mut _last_number = 0;
        let mut _new_line = String::from("");
        while _c  <= line.len()
        {
            if line[_last_number.._c].contains("one"){
                _new_line.push('1');
                _last_number = _c-1;
            }
            if line[_last_number.._c].contains("two")
            {
                _new_line.push('2');
                _last_number = _c-1;
            }
            if line[_last_number.._c].contains("three")
            {
                _new_line.push('3');
                _last_number = _c-1;
            }
            if line[_last_number.._c].contains("four")
            {
                _new_line.push('4');
                _last_number = _c-1;
            }
            if line[_last_number.._c].contains("five")
            {
                _new_line.push('5');
                _last_number = _c-1;
            }
            if line[_last_number.._c].contains("six")
            {
                _new_line.push('6');
                _last_number = _c-1;
            }
            if line[_last_number.._c].contains("seven")
            {
                _new_line.push('7');
                _last_number = _c-1;
            }
            if line[_last_number.._c].contains("eight")
            {
                _new_line.push('8');
                _last_number = _c-1;
            }
            if line[_last_number.._c].contains("nine")
            {
                _new_line.push('9');
                _last_number = _c-1;
            }
            if line[_last_number.._c].contains("1"){
                _new_line.push('1');
                _last_number = _c;
            }
            if line[_last_number.._c].contains("2")
            {
                _new_line.push('2');
                _last_number = _c;
            }
            if line[_last_number.._c].contains("3")
            {
                _new_line.push('3');
                _last_number = _c;
            }
            if line[_last_number.._c].contains("4")
            {
                _new_line.push('4');
                _last_number = _c;
            }
            if line[_last_number.._c].contains("5")
            {
                _new_line.push('5');
                _last_number = _c;
            }
            if line[_last_number.._c].contains("6")
            {
                _new_line.push('6');
                _last_number = _c;
            }
            if line[_last_number.._c].contains("7")
            {
                _new_line.push('7');
                _last_number = _c;
            }
            if line[_last_number.._c].contains("8")
            {
                _new_line.push('8');
                _last_number = _c;
            }
            if line[_last_number.._c].contains("9")
            {
                _new_line.push('9');
                _last_number = _c;
            }
            _c += 1;
        }

        _new_line.push('\n');
        println!("New Line = {_new_line}");
        _new_content.push_str(&_new_line);
    }

    return _new_content;
}