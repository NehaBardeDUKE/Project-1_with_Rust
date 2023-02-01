use std::collections::HashMap;
//create a data function that takes an integer value and returns 3 values- 1 string and other 2 integers
fn data() {
    //create an array of strings
    //accept a string from the user

    //let a_name = ["Gal Gadot", "Margot Robbie", "Diane Keaton"];
    //create an integer array
    let a_grade_bio_y1 = [98, 72, 86];
    let a_grade_bio_y2 = [92, 88, 90];
    let a_grade_bio_y3 = [70, 82, 84];
    let a_grade_bio_y4 = [68, 80, 77];
    //create an integer array
    //let access_level = [0, 1, 2];
    //create a dictionary array for access level descriptions
    //let access_level_desc = ["Summary", "Read Only all the info", "No Access"];
    //create a dictionary with names of people as string and their access levels as integers
    let mut access_level_dict = HashMap::new();
    access_level_dict.insert("Gal Gadot".to_owned(), 1);
    access_level_dict.insert("Margot Robbie".to_owned(), 1);
    access_level_dict.insert("Diane Keaton".to_owned(), 1);
    access_level_dict.insert("DS tech".to_owned(), 0);
    access_level_dict.insert("Paparazzi".to_owned(), 2);
    //match sring x to the dictionaly access_level_dict's keys
    let mut x = String::new();
    println!("Who's there?");
    std::io::stdin()
        .read_line(&mut x)
        .expect("Failed to read line");
    x = x.trim().to_owned();
    match access_level_dict.get(&x) {
        //if the key is found, then return the value of the key
        Some(value) => {
            println!("{}", value);
            //match the value of the key to the access_level array
            match value {
                //if the value of the key is 0, then return the value of the key
                0 => {
                    //create a string with the value of the key

                    //Do the average of the grades for y1
                    let mut sum1 = 0;
                    for i in 0..a_grade_bio_y1.len() {
                        sum1 += a_grade_bio_y1[i];
                    }
                    let mut sum2 = 0;
                    for i in 0..a_grade_bio_y2.len() {
                        sum2 += a_grade_bio_y2[i];
                    }
                    let mut sum3 = 0;
                    for i in 0..a_grade_bio_y3.len() {
                        sum3 += a_grade_bio_y3[i];
                    }
                    let mut sum4 = 0;
                    for i in 0..a_grade_bio_y4.len() {
                        sum4 += a_grade_bio_y4[i];
                    }
                    let avg1 = (sum1 + sum2 + sum3 + sum4) / 4;
                    //return the string and the value of the key
                    let s:String = format!("You are the dS tech and will get a summary of the data. Heres the average {}", avg1);
                    println!("{}", s);
                }
                //if the value of the key is 1, then return the value of the key
                1 => {
                    //ask for the year of the grades
                    let mut year = String::new();
                    println!("Please enter the year of the grades you want to see");
                    std::io::stdin()
                        .read_line(&mut year)
                        .expect("Failed to read line");
                    let year: i32 = year.trim().parse().expect("Please type a number!");
                    //if the year is 1 and key is "Gal Gadot", then return the first value of a_grade_bio_y1 array
                    if year == 1 && x == "Gal Gadot" {
                        //create a string with the value of the key
                        let s:String = format!("You are a Student and will receive your grades and here are your grades{}", a_grade_bio_y1[0]);
                        //return the string and the value of the key
                        println!("{}", s);
                    }
                    //if the year is 1 and key is "Margot Robbie", then return the second value of a_grade_bio_y1 array
                    else if year == 1 && x == "Margot Robbie" {
                        //create a string with the value of the key
                        let s:String = format!("You are a Student and will receive your grades and here are your grades{}", a_grade_bio_y1[1]);
                        //return the string and the value of the key
                        println!("{}", s);
                    } else if year == 1 && x == "Diane Keaton" {
                        //create a string with the value of the key
                        let s:String = format!("You are a Student and will receive your grades and here are your grades{}", a_grade_bio_y1[2]);
                        //return the string and the value of the key
                        println!("{}", s);
                    } else if year == 2 && x == "Gal Gadot" {
                        //create a string with the value of the key
                        let s:String = format!("You are a Student and will receive your grades and here are your grades{}", a_grade_bio_y2[0]);
                        //return the string and the value of the key
                        println!("{}", s);
                    } else if year == 2 && x == "Margot Robbie" {
                        //create a string with the value of the key
                        let s:String = format!("You are a Student and will receive your grades and here are your grades{}", a_grade_bio_y2[1]);
                        //return the string and the value of the key
                        println!("{}", s);
                    } else if year == 2 && x == "Diane Keaton" {
                        //create a string with the value of the key
                        let s:String = format!("You are a Student and will receive your grades and here are your grades{}", a_grade_bio_y2[2]);
                        //return the string and the value of the key
                        println!("{}", s);
                    } else if year == 3 && x == "Gal Gadot" {
                        //create a string with the value of the key
                        let s:String = format!("You are a Student and will receive your grades and here are your grades{}", a_grade_bio_y3[0]);
                        //return the string and the value of the key
                        println!("{}", s);
                    }
                    //if the year is 1 and key is "Margot Robbie", then return the second value of a_grade_bio_y1 array
                    else if year == 3 && x == "Margot Robbie" {
                        //create a string with the value of the key
                        let s:String = format!("You are a Student and will receive your grades and here are your grades{}", a_grade_bio_y3[1]);
                        //return the string and the value of the key
                        println!("{}", s);
                    } else if year == 3 && x == "Diane Keaton" {
                        //create a string with the value of the key
                        let s:String = format!("You are a Student and will receive your grades and here are your grades{}", a_grade_bio_y3[2]);
                        //return the string and the value of the key
                        println!("{}", s);
                    } else if year == 4 && x == "Gal Gadot" {
                        //create a string with the value of the key
                        let s:String = format!("You are a Student and will receive your grades and here are your grades{}", a_grade_bio_y4[0]);
                        //return the string and the value of the key
                        println!("{}", s);
                    } else if year == 4 && x == "Margot Robbie" {
                        //create a string with the value of the key
                        let s:String = format!("You are a Student and will receive your grades and here are your grades{}", a_grade_bio_y4[1]);
                        //return the string and the value of the key
                        println!("{}", s);
                    } else if year == 4 && x == "Diane Keaton" {
                        //create a string with the value of the key
                        let s:String = format!("You are a Student and will receive your grades and here are your grades{}", a_grade_bio_y4[2]);
                        //return the string and the value of the key
                        println!("{}", s);
                    } else {
                        //create a string with the value of the key
                        let s = "Do you have access to this information?";
                        //return the string and the value of the key
                        println!("{}", s);
                    }
                }
                //if the value of the key is 2, then return the value of the key
                2 => {
                    //create a string with the value of the key
                    let s = "You Do not have access to this information";
                    //return the string and the value of the key
                    println!("{}", s);
                }
                //if the value of the key is not 0, 1, or 2, then return the value of the key
                _ => {
                    //create a string with the value of the key
                    let s = "You are not in our database";
                    //return the string and the value of the key
                    println!("{}", s);
                }
            }
        }
        //if the key is not found, then return the value of the key
        None => {
            //create a string with the value of the key
            let s = format!("The value of x is {}", x);
            //return the string and the value of the key
            println!("{}", s);
        }
    }
}

//create a main function that calls the data function
fn main() {
    //call the data function
    data();
}
