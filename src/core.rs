use std::process::{Command, Stdio};
use std::str;
use std::path::Path;


// rsync --verbose --archive --partial --progress --filter=._rsync.rules --checksum --prune-empty-dirs /home/bencaddyro/doit/3bf/openDIT/sourceA/ /home/bencaddyro/doit/3bf/openDIT/sourceB/

pub fn sync_a_b(sourcea: &str, sourceb: &str) {

copy_a_b(sourcea, sourceb);
integrity_a_b(sourcea, sourceb);
}


pub fn copy_a_b(sourcea: &str, sourceb: &str) {

// Check that dir exist, no need for destination thanks to --mkpath
if !Path::new(sourcea).is_dir() { panic!("{sourcea} not an existing directory !") };

let mut handler = Command::new("rsync")
                    .args(["--archive", "--partial", "--progress", "--mkpath", "--prune-empty-dirs"])
                    .args(["--checksum", "--verbose", "--filter=._rsync.rules"])
                    .args([sourcea, sourceb])
                    .stdin(Stdio::null())
                    .spawn().unwrap();
handler.wait().unwrap();
}


pub fn integrity_a_b(sourcea: &str, sourceb: &str){

// Check that dir exist
if !Path::new(sourcea).is_dir() { panic!("{sourcea} not an existing directory !") };
if !Path::new(sourceb).is_dir() { panic!("{sourceb} not an existing directory !") };

println!("Integrity check: {sourcea} == {sourceb} ?");
let handler = Command::new("rsync")
                    .args(["--archive", "--partial", "--progress", "--dry-run", "--prune-empty-dirs"])
                    .args(["--checksum", "--verbose", "--filter=._rsync.rules"])
                    .args([sourcea, sourceb])
                    .stderr(Stdio::piped())
                    .stdout(Stdio::piped())
                    .stdin(Stdio::null())
                    .output().unwrap();

// Get output, third line should be empty as no files should be transferred
let out = str::from_utf8(&handler.stdout).unwrap();
let lines: Vec<&str> = out.split('\n').collect();

if !lines[2].eq("") { println!("Integrity check fail ! {sourceb} != {sourcea}"); }
else { println!("Integrity check success ! {sourcea} == {sourceb}"); }
}
