mod pandas;
use pandas as pd;

// When running this project from the directory other than the workspace
// directory, do not forget to change the relative path below
fn main() {
    let df = pd::read_csv("projects/pandas/sample/students.csv", true);
    df.describe();
    df.head(5);
}
