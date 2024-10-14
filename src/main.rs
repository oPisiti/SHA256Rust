use std::io;

fn sha256(msg: &str) -> Result<String, io::Error>{
    let msg_len: u64 = msg.as_bytes().len()
                            .try_into()
                            .map_err(|_|
                                io::Error::new(io::ErrorKind::InvalidInput, "Msg is too big! How did you manage that??")
                            )?;

    let mut msg_block: Vec<u8> = vec![0; ((msg.as_bytes().len() + 8)/64 + 1) * 64];
    
    // Copy the message to the msg block
    let mut index: usize = 0;
    for b in msg.as_bytes(){
        msg_block[index] = *b;
        index += 1;
    }

    // Append a single 1 
    msg_block[index] = 0x80;

    // Append the original message length to the end of the message block
    let mut tmp_msg_len = msg_len * 8;
    for len_index in (msg_block.len()-8..=msg_block.len()-1).rev(){
        msg_block[len_index] = (tmp_msg_len & 0xff) as u8;
        tmp_msg_len = tmp_msg_len >> 8;
    }

    // print_binary(&msg_block);

    // Create 32-bit words chunks for every 512 bits
    // TODO: Optimize
    let mut w = vec![0u32; 64];
    for chunk in 0..(msg_len/512 + 1) as usize{
        // Move msg data into words
        for msg_index in 0..64{
            w[msg_index/4] |= (msg_block[chunk*64 + msg_index] as u32) << (3 - msg_index%4) * 8;
            // println!("{}", w[msg_index/4]);
        }
        
        // Rotations
        let mut sigma_0: u32;
        let mut sigma_1: u32;
        for w_index in 16..64{
            sigma_0 = (w[w_index - 15].rotate_right(7)) ^ (w[w_index - 15].rotate_right(18)) ^ (w[w_index - 15] >> 3);
            sigma_1 = (w[w_index - 2].rotate_right(17)) ^ (w[w_index - 2].rotate_right(19))  ^ (w[w_index - 2] >> 10);
            w[w_index] = w[w_index - 16].wrapping_add(sigma_0).wrapping_add(w[w_index - 7]).wrapping_add(sigma_1);
            
        }
    }
    // print_words_binary(&w);

    // --- Compression --- 
    // Initialize hash values (h)
    let h0: u32 = 0x6a09e667;
    let h1: u32 = 0xbb67ae85;
    let h2: u32 = 0x3c6ef372;
    let h3: u32 = 0xa54ff53a;
    let h4: u32 = 0x510e527f;
    let h5: u32 = 0x9b05688c;
    let h6: u32 = 0x1f83d9ab;
    let h7: u32 = 0x5be0cd19;

    // Initialize Round Constants (k)
    let k: [u32; 64] = [0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4, 0xab1c5ed5,
        0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe, 0x9bdc06a7, 0xc19bf174,
        0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f, 0x4a7484aa, 0x5cb0a9dc, 0x76f988da,
        0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7, 0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967,
        0x27b70a85, 0x2e1b2138, 0x4d2c6dfc, 0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85,
        0xa2bfe8a1, 0xa81a664b, 0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070,
        0x19a4c116, 0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7, 0xc67178f2];

    // Define a - h
    let mut a = h0;
    let mut b = h1;    
    let mut c = h2;    
    let mut d = h3;    
    let mut e = h4;    
    let mut f = h5;    
    let mut g = h6;    
    let mut h = h7;

    let mut temp1: u32;
    let mut temp2: u32;
    let mut sum0: u32;
    let mut sum1: u32;
    let mut choice: u32;
    let mut maj: u32;
    for i in 0..64{
        sum0 = (a.rotate_right(2)) ^ (a.rotate_right(13)) ^ (a.rotate_right(22));
        sum1 = (e.rotate_right(6)) ^ (e.rotate_right(11)) ^ (e.rotate_right(25));

        choice = (e & f) ^ ((!e) & g);
        maj = (a & b) ^ (a & c) ^ (b & c);

        temp1 = h.wrapping_add(sum1).wrapping_add(choice).wrapping_add(k[i]).wrapping_add(w[i]);
        temp2 = sum0.wrapping_add(maj);
        
        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(temp1);
        d = c;
        c = b;
        b = a;
        a = temp1.wrapping_add(temp2);
    }

    // Modify final values
    a = a.wrapping_add(h0);
    b = b.wrapping_add(h1);
    c = c.wrapping_add(h2);
    d = d.wrapping_add(h3);
    e = e.wrapping_add(h4);
    f = f.wrapping_add(h5);
    g = g.wrapping_add(h6);
    h = h.wrapping_add(h7);

    Ok(format!("{:x}{:x}{:x}{:x}{:x}{:x}{:x}{:x}", a, b, c, d, e, f, g, h))
    // Ok(format!("{:x}", a))
}

#[allow(dead_code)]
fn print_binary(block: &Vec<u8>) -> (){
    let mut line_index: u8 = 0;
    for char in block{
        print!("{} ", (0..8).rev() // Iterate from 7 down to 0
            .map(|i| if (*char & (1 << i)) != 0 { '1' } else { '0' })
            .collect::<String>() // Collect characters into a String
        );

        line_index += 1;
        
        if line_index > 3{
            println!();
            line_index = 0;
        }

    }
}

#[allow(dead_code)]
fn print_words_binary(block: &Vec<u32>) -> (){
    for word in block{
        println!("{}", (0..32).rev() 
            .map(|i| if (*word & (1 << i)) != 0 { '1' } else { '0' })
            .collect::<String>() // Collect characters into a String
        );
    }
}


fn main() {
    // let _ = sha256("abcdefghi");
    let ans = sha256("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap_or_default();
    println!("Hash: {ans}");
}
