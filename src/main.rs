use proconio::input;
mod naive;
mod plot_graph;

fn main() {
    input! {
        n : u32,
        data : [(f64,f64);n]
    }
    println!("{}", n);
    let result = naive::dbscan(&data, 5.0, 4);
    println!("NOISE:{}", result.iter().filter(|x| { x == &&-1 }).count());
    println!("Num of Cluster:{}", result.iter().max().unwrap() + 1);
    plot_graph::render_data(&data, &result);
}
