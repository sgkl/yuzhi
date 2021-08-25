use clap::{Arg, App};
use std::hint::unreachable_unchecked;

fn main() {
    let matches = App::new("yuzhi")
        .version("0.0.1")
        .author("sguanke")
        .about("欲知天下事...")
        .subcommand(
            App::new("ip").about("ip地址").arg(
                Arg::new("ip_address")
            ),
        )
        .subcommand(
            App::new("book")
                .about("书籍")
                .subcommand(
                    App::new("list")
                        .about("list local books")
                )
                .subcommand(
                    App::new("delete")
                        .about("delete local book")
                        .arg("<book> 'book'")
                )
        ).get_matches();

    match matches.subcommand() {
        Some(("ip", ip_matches)) => {
            println!("{}", ip_matches.value_of("ip_address").unwrap());
        },
        Some(("book", book_matches)) => {
            match book_matches.subcommand() {
                Some(("list", _)) => {
                    println!("list local book");
                }
                Some(("delete", delete_matches)) => {
                    println!("delete local book-{}", delete_matches.value_of("book").unwrap());
                }
                _ => unreachable!(),
            }
        },
        _                     => unreachable!(),
    }
}
