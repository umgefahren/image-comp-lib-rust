use bytes::Bytes;

pub fn create_list(inp: Vec<u8>) -> Vec<[u8; 3]> {
    let mut iter = inp.iter();
    let mut res = vec![];
    loop {
        let r = match iter.next() {
            Some(d) => *d,
            None => {
                break
            }
        };
        let g = match iter.next() {
            Some(d) => *d,
            None => {
                break
            }
        };
        let b = match iter.next() {
            Some(d) => *d,
            None => {
                break
            }
        };
        res.push([r, g, b]);
    }
    res
}

pub fn list_f_bytes(inp: &Bytes) -> Vec<u8> {
    inp.to_owned().to_vec()
}