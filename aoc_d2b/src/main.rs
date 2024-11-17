use std::fs;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path: &String = &args[1];
    let content = fs::read_to_string(file_path)                        //reads the file into memory
                     .expect("Should have been able to read the file!"); //Provides an error statement should read to string fail.

    let mut game_total = 0;
    for lines in content.lines()
    {
        let game_info: Vec<&str> = lines.split(":").collect();

        let mut reds = Vec::new();
        let mut blues = Vec::new();
        let mut greens = Vec::new();


        for turns in game_info[1].split(";")
        {
           
            for draws in turns.split(",")
            {
                let draw_info: Vec<&str> = draws.trim().split(" ").collect();
                
                let color = draw_info[1];
                let val = draw_info[0].parse::<i32>().unwrap();

                if color == "red"
                {
                    reds.push(val);
                }
                if color == "blue"
                {
                    blues.push(val);
                }
                if color == "green"
                {
                    greens.push(val);
                }
            }
        }

        let mut set_power = 1;

        let min_red = reds.iter().max();
        
        match min_red {
            Some(min) => {println!("Min red: {min}");
                                set_power = set_power * min},
            None      => set_power = set_power * 1,
        };

        let min_blue = blues.iter().max();
        match min_blue {
            Some(min) => {println!("Min blue: {min}");
                                set_power = set_power * min},
            None      => set_power = set_power * 1,
        };

        let min_green = greens.iter().max();
        match min_green {
            Some(min) => {println!("Min green: {min}");
                                set_power = set_power * min},
            None      => set_power = set_power * 1,
        };
        
       
        println!("Set power: {set_power}");
        game_total += set_power;
    }

    println!("Sum of IDs: {game_total}")
}
