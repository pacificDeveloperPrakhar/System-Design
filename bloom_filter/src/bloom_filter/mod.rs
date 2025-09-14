use std::error::Error;
use std::io::Cursor;
use murmur3::murmur3_x86_128;
#[derive(Debug)]
pub struct Bloom_filter
{
    pub m:u128,
    pub buffer:Vec<u8>,
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
     let number_of_bits=(number_of_bits.abs() as u128).try_into().unwrap();
     let buffer=vec![0;number_of_bits as usize];
    //  now create the broom filter array
     return Bloom_filter{
        probability:false_positivity,
        expected_inputs:expected_inputs,
        num_of_hash:number_of_hash,
        m:number_of_bits,
        buffer:buffer,
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
        
        // finding out to which index to put the value
        let index:usize=((modu/8_u128) as usize).try_into().unwrap();
        // now get the bit which need to be modified
        let bit=modu%8;

        self.buffer[index]|=(1<<bit);
    }
    
    // =============================================================================================
     return Result::Ok(());
    }


    // // =================================================================================================
    // // now the function to check if the input exists or not

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
        
        // finding out to which index to put the value
        let index:usize=((modu/8_u128) as usize).try_into().unwrap();
        // now get the bit which need to be modified
        let bit=modu%8;

        let result=self.buffer[index]&(1<<bit);
        if result==0
        {
            return true;
        }
        }
        return false;

    }
}