use ramdisk::{cli, mount_ramdisk, unmount_ramdisk};
fn main() {
    let maches = cli();
    match maches.subcommand() {
        Some(("mount", mount)) => {
            let target = mount.get_one::<String>("target").expect("No target provided");
            mount_ramdisk(target);
        }
        Some(("unmount", _)) => unmount_ramdisk(),
        _ => println!("No subcommand provided"),
    }
}

