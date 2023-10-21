pub mod lib {
    fn partition<T: PartialOrd>(data: &mut [T], from: usize, to: usize) -> usize {
        if data.len() < 2 {
            return 0usize;
        }

        //let pivot = data.last_mut().unwrap();
        let mut p_i = from;
        for i in from..to-1 {
            if data[i] <= *data.last().unwrap() {
                data.swap(i, p_i);
                p_i += 1;
            }
        }
        data.swap(p_i, to-1);

        p_i
    }
    pub fn quicksort<T: PartialOrd>(data: &mut [T]) {
        if data.len() < 2 {
            return;
        }
        let idx = partition(data, 0, data.len()); 

        quicksort(&mut data[0..idx]);
        let len = data.len();
        quicksort(&mut data[idx+1..len]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quicksort() {
        let mut v = vec![6, 99, 12, 12, 333, 1, 1024, 34];

        lib::quicksort(&mut v);

        println!("{:?}", &v);
        assert_eq!(v.len(), 8);
        assert_eq!(v, vec![1, 6, 12, 12, 34, 99, 333, 1024]);

        v = vec![10, 5];
        lib::quicksort(&mut v);
        assert_eq!(v, vec![5, 10]);
    }
}
