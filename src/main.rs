use std::io;
//use std::num::Float;
use std::f64::INFINITY;
use std::process::exit;
use std::collections::HashSet;
struct Dejkstra{
  matrix: Vec<Vec<f64>>,
  t: HashSet<usize>,
  d: Vec<usize>,
  target: usize,
}

impl Dejkstra{
// l -- length, t -- target
// [something as type]
  fn new(l: usize,t:usize)->Dejkstra{
    let mut result = Dejkstra{
      matrix: vec![vec![0.0; l];l],
      t: HashSet::new(),
      d: vec![t;l],
      target: t
    };
    for x in 0..l {
      result.t.insert(x as usize);
    }
    result.t.remove(&result.target);
    result
  }
// let fill matrix from console
  fn fill_from_console(&mut self){
    let mut buffer = String::new();
    for i in  0..self.matrix.len() {
      for j in (i + 1)..self.matrix.len() {
        println!("[{}][{}]:",i + 1,j + 1);
        io::stdin().read_line(&mut buffer).ok().expect("Ошибка ввода");
        let mut number: f64 = buffer.trim().parse().ok().expect("Это не число"); buffer.clear();
        if number == -1.0 {
          number = INFINITY;
        }
        if number.is_sign_positive() {
          self.matrix[i][j] = number;
        }
        else{
          panic!("Введено отрицательное число");
        }
      }
    }
  }
// just to debug
  fn dumb_print(&self){
      for i in 0..self.matrix.len(){
          for _ in 0..(i+1){
              print!("  ");
          }
          for j in (i+1)..self.matrix.len(){
              print!("{} ",self.matrix[i][j]);
          }
          print!("\n");
      }
  }
  fn get_min(&self) -> (i64,f64){
      let mut value:f64 = INFINITY;
      let mut position:i64 = -1;
      for i in self.t.iter(){
          let current = self.matrix[self.target][*i];
          if current < value {
              value = current;
              position = *i as i64;
          }
      }
      (position,value)
  }
  fn algo(&self){
  //  let value = self.matrix[self.target].iter().fold(std::f64::INFINITY, |old, new| { if old < *new { old } else { *new } }).zip(self.t.into_iter());
    let (position,value) = self.get_min();
    println!("{} -- {}", position,value);
  }
}

fn main() {
    //    let foo: Vec<Vec<f64>> = vec![vec![1.0],vec![2.0]];
    //    println!("Hello, world!{}",foo[0][0]);
    let mut buffer = String::new();
    println!("Введите число вершин графа:");
    io::stdin().read_line(&mut buffer).ok().expect("Ошибка ввода");
    let number_of_vertexes: usize = buffer.trim().parse().ok().expect("Это не число");buffer.clear();
//    println!("Число вершин: {}", number_of_vertexes);
    if number_of_vertexes < 3 {
        println!("Число вершин не может быть меньше 3");
        exit(-1i32);
    }
    println!("Введите номер целевой вершины:");
    io::stdin().read_line(&mut buffer).ok().expect("Ошибка ввода");
    let target_vertex: usize = buffer.trim().parse().ok().expect("Это не число");
    let mut dejkstra = Dejkstra::new(number_of_vertexes, target_vertex - 1);
//    println!("{}", dejkstra.target);
    dejkstra.fill_from_console();
    dejkstra.dumb_print();
    dejkstra.algo();
}
