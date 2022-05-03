// fn main() {
//     let number_list = vec![34, 50, 25, 100, 65];
//     let mut largest = number_list[0];

//     for number in number_list {
//         if number > largest {
//             largest = number;
//         }
//     }

//     println!("largest number: {}", largest);
// }

// fn largest(list : &[i32]) -> i32 {
//     let mut largest = list[0];

//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//     largest 
// }

// fn largest<T>(list : &[T]) -> T {
fn largest<T: std::cmp::PartialOrd + Copy>(list : &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest    
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("largest number: {}", result);
}