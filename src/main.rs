fn main() {
    let mut vector = vec![1,2,6,7,3,2,3,4,6,16,22,1,0];
    q_sort(&mut vector);
    println!("{:?}",vector);
}

fn q_sort(vector: &mut Vec<i32>) {
    iterate_q_sort(vector, 0, vector.len() - 1);
}

fn iterate_q_sort(vector: &mut Vec<i32>, first:usize, last: usize){

    let middle = last/2;
    iterate_q_sort(vector, 0, middle - 1 );
    iterate_q_sort(vector, middle + 1, last);
}

fn bubble_sort(vector: &mut Vec<i32>) {

}