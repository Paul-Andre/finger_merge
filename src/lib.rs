/// Merges two sorted arrays into one.
///
/// This is a generalization of the standard merge and of binary search and should run in
/// `O(m*log(1+n/m))` time, where `n` is the size of the bigger array and `m` the size of the
/// smaller one.
///
/// It works by separating the big array into equal parts using m "fingers", merges the small array
/// with the fingers to figure out in which subarray each element should be merged in. Then it
/// merges elements into that subarray recursively.
pub fn finger_merge<T: Ord + Clone >(in_a: &[T], in_b: &[T]) -> Vec<T> {
    let a: &[T];
    let b: &[T];

    // Make sure that a is bigger than b.
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
        if a_finger_ptr<b.len() && b_ptr<b.len() && b[b_ptr]<a[calculate_finger(a_finger_ptr)] {
            b_ptr += 1;
        }
        else if a_finger_ptr<b.len() &&
            (b_ptr>=b.len() || b[b_ptr] >= a[calculate_finger(a_finger_ptr)])
        {
            out.extend_from_slice(& finger_merge(
                    &a[a_prev_ptr .. calculate_finger(a_finger_ptr)],
                    &b[b_prev_ptr .. b_ptr]
                    ));
            out.push(a[calculate_finger(a_finger_ptr)].clone());

            a_prev_ptr = calculate_finger(a_finger_ptr)+1;
            a_finger_ptr += 1;
            b_prev_ptr = b_ptr;
        }
        else {

            out.extend_from_slice(& finger_merge(
                    &a[a_prev_ptr.. ],
                    &b[b_prev_ptr .. ]
                    ));
            break;
        }
    }

    return out; 
}

#[cfg(test)]
mod tests {
    use super::finger_merge;

    extern crate quickcheck;

    use self::quickcheck::quickcheck;

    fn test_finger_merge_with_one_input<T: Ord + Clone>
     (mut a: Vec<T>,mut  b: Vec<T>) -> bool {
        a.sort();
        b.sort();
        let merged = finger_merge(&a[..], &b[..]);

        let mut brute_merged = a.clone();
        brute_merged.extend_from_slice(&b[..]);
        brute_merged.sort();


        brute_merged==merged
    }


    #[test]
    fn finger_merge_quickcheck_i32() {
        quickcheck(test_finger_merge_with_one_input::<i32> as fn(Vec<i32>, Vec<i32>)->bool);
    }
}
