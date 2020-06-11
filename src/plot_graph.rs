use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::style::{PointMarker, PointStyle};
use plotlib::view::ContinuousView;

fn xorshift64(mut x: u64) -> u64 {
    x ^= x << 13;
    x ^= x >> 5;
    x ^= x << 17;
    x
}

pub fn render_data(data: &Vec<(f64, f64)>, cluster: &Vec<i32>) {
    let n = cluster.iter().max().unwrap().to_owned() as usize;
    let mut points: Vec<Vec<(f64, f64)>> = vec![vec![]; n + 1];
    let mut noises: Vec<(f64, f64)> = Vec::new();
    for (i, c) in cluster.iter().enumerate() {
        let c = c.to_owned();
        if c == -1 {
            noises.push(data[i]);
            continue;
        }
        points[c as usize].push(data[i]);
    }
    let mut v = ContinuousView::new();
    for (i, vec) in points.iter().enumerate() {
        let p = Plot::new(vec.to_owned()).point_style(
            PointStyle::new()
                .marker(PointMarker::Cross)
                .colour(format!("#{:X<06}", xorshift64(i as u64))),
        );
        v = v.add(p);
    }
    let noises = Plot::new(noises).point_style(
        PointStyle::new()
            .marker(PointMarker::Cross)
            .colour("#000000"),
    );
    v = v.add(noises);

    Page::single(&v).save("sample.svg").unwrap();
}
