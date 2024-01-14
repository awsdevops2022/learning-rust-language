/*
const is always in UPPER_SNAKE_CASE
*/

/*Expressions do not include ending semicolons. 
If you add a semicolon to the end of an expression, 
you turn it into a statement, and it will then not return a value*/

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

fn working_on_operators() {
    let a = 2;
    let b = 4;
    println!("Addition: {}", a + b);
    println!("Multiplication: {}", a * b);
    println!("Div: {}", b / a);
    println!("a < b: {}", a < b);
    println!("a >= b: {}", a >= b);
    println!("a != b: {}", a != b);
}

fn logical_operators() {
    let a = true;
    let b = false;
    println!("AND: {}", a && b);
    println!("OR: {}", a || b);
    println!("NOT: {}", !b);
}

// Type casting
fn type_casting() {
    let a = 4;
    let b = 2.0;
    let c = a as f32 / b;
    println!("The value of c is: {}", c);
}

fn borrow_and_dereferencing() {
    let a = 10;
    let mut b = 4;
    let x = &a; // shared borrowing. We can't alter the data value.
    let y = & mut b; // mutable borrowing. We can alter the data value.
    println!("The value of x:{} and y:{}", x, y);
    *y = 6;
    println!("The value of y:{}", y);
    println!("The value of b:{} after dereferencing", b);
}

fn precedence_and_associativity() {
        println!("{}", 3 + 4 - 9 / 6 * 6 ^ 8 & 3);
}

fn conditional_expressions() {
    // if, if let and match
    let lang = "Rust";
    if lang == "Rust" {
        println!("The language is: {}", lang);
        let another_lang = "Python";
        if another_lang == "Python" {
            println!("Another language is: {}", another_lang)
        }
    }
    else if lang == "Java" {
        println!("He knows {}", lang);
    }
    else {
        println!("He has to learn any one programmming language!");
    }
    {
        //define a variable 
        let learn_language1 = "Rust";
        let learn_language2 = "Java";
        // outer if statement
        if learn_language1 == "Rust" {  // inner if statement
            if learn_language2 == "Java"{
                  println!("You are learning Rust and Java language!");
            }
        }
        else {
          println!("You are learning some other language!");
        } 
    }
    {
        let name="RUST"; 
        if name.len() >= 4 { 
            println!("The language is RUST !!");
        }
        else if name.len() <= 2 {
        println!("The language is Go or C# or C or R");
        }
        println!("Value Printed");
    }
    {
        let age=23; 
        let play=true; 
        let activity="Football" ;
        if age >=21 && play==false && activity=="Tennis"{ 
            println!("Age is greater than 21");
            println!("You are not allowed to play");
            println!("The sport is {}",activity);
        }
        else if  age >=21 && play==true && activity=="Football"{ 
            println!("Age is greater than 21");
            println!("You are allowed to play");
            println!("The sport is {}",activity);
        }
        else if age <21 && play==false && activity=="Hockey"{
            println!("Age is less than 21");
            println!("You are allowed to play");
            println!("The sport is {}",activity);
        }
        else {
            println!("Value Printed");
        }
    }
    {
        let age = 23; 
        let play = true; 
        let activity="Baseball" ;
        if age >= 21 && play==true || activity == "Tennis" { 
            println!("Age is greater than 21");
            println!("You are allowed to play");
            println!("The sport is {}",activity);
        }
        else if  age >= 21 && play == true && activity == "Tennis"{ 
            println!("Age is greater than 21");
            println!("You are allowed to play");
            println!("The sport is {}",activity);
        }
        else if age <21 && play == false && activity == "Tennis"{
            println!("Age is less than 21");
            println!("You are allowed to play");
            println!("The sport is {}",activity);
        }
        else{
            println!("Value Printed");
        }
    }
    // if-let-else conditional construct
    let languages = ("Python", "Rust");
    // check pattern matching
    if let ("Python", "Rust") = languages {
        println!("All patterns match");
    }
    else {
        println!("no pattern matched")
    }
    if let ("Rust", "Python") = languages {
        println!("Didn't match pattern");
    }
    else {
        println!("something else");
    }
    if let ("Python", a) = languages {
        println!("first pattern match and the second value is guessed as a:{}", a);
    }
    else {
        println!("no pattern matched")
    }
    {
        // match expression
        let x = 2;
        match x {
            1 => {println!("Python")},
            2 => {println!("Rust")},
            _ => {println!("Unknown lang")}
        }
        // define a variable
        let lang = "Rust";
        // return value of match expression in a variable
        let found_lang = match lang {
            "Rust" => "Rust",
            "Go" => "Go",
            "C#" => "C Sharp",
            _ => "Unknown Language"
        };
        println!("Course name : {}", found_lang);
    }
    {
        let lang = "Rust";
        let f_lang= match lang {
            "C" => {
                if lang.len() == 1 {
                    "You've predicted C or R".to_string()
                }
                else {
                    "Not Rust !!".to_string()
                }
            },
            "Go" => {
                if lang.len() == 2 {
                    "You've predicted Go or C#".to_string()
                }
                else {
                    "Not Rust !!".to_string()
                }
            },
            "Rust" => {
                if lang.len() == 4 {
                    "You've predicted Rust".to_string()
                }
                else {
                    "Not Rust !!".to_string()
                }
            },
            _ => "Unknown language".to_string()
        };
        println!("{:#?}", f_lang);
    }
}

fn loops() {
    for i in 0..5 {
        println!("{}", i);
    }
    {
        for (count, i) in (0..4).enumerate(){
            println!("{} -> {}", count, i);
        }
    }
    {
        for i in 0..5 {
            if i % 4 == 0 {
                print!("{}", i);
            }
        }
    }
    println!("");
    for (count, variable) in (7..10).enumerate() {
        if count * 2 == 4{
            println!("count = {}, variable = {}", count, variable);
        }
    }
    for i in 0..5 {
        println!("i:{}", i);
        if i == 2 {
            break;
        }
    }
    println!("\n");
    {
        for i in 0..8 {
            if i == 6 {
                println!("I encoutered a continue statement");
                continue;
            }
            println!("i:{}", i);
            println!("I didn't encounter a continue statement");
        }
    }
    let arr = ["foo", "bar", "baz"];
    for i in arr {
        println!("{}", i)
    }
    {   // Example of loop label to break or continue with Nested loops
        'outer:for i in 1..5 { //outer loop
            println!("Muliplication Table : {}", i);
           'inner:for j in 1..5 { // inner loop
                if i == 3 { continue 'outer; } // Continues the loop over `i`.
                if j == 2 { continue 'inner; } // Continues the loop over `j`.
                println!("{} * {} = {}", i, j, i * j);
           }
         }
    }
}

fn test(n:i32) {
    // define a nested for loop
    for i in 0..n { //outer loop
        for _ in 0..i + 1 { // inner loop
            print!("&");
    }
    println!("");
    }
}

fn function_(param_1: &str, param_2: &str){
    println!("{} is testing {} functions!!", param_1, param_2);
}

fn cube_(mut n:i32) {
    n = n * n * n;
    println!("The value of n inside function call: {}", n);
}

fn pass_by_ref(n:&mut i32) {
    *n = *n * *n;
    println!("The value of n inside the fn: {}", n);
}

fn return_fn(param:&mut &str, n:&mut i32) -> i32 {
    println!("The name of the language is:{}", param);
    let m = *n * *n;
    m
}


fn main() {
    println!("Hello, world!");
    println!("This is my first Rust program !!");
    // The below line comments out a block of code.
    /*println!("Number is: {}", 1);*/ 
    let mut city = "OHIO"; // we should "mut" only if we intend to change it again.
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
    working_on_operators();
    logical_operators();
    type_casting();
    borrow_and_dereferencing();
    precedence_and_associativity();
    conditional_expressions();
    loops();
    test(5);
    let name = "Tony";
    let lang = "Rust";
    function_(name, lang);
    let n = 2;
    println!("The value of n before function call:{}", n);
    cube_(n);
    println!("The value of n: {}", n);

    let mut n = 2;
    println!("The value of n before fn call : {}", n);

    pass_by_ref(&mut n);
    println!("The value of n: {}", n);

    let mut name = "Rust";
    // Here n will be 4 not 2 as it gets changed because of pass by reference
    println!("The output is {}", return_fn(&mut name, &mut n));

    let x = 2;
    let y = 4;

    let (sq, peri) = return_fn_multiple_values(x, y);
    println!("square_rectangle:{} and perimeter_rectangle:{}", sq, peri);

    let mut arr = [1,4,8,5];

    println!("Before function call:{:?}",&mut arr);

    parsing_arrays_to_fn(&mut arr);

    println!("After function call{:?}", arr);

    let arr1 = ["Rust", "Python", "Go"];

    return_arrays_fn(arr1);


    println!(" Number = 12 : {}", test_divisibility_by_3_4(12));
    println!(" Number = 9  : {}", test_divisibility_by_3_4(9));
    println!(" Number = 8  : {}", test_divisibility_by_3_4(8));
    println!(" Number = 23 : {}", test_divisibility_by_3_4(23));


    let arr = [1, 2, 3, 4, 5];

    println!("The squared array is {:?}", arr_square(arr));

    // Strings in Rust
    // empty string
    let str_1 = String::new();

    let convert_str_new_string_object = str_1.to_string();

    println!("The string is: {}", convert_str_new_string_object);
    println!("The string length is: {}", convert_str_new_string_object.len());

    // from string literal

    let str_2 = "Rust";

    let convert_str_literal_string_object = str_2.to_string();

    println!("Length is : {}", convert_str_literal_string_object.len());

    // using String::from() method

    let str_3 = String::from("Rust Language");

    println!("The length of str_3 is: {}", str_3.len());

    string_objects();

    string_iteration();

    string_concantenation();

    string_slicing();

    let my_sport = "football";
    some_random_func(my_sport);
    println!("Accessing after function call: {}", my_sport);

    let game = String::from("boxing");
    some_random_func_1(game);
    // println!("Accessing after function call: {}", game); // cannot borrow here

    let  my_str= "This is a comprehensive course in Rust programming language on Educative. Read it with full concentration to grasp the content of the course".to_string();

    let results = extract_specific_words_from_string(my_str);

    println!("{}", results);

    vector_example();

    let mut arr_challenge = vec![1, 5, 7, 9];

    println!("{:?}", vector_challenge(&mut arr_challenge));
}


fn return_fn_multiple_values(x: i32, y: i32) -> (i32, i32){
    let square_rectangle = x * y;
    let perimeter_rectangle = 2 * (x + y);
    println!("Doing calculations!!");
    (square_rectangle, perimeter_rectangle)
}

// parsing an array with pass by reference
fn parsing_arrays_to_fn (arr:&mut [i32;4]) {

    println!("The array: {:?}", arr);
    
    println!("The first element is {} and 2nd element is {}", arr[0], arr[1]);

    let mut sum = 0;
    for i in 0..4 {
        sum += arr[i];
    }

    println!("The sum of array values: {}", sum);

    arr[2] = -4;
    arr[arr.len() - 1] = 6;
    println!("The changed array is:{:?}", arr);

    let val = [1, 4, 9, 3];
    pass_by_value(val);
    println!("{:?}", val);
}

// parsing an array with pass by value
fn pass_by_value(mut val: [i32;4]) {
    val[0] = 0;
    val[2] = 7;
    println!("The value of 1st element {} and 2nd element {}",val[0], val[2]);
    println!("{:?}", val);
}

fn return_arrays_fn(arr1: [&str;3]) -> [&str; 3] {
    arr1
}

fn test_divisibility_by_3_4(n: i32) -> i32 {

    if (n % 4) == 0 && (n % 3) == 0 {
        0
    } else if (n % 3) == 0 && (n % 4) != 0 {
        1
    } else if (n % 4) == 0 && (n % 3) != 0 {
        2
    } else {
        -1
    }
}

fn arr_square(mut arr: [i32;5]) -> [i32; 5] {
    for i in 0..5 {
        arr[i] = arr[i] * arr[i];
    }
    arr
}

fn string_objects() {

    // calculating the capacity = number of bytes allocated to string
    // It's always equal or less than the length of the string.
    let str1 = String::from("Rust Language");

    println!("Capacity of {} is {}", str1, str1.capacity());

    // substring part of string. return bool.
    let str2 = "Rustaceans";
    println!("Is 'Rust' part of {}? \nAnswer: {}", str2, str2.contains("Rust"));

    // replace a substring in a string
    let str3 = "Rust Program";
    println!("{} is now {}", str3, str3.replace("Program", "Language"));

    // trimming a string or leading and trailing white spaces and lines

    let str4 = String::from("  My Rust  ");
    println!("Trimming '{}' to '{}'", str4, str4.trim());
}


// String iteration
fn string_iteration() {
    let language = String::from("Rust Programming");
    for i in language.chars() {
        println!("{}", i);
    }

    for i in language.split_whitespace() {
        println!("{}", i);
    }
    let language = String::from("Rust, programming");
    for i in language.split(",") {
        println!("{}", i)
    }
}

fn string_concantenation() {
    let program_name = String::from("Rust");

    let experience = String::from("4");

    let time = "days".to_string();

    // using format! macro

    let result = format!("{1} {2} of {0}", program_name, experience, time);

    println!("{}", result);

    // using push method
    let mut p_name = String::from("Rus");

    p_name.push('t'); // only character

    println!("{}", p_name);

    // using + operator
    let results = program_name + " " + &experience + " " + &time;
    println!("{}", results);

    let mut lang = String::from("Rust");
    // push a string
    lang.push_str(" Programming");

    println!("This is {}.", lang);
}



fn string_slicing() {
    let string = "Rust Programming".to_string();
    let slice = &string[0..3]; 
    println!("{}", slice);
}

// String object which is growable vs primitive string (&str) use in functions

fn some_random_func(my_sport:&str) {
    println!("{}", my_sport);
}

fn some_random_func_1(game:String) {
    println!("{}", game);
}


fn extract_specific_words_from_string(my_str:String) -> String {
    let mut new_str = String::new();
    for word in my_str.split(" ") {
        if word.starts_with("c") {
            new_str.push_str(word);
            new_str.push(' ');
        }
    }
    new_str.pop();
    new_str
}


// Vectors
fn vector_example() {
    let my_vec = vec![1, 3, 9, 21];
    println!("My vector array is: {:?}", my_vec);

    let mut vec_arr = Vec::new();

    // adding elements to array
    vec_arr.push(1);
    vec_arr.push(2);
    vec_arr.push(6);
    vec_arr.push(-4);

    println!("The new array is : {:?}", vec_arr);

    // removing the element from array
    println!("Removing last element -4 in array: {:?}", vec_arr.pop());

    // removing specific value from array using index
    println!("Removing second element i.e. 2 from array: {:?}", vec_arr.remove(1));

    // Now finding the length of the array
    println!("The length of the array is : {}", vec_arr.len());

    // checking if the element 6 is in array.
    println!("Is element 6 part of the array?: {}", vec_arr.contains(&6));

    // accessing element in an array
    println!("The final array is: {:?}", vec_arr);
    println!("The last element in the array: {}", vec_arr[vec_arr.len()-1]);

    // handling out of bound index values
    if let Some(element) = vec_arr.get(6) {
        println!("The element is: {}", element);
    } else {
        println!("The provided index is out of bounds!!");
    }

    match vec_arr.get(1) {
        Some(x) => {
            println!("Value at given index is: {}", x);
        }
        None => println!("Out of bounds !!")
    }

    // looping in vector array
    for i in my_vec {
        println!("{}", i);
    }

    // to remove an element from an array we need to know the
    // index and use .remove(index). To avoid that -
    let mut vec_array = vec![1, 3, 9, 28, 12];

    let value = 9; // the element 9 will be removed

    let index = vec_array.iter().position(|&e| e == value).unwrap();


    match vec_array.get(index) {
        Some(x) => println!("Value: {}", x),
        None => println!("Not found")
    }

    vec_array.remove(index);

    println!("Updated vector: {:?}", vec_array);

    // loops and mutate values

    for i in vec_array.iter_mut() {
        *i *= 3;
    }
    println!("{:?}", vec_array);

    let mut new_vec = vec![1, 2, 3, 4, 5];

    println!("Vector: {:?}", new_vec);

    for each in new_vec.iter_mut() {
        *each = *each + 4;
    }

    new_vec.push(23);

    println!("Updated Vector: {:?}", new_vec);

    println!("Vector length: {}", new_vec.len());

    // slicing a vector

    let sliced_val = &mut new_vec[2..4];

    println!("sliced arr: {:?}", sliced_val);

}


fn vector_challenge(vec_arr: &mut Vec<u32>) -> &mut Vec<u32> {
    let middle = (vec_arr.len()) / 2;

    vec_arr.pop();

    vec_arr.remove(middle-1);

    let mut sum = 0;

    for i in vec_arr.iter() {
        sum += i;
    }

    vec_arr.push(sum);

    vec_arr
}
// fn main() {
//     eprintln!("Error: Couldn't complete the task!");
// }


// use std::io;
// use rand::Rng;
// use std::cmp::Ordering;
// fn main() {
//     println!("Guess the number!");
    
//     let secret_number = rand::thread_rng().gen_range(1..=100);

//     println!("The secret number is: {secret_number}");

//     loop {
//         println!("Please input your guess.");

//         let mut guess = String::new();

//         io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");

//         let guess: u32 = match guess.trim().parse() {
//             Ok(num) => num,
//             Err(_) => continue
//         };

//         println!("You guessed: {}", guess);

//         match guess.cmp(&secret_number) {
//             Ordering::Less => println!("Too small!"),
//             Ordering::Greater => println!("Too big!"),
//             Ordering::Equal => {
//                 println!("You win!");
//                 break;
//             }
//         }
//     }  
// }