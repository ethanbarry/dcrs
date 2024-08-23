mod cli;
mod io;

use bigdecimal::{BigDecimal as Rational, FromPrimitive, One, Zero};
use colored::*;

/*
Input/Output Design
===================
The prompt and other messages should be printed in blue using the prompt!() or println!() macro.
Outputs should be printed in green using the println!() macro.
Warnings should be yellow, and errors should be red.

Language
========
The calculator should support add (+), sub (-), mul (*), div (/), pow (^),
as well as sqrt (v), cbrt (b), and mod (%) for integers.
*/

static OPS: [char; 11] = ['+', '-', '*', '/', '^', 'v', 'b', '%', 'p', 'c', 't'];

fn main() {
    let msg = String::from(
        "dcrs -- An arbitrary-precision calculator.\n(c) 2024 Ethan Barry. MIT License.",
    );
    println!("{}", msg.blue());

    // Set up a stack.
    let mut stack = vec![];

    loop {
        parse(prompt(), &mut stack);
    }
}

/// Use the prompt macro to get input.
fn prompt() -> String {
    prompt!("(in) ".blue())
}

fn parse(input: String, stack: &mut Vec<Rational>) {
    for t in input.split_whitespace() {
        if t.chars().all(|c| c.is_numeric() || c == '-' || c == '.')
            && t.chars().any(|c| c.is_numeric())
        {
            // It's probably a valid decimal-formatted number...
            match Rational::from_f64(t.parse::<f64>().unwrap()) {
                Some(rat) => stack.push(rat),
                None => eprintln!(
                    "{}",
                    format!("(out) Could not parse {t} into a rational number!").red()
                ),
            }
        } else if t.chars().all(|c| OPS.contains(&c)) {
            for c in t.chars() {
                match c {
                    '+' => stack_add(stack),   // Addition
                    '-' => stack_sub(stack),   // Subtraction
                    '*' => stack_mul(stack),   // Multiplication
                    '/' => stack_div(stack),   // Division
                    '%' => stack_mod(stack),   // Modulus
                    '^' => stack_pow(stack),   // Exponentiation
                    'v' => stack_sqrt(stack),  // Root two
                    'b' => stack_cbrt(stack),  // Root three
                    'p' => stack_print(stack), // Print the stack
                    'c' => stack_clear(stack), // Clear the stack
                    't' => stack_top(stack),   // Print the topmost element in the stack
                    _ => eprintln!("{}", "(out) Unknown operator {c}!".red()),
                }
            }
        } else {
            eprintln!("{}", format!("(out) Unknown character(s) '{t}'").red())
        }
    }
}

// Easier to type
fn zero() -> Option<Rational> {
    Some(Rational::zero())
}

fn stack_add(stack: &mut Vec<Rational>) {
    let res = stack.pop().or_else(zero).unwrap() + stack.pop().or_else(zero).unwrap();
    stack.push(res);
}

fn stack_sub(stack: &mut Vec<Rational>) {
    // Correct precedence from this leading minus sign.
    let res = -stack.pop().or_else(zero).unwrap() + stack.pop().or_else(zero).unwrap();
    stack.push(res);
}

fn stack_mul(stack: &mut Vec<Rational>) {
    let res = stack.pop().or_else(zero).unwrap() * stack.pop().or_else(zero).unwrap();
    stack.push(res);
}

fn stack_div(stack: &mut Vec<Rational>) {
    let divisor = stack.pop().or_else(zero).unwrap();
    let quotient = stack.pop().or_else(zero).unwrap();
    let res = quotient / divisor;
    stack.push(res);
}

fn stack_mod(_stack: &mut Vec<Rational>) {
    unimplemented!()
}

fn stack_pow(stack: &mut Vec<Rational>) {
    let exponent = stack.pop().or_else(zero).unwrap();
    let base = stack.pop().or_else(zero).unwrap();
    let mut res = Rational::one();
    match exponent.to_string().parse::<i32>() {
        Ok(exp) => {
            if !exponent.is_integer() {
                // Emit warning.
                println!(
                    "{}",
                    "(out) Reminder: Fractional exponents are truncated to 32 signed integer bits."
                        .yellow()
                );
            }
            for _ in 0..exp {
                res *= base.clone();
            }
            stack.push(res);
        }
        Err(_) => {
            eprintln!(
                "{}",
                "(out) Cannot exponentiate with this large a number!\n      Replacing numbers onto the stack!".red()
            );
            stack.push(base);
            stack.push(exponent);
        }
    };
}

fn stack_sqrt(stack: &mut Vec<Rational>) {
    let val = stack.pop().or_else(zero).unwrap();
    match val.sqrt() {
        Some(res) => stack.push(res),
        None => {
            eprintln!(
            "{}",
            "(out) Cannot take the sqrt of a negative number!\n      Replacing number onto the stack!".red()
        );
            stack.push(val);
        }
    }
}

fn stack_cbrt(stack: &mut Vec<Rational>) {
    let res = stack.pop().or_else(zero).unwrap().cbrt();
    stack.push(res);
}

fn stack_print(stack: &mut Vec<Rational>) {
    println!("{}", "(out) STACK\n(out) -----".green());
    for v in stack {
        println!("{}", format!("(out) {}", v).green());
    }
}

fn stack_clear(stack: &mut Vec<Rational>) {
    println!("{}", "(out) Cleared the stack!".green());
    stack.clear();
}

fn stack_top(stack: &mut Vec<Rational>) {
    match stack.last() {
        Some(v) => println!("{}", format!("(out) {v}").green()),
        None => eprintln!("{}", "(out) Stack is empty.".yellow()),
    }
}
