use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    let food_list_file = File::open("food_list.txt")
        .expect("Expected food_list.txt file in project");
    let mut calorie_list = Vec::new();
    let food_items = BufReader::new(food_list_file).lines();
    let mut count_calories = 0;
    for item in food_items {
        let food = item.unwrap();
        if !food.is_empty() {
            count_calories += food.parse::<i32>().unwrap();
        } else {
            calorie_list.push(count_calories);
            count_calories = 0;
        }
    }
    let mut top_3_calories: [i32; 3] = [0, 0, 0];
    for cal in calorie_list {
        if cal >= top_3_calories[0] {
            top_3_calories[2] = top_3_calories[1];
            top_3_calories[1] = top_3_calories[0];
            top_3_calories[0] = cal;
        } else if cal >= top_3_calories[1] {
            top_3_calories[2] = top_3_calories[1];
            top_3_calories[1] = cal;
        } else if cal > top_3_calories[2] {
            top_3_calories[2] = cal;
        }
    }
    print!("{}", top_3_calories.iter().sum::<i32>());
}
