use std::io;
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
                    self.matrix[j][i] = number;
                }
                else{
                    panic!("Введено отрицательное число");
                }
            }
        }
    }
    // just to debug
    fn dumb_above_the_diagonal_print(&self){
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
    fn dumb_print(&self){
        for i in 0..self.matrix.len(){
            for j in 0..self.matrix.len(){
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
    fn algo(&mut self){
        //  let value = self.matrix[self.target].iter().fold(std::f64::INFINITY, |old, new| { if old < *new { old } else { *new } }).zip(self.t.into_iter());
        while !self.t.is_empty() {
            let (position,value) = self.get_min();
            if position == -1 {
                panic!("Something wrong!Your vertex haven't path to others");
            }
            self.t.remove(&(position as usize));// удаляем ближайшую вершину из множества всех
            for i in self.t.iter() {
                let other_path: f64 = (value as f64) + self.matrix[position as usize][*i];
                if other_path < self.matrix[self.target][*i]{
                    self.matrix[self.target][*i] = other_path;
                    self.d[*i] = position as usize;
                }
            }
        }
    }
}

fn main() {
// TODO: переписать вывод, слишком уж некрасиво с inf и в принципе неровно
    let mut buffer = String::new();
    println!("Введите число вершин графа:");
    io::stdin().read_line(&mut buffer).ok().expect("Ошибка ввода");
    let number_of_vertexes: usize = buffer.trim().parse().ok().expect("Это не число");buffer.clear();
    if number_of_vertexes < 3 {
        println!("Число вершин не может быть меньше 3");
        exit(-1i32);
    }
    println!("Введите номер целевой вершины:");
    io::stdin().read_line(&mut buffer).ok().expect("Ошибка ввода");
    let target_vertex: usize = buffer.trim().parse().ok().expect("Это не число");
    let mut dejkstra = Dejkstra::new(number_of_vertexes, target_vertex - 1);
    dejkstra.fill_from_console();
    dejkstra.dumb_above_the_diagonal_print();
    dejkstra.algo();
    dejkstra.dumb_print();
}
