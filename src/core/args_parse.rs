/*
 * k-cloud-sync-rust
 * Copyright(c) Kintaro Ponce
 * MIT Licensed
 */
use std::env;
use std::process;

use crate::core::utils;

pub struct ArgsInput {
  pub action: String,
  pub mode: String,
  pub file: String,
  pub dir: u16
}

struct ParseResult {
  success: bool,
  value: u16
}

fn parse_to_number(value: &String) -> ParseResult {
  let num_value_result = value.parse::<u16>();
  
  let mut result = ParseResult {
    success: false,
    value: 0
  };

  match num_value_result {
    Ok(val) => {
      result.success = true;
      result.value = val;
    },
    Err(_) => {
      result.success = false;
      result.value = 0;
    }
  }

  result
}

fn validate_action_option(action_input: &String) -> String {
  match action_input.as_str() {
      "sync" => {
        return action_input.clone();
      },
      "list" => {
        return action_input.clone();
      },
      "usage" => {
        return action_input.clone();
      },
      "help" => {
        return action_input.clone();
      },
      _ => {
        println!("Error: invalid option");
        utils::display_help();
        process::exit(1);
      }
  }
}

pub fn get_args_input() -> ArgsInput {
  let args = env::args().enumerate();

  if args.len() < 3 {
    println!("Error: not enough arguments");
    process::exit(1);
  }

  let mut args_input = ArgsInput {
    action: String::from("sync"),
    mode: String::from("all"),
    file: String::from(""),
    dir: 0
  };

  for (i, arg) in args {
      
      match i {
        1 => {
          args_input.action = validate_action_option(&arg.clone());
        },
        2 => {
          args_input.file = arg.clone();
        },
        3 => {
          let arg_val = arg.clone();
          let value_num = parse_to_number(&arg_val);
          if value_num.success {
            args_input.mode = String::from("single");
            args_input.dir = value_num.value;
          } else {
            println!("Error: invalid dir option");
            process::exit(1);
          }
        },
        _ => {}
      }
    }

  args_input
}
