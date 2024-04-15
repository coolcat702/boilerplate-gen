use std::fs::File;
use std::io::Write;

fn get_input<T: std::str::FromStr>(message: &str) -> T {
    print!("{}", message);
    let _ = std::io::Write::flush(&mut std::io::stdout());
    let mut input: String = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => input.trim().parse::<T>().unwrap_or(get_input::<T>(message)),
        Err(_) => get_input::<T>(message),
    }
}

fn main() {
    let language = std::env::args().nth(1).unwrap_or(String::new());
    if !["c", "python", "py", "cpp", "c++", "rust", "java", "rs", "cs", "c#"].contains(&language.as_str()) {
        match get_input::<String>("language: ").to_lowercase().as_str() {
            "c" => c_bp(),
            "python" | "py" => python_bp(),
            "cpp" | "c++" => cpp_bp(),
            "java" => java_bp(),
            "rust" | "rs" => rust_bp(),
            "cs" | "c#" => cs_bp(),
            _ => println!("Unsupported language"),
        }
    } else {
        match language.as_str() {
            "c" => c_bp(),
            "python" | "py" => python_bp(),
            "cpp" | "c++" => cpp_bp(),
            "java" => java_bp(),
            "rust" | "rs" => rust_bp(),
            "cs" | "c#" => cs_bp(),
            _ => unreachable!(),
        }
    }
}

fn c_bp() {
    let mut file = File::create("program.c").unwrap();
    file.write_all(
b"#include <stdio.h>


int main(void)
{
    printf(\"hello world\\n\");
}");
}

fn python_bp() {
    let mut file = File::create("program.py").unwrap();
    file.write_all(
b"def main() -> None:
    print(\"hello world\")


if __name__ == \"__main__\":
    main()
");
}

fn cpp_bp() {
    let mut file = File::create("program.cpp").unwrap();
    file.write_all(
b"#include <iostream>

int main(void) {
    std::cout << \"hello world\" << std::endl;
}");
}

fn java_bp() {
    let mut file = File::create("program.java").unwrap();
    file.write_all(
b"class Main {
    public static void main(String[] args) {
        system.out.println(\"hello world\");
    }
}");
}

fn rust_bp() {
    let mut file = File::create("program.rs").unwrap();
    file.write_all(
b"fn main() {
    println!(\"hello world\");
}");
}

fn cs_bp() {
    let mut file = File::create("program.cs").unwrap();
    file.write_all(
b"using System;

namespace MyApp
{
    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine(\"hello world\");
        }
    }
}");
}
