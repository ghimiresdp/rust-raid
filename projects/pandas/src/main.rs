mod pandas;
use std::collections::HashMap;

use pandas as pd;

// When running this project from the directory other than the workspace
// directory, do not forget to change the relative path below
fn main() {
    // creating own dataframe
    let df1 = pd::DataFrame::from_mapping(HashMap::from_iter([
        (
            String::from("First Name"),
            vec!["John", "Jane", "Satoshi", "Adam"]
                .iter()
                .map(|v| v.to_string())
                .collect(),
        ),
        (
            String::from("Last Name"),
            vec!["Doe", "Deo", "Nakamoto", "Av"]
                .iter()
                .map(|v| v.to_string())
                .collect(),
        ),
    ]));

    df1.describe();
    df1.head(5);
    println!("Shape of the dataframe 1: {:?}", df1.shape());

    // reading from a csv file
    println!("\n\n");
    let df2 = pd::read_csv("projects/pandas/sample/students.csv", true);
    df2.describe();
    df2.head(5);
    println!("Shape of the dataframe 2: {:?}", df2.shape());
}
