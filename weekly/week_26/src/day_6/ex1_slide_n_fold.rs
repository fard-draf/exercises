pub fn analyze_sliding_windows(data: &[u32], window_size: usize) -> Vec<(usize, u32, f64)> {
    if window_size == 0 || window_size > data.len() {
        return Vec::new();
    }

    let init = (Vec::<(usize, u32,f64)>::new(), (0usize, 0u32, 0f64), (false, 0)); //first, somme, moyenn
    let index_view = window_size - 1;

    let (vec, _, _) = 
        data.iter()
            .enumerate()
            .fold(init, 
                |(mut acc, mut temp_acc,( mut is_first_view_indx, mut start)), (mut index, item)| {

        println!("");
        println!("Entry n{} index view {}", index+1, index_view);
        
        let window_scale = (window_size - 1) + temp_acc.0;

        if index == 0 {
            temp_acc.0 = index;

        } else if is_first_view_indx {
            index = start;
            temp_acc.0 = index;
            is_first_view_indx = false;
            println!("INDEX {} / TEMPACC.0 {}", index, temp_acc.0);
            println!("Windows scale = {}", window_scale);
            println!("");
        } 
        
        if index <= window_scale {
            temp_acc.1 += item;
            temp_acc.2 += 1.0;
            println!("acc.1 = {}, acc.2 = {}", temp_acc.1, temp_acc.2);
            println!("");
        } 
        
        if  index == window_scale {
            temp_acc.2 = temp_acc.1 as f64 / temp_acc.2;
            acc.push(temp_acc);
            
            temp_acc.0 += 1;
            start = temp_acc.0;
            is_first_view_indx = true;

            temp_acc.1 = 0;
            temp_acc.2 = 0.0; // RESET TEMP_ACC
        }


        
        
        println!("acc {:?}, start: {} ", acc, start);
    (acc, temp_acc, (is_first_view_indx, start))
    });


    vec

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nominal_case() {
        let data = &[10, 20, 30, 40, 50, 60];
        let window_size = 3;
        let expected = vec![
            (0, 60, 20.0),
            (1, 90, 30.0),
            (2, 120, 40.0),
            (3, 150, 50.0),
        ];
        assert_eq!(analyze_sliding_windows(data, window_size), expected);
    }

    #[test]
    fn test_empty_data() {
        let data = &[];
        let window_size = 3;
        assert!(analyze_sliding_windows(data, window_size).is_empty());
    }

    #[test]
    fn test_window_size_zero() {
        let data = &[1, 2, 3];
        let window_size = 0;
        assert!(analyze_sliding_windows(data, window_size).is_empty());
    }

    #[test]
    fn test_window_size_one() {
        let data = &[10, 20, 30];
        let window_size = 1;
        let expected = vec![(0, 10, 10.0), (1, 20, 20.0), (2, 30, 30.0)];
        assert_eq!(analyze_sliding_windows(data, window_size), expected);
    }

    #[test]
    fn test_window_size_equals_data_len() {
        let data = &[10, 20, 30];
        let window_size = 3;
        let expected = vec![(0, 60, 20.0)];
        assert_eq!(analyze_sliding_windows(data, window_size), expected);
    }

    #[test]
    fn test_window_size_greater_than_data_len() {
        let data = &[10, 20, 30];
        let window_size = 5;
        assert!(analyze_sliding_windows(data, window_size).is_empty());
    }
}