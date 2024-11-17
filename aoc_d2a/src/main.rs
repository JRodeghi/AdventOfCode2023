use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let max_red = 12;
    let max_blue = 14;
    let max_green = 13;

    let file_path: &String = &args[1];
    let content = fs::read_to_string(file_path)                        //reads the file into memory
                     .expect("Should have been able to read the file!"); //Provides an error statement should read to string fail.

    let mut game_total = 0;

    for lines in content.lines()
    {
        let mut possible = true;
        let game_info: Vec<&str> = lines.split(":").collect();
        let game_str: Vec<&str> = game_info[0].split(" ").collect();
        let game_number = game_str[1].parse::<i32>().unwrap();

        for turns in game_info[1].split(";")
        {
           
            for draws in turns.split(",")
            {
                let draw_info: Vec<&str> = draws.trim().split(" ").collect();
                
                let color = draw_info[1];
                let val = draw_info[0].parse::<i32>().unwrap();

                if color == "red"
                {
                    if val > max_red
                    {
                        possible = false;
                    }
                }
                if color == "blue"
                {
                    if val > max_blue
                    {
                        possible = false;
                    }
                }
                if color == "green"
                {
                    if val > max_green
                    {
                        possible = false;
                    }
                }
            }
        }

        if possible
        {
            println!("Game number: {game_number}");
            game_total += game_number;
        }
    }

    println!("Sum of IDs: {game_total}")

}
