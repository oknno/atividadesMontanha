pub fn produto_de_matrizes(a: &[Vec<i64>], b: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let n = a.len();
    let mut c = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                c[i][j] = c[i][j] + a[i][k] * b[k][j];
            }
        }
    }

    c
}