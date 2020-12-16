use rand::Rng;
use std::time::{Instant};

fn main() {
    let mut rng = rand::thread_rng();
    let mut vector1: Vec<i32> = (0..100000).map(|_| rng.gen_range(0, 10000)).collect();
    let mut vector2 = vector1.clone();

    let now1 = Instant::now();
    q_sort(&mut vector1);
    println!("{}", now1.elapsed().as_millis());
    let now2 = Instant::now();
    q_sort2(&mut vector2);
    println!("{}", now2.elapsed().as_millis());

    println!("{}",is_well_sorted(&vector1));
    println!("{}",is_well_sorted(&vector2));
}

fn q_sort(vector: &mut Vec<i32>) {
    iterate_q_sort(vector, 0, vector.len() - 1);
}

fn q_sort2(vector: &mut Vec<i32>) {
    iterate_q_sort2(vector, 0, vector.len() - 1);
}

fn iterate_q_sort(vector: &mut Vec<i32>, first:usize, last:usize) {
    if first >= last {
        return;
    }
    let mut wall = first; // слева от элемента
    for current in first..last{
        if vector[current] < vector[last] {
            vector.swap(current, wall);
            wall +=1;
        }
    }
    vector.swap(wall, last);
    //println!("{:?}",vector);
    if wall > 0 {
        iterate_q_sort(vector, first, wall - 1);
    }
    iterate_q_sort(vector, wall + 1, last);
}

fn iterate_q_sort2(vector: &mut Vec<i32>, first:usize, last:usize) {
    if first >= last {
        return;
    }
    let middle = partition2(vector, first, last);
    //println!("{:?}",vector);
    if middle > 0 {
        iterate_q_sort(vector, first, middle - 1);
    }

    if middle < vector.len() {
        iterate_q_sort(vector, middle + 1, last);
    }
}

fn partition2(vector: &mut Vec<i32>, first:usize, last: usize) -> usize {
    let divide_element = last;
    let mut first_high = first;

    for i in first..last  {
        if vector[i] < vector[divide_element] {
            vector.swap(i, first_high);
            first_high+=1;
        }
    }
    vector.swap(divide_element, first_high);
    first_high
}

fn is_well_sorted(vector: &Vec<i32>) -> bool {
    for x in 0..vector.len() - 2 {
        if vector[x] > vector[x + 1]{
            return false;
        }
    }
    true
}