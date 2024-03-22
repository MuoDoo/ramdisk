use clap::{Arg, ArgAction, Command};
use std::os::unix::fs;
use std::path::{Path, PathBuf};


pub fn cli() -> clap::ArgMatches {
    Command::new("rDisk")
        .version("0.1.0")
        .author("MuoDoo")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("mount")
                .short_flag('m')
                .long_flag("mount")
                .about("Mount a ramdisk")
                .arg(
                    Arg::new("target")
                        .short('t')
                        .long("target")
                        .help("The target directory to mount the ramdisk")
                        .required(true)
                        .action(ArgAction::Set)
                        .num_args(1..),
                ),
        )
        .subcommand(
            Command::new("unmount")
                .short_flag('u')
                .long_flag("unmount")
                .about("Unmount a ramdisk")
        )
        .get_matches()
}
pub fn mount_ramdisk(_target: &String) {
    let target = Path::new(_target);
    let mut path = PathBuf::from(_target);
    if Path::is_relative(&target) {
        path = Path::new("/").to_path_buf();
        for p  in std::env::current_dir().unwrap().as_path().join(target).components() {
            path.push(p.as_os_str());
        }
    }
    println!("Mounting ramdisk at {:?}", path);
    let shm_path = Path::new("/dev/shm/ramdisk");
    if shm_path.exists() {
        println!("Ramdisk already mounted somewhere");
        return;
    }else if path.exists() {
        println!("Directory already existed: {:?}", path);
        return;
    } else if path.read_link().is_ok() {
        if path.read_link().unwrap() == shm_path {
            let _ = std::fs::remove_file(&path).unwrap();
            println!("Symlink removed: {:?}", path);
        } else {
            println!("Symlink already existed: {:?}", path);
            return;
        }
    } 
    std::fs::create_dir(&shm_path).expect("Failed to create ramdisk directory");
    fs::symlink(shm_path.to_path_buf(), path.to_path_buf()).expect("Failed to create symlink");
    println!("✅ Ramdisk mounted at {:?}", path);
}
pub fn unmount_ramdisk() {
    let path = Path::new("/dev/shm/ramdisk");
    if Path::exists(&path) {
        println!("Unmounting ramdisk ...");
        std::fs::remove_dir_all(&path).expect("Failed to remove ramdisk directory");
    } else {
        println!("No ramdisk mounted");
    }
}