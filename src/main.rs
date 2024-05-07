use std::collections::HashMap;
use std::process::exit;
use std::process::Command;
use std::str;
use std::io;

fn main() {
    let output = Command::new("efibootmgr")
        .output()
        .expect("Failed to execute 'efibootmgr' program. Check if 'efibootmgr' is installed.");

    let stdout_str = str::from_utf8(&output.stdout)
        .expect("Invalid UTF-8");

    let lines: Vec<&str> = stdout_str
        .split('\n')
        .collect();

    println!("Choose the next boot option:\n");

    let first_index = 3;
    let last_index = lines.len()-1;
    let mut answers = HashMap::new();
    for (i, l) in lines[first_index..last_index].iter().enumerate() {
        let num_boot = &l[4..8];
        let desc_boot = &l[10..];

        println!("[{}] {}: {}", i+1, num_boot, desc_boot);

        answers.insert((i+1).to_string(), num_boot.to_string());
    }

    print!("\n");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input.");

    let input = input.trim();
    match answers.get(input) {
        Some(v) => {
            let efibootmgr_input = v;

            let next_boot_output = Command::new("efibootmgr")
                .args(["-n", efibootmgr_input])
                .output()
                .expect("Failed to set next boot using efibootmgr program. Check if 'efibootmgr' is installed or either the next boot option selected is valid.");
            
            let stdout_str = str::from_utf8(&next_boot_output.stdout)
                .expect("Invalid UTF-8");

            let stderr_str = str::from_utf8(&next_boot_output.stderr)
                .expect("Invalid UTF-8");

            if !stderr_str.is_empty() {
                println!("efibootmgr err: {}", stderr_str);
                exit(1);
            }

            println!("Congratulation! Your Next boot option is set.");
            print!("\n");
            println!("{}", stdout_str);
        },
        None => println!("Invalid input")
    }
    

}
