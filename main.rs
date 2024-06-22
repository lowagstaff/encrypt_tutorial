
use std::{fs, env, io::Write};

/// Shift bytes in a vector
/// Bytes are shifted by a random number
fn byte_shift(text: Vec<u8>, shift_by: u8, backwards: bool) -> Vec<u8>
{
    // map over the vector
    text.iter()
        .map(|byte| {
            // if backwards, subtract shift, else add shift
            if backwards
            {
                // subtract shift
                byte.wrapping_sub(shift_by)
            }
            else 
            {
                // add shift
                byte.wrapping_add(shift_by)
            }
        })
        // collect into a vector
    .collect()
}

fn main()
{
    // get command line arguments
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    // check if the arguments are correct
    if args.len() != 3 || (args[2].clone() != *"false" && args[2].clone() != *"true")
    {
        // print usage
        println!("Usage: {} <file> <decrypting?>", args[0].clone());
        // print examples of byte shifting
        println!("Example: {} hello.txt true", args[0].clone());
        println!("Example: {} hello.txt false", args[0].clone());
        return;
    }

    // get the arguments for the reverse shift
    let decrypting = args[2] == *"true";

    // read the file
    match fs::read(args[1].clone())
    {
        // if the file can be read
        Ok(contents) =>
        {
                // shift the bytes
                let new_contents = byte_shift(contents.clone(), 2, decrypting);
                // open the file
                let mut file = fs::OpenOptions::new()
                    .write(true)
                    .open(args[1].clone())
                    .unwrap();
                // write the new bytes
                
                // if the file can't be written create an error
                if let Err(e) = file.write_all(&new_contents)
                {
                    println!("Error: {:?}", e);
                }

        }

        // if the file can't be read create an error
        Err(e) =>
        {
            println!("Could not open file {} : {}", args[1], e);
        }
    }
    println!("Success!")
}