use rand::Rng;

#[derive(Debug, Clone)]
pub struct Matrix {
    data: Vec<Vec<f64>>,
    size: usize,
}

impl Matrix {
    // Пустий конструктор
    pub fn new_empty() -> Self {
        Self { data: Vec::new(), size: 0 as usize}
    }

    // Конструктор з розміром матриці
    pub fn new_with_size(size: usize) -> Self {
        let data = vec![vec![0.0; size]; size];
        Self { data, size }
    }

    // Конструктор з розміром та заповненням випадковими значеннями
    pub fn new_with_random_values(size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let data: Vec<Vec<f64>> = (0..size)
            .map(|_| {
                (0..size)
                    .map(|_| (rng.gen_range(0..=10000) as f64) / 100.0)
                    .collect()
            })
            .collect();
        Self { data, size }
    }

    pub fn set(&mut self, row: usize, col: usize, value: f64) {
        if row < self.size && col < self.size {
            self.data[row][col] = value;
        }
    }

    // Отримати значення біта з матриці
    pub fn get(&self, row: usize, col: usize) -> Option<f64> {
        if row < self.size && col < self.size {
            Some(self.data[row][col])
        } else {
            None
        }
    }

    pub fn print(&self) {
        for i in 0..self.size {
            for j in 0..self.size {
                if let Some(value) = self.get(i, j) {
                    print!("{} ", value);
                }
            }
            println!();
        }
    }

    
}

#[derive(Debug, Clone)]
pub struct BitMatrix {
    data: Vec<Vec<bool>>,
    size: usize,
}

impl BitMatrix {
    // Конструктор для створення нової бітової матриці
    pub fn new(size: usize) -> Self {
        let data = vec![vec![false; size]; size];
        Self {data, size}
    }

    // Задати значення біта в матриці
    pub fn set(&mut self, row: usize, col: usize, value: bool) {
        if row < self.size && col < self.size {
            self.data[row][col] = value;
        }
    }

    // Отримати значення біта з матриці
    pub fn get(&self, row: usize, col: usize) -> Option<bool> {
        if row < self.size && col < self.size {
            Some(self.data[row][col])
        } else {
            None
        }
    }

    pub fn new_with_random_values(size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let data: Vec<Vec<bool>> = (0..size)
            .map(|_| {
                (0..size)
                    .map(|_| (rng.gen_range(0..100) as i8 %2==0))
                    .collect()
            })
            .collect();
        Self { data, size }
        
    }

    pub fn print(&self) {
        for i in 0..self.size {
            for j in 0..self.size {
                if let Some(value) = self.get(i, j) {
                    print!("{} ", if value { 1 } else { 0 });
                }
            }
            println!();
        }
    }


}


pub mod Parallel {
    use super::*;

    pub fn multiply(matrix1: &Matrix, matrix2: &Matrix) -> Matrix {
        // Реалізація паралельного множення

        unimplemented!()
    }

    pub fn addition(matrix1: &Matrix, matrix2: &Matrix) -> Matrix {
        // Реалізація паралельного множення

        unimplemented!()
    }

    pub fn bit_multiply(matrix1: &BitMatrix, matrix2: &BitMatrix) -> BitMatrix {
        // Реалізація паралельного множення

        unimplemented!()
    }

    pub fn bit_addition(matrix1: &BitMatrix, matrix2: &BitMatrix) -> BitMatrix {
        // Реалізація паралельного множення

        unimplemented!()
    }

}

pub mod NonParallel {
    use super::*;

    pub fn multiply(matrix1: &Matrix, matrix2: &Matrix) -> Matrix {
        // Реалізація непаралельного множення
        assert_eq!(matrix1.size, matrix2.size, "Matrix sizes must match for multiplication");

        let size = matrix1.size;
        let mut result = Matrix::new_with_size(size);

        for i in 0..size {
            for j in 0..size {
                let mut sum = 0.0;
                for k in 0..size {
                    sum += matrix1.get(i, k).unwrap() * matrix2.get(k, j).unwrap();
                }
                result.set(i, j, sum);
            }
        }

        result
    }

    pub fn addition(matrix1: &Matrix, matrix2: &Matrix) -> Matrix {
        assert_eq!(matrix1.size, matrix2.size, "Matrix sizes must match for addition");

        let size = matrix1.size;
        let mut result = Matrix::new_with_size(size);

        for i in 0..size {
            for j in 0..size {
                let sum = matrix1.get(i, j).unwrap() + matrix2.get(i, j).unwrap();
                result.set(i, j, sum);
            }
        }

        result
    }

    pub fn bit_multiply(matrix1: &BitMatrix, matrix2: &BitMatrix) -> BitMatrix {

        assert_eq!(matrix1.size, matrix2.size, "BitMatrix sizes must match for bit multiplication");

        let size = matrix1.size;
        let mut result = BitMatrix::new(size);

        for i in 0..size {
            for j in 0..size {
                let bit_value = (0..size)
                    .fold(false, |acc, k| acc ^ (matrix1.get(i, k).unwrap() && matrix2.get(k, j).unwrap()));
                result.set(i, j, bit_value);
            }
        }

        result
    }

    pub fn bit_addition(matrix1: &BitMatrix, matrix2: &BitMatrix) -> BitMatrix {

        assert_eq!(matrix1.size, matrix2.size, "BitMatrix sizes must match for bit addition");

        let size = matrix1.size;
        let mut result = BitMatrix::new(size);

        for i in 0..size {
            for j in 0..size {
                let bit_value = matrix1.get(i, j).unwrap() ^ matrix2.get(i, j).unwrap();
                result.set(i, j, bit_value);
            }
        }

        result
    }
}
