use std::fmt::Debug;

pub fn merge_sort<T: PartialOrd + Debug>(mut v: Vec<T>) -> Vec<T> {
    let mut res = Vec::with_capacity(v.len());
    // sort the left half
    // sort the right half         O(n*log(n))
    // merge the two sorted halves O(n)

    if v.len() <= 1 {
        return v;
    }

    let mut b = v.split_off(v.len() / 2);
    let a = merge_sort(v);
    let b = merge_sort(b);

    let mut a_it = a.into_iter();
    let mut b_it = b.into_iter();
    let mut a_peek = a_it.next();
    let mut b_peek = b_it.next();

    loop {
        match a_peek {
            Some(ref a_val) => match b_peek {
                Some(ref b_val) => {
                    if b_val < a_val {
                        res.push(b_peek.take().unwrap());
                        b_peek = b_it.next();
                    } else {
                        res.push(a_peek.take().unwrap());
                        a_peek = a_it.next();
                    }
                }
                None => {
                    res.push(a_peek.take().unwrap());
                    res.extend(a_it);
                    return res;
                }
            },
            None => {
                if let Some(b_val) = b_peek {
                    res.push(b_val);
                }
                res.extend(b_it);
                return res;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_merge_sorted() {
        let v = vec![1, 2, 3, 8, 9, 10, 4, 5, 6, 7];
        let res = merge_sort(v);
        // res.reverse();
        assert_eq!(res, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
