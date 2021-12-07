const TEST : [usize;5]   = [3,4,3,1,2];
const INPUT: [usize;300] = [4,5,3,2,3,3,2,4,2,1,2,4,5,2,2,2,4,1,1,1,5,1,1,2,5,2,1,1,4,4,5,5,1,2,1,1,5,3,5,2,4,3,2,4,5,3,2,1,4,1,3,1,2,4,1,1,4,1,4,2,5,1,4,3,5,2,4,5,4,2,2,5,1,1,2,4,1,4,4,1,1,3,1,2,3,2,5,5,1,1,5,2,4,2,2,4,1,1,1,4,2,2,3,1,2,4,5,4,5,4,2,3,1,4,1,3,1,2,3,3,2,4,3,3,3,1,4,2,3,4,2,1,5,4,2,4,4,3,2,1,5,3,1,4,1,1,5,4,2,4,2,2,4,4,4,1,4,2,4,1,1,3,5,1,5,5,1,3,2,2,3,5,3,1,1,4,4,1,3,3,3,5,1,1,2,5,5,5,2,4,1,5,1,2,1,1,1,4,3,1,5,2,3,1,3,1,4,1,3,5,4,5,1,3,4,2,1,5,1,3,4,5,5,2,1,2,1,1,1,4,3,1,4,2,3,1,3,5,1,4,5,3,1,3,3,2,2,1,5,5,4,3,2,1,5,1,3,1,3,5,1,1,2,1,1,1,5,2,1,1,3,2,1,5,5,5,1,1,5,1,4,1,5,4,2,4,5,2,4,3,2,5,4,1,1,2,4,3,2,1];

fn main() {
    // the chum buckets (we count the number of fish in each one)
    let mut chum : [u64;9]=[0;9];
    for f in 0..INPUT.len() {
        if INPUT[f]>8 { println!("oops, fish count >8"); return; }
        chum[INPUT[f]] += 1;
    }

    // breed some fish...
    for day in 0..256 {
        // save count of chum[0] fish..
        let zero = chum[0];
        // copy down all remaining buckets.. (and count total fish)
        let mut total : u64 = zero;
        for b in 0..8 {
            chum[b] = chum[b+1];
            total += chum[b];
        }
        // add chum[0] count from above to bucket 6, and re-create bucket 8 (new fish)
        chum[6] += zero;
        chum[8] = zero;
        println!("day:{} count:{}", day, total+zero);
    }
}