mod sieve;
use sieve::core::*;


fn main() {
    //Create a vec of integers from 2 to n(30)
    let mut v = init_range(50);

    //Find the prime numbers by running the algorithm
    let primes = run_sieve(&mut v, 50);

    //Print all the prime numbers
    /* NOTE: By using iter().enumerate() we can get the number
       of iterations as i and current element as j
    */
    for (i, j) in primes.iter().enumerate(){
        //If i is divisible by 7 we append a newline        
        if i % 7 == 0{
            println!()
        }
        if let Some(x) = j{
            print!("{0:>5}", x);
        }else{
            print!("{0:>5}", "-");
        }
    }
    println!()
}
