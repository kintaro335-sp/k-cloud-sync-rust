/*
 * k-cloud-sync-rust
 * Copyright(c) Kintaro Ponce
 * MIT Licensed
 */
use std::env;
use std::println;
use std::process;

use crate::core::utils;

pub struct ArgsInput {
  pub action: String,
  pub mode: String,
  pub file: String,
  pub dirs: Vec<u16>
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
    dirs: [0].to_vec()
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
          let arg_str_indexes = arg_val.split(",");
          let mut arg_num_indexes: Vec<u16> = Vec::new();
          for arg_str in arg_str_indexes {
            let index_num_result = parse_to_number(&arg_str.to_string());
            if index_num_result.success {
              arg_num_indexes.push(index_num_result.value);
            }
          }
          args_input.mode = String::from("some");
          if arg_num_indexes.len() == 0 {
            println!("Error: invalid dir options");
            process::exit(1);
          }
          args_input.dirs = arg_num_indexes;
        },
        _ => {}
      }
    }

  args_input
}
