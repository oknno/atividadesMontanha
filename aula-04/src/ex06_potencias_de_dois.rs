pub fn potencias_de_dois(n: u64) {
    let mut i = 1;

    while i < n {
        println!("{}", i);
        i = i * 2;
    }
}