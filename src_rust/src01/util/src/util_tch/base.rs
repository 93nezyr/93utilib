use tch::Tensor;
use anyhow::Result;
use csv::Writer;

/// デバッグ用関数
#[allow(dead_code)]
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

/// tch::Tensorを二次元に変換(二次元目以降がflattenされる)し、csvに書き込みます.
pub fn tensor_to_csv(tensor: &Tensor, path: String) -> Result<()> {
    // tensorをVecに変換します.
    let t: Tensor;
    let shape = tensor.size();
    if shape.len() != 2 {
        t = tensor.copy().flatten(1, (shape.len() - 1) as i64);
    } else {
        t = tensor.copy();
    }
    let reshape = t.size();
    let v = Vec::<f64>::from(&t);
    let v: Vec<_> = v.chunks(*reshape.last().unwrap() as usize).collect();
    // println!("{0:?}", v);
    // print_type_of(&v);

    // Vecをcsvに保存します.
    let col_size = *reshape.last().unwrap() as usize;
    let mut wtr = Writer::from_path(&path)?;
    // Vecの各行ごとに処理を行う
    for row in v.iter() {
        let mut vm = row.to_vec();
        // 列数が異なる場合は0埋め
        if vm.len() != col_size {
            let dif = (vm.len() - col_size) as isize;
            let dif = dif.abs() as usize;
            for _i in 0..dif {
                vm.push(0.0);
            }
        }
        // write_recordのために文字列に変換
        let vm = vm.iter().map(|e| e.to_string()).collect::<Vec<String>>();
        wtr.write_record(&vm)?;
    }
    wtr.flush()?;

    Ok(())
}

pub fn csv_to_tensor(path: String) -> Result<Tensor> {
    let mut rdr = csv::Reader::from_path(&path)?;
    let mut v = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let mut row = Vec::new();
        for field in record.iter() {
            row.push(field.parse::<f64>()?);
        }
        v.push(row);
    }
    let t = Tensor::of_slice2(v.as_slice()).to_kind(tch::Kind::Float);
    Ok(t)
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_tensor_to_csv() {
        let v = vec![vec![1,2,3], vec![2,3,4], vec![3,4,5]];
        let t = Tensor::of_slice2(v.as_slice());
        t.print();
        tensor_to_csv(&t, "/workspaces/src01/test.csv".to_string()).unwrap();
    }

    #[test]
    fn test_csv_to_tensor() {
        let t = csv_to_tensor("/workspaces/src01/test.csv".to_string()).unwrap();
        t.print();
    }
}
