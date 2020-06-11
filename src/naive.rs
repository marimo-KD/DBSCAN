pub const NOISE: i32 = -1;
pub const NOT_CLASSIFIED: i32 = -10;

fn regionquery(data: &Vec<(f64, f64)>, eps: f64, point: usize) -> Vec<usize> {
    let mut res: Vec<usize> = Vec::new();
    let (x, y) = data[point];
    for (i, (nx, ny)) in data.iter().enumerate() {
        let d = ((nx - x) * (nx - x) + (ny - y) * (ny - y)).sqrt();
        if d <= eps {
            res.push(i);
        }
    }
    return res;
}

fn expand_cluster(
    data: &Vec<(f64, f64)>,
    neighbor: &mut Vec<usize>,
    res: &mut Vec<i32>,
    point: usize,
    cluster: i32,
    eps: f64,
    minpts: usize,
) {
    res[point] = cluster;
    while let Some(np) = neighbor.pop() {
        if res[np] == NOT_CLASSIFIED {
            res[np] = cluster;
            let new_neighbor = regionquery(data, eps, np);
            if new_neighbor.len() >= minpts {
                for i in new_neighbor {
                    if res[i] == NOT_CLASSIFIED {
                        neighbor.push(i);
                    }
                }
            }
        }
    }
}

pub fn dbscan(data: &Vec<(f64, f64)>, eps: f64, minpts: usize) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::with_capacity(data.len());
    res.resize(data.len(), NOT_CLASSIFIED);
    let mut count = 0;
    for (i, _) in data.iter().enumerate() {
        if res[i] != NOT_CLASSIFIED {
            continue;
        }
        let mut neighbor = regionquery(data, eps, i);
        if neighbor.len() < minpts {
            res[i] = NOISE;
        } else {
            expand_cluster(data, &mut neighbor, &mut res, i, count, eps, minpts);
            count += 1;
        }
    }
    return res;
}
