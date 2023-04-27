use std::{fs::File, io::{BufReader, BufRead}};

fn main() {
    let food_list_file = File::open("food_list.txt")
        .expect("Expected food_list.txt file in project");
    let mut most_calories = 0;
    let mut count_calories = 0;
    let food_items = BufReader::new(food_list_file).lines();
    for item in food_items {
        let food = item.unwrap();
        if !food.is_empty() {
            count_calories += food.parse::<i32>().unwrap();
        } else {
            if count_calories > most_calories {
                most_calories = count_calories;
            }
            count_calories = 0;
        }
    }
    print!("{}", most_calories);
}
