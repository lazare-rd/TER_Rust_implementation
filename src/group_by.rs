pub fn  group_by(keys: &[char], values: &[i32]) -> Vec<(char, i32)>{
    if keys.len() != values.len() {
        panic!("Error in parameter size");
    }
    let mut result: Vec<(char, i32)> = Vec::new();
    for n in 0..keys.len(){
        let index = result.iter().position(|&r|  r.0 == keys[n]);
        match index {
            Some(i) => result[i].1 += values[n],
            None => result.push((keys[n], values[n])),
        }
    }
    return result;
}

#[cfg(test)]
mod test {
    use super::* ;

    #[test]
    fn group_by_test() {
        let keys = ['A', 'B', 'C', 'A', 'A', 'B', 'C', 'C'];
        let val = [12, 4, 5, 7, 9, 6, 8, 6];
        let result = group_by(&keys, &val);
        assert_eq!(result, [('A', 28), ('B', 10), ('C', 19)]);
    }
}