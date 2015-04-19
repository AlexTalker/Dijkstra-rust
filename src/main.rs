use std::collections::HashSet;
struct Dejkstra{
  matrix: Vec<Vec<f64>>,
  t: HashSet<i64>,
  d: Vec<i64>,
  target: i64,
}

impl Dejkstra{
// l -- length, t -- target
// [something as type]
  fn new(l: usize,t:i64)->Dejkstra{
    let mut result = Dejkstra{
      matrix: vec![vec![0.0; l];l],
      t: HashSet::new(),
      d: vec![t;l],
      target: t
    };
    for x in 0..l {
      result.t.insert(x as i64);
    }
    result.t.remove(&result.target);
    result
  }
}

fn main() {
    let foo: Vec<Vec<f64>> = vec![vec![1.0],vec![2.0]];
    println!("Hello, world!{}",foo[0][0]);
}
