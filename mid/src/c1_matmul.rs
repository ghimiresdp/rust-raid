/**
 * matrix multiplication
 *
 * Matrix multiplication process involves checking compatibility of dimension
 * and multiplying nth row with nth column to get one cell output.
 *
 * Please refer to the matrix multiplication resource to understand it easily.
 *
 * given,
 *
 * x =  | 1  2 |  and y = | 1  3 |
 *      | 3  4 |          | 2  1 |
 *
 * if z is the resultant matrix,
 *
 * z[0][0] = x[0][0] * y[0][0] + x[0][1] * y[1][0] = 1 + 4 = 5
 * z[0][1] = x[0][0] * y[0][1] + x[0][1] * y[1][1] = 3 + 2 = 5
 * z[1][0] = x[1][0] * y[0][0] + x[1][1] * y[1][0] = 3 + 8 = 11
 * z[1][1] = x[1][0] * y[0][1] + x[1][1] * y[1][1] = 9 + 4 = 13
 *
 * In rustlang, matrix multiplication is fairly tricky since borrowing will
 * occur in each loop. We need to clone the variable if the borrow occurs before
 * looping through vectors.
 */

fn mat_mul(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    if a[0].len() != b.len() {
        panic!("Matrix dimensions are incompatible")
    }
    let i = b[0].len();
    let j = a.len();
    let mut new_matrix: Vec<Vec<i32>> = vec![vec![0; i]; j];
    for x in 0..i {
        for y in 0..j {
            let row_x = a[x].clone();
            let col_y: Vec<i32> = b.iter().map(|v| v[y]).collect();
            new_matrix[x][y] = row_x
                .iter()
                .zip(col_y.iter())
                .map(|(a, b)| a * b)
                .reduce(|a, b| a + b)
                .unwrap();
        }
    }
    new_matrix
}
fn main() {
    let mat1 = vec![vec![1, 2], vec![3, 4]];

    let mat2 = vec![vec![1, 3], vec![2, 1]];

    let mat3 = mat_mul(mat1, mat2);
    println!("The resultant matrix is: {:?}", mat3);
}

#[cfg(test)]
mod tests {
    use crate::mat_mul;

    #[test]
    fn multiply_success() {
        let mat1 = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let mat2 = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        assert_eq!(mat_mul(mat1, mat2), vec![vec![22, 28], vec![49, 64]]);
    }

    #[test]
    #[should_panic(expected = "Matrix dimensions are incompatible")]
    fn multiply_failure() {
        let mat1 = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let mat2 = vec![vec![1, 2, 3], vec![4, 5, 6]];
        mat_mul(mat1, mat2);
    }

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn multiply_failure_2() {
        // The matrix multiplication will cause index out of bound since
        // the second matrix is invalid

        let mat1 = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let mat2 = vec![vec![1], vec![3, 4], vec![5, 6]];
        mat_mul(mat1, mat2);
    }
}
