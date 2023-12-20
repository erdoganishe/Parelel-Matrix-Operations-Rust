mod matrix;

use matrix::{Matrix, BitMatrix, NonParallel, Parallel};


fn main() {

    let sized_matrix = Matrix::new_with_size(3);
    let random_matrix = Matrix::new_with_random_values(3);
    let random_matrix_2 = Matrix::new_with_random_values(3);

    let sized_bit_matrix = BitMatrix::new(3);
    let random_bit_matrix = BitMatrix::new_with_random_values(3);
    let random_bit_matrix_2 = BitMatrix::new_with_random_values(3);


    random_bit_matrix.print();
    random_bit_matrix_2.print();

    NonParallel::bit_addition(&random_bit_matrix.clone(), &random_bit_matrix_2.clone()).print();
    NonParallel::bit_multiply(&random_bit_matrix.clone(), &random_bit_matrix_2.clone()).print();

    random_matrix.print();
    random_matrix_2.print();

    NonParallel::addition(&random_matrix.clone(), &random_matrix_2.clone()).print();
    NonParallel::multiply(&random_matrix.clone(), &random_matrix_2.clone()).print();
}
