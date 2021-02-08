use md5::Digest;

pub fn day04() {
    let secret = "iwrupvqb".as_bytes();
    println!("{}", find_proof(secret));
}

fn find_proof(seed: &[u8]) -> i32 {
    for i in 1.. {
        let candidate = [seed, i.to_string().as_bytes()].concat();
        let digest = md5::compute(candidate);
        if prefix_check(&digest) {
            return i
        }
    }
    panic!()
}

fn prefix_check(digest: &Digest) -> bool {
    // digest[0] == 0 && digest[1] == 0 && digest[2] >> 4 == 0
    digest[0] == 0 && digest[1] == 0 && digest[2] == 0
}

#[test]
fn examples1() {
    let seed = "abcdef".as_bytes();
    let candidate = [seed, 609043.to_string().as_bytes()].concat();
    assert!(prefix_check(&md5::compute(&candidate)));

    let seed = "pqrstuv".as_bytes();
    let candidate = [seed, 1048970.to_string().as_bytes()].concat();
    assert!(prefix_check(&md5::compute(&candidate)));
}