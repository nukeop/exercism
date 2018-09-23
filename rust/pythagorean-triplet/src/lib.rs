pub fn find() -> Option<u32> {
    for m in 1..100 {
        for n in 1..100 {
            if (m <= n){
                continue;
            }
            
            let a = m*m - n*n;
            let b = 2*m*n;
            let c = m*m + n*n;

            if a+b+c == 1000 {
                return Some(a*b*c);
            }
        }
    }

    return None;
}
