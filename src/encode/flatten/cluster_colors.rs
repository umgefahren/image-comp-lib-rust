use std::collections::HashMap;

pub fn flatten_cc(hmap: &HashMap<u8, [u8; 3]>) -> Vec<u8> {
    let mut ret = vec![];
    let keys: Vec<u8> = hmap.keys().map(|i| i.to_owned()).into_iter().collect();
    for key in keys.iter() {
        ret.push(key.clone());
        let color = hmap.get(key).unwrap().to_owned();
        for c in color.iter() {
            ret.push(c.to_owned());
        }
    }
    ret
}