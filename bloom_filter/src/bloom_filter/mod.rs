use std::error::Error;
use std::io::Cursor;
use murmur3::murmur3_x86_128;

pub struct Bloom_filter
{
    pub m:u64,
    pub buffer:u128,
    pub probability:f64,
    pub num_of_hash:f64,
    pub expected_inputs:f64,
}


impl Bloom_filter
{
//  this is an associated function
    pub fn new(n:f64,p:f64)->Bloom_filter
    {
     let false_positivity=p;
     let expected_inputs=n;
     let number_of_bits:f64=(n*p.ln())/((2.0_f64.ln()).powi(2));

     let number_of_hash:f64=(number_of_bits/n)*2.0_f64.ln();
    //  now create the broom filter array

     return Bloom_filter{
        probability:false_positivity,
        expected_inputs:expected_inputs,
        num_of_hash:number_of_hash,
        m:((number_of_bits as u64).try_into().unwrap()),
        buffer:0,
     };
    }

    pub fn add_new_value(& mut self,input:&String)->Result<(),Box<dyn Error>>
    {
    let mut hashes:Vec<u128>=Vec::new();

    for i in 0..=(self.num_of_hash.floor() as u64).try_into().unwrap()
    {
        let hash_result = murmur3_x86_128(&mut Cursor::new(input), i);
        match hash_result
        {
            Ok(x)=>hashes.push(x),
            Err(_)=>panic!("unable to generate the hash"),
        };
    
    }
    
    for (ind,hash) in hashes.iter().enumerate()
    {
        let modu: u128 = *hash % (self.m as u128);
        self.buffer |= 1 << modu;
    }
    
    // =============================================================================================
     return Result::Ok(());
    }


    // =================================================================================================
    // now the function to check if the input exists or not

    pub fn check(& mut self,input:&String)->bool
    {
        let mut hashes:Vec<u128>=Vec::new();

        for i in 0..=(self.num_of_hash.floor() as u64).try_into().unwrap()
        {
            let hash_result = murmur3_x86_128(&mut Cursor::new(input), i);
            match hash_result
            {
                Ok(x)=>hashes.push(x),
                Err(_)=>panic!("unable to generate the hash"),
            };
        
        };


        // now checking if any of the bit is 0
        for (ind,hash) in hashes.iter().enumerate()
        {
            let modu: u128 = *hash % (self.m as u128);
            let result=self.buffer & (1 << modu);
            if result ==0
            {
                return true;
            }
        }
        return false;

    }
}