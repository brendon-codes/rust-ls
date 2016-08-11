
#![allow(unused_parens)]


extern crate argparse;

use std::process;
use std::fs;
use std::path;


#[derive(Debug)]
struct Options {
    start: String,
    full: bool,
    filtres: String
}


#[derive(Debug)]
struct Row {

}


fn get_dir_listing (start: &str, filtres: &str) -> Vec<String> {
    //let relstart: &str = (
    //    if start != "./" {
    //    }
    //    else {
    //        &start
    //    }
    //);
    let can = fs::canonicalize("./");
    match can {
        Ok(v) => println!("{:?}", v),
        Err(e) => println!("{:?}", e)
    }
    //println!("{}", relstart);
    return vec![];
    // joinit = (
    //     lambda f: (
    //         os.path.join(
    //             (
    //                 start[2:] if
    //                 (start[:2] == './') else
    //                 start
    //             ),
    //             f
    //         )
    //     )
    // )
    // filterit = (
    //     lambda f: (
    //         filtres in f
    //     )
    // )
    // if (not os.path.exists(start)) or (not os.path.isdir(start)):
    //     return None
    // files = os.listdir(start)
    // fpaths = (
    //     iter(files) if
    //     (filtres is None) else
    //     filter(filterit, files)
    // )
    // paths = map(joinit, fpaths)
    // return paths;
}


//fn processrows (paths: &Vec<String>, full: bool) -> Vec<Row> {
//
//}


fn getfiles (start: &str, full: bool, filtres: &str) -> Vec<String> {
    let paths: Vec<String> = get_dir_listing(&start, &filtres);
    return paths;
    //if paths is None:
    //    return None
    //let processed: Vec<Row> = processrows(&paths, full);
    //sfiles = sorted(processed, key=sortfile, reverse=False)
    //out = list(sfiles)
    //return out;
}


fn run (start: &str, full: bool, filtres: &str) -> bool {
    let files: Vec<String> = getfiles(&start, full, &filtres);
    //if files is None:
    //    rendererror()
    //    return false;
    //rows = renderrows(files, full=full)
    //display(rows)
    println!("{:?}", &files);
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
    let start: &str = options.start.as_str();
    let full: bool = options.full;
    let filtres: &str = options.filtres.as_str();
    let ret: bool = run(&start, full, &filtres);
    if !ret {
        process::exit(1);
    }
    process::exit(0);
}
