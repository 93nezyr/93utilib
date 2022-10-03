use util::tensor_to_csv;
use tch::Tensor;
use std::time::Instant;

fn main() {
    println!("Hello, world!");
    let mut vs = Vec::new();
    for i in 0..1000 {
        let v = vec![i; 1000];
        vs.push(v);
    }
    let t = Tensor::of_slice2(vs.as_slice());
    // t.print();
    let start = Instant::now();
    tensor_to_csv(&t, "test.csv".to_string()).unwrap();
    let end = start.elapsed();
    println!("elapsed time: {}.{:03}sec.", end.as_secs(), end.subsec_nanos() / 1000000);
}
