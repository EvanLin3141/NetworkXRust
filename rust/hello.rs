fn main() {
    // let x: i32 = 42;
    // let y: u64 = 100;
    // println!("Signed int: {}", x);
    // println!("Signed int: {}", y);

    // Compound data types
    // arrays, tuples, slices and strings (slice string)

    let numbers: [i32; 5] = [1,2,3,4,5];
    println!("Numbers Array: {:?}", numbers);

    // Tuples
    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human Tuples: {:?}", human);

    let mix_tuple = ("Kratos", 32, true, [1,2,3,4,5]);
    println!("My {:?}", mix_tuple);

    // Slices: 
    let number_slices :&[i32] = &[1,2,3,4,5];
    println!("Number Slices: {:?}", number_slices);
    let animal_slices :&[&str] = &["Lion", "Elephant", "Croc"];
    println!("Number Slices: {:?}", animal_slices);
    let book_slices :&[&String] = &[&"Lion".to_string(), &"Elephant".to_string(), &"Croc".to_string()];
    println!("Number Slices: {:?}", book_slices);

    let weight: f64 = 70.00;
    let height: f64 = 1.82;
    // strings vs string slices:
    let bmi1: f64 = bmi(weight, height);
    println!("BMI is {:.2}", bmi1);
}

fn bmi(weight: f64, height: f64) -> f64 {
    weight / (height * height)
}