// Copyright
//
// License

//! Port of dnav to rust

#![feature(path_ext)]

extern crate docopt;
extern crate rustc_serialize;

use std::env;
use std::fs::PathExt;
use std::path::PathBuf;

use docopt::Docopt;

static USAGE: &'static str = "
RNAV.

Usage:
  rnav cd <alias>
  rnav add <alias> [directory]
  rnav show <alias>
  rnav rm <alias>
  rnav clear
  rnav list
  rnav prune
";

#[derive(RustcDecodable)]
struct Args {
    cmd_cd: bool,
    cmd_add: bool,
    cmd_show: bool,
    cmd_rm: bool,
    cmd_clear: bool,
    cmd_list: bool,
    cmd_prune: bool,
}

/// Get the directory where symlinks for aliases are stored
fn get_symlink_directory() -> Option<PathBuf> {
    match env::home_dir() {
        Some(p) => Some(p.join(".dnav/symlinks")),
        None => None
    }
}

/// Get the full path to the directory pointed to by the provided alias.
///
/// If the alias does not exist, this method will return None
fn get_target_for_alias(alias: &str) -> Option<PathBuf> {
    match get_symlink_directory() {
        Some(p) => if p.join(alias).exists() {
            Some(p.join(alias))
        } else {
            None
        },
        None => None
    }
}

fn do_cd(alias: String) -> i32 {
    match get_target_for_alias(alias) {
        Some(p) => print!("cd {}", alias),
        None => printf!("echo \"Alias {} is not defined\" >&2", alias)
    };
}

fn do_add(opt: Args) -> String {
    "add".to_string()
}

fn do_show(opt: Args) -> String {
    "show".to_string()
}

fn do_rm(opt: Args) -> String {
    "rm".to_string()
}

fn do_clear(opt: Args) -> String {
    "clear".to_string()
}

fn do_list(opt: Args) -> String {
    "list".to_string()
}

fn do_prune(opt: Args) -> String {
    "prune".to_string()
}

fn dispatch_command() -> i32 {
    let opt: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    let args = Docopt::new(USAGE)
                       .and_then(|d| d.parse())
                       .unwrap_or_else(|e| e.exit());
    if opt.cmd_cd {
        let alias = args.get_str("<alias>").to_string();
        do_cd(alias)
    } else if opt.cmd_add {
        do_add(opt)
    } else if opt.cmd_show {
        do_show(opt)
    } else if opt.cmd_rm {
        do_rm(opt)
    } else if opt.cmd_clear {
        do_clear(opt)
    } else if opt.cmd_list {
        do_list(opt)
    } else if opt.cmd_prune {
        do_prune(opt)
    } else {
        panic!("Should not happen")
    }
}

fn main() {
    // let dir = get_symlink_directory().unwrap();
    // println!("The homedir is {}", dir.display());
    dispatch_command()
}
