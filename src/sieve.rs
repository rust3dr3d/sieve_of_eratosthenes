pub mod core{
    /*                      ** Sieve of Eratosthenes Algorithm **
        1. create an array of integers from 2 to N where N = Upper bound of the search (lets say 1000)
        2. Initialize a variable called P and initialize it to the first element and the smallest prime no: 2
        3. Iterate through the array and mark the multiples of P except p(4, 6, 8, 10...) until you reach N
        4. Find the next smallest greater than P which isn't market yet. If there's no such number, Stop.
        5. Otherwise repeat from step 3

        When algorithm terminates, remaining numbers which aren't marked are prime numbers.
    */


    /*                      ** init_range() Function **
        First, we are creating a vector and filling it with integers from 2 to n
        
        Note that we're using Vec<Option<usize>>. Reason for this is we have to mark
        the multiples of P(Step 3 above). We can mark the multiples as None and the unmarked elements
        will remain as Some(n).
        for ex: After marking multiples of P where p = 2, our vec will look like this..
        [Some(2), Some(3), None, Some(5), None,...]

        Note: usize in a 32bit target will occupy 4 bytes whereas on a 64bit it'll occupy 8bytes.
    */
    pub fn init_range(n:usize) -> Vec<Option<usize>>{
        //Initialize a new vector as mutable because we have mark some of its elements to None
        let mut range:Vec<Option<usize>> = Vec::new();

        for i in 0..n-1{
            //First element will be Some(P+i) = Some(2 + 0) = 2,
            //Second element will be Some(2+i) = Some(2 + 1) = 3,
            //Third 2+2 = 4, so forth and so on..
            range.push(Some(2+i));
        }

        //We return the vector so we can run the algorithm on it from "run_sieve()" function
        range
    }


    /*
                            ** run_sieve() Function **
        
    */
    pub fn run_sieve(range:&mut Vec<Option<usize>>, n:usize) -> Vec<Option<usize>>{
        /* As explained in the algorithm above(Step 1), we initialize P to 2 since its the first
           element in the vector
        */
        let mut p:usize = 2;

        /*
            According the algorithm, we have to mark the multiples of P (2p, 3p, 4p...) until
            there's nothing left to mark. While loop will run until until P * P < n
            for ex: if n = 30, and if P is 4, it will still run because 4 * 4 < 30
            but if P is 6, then 6 * 6 < 30 becomes false, terminating the while loop.
        */
        while p * p < n{
            /*
                Inner for loop will iterate over each element in the vector(range)
                Note "range.iter_mut()". We need to iterate over each element but
                also need to modify where necessary by marking some elements to "None".
                .iter_mut(&mut self) will enable us to do that.
                https://doc.rust-lang.org/std/vec/struct.Vec.html#method.iter_mut
            */
            for i in range.iter_mut(){
                /* If current element i is equal to the smallest multiple of current P
                   continue to the next iteration.
                   For ex: if P = 2, and current element i = Some(2), we don't want to mark it
                   if P = 3, don't mark..
                   if P = 4, don't mark but its already marked in a prev iteration when P was 2

                   NOTE: we have dereference the element i as *i because its a mutable reference...
                   so that we can compare it with Some(p)
                */              
                if Some(p) == *i {continue}

                /*
                    If current element i is a multiple of P and not marked as None,
                    if let statement will provide us the unwrapped value of that element as x
                */
                if let Some(x) = *i {
                /*  
                    If x is divisible by current P without any remainder, mark it as None(Step 3)
                    for ex: if P = 2 and the x value is 4, 4 % 2 = 0 therefore, mark it as None
                    Remember to dereference i as it is a mutable reference to the current element.
                */
                   if x % p == 0{
                    *i = None;
                   }
                }
            }

            /*
                    According to the algorithm(Step 4) we need to assign P to the next smallest
                    number which isn't marked yet.
                    For ex: If current P is 2, we marked: 4, 6, 8...30
                    Now we increment P by 1 therefore P will be 3
                    Next iteration will mark: 6(already marked), 9, 12(already marked), 15...            
            */
            p+=1;
        }

        /*
            After while loop is terminated, we return a new copy of the vector
            from the mutable reference - range
            Now we can use that to print out the resulting vector
            which should only contain PRIME NUMBERS.
        */
        range.to_vec()
    }

}


/*

fn main(){
    
    let mut p = 2_usize;
    let n  = 30_usize;
    let mut v:Vec<usize> = Vec::new();
    
    
    for i in 0..n-1{
        v.push(p+i);
    }
    

    while p * p < n{
       for i in v.iter_mut(){
            if p == *i as usize{continue;}
            else if *i % p == 0 {*i = 0 as usize}
        } 
        p +=1;
    }

    
    for i in v.iter(){
        println!("{}", i);
    }
}

*/