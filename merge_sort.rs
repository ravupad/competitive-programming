use std::{ptr, mem, io::{self, StdinLock, Read}};

/*
Given number of elements and the value of elements, print them in
increasing order. So, for given input:
5
0 9 48 2 1
Output should be:
0 1 2 9 48
*/

fn merge<T>(array: &mut [T], offset: usize, buffer: &mut [T])
    where T: std::cmp::PartialOrd, T: std::fmt::Debug
{
    let mut o1: usize = 0;
    let mut o2: usize = offset;
    let len: usize = array.len();
    if offset == len || offset == 0 || array[offset - 1] <= array[offset] {
        return;
    }
    unsafe {
        ptr::copy_nonoverlapping(
            array.as_ptr(), 
            buffer.as_mut_ptr(), 
            len);
    }
    for val in array {
        if o1 == offset {
            mem::swap(val, &mut buffer[o2]);
            o2 += 1;
        } else if o2 == len {
            mem::swap(val, &mut buffer[o1]);
            o1 += 1;
        } else if buffer[o1] > buffer[o2] {
            mem::swap(val, &mut buffer[o2]);;
            o2 += 1;
        } else {
            mem::swap(val, &mut buffer[o1]);
            o1 += 1;
        }
    }
}

fn merge_sort<T>(array: &mut [T]) 
    where T: std::cmp::PartialOrd, T: std::fmt::Debug
{
    let len = array.len();
    let mut buffer: Vec<T> = Vec::with_capacity(len);
    unsafe { buffer.set_len(len); }
    let mut sorted_len = 1;
    loop {
        if sorted_len >= len {
            break;
        }
        let mut block = 0;
        loop {
            let start = block * 2 * sorted_len;
            let mid = start + sorted_len;
            if mid >= len { break; }
            let mut end = mid + sorted_len;
            if end > len { end = len; }
            merge(&mut array[start..end], sorted_len, &mut buffer[0..len]);
            block += 1;
            if end == len {
                break;
            }
            if start >= len {
                break;
            }
        }
        sorted_len *= 2;
    }
}



fn main() {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut buffer = [0; 1024];
    let mut array: Vec<i64> = Vec::new();
    let mut offset = 0;
    let mut buffer_len = 0;
    let n = read_num(&mut stdin, &mut buffer, &mut buffer_len, &mut offset);
    for _ in 0..n {
        array.push(read_num(&mut stdin, &mut buffer, &mut buffer_len, &mut offset));
    }
    merge_sort(&mut array);
    print!("[");
    for (i, val) in array.iter().enumerate() {
        print!("{}", val);
        if i != (array.len() - 1) {
            print!(",");
        }
    }
    print!("]");
}


fn read_num(
    f: &mut StdinLock, 
    buffer: &mut [u8], 
    buffer_len: &mut usize,
    offset: &mut usize) -> i64 
{
    let mut num: i64;
    let mut char: u8;
    let mut digit: u8;
    let mut negative = false;
    loop {
        char = get_char(f, buffer, buffer_len, offset);
        if char == b'-' {
            negative = true;
        }
        if char >= b'0' && char <= b'9' {
            break;
        }
    }
    num = i64::from(char - b'0');
    loop {
        char = get_char(f, buffer, buffer_len, offset);
        if char < b'0' || char > b'9' {
            break;
        }
        digit = char - b'0';
        num = (num * 10) + i64::from(digit);
        
    }
    if negative {
        num *= -1;
    }
    num
}

fn get_char(
    f: &mut StdinLock,
    buffer: &mut [u8],
    buffer_len: &mut usize,
    offset: &mut usize) -> u8
{
    let char: u8;
    if *offset >= *buffer_len {
        *buffer_len = f.read(buffer).unwrap();
        *offset = 0;
    }
    if *buffer_len == 0 {
        char = 0;
    } else {
        char = buffer[*offset];
    }
    *offset += 1;
    char
}
