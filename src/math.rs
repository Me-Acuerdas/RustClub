use super::test;
fn primes(data: u64) -> Vec<u64>{
    vec![]
}//End Function
// struct Fibo {
//     current_value : u64,
//     previous_value : u64,
//     next_value: u64,
//     first_value: u64,
//     second_value: u64,
// }//End Fibo



fn fibo(data: u64) -> u64{
    // let count = Fibo{
    //     current_value : 0,
    //     previous_value : 0,
    //     next_value: 0,
    //     first_value: 0,
    //     second_value: 1,
    // };//End Let
    // for reps in (0..data){
    //     if reps < 0 {
    //
    //     }//End If
    // }//End For loop
    // count.current_value

    if data == 0{
        0
    }
    else if data == 1 {
        1
    }
    else {
        fibo (data - 1) +fibo(data - 2)
    }
}
fn gcd (data: u64, data2: u64) -> u64{
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::test::Bencher;

    mod primes {
        use super::*;

        #[bench] fn bench_large(b: &mut Bencher) {
            b.iter(|| primes(1000000))
        }

        #[test] fn api() {
            let _ : Vec<u64> = primes(0u64);
        }

        #[test] fn small() {
            assert_eq!(primes(10), [2, 3, 5, 7]);
            assert_eq!(primes(100), [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59,
                                     61, 67, 71, 73, 79, 83, 89, 97]);
        }

        #[test] fn large() {
            let large_primes_list = primes(10000000);
            let len = large_primes_list.len();
            assert_eq!(&large_primes_list[len-10..len], [9999889, 9999901, 9999907, 9999929, 9999931,
                9999937, 9999943, 9999971, 9999973, 9999991]);
        }
    }

    mod fibo {
        use super::*;

        #[bench] fn bench_large(b: &mut Bencher) {
            b.iter(|| fibo(93))
        }

        #[test] fn api() {
            let _ : u64 = fibo(0u64);
        }

        #[test] fn small() {
            assert_eq!(fibo(0), 0);
            assert_eq!(fibo(1), 1);
            assert_eq!(fibo(2), 1);
            assert_eq!(fibo(3), 2);
            assert_eq!(fibo(19), 4181);
            assert_eq!(fibo(20), 6765);
        }

        #[test] fn large() {
            assert_eq!(fibo(69), 117_669_030_460_994);
            assert_eq!(fibo(93), 12_200_160_415_121_876_738);
        }
    }

    mod gcd {
        use super::*;

        #[bench] fn bench_large(b: &mut Bencher) {
            b.iter(|| gcd(12_200_160_415_121_876_738, 7_540_113_804_746_346_429))
        }

        #[test] fn api() {
            let _ : u64 = gcd(1u64, 1u64);
        }

        #[test] fn small() {
            assert_eq!(gcd(1, 2), 1);
            assert_eq!(gcd(6, 2), 2);
            assert_eq!(gcd(2, 6), 2);
            assert_eq!(gcd(100, 60), 20);
            assert_eq!(gcd(13, 97), 1);
        }

        #[test] fn large() {
            assert_eq!(gcd(9999889, 9999991), 1);
            let p = 9999889;
            assert_eq!(gcd(p, p * 12345), p);
        }
    }
}
