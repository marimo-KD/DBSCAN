use proconio::input;
mod naive;

fn main() {
    input! {
        n : u32,
        data : [(f64,f64);n]
    }
    println!("{}", n);
    let result = naive::dbscan(&data, 10.0, 5);
    println!("NOISE:{}", result.iter().filter(|x| { x == &&-1 }).count());
    println!("Num of Cluster:{}", result.iter().max().unwrap());
    println!("No.0:{}", result.iter().filter(|x| (x == &&0)).count());
    println!("No.1:{}", result.iter().filter(|x| (x == &&1)).count());
    println!("No.2:{}", result.iter().filter(|x| (x == &&2)).count());
}
