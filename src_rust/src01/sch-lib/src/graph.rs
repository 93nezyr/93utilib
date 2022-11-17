/// ## About
///
/// 3次元のVecを1次元方向に走査し、変化点の情報のみを抽出して返す関数です.
///
/// ## Arguments
///
/// - `v` - `Vec<Vec<Vec<f64>>>`型の3次元のVec
///
/// ## Return
///
/// - `Vec<Vec<Vec<f64>>>`型の変化点のみを抽出した3次元のVec
pub fn extract_vec3d_change_point(v: Vec<Vec<Vec<f64>>>) -> Vec<Vec<Vec<f64>>> {
    let mut result: Vec<Vec<Vec<f64>>> = Vec::new();
    for ix_t in 0..v.len() {
        if ix_t == 0 {
            result.push(v[ix_t].clone());
        } else {
            if v[ix_t] != v[ix_t - 1] {
                result.push(v[ix_t].clone());
            }
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_extract_vec3d_change_point() {
        let mut v = vec![];
        for t in 0..100 {
            let mut v2 = vec![];
            for _r in 0..8 {
                let mut v3 = vec![];
                for _e in 0..4 {
                    v3.push((t / 10) as f64);
                }
                v2.push(v3);
            }
            v.push(v2);
        }

        println!("v");
        for r in v.iter() {
            println!("{:?}", r);
        }

        let result = extract_vec3d_change_point(v);

        println!("result");
        for r in result.iter() {
            println!("{:?}", r);
        }
    }
}
