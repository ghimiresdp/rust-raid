mod pandas;
use std::collections::HashMap;

use pandas as pd;

use crate::pd::DType;

// When running this project from the directory other than the workspace
// directory, do not forget to change the relative path below
fn main() {
    // creating own dataframe
    let df1 = pd::DataFrame::new([
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
    ]);

    df1.describe();
    df1.head(5);
    println!("Shape of the dataframe 1: {:?}", df1.shape());

    // reading from a csv file
    println!("\n\n");
    let mut df2 = pd::read_csv("projects/pandas/sample/students.csv", true);
    df2.head(5);
    df2.describe();
    println!("Shape of the dataframe 2: {:?}", df2.shape());

    // Data Processing
    df2.as_type([
        (String::from("SN"), DType::Int),
        (String::from("Age"), DType::Int),
        (String::from("CGPA"), DType::Float),
        (String::from("Graduated"), DType::Bool),
    ]);
    df2.head(5);
    df2.describe();
}
