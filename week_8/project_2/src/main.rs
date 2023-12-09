fn main() {
    let ages = vec![45, 64, 23, 53, 37, 24, 33, 40, 47, 55];
    fn findHeighest(vector) {
        let mut test:i32 = 0;
        for i in 0..ages.len() {
            if ages[i] > test {
                test =  ages[i];
            }
        }
        println!("{}", test);
    }
}


// fn main() {
//     let ages = vec![45, 64, 23, 53, 37, 24, 33, 40, 47, 55];
    
//     fn find_highest(vector: &[i32]) {
//         let mut test: i32 = 0;
//         for &age in vector {
//             if age > test {
//                 test = age;
//             }
//         }
//         println!("{}", test);
//     }

//     find_highest(&ages);
// }