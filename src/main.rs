use clap::{App, Arg};
use elf::File;
use std::path::Path;

fn main() {
    let matches = App::new("size tools")
        .arg(
            Arg::with_name("input")
                .long("input")
                .short("i")
                .takes_value(true)
                .required(true),
        )
        .get_matches();
    let input = matches
        .value_of("input")
        .expect("argument for input not found");

    let path = Path::new(input);
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
