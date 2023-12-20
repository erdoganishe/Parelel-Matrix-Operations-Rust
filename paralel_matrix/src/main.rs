mod matrix;

use matrix::{Matrix, BitMatrix, NonParallel, Parallel};
use std::time::Instant;

fn main() {

    let random_matrix = Matrix::new_with_random_values(300);
    let random_matrix_2 = Matrix::new_with_random_values(300);

    let random_bit_matrix = BitMatrix::new_with_random_values(300);
    let random_bit_matrix_2 = BitMatrix::new_with_random_values(300);

    // Вимірюємо час для NonParallel::bit_addition
    measure_time(|| {
        let result = NonParallel::bit_addition(&random_bit_matrix.clone(), &random_bit_matrix_2.clone());
        //result.print();
    }, "NonParallel::bit_addition");

    
    // Вимірюємо час для Parallel::bit_addition
    measure_time(|| {
        let result = Parallel::bit_addition(&random_bit_matrix.clone(), &random_bit_matrix_2.clone());
        //result.print();
    }, "Parallel::bit_addition");

    // Вимірюємо час для NonParallel::bit_multiply
    measure_time(|| {
        let result = NonParallel::bit_multiply(&random_bit_matrix.clone(), &random_bit_matrix_2.clone());
        //result.print();
    }, "NonParallel::bit_multiply");

    // Вимірюємо час для Parallel::bit_multiply
    measure_time(|| {
        let result = Parallel::bit_multiply(&random_bit_matrix.clone(), &random_bit_matrix_2.clone());
       // result.print();
    }, "Parallel::bit_multiply");
    

    // Вимірюємо час для NonParallel::multiply
    measure_time(|| {
        let result = NonParallel::multiply(&random_matrix.clone(), &random_matrix_2.clone());
        //result.print();
    }, "NonParallel::multiply");

    // Вимірюємо час для Parallel::multiply
    measure_time(|| {
        let result = Parallel::multiply(&random_matrix.clone(), &random_matrix_2.clone());
        //result.print();
    }, "Parallel::multiply");

    // Вимірюємо час для NonParallel::addition
    measure_time(|| {
        let result = NonParallel::addition(&random_matrix.clone(), &random_matrix_2.clone());
        //result.print();
    }, "NonParallel::addition");

    // Вимірюємо час для Parallel::addition
    measure_time(|| {
        let result = Parallel::addition(&random_matrix.clone(), &random_matrix_2.clone());
        //result.print();
    }, "Parallel::addition");


}

// Допоміжна функція для вимірювання часу виконання блоку коду
fn measure_time<F>(code_block: F, operation_name: &str)
where
    F: FnOnce(),
{
    let start_time = Instant::now();
    code_block();
    let elapsed_time = start_time.elapsed();
    println!("{} Time: {:?}", operation_name, elapsed_time);
}
