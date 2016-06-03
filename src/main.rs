extern crate quickcheck;

use quickcheck::quickcheck;

//use std::debu;
// merges two sorted arrays into one
// This is a generalization of the standard merge and a binary search
// This should run in time O(m*log(1+n/m))
fn finger_merge<T: Ord + Clone /*+ std::fmt::Debug */ >(in_a: &[T], in_b: &[T]) -> Vec<T> {
    let a: &[T];
    let b: &[T];

    if in_a.len() >= in_b.len() {
        a = in_a;
        b = in_b;
    }
    else {
        a = in_b;
        b = in_a;
    }

    let mut out = Vec::with_capacity(a.len() + b.len());

    if b.len() == 0 {
        out.extend_from_slice(a);
        return out;
    }

    let mut a_finger_ptr: usize = 0;
    let mut a_prev_ptr: usize = 0;
    let mut b_prev_ptr: usize = 0;
    let mut b_ptr: usize = 0;

    let calculate_finger = |finger: usize| -> usize {
        finger * a.len() / b.len()
    };

    loop { 
        if a_finger_ptr<b.len() && b_ptr<b.len() && b[b_ptr] < a[calculate_finger(a_finger_ptr)] {
            b_ptr += 1;
        }
        else if a_finger_ptr<b.len() &&
            ((b_ptr<b.len() && b[b_ptr] >= a[calculate_finger(a_finger_ptr)])
             || b_ptr>=b.len()) {
                 out.extend_from_slice(& finger_merge(
                         &a[a_prev_ptr .. calculate_finger(a_finger_ptr)],
                         &b[b_prev_ptr .. b_ptr]
                         ));
                 out.push(a[calculate_finger(a_finger_ptr)].clone());

                 a_prev_ptr = calculate_finger(a_finger_ptr)+1;
                 a_finger_ptr += 1;
                 b_prev_ptr = b_ptr;
             }
        else if a_finger_ptr>=b.len() {

            out.extend_from_slice(& finger_merge(
                    &a[a_prev_ptr.. ],
                    &b[b_prev_ptr .. ]
                    ));
            break;
        }
        else {
            panic!("This shouldn't happen.");
        }
    }

    //assert!(out.len() == a.len() + b.len(), "out.len={} a.len={} b.len={}", out.len(), a.len(), b.len());
    return out; 
}


fn test_finger_merge<T: Ord + Clone+ std::fmt::Debug>(mut a: Vec<T>,mut  b: Vec<T>) -> bool {
    a.sort();
    b.sort();
    let merged = finger_merge(&a[..], &b[..]);

    let mut brute_merged = a.clone();
    brute_merged.extend_from_slice(&b[..]);
    brute_merged.sort();


    brute_merged==merged
}


fn main() {
    quickcheck(test_finger_merge::<i32> as fn(Vec<i32>, Vec<i32>)->bool);
}
