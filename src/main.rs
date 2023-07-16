//mod parser;
//use crate::parser;

mod parser;
//use parser::parser::load;
use parser::parser::load;
/*
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn in_poor_elf1()
{
    let mut vec:Vec<i32> = Vec::new();
    
    let mut running:i32 = 0;

    if let Ok(lines) = read_lines("data.txt")
    {
        for line in lines 
        {
            if let Ok(ip) = line
            {
                if ip.len()<2
                {
                    vec.push(running);
                    running = 0;                 
                }
                else
                {
                    let value = ip.parse::<i32>().unwrap();
                    running += value;
                }
            }
        }
    }

    let mut total:i32 = 0;

    vec.sort_by(|a, b| b.cmp(a));

    for n in 0usize..3
    {
        println!("{}", vec[n]);
        total += vec[n];
    }

    println!("Total {}", total);
}

fn in_poor_elf3()
{
    let mut sum:u32 = 0;

    if let Ok(lines) = read_lines("elf3.txt")
    {
        for line in lines 
        {
            if let Ok(ip) = line
            {
             
                let mut len = ip.len();
                if len % 2 == 0 
                {
                    len /= 2;
                    
                    let mut a:Vec<u32> = Vec::new();
                    let mut b:Vec<u32> = Vec::new();

                    let mut counter = 0;
                    
                    for c in ip.chars()
                    {                        
                        let mut v = c as u32; 

                        if v >= 97 && v <= 122 
                        {
                            v = v - 96;
                        }
                        else if v >= 65 && v <= 90
                        {
                            v = v - (65 - 27);
                        }
                        
                        if counter < len
                        {
                            a.push(v);
                        }
                        else
                        {
                            b.push(v);
                        }

                        counter += 1;
                    }

                    a.sort();
                    a.dedup();

                    for w in a
                    {
                        let res = b.iter().find(|&&x| x == w);
                        if res != None 
                        {
                            sum += w;                         
                        }                        
                    }

                }
            }
        }
    }

   println!("Sum {}", sum);
}

fn in_poor_elf3_b()
{
    let mut sum:u32 = 0;
   
    let mut a:Vec<u32> = Vec::new();
    let mut b:Vec<u32> = Vec::new();

    let mut elf_counter = 0;

    if let Ok(lines) = read_lines("elf3.txt")
    {
        for line in lines 
        {
            if let Ok(ip) = line
            {
                for ch in ip.chars()
                {                        
                    let mut v = ch as u32; 

                    if v >= 97 && v <= 122 
                    {
                        v = v - 96;
                    }
                    else if v >= 65 && v <= 90
                    {
                        v = v - (65 - 27);
                    }
                    
                    a.push(v);                        
                }

                a.sort();
                a.dedup();

                b.append(&mut a);
                a.clear();                    

                if elf_counter % 3 == 2 
                {
                    b.sort();

                    let mut previous:u32 = 0;
                    let mut count = 0;

                    for moo in &b 
                    {
                        if previous == *moo
                        {
                            count += 1;        
                        }
                        else
                        {
                            count = 0;
                        }

                        if count >= 2
                        {
                            sum += *moo;
                        }

                        previous = *moo;
                    }

                    b.clear();

                }

                elf_counter += 1;
            }
        }
    }

   println!("Sum {}", sum);
}

use std::iter::FromIterator;

fn in_poor_elf4()
{
    let mut sum:u32 = 0;
   
    if let Ok(lines) = read_lines("elf4.txt")
    {
        for line in lines 
        {
            if let Ok(ip) = line
            {
                let res = Vec::from_iter(ip.split(',').map(String::from));

                let left:Vec<&str> = res[0].split('-').collect();

                let a = left[0].parse::<u32>().unwrap();
                let b = left[1].parse::<u32>().unwrap();

                let right:Vec<&str> = res[1].split('-').collect();

                let c = right[0].parse::<u32>().unwrap();
                let d = right[1].parse::<u32>().unwrap();

                /*
                if inside(a,b,c,d)
                {
                    sum += 1;
                }
                else if inside(c,d,a,b)
                {
                    sum += 1;
                }
                */

                if intersect(a,b,c,d) 
                {
                    sum += 1;
                }
            }
        }
    }

   println!("Sum {}", sum);
}

fn in_poor_elf5()
{
    /*
    [N] [G]                     [Q]    
    [H] [B]         [B] [R]     [H]    
    [S] [N]     [Q] [M] [T]     [Z]    
    [J] [T]     [R] [V] [H]     [R] [S]
    [F] [Q]     [W] [T] [V] [J] [V] [M]
    [W] [P] [V] [S] [F] [B] [Q] [J] [H]
    [T] [R] [Q] [B] [D] [D] [B] [N] [N]
    [D] [H] [L] [N] [N] [M] [D] [D] [B]
    1   2   3   4   5   6   7   8   9 
    */

    //let mut sum:u32 = 0;

    const width:usize = 50;
    const height:usize = 50;

    let mut crates = [[' '; width]; height];
    let mut top = [0i32; width];

    for x in 0usize..width
    {
        top[x] = 0;
    }

    populate_column(&mut crates, &mut top, 1, "DTWFJSHN");
    populate_column(&mut crates, &mut top, 2, "HRPQTNBG");
    populate_column(&mut crates, &mut top, 3, "LQV");
    populate_column(&mut crates, &mut top, 4, "NBSWRQ");
    populate_column(&mut crates, &mut top, 5, "NDFTVMB");
    populate_column(&mut crates, &mut top, 6, "MDBVHTR");
    populate_column(&mut crates, &mut top, 7, "DBQJ");
    populate_column(&mut crates, &mut top, 8, "DNJVRZHQ");
    populate_column(&mut crates, &mut top, 9, "BNHMS");

    if let Ok(lines) = read_lines("elf5.txt")
    {
        for line in lines 
        {
            if let Ok(ip) = line
            {
                let bobbins:Vec<&str> = ip.split(' ').collect();

                let total = bobbins[1].parse::<u32>().unwrap() as usize;
                let from = bobbins[3].parse::<u32>().unwrap() as usize;
                let to = bobbins[5].parse::<u32>().unwrap() as usize;

                /*
                // PART A 
                for x in (0usize..total).rev()
                {
                    let ch = crates[from][top[from] as usize];
                    crates[from][top[from] as usize] = ' ';
                    top[from] -= 1;

                    top[to] += 1;
                    crates[to][top[to] as usize] = ch;
                }
                */
                
                let mut susan = String::new();

                for x in (0usize..total)
                {
                    let ch = crates[from][top[from] as usize];
                    crates[from][top[from] as usize] = ' ';
                    top[from] -= 1;
                    susan.push(ch);
                }

                let rev = susan.chars().rev().collect::<String>();
                for ch in rev.chars()
                {
                    top[to] += 1;
                    crates[to][top[to] as usize] = ch;
                }

                //println!("{}","hello");
            }
        }
    }

    let mut dave = String::new();
    for x in 1usize..width
    {
        let ch = crates[x][top[x] as usize];        
        if ch != ' '
        {
            dave.push(ch);
        }
    }

    println!("Crates {}", dave);
}

fn in_poor_elf6()
{
    //let mut sum:u32 = 0;
    let mut dups = false;

    if let Ok(lines) = read_lines("elf6.txt")
    {
        for line in lines 
        {
            if let Ok(ip) = line
            {
                //println!("{}","ip");
                let mut arr = [' '; 4];
                let mut index = 0;

                for ch in ip.chars()
                {
                    for x in 1usize..4
                    {
                        arr[x-1] = arr[x];
                    }                
                    arr[3] = ch;
                    dups = false;
                    
                    for x in 0usize..4
                    {
                        if arr[x] != ' '
                        {
                            for y in 0usize..4
                            {
                                if arr[y] != ' ' && x != y
                                {
                                    if arr[x] == arr[y] 
                                    {
                                        dups = true;
                                    }
                                }
                            }
                        }
                    }

                    if !dups && index >= 3
                    {
                        println!("Index {}", index + 1);
                    }

                    index += 1;
                }
            }
        }
    }
}


fn in_poor_elf6_b()
{
    //let mut sum:u32 = 0;
    let mut dups = false;

    if let Ok(lines) = read_lines("elf6.txt")
    {
        for line in lines 
        {
            if let Ok(ip) = line
            {
                //println!("{}","ip");
                let mut arr = [' '; 14];
                let mut index = 0;

                for ch in ip.chars()
                {
                    for x in 1usize..14
                    {
                        arr[x-1] = arr[x];
                    }                
                    arr[13] = ch;
                    dups = false;
                    
                    for x in 0usize..14
                    {
                        if arr[x] != ' '
                        {
                            for y in 0usize..14
                            {
                                if arr[y] != ' ' && x != y
                                {
                                    if arr[x] == arr[y] 
                                    {
                                        dups = true;
                                    }
                                }
                            }
                        }
                    }

                    if !dups && index >= 13
                    {
                        println!("Index {}", index + 1);
                    }

                    index += 1;
                }
            }
        }
    }
}

fn populate_column(
    arr: &mut[[char; 50];50], 
    top: &mut[i32; 50],
    column: usize, 
    value: &str)
{
    let mut index = 0;

    for ch in value.chars()
    {
        arr[column][index] = ch;
        index += 1;        
    }

    top[column] = value.len() as i32 - 1;
}

fn inside(a: u32, b:u32, c:u32, d:u32) -> bool
{
    if d > b || c > b
    {
        return false;
    }

    if c < a || d < a
    {
        return false;
    }

    return true;
}

fn intersect(a: u32, b:u32, c:u32, d:u32) -> bool
{
    return a <= d && b >= c;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> 
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

*/
fn main() 
{
    let bob = parser::parser::load("test.txt");
    println!("{}","hello");
    //in_poor_elf6_b();
}
