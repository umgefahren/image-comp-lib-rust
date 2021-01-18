use average::Mean;

pub fn calc_min(cluster: &[usize], points: &[[u32; 5]]) -> [u8; 3] {
    let mut ret_color: [u8; 3] = [255, 255, 255];
    for p in cluster {
        let color = [points[*p][2] as u8, points[*p][3] as u8, points[*p][4] as u8];
        if color[0] < ret_color[0] || color[1] < ret_color[1] || color[2] < ret_color[2] {
            ret_color = color
        }
    }
    ret_color
}

pub fn calc_mean(cluster: &[usize], points: &[[u32; 5]]) -> [i8; 3] {
    let mut r = vec![];
    let mut g = vec![];
    let mut b = vec![];
    for p in cluster {
        let color = [points[*p][2] as u8, points[*p][3] as u8, points[*p][4] as u8];
        r.push(color[0]);
        g.push(color[1]);
        b.push(color[2]);
    }
    let r_mm: Mean = r.iter().map(|i| *i as f64).collect();
    let g_mm: Mean = g.iter().map(|i| *i as f64).collect();
    let b_mm: Mean = b.iter().map(|i| *i as f64).collect();
    [r_mm.mean().round() as i8, g_mm.mean().round() as i8, b_mm.mean().round() as i8]
}