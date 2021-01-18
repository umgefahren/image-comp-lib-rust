use std::collections::HashMap;

pub fn create_cluster_colors(inp: &[u8]) -> HashMap<u8, [u8; 3]> {
    let mut cc_list_iter = inp.iter();
    let mut ret = HashMap::new();
    for _n in 0..(inp.len() / 4) {
        let key = cc_list_iter.next().unwrap().to_owned();
        let r = cc_list_iter.next().unwrap().to_owned();
        let g = cc_list_iter.next().unwrap().to_owned();
        let b = cc_list_iter.next().unwrap().to_owned();
        ret.insert(key, [r, g, b]);
    }
    ret
}