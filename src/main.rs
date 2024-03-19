use std::arch::x86_64::*;
use std::env;
use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

// unsafe fn xor_avx2(data1: &[u8], data2: &[u8]) -> Vec<u8> {
//     let mut result = vec![0u8; data1.len()];
//     let chunks = data1.chunks_exact(32);
//     let remainder = chunks.remainder();

/*
    for (i, (&chunk1, &chunk2)) in chunks.zip(data2.chunks_exact(32)).enumerate() {
        let a = _mm256_loadu_si256(chunk1.as_ptr() as *const __m256i);
        let b = _mm256_loadu_si256(chunk2.as_ptr() as *const __m256i);
        let res = _mm256_xor_si256(a, b);
        _mm256_storeu_si256(result[i * 32..].as_mut_ptr() as *mut __m256i, res);
    }
*/

//     for (i, (&r1, &r2)) in remainder.iter().zip(remainder.iter()).enumerate() {
//         result[chunks.len() * 32 + i] = r1 ^ r2;
//     }

//     result
// }

unsafe fn xor_avx2(data1: &[u8], data2: &[u8]) -> Vec<u8> {
    assert_eq!(data1.len(), data2.len(), "Data slices must be of the same length.");

    let mut result = vec![0u8; data1.len()];
    let len = data1.len();
    let mut i = 0;

    while i + 32 <= len {
        let a = _mm256_loadu_si256(data1[i..].as_ptr() as *const __m256i);
        let b = _mm256_loadu_si256(data2[i..].as_ptr() as *const __m256i);
        let res = _mm256_xor_si256(a, b);
        _mm256_storeu_si256(result[i..].as_mut_ptr() as *mut __m256i, res);
        i += 32;
    }

    // Handle any remaining bytes that don't fit into a 32-byte chunk
    for j in i..len {
        result[j] = data1[j] ^ data2[j];
    }

    result
}

fn read_file(path: &Path) -> io::Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn write_output(path: &Path, data: &[u8]) -> io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(data)?;
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() != 4 {
        eprintln!("Usage: xorfiles_avx2 <filepath1> <filepath2> <output>");
        std::process::exit(1);
    }
    
    let path1 = Path::new(&args[1]);
    let path2 = Path::new(&args[2]);
    let output_path = Path::new(&args[3]);

    let data1 = read_file(&path1)?;
    let data2 = read_file(&path2)?;

    if data1.len() != data2.len() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Files are not of the same size"));
    }

    let result = unsafe { xor_avx2(&data1, &data2) };
    write_output(&output_path, &result)?;
    
    Ok(())
}
