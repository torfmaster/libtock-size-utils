use elf::File;
use std::path::Path;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "libtock size tool",
    about = "A simple app to measure the size of userland binaries"
)]
struct Arguments {
    #[structopt(short, long)]
    input: String,
}

fn main() {
    let arguments = Arguments::from_args();

    let path = Path::new(&arguments.input);
    let file = File::open_path(path).expect("file not found");
    let text_size = file
        .get_section(".text")
        .map(|section| section.shdr.size)
        .unwrap_or(0);
    let data_size = file
        .get_section(".data")
        .map(|section| section.shdr.size)
        .unwrap_or(0);
    let rodata_size = file
        .get_section(".rodata")
        .map(|section| section.shdr.size)
        .unwrap_or(0);
    let bss_size = file
        .get_section(".bss")
        .map(|section| section.shdr.size)
        .unwrap_or(0);
    println!(
        "Size of sections in bytes: 
        text: {} 
        data: {}
        rodata: {}
        bss: {}",
        text_size, data_size, rodata_size, bss_size
    );
}
