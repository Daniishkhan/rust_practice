// This program outputs sum of all even numbers in an array

fn even_display(numbers: &Vec<i32>) -> i32 {
    let mut total = 0;
    for items in numbers {
        if (items % 2 == 0) {
            total += items
        }
    }
    total
}

fn main() {
    let list = vec![14, 6, 5, 6, 2, 3, 2];
    let total = even_display(&list);
    println!("The total is {}", total)
}
