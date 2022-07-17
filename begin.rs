use std::io;
#[inline(always)]
pub fn greet_someone() {
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim();
    println!("It's nice to meet you, {name}!");
}


pub fn bubble_sort(array: &mut [i32]) {
    for i in 0..array.len()-1{
        let mut swapped: bool = false;
        for j in 0..array.len()-1-i{
            if array[j] > array[j+1]{
                let temp = array[j];
                array[j] = array[j+1];
                array[j+1] = temp;
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
        else {}
    }
}

pub fn increment(mut number: i32) -> i32 {
    number+=1;
    number
}

pub fn say_something(food: &mut String){
    println!("{} is my favorite food!", food);
}

