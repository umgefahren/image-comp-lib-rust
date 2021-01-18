use bytes::Bytes;

pub fn flatten_list(list: &[[u8; 3]]) -> Vec<u8> {
    let mut res = vec![];
    for p in list {
        res.push(p[0]);
        res.push(p[1]);
        res.push(p[2]);
    }
    res
}

pub fn bytes_list(list: &Vec<u8>) -> Bytes {
    let clone = list.clone();
    Bytes::from(clone)
}