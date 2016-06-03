//use std::debu;
// merges two sorted arrays into one
// this is a generalization of the standard merge and a binary search
fn finger_merge<T: Ord + Clone + std::fmt::Debug>(in_a: &[T], in_b: &[T]) -> Vec<T> {
            println!("mergere, {:?} {:?}", in_a, in_b);
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
    println!("return {:?}", out);
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
    match (a_finger_ptr<b.len() , b_ptr<b.len()) {
        (true, true) => {
        if b[ b_ptr ] < a[ calculate_finger(a_finger_ptr) ] {
        println!("b+=1, {}({}), {}, {:?}, {:?}", a_finger_ptr, calculate_finger(a_finger_ptr),b_ptr, a, b);
            b_ptr += 1;
        }
        else {
        println!("a+=1, {}({}), {}, {:?}, {:?}", a_finger_ptr, calculate_finger(a_finger_ptr),b_ptr, a, b);
            out.extend_from_slice(& finger_merge(
                    &a[a_prev_ptr .. calculate_finger(a_finger_ptr)],
                    &b[b_prev_ptr .. b_ptr]
                    ));
            out.push(a[calculate_finger(a_finger_ptr)].clone());

            a_prev_ptr = calculate_finger(a_finger_ptr)+1;
            a_finger_ptr += 1;
            b_prev_ptr = b_ptr;
        }
    },
    (true,false)  => {
            out.extend_from_slice(& finger_merge(
                    &a[a_prev_ptr .. calculate_finger(a_finger_ptr)],
                    &b[b_prev_ptr .. b_ptr]
                    ));
            out.push(a[calculate_finger(a_finger_ptr)].clone());

            a_prev_ptr = calculate_finger(a_finger_ptr)+1;
            a_finger_ptr += 1;
            b_prev_ptr = b_ptr;

    println!("other 1, {}({}), {}, {:?}, {:?}", a_finger_ptr, calculate_finger(a_finger_ptr),b_ptr, a, b);
    out.extend_from_slice(& finger_merge(
            &a[a_prev_ptr.. ],
            &b[b_prev_ptr .. ]
    ));

    break;

    },
    (false ,true)  => {

    println!("other 2, {}({}), {}, {:?}, {:?}", a_finger_ptr, calculate_finger(a_finger_ptr),b_ptr, a, b);
    out.extend_from_slice(& finger_merge(
            &a[a_prev_ptr.. ],
            &b[b_prev_ptr .. ]
    ));

    break;

    },
    _ => {
        panic!();
    }
    
    }
    }

    println!("return s sf {:?}", out);
    assert!(out.len() == a.len() + b.len(), "out.len={} a.len={} b.len={}", out.len(), a.len(), b.len());
    return out; 
}

fn main() {
    let a = vec![0,2,4,6];
    let b = vec![1,3,5,7,9,11];
    let merged = finger_merge(&a[..], &b[..]);
    println!("merged: {:?}", merged);
}
