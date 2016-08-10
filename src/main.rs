
#![allow(unused_parens)]


extern crate argparse;

//use argparse::{ArgumentParser, StoreTrue, Store};


struct Options {
    start: String,
    full: bool,
    filtres: String
}


fn run(start: String, full: bool, filtres: String) -> bool {
    //files = getfiles(start=start, full=full, filtres=filtres)
    //if files is None:
    //    rendererror()
    //    return false;
    //rows = renderrows(files, full=full)
    //display(rows)
    println!("{}", start);
    println!("{}", full);
    println!("{}", filtres);
    return true;
}


fn getargs () -> Options {
    let mut options: Options = Options {
        full: false,
        start: "./".to_string(),
        filtres: "".to_string()
    };
    {
        let mut aparse: argparse::ArgumentParser = argparse::ArgumentParser::new();
        aparse.set_description("Replacement for ls");
        aparse
            .refer(&mut options.start)
            .add_option(
                &["-s", "--start"],
                argparse::Store,
                "Starting Path"
            );
        aparse
            .refer(&mut options.full)
            .add_option(
                &["-f", "--full"],
                argparse::StoreTrue,
                "Full Output"
            );
        aparse
            .refer(&mut options.filtres)
            .add_option(
                &["-g", "--filter"],
                argparse::Store,
                "Filter Results"
            );
        aparse.parse_args_or_exit();
    }
    return options;
}


fn main () -> () {
    let options: Options = getargs();
    let ret: bool = (
        run(
            options.start,
            options.full,
            options.filtres
        )
    );
    if !ret {
        std::process::exit(1);
    }
    std::process::exit(0);
}
