// const is always in UPPER_SNAKE_CASE
const REGION:&str = "us-east-1"; // global scope. Defining data type is mandatory

// This is a comment.
fn data_types() {
    println!("This is another function");
    let a = true;
    let c = 10 > 2;
    println!("a: {}", a);
    println!("c: {}", c);
}

fn array_data_type() {
    // array defining
    // arrays must be of homogenous data type
    let info_details = [1, 2, 3, 4]; // immutable array
    let mut bio_data = ["John", "NYC"]; // mutable array
    println!("The name is: {}", bio_data[0]);
    bio_data[1] = "FL";
    println!("The info of John: {} and city: {}", info_details[0], bio_data[1]);
    println!("length of the array: {}", bio_data.len());
    let arr = [1,2,3,4,5];
    let slice_arr = &arr[0..3];
    println!("The slice of the arr: {:?}", slice_arr);
}

fn tuple_data_type() {
    let mut person_data = ("John", 26, "Munich", true);
    println!("The person data: {:?}", person_data);
    let (w,x,y, z) = person_data;
    println!("{} stays in {} and he's {}years old", person_data.0, person_data.2,person_data.1);
    println!("{} stays in {} and he's {}years old and he's {} to his thoughts", w, y, x, z);
    person_data.0 = "Wick";
    person_data.2 = "NYC";
    println!("{} is the character. He's {}years old and lives in {}", person_data.0, person_data.1, person_data.2)
}

fn using_const() {
    const ID_1:i32 = 1021;
    println!("The id is {} and routing is from {}", ID_1, REGION) // local and global variables
}

fn main() {
    println!("Hello, world!");
    println!("This is my first Rust program !!");
    // The below line comments out a block of code.
    /*println!("Number is: {}", 1);*/ 
    let mut city = "OH"; // we should "mut" only if we intend to change it again.
    println!("I stay in {}", city);
    println!("{} is interested to learn {}", "Mark", "Rust");
    println!("{name} knows other programming languages like {language} as well", 
    name="Krishna", language = "Python");
    println!("Learning language :");
    print!("Rust"); // doesn't add new line character in end.
    print!("Programming");
    print!("Language");
    println!("");
    city = "NYC";
    println!("My work place is in {} too", city);
    // using multiple variables
    let (lang, proficiency) = ("Rust","amateur");
    println!("I'm {} in {} programming.", proficiency, lang);
    {
        // The variable in this block can only
        // be used here not in the above or below block.
        let profession = "SWE";
        println!("My profession is {}", profession);
    }
    // Trying to access the "profession" variable in this block
    // println!("My profession is {}", profession); // not found in this scope
    let outer_variable = "Neptune";
    {
        let inner_variable = "Venus";
        let outer_variable = "Uranus";
        println!("The block variable is: {}", inner_variable);
        println!("The block variable is: {}", outer_variable);
    }
    println!("The outer variable is: {}", outer_variable);
    data_types(); // The above function is called here.
    array_data_type();
    tuple_data_type();
    using_const();
}

// fn main() {
//     eprintln!("Error: Couldn't complete the task!");
// }