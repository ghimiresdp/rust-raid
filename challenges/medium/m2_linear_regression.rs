/**
 * Linear regression
 *
 * The linear regression model assumes that the relationship between independent
 * and dependent variables is linear.
 *
 * For example, if x1 and x2 are independent variables and y is a dependent
 * variable, then  the equation that describes the relation between two variables
 * will be as follows:
 *
 * `y = w0 + w1.x1 + w2.x2 + ... + wn.xn + e`
 *
 * where,
 *  * y             => dependent variable
 *  * x1, x2, ...   => independent variables
 *  * w0, w1, ...   => model parameters (intercepts, slops, etc.)
 *  * e             => error parameter
 *
 *
 *  *
 * To run the code, run the following:
 * =============================================================================
 *
 * cargo run --bin m2
 *
 * =============================================================================
 */

pub mod losses {
    pub fn mean_squared_error(actual: Vec<f64>, predicted: Vec<f64>) -> f64 {
        if actual.len() != predicted.len() {
            panic!("Dimension mismatch between actual and predicted values");
        }
        actual
            .iter()
            .zip(predicted)
            .map(|a| (a.0 - a.1).powi(2))
            .reduce(|a, b| a + b)
            .unwrap()
    }

    pub fn mean_absolute_error(actual: Vec<f64>, predicted: Vec<f64>) -> f64 {
        if actual.len() != predicted.len() {
            panic!("Dimension mismatch between actual and predicted values");
        }
        actual
            .iter()
            .zip(predicted)
            .map(|a| (a.0 - a.1).abs())
            .reduce(|a, b| a + b)
            .unwrap()
    }
}

struct LinearRegressionModel {
    w0: f64,
    w: Vec<f64>,
}

impl LinearRegressionModel {
    fn new() -> Self {
        Self {
            w0: 0.1,
            w: vec![], // coefficients
        }
    }
    fn _predict(&self, x: &Vec<f64>) -> f64 {
        let mut y_pred = self.w0;
        for i in 0..x.len() {
            y_pred += x[i] * self.w[i];
        }
        y_pred
    }

    fn fit(&mut self, x: Vec<Vec<f64>>, y: Vec<f64>, learning_rate: f64, epochs: usize) {
        if x.len() != y.len() {
            panic!("input and output variable lengths mismatch")
        }
        self.w = vec![0.0; x.get(0).unwrap().len()];
        for epoch in 0..epochs {
            // use gradient descent method to optimize the algorithm
            let mut gradients = vec![0.0; self.w.len()];
            for idx in 0..x.len() {
                let prediction = self._predict(&x[idx]);
                let error = prediction - y[idx];

                gradients = gradients
                    .iter()
                    .zip(x[idx].clone())
                    .map(|(g, _x)| g + 2.0 * _x * error)
                    .collect();
            }
            self.w = self
                .w
                .iter()
                .zip(gradients.clone())
                .map(|(v, g)| v - (learning_rate / gradients.len() as f64) * g)
                .collect();
            println!("Epoch: {epoch}\t loss: {}", self.test(x.clone(), y.clone()));
        }
    }

    fn predict(&mut self, x: &Vec<f64>) -> f64 {
        if self.w.len() == 0 {
            panic!("Model is not yet trained!")
        }
        if x.len() != self.w.len() {
            panic!("training and prediction input parameters dimension mismatch");
        }
        self._predict(x)
    }

    fn test(&mut self, x: Vec<Vec<f64>>, y: Vec<f64>) -> f64 {
        let predictions: Vec<f64> = x.iter().map(|row| self._predict(row)).collect();
        let error = losses::mean_squared_error(y.clone(), predictions.clone());
        error
    }
}

fn main() {
    let x = vec![
        vec![1.0, 2.0],
        vec![2.0, 3.0],
        vec![3.0, 4.0],
        vec![4.0, 5.0],
        vec![5.0, 6.0],
    ];
    let y = vec![5.0, 8.0, 11.0, 14.0, 17.0];

    let mut model = LinearRegressionModel::new();

    model.fit(x, y, 0.001, 1000);

    let out = model.predict(vec![1.0, 2.0].as_ref());
    println!("Actual: 5.0, Prediction: {out}");
}

#[cfg(test)]
mod tests {
    use crate::LinearRegressionModel;

    #[test]
    fn test_correct_prediction() {
        let x = vec![vec![1.0, 1.0], vec![2.0, 2.0], vec![3.0, 3.0]];
        let y = vec![2.0, 4.0, 6.0];
        let mut model = LinearRegressionModel::new();
        model.fit(x, y, 0.001, 1000);
        let loss = (model.predict(vec![4.0, 4.0].as_ref()) - 8.0).abs();
        assert_eq!(loss < 0.2, true);
    }
}
