
#![allow(unused_parens)]


extern crate argparse;

//use argparse::{ArgumentParser, StoreTrue, Store};


#[derive(Debug)]
struct Options {
    start: String,
    full: bool,
    filtres: String
}


#[derive(Debug)]
struct Row {

}


fn get_dir_listing (start: String, filtres: String) -> Vec<String> {
    if start != "./" {
        start = os.path.relpath(start)
    }
    joinit = (
        lambda f: (
            os.path.join(
                (
                    start[2:] if
                    (start[:2] == './') else
                    start
                ),
                f
            )
        )
    )
    filterit = (
        lambda f: (
            filtres in f
        )
    )
    if (not os.path.exists(start)) or (not os.path.isdir(start)):
        return None
    files = os.listdir(start)
    fpaths = (
        iter(files) if
        (filtres is None) else
        filter(filterit, files)
    )
    paths = map(joinit, fpaths)
    return paths;
}


fn getfiles (start: String, full: bool, filtres: String) -> Vec<String> {
    let paths: Vec<String> = get_dir_listing(start, filtres);
    return paths;
    //if paths is None:
    //    return None
    //processed = processrows(paths, full=full)
    //sfiles = sorted(processed, key=sortfile, reverse=False)
    //out = list(sfiles)
    //return out;
}


fn run (start: String, full: bool, filtres: String) -> bool {
    let files: Vec<Row> = getfiles(start, full, filtres);
    //if files is None:
    //    rendererror()
    //    return false;
    //rows = renderrows(files, full=full)
    //display(rows)
    println!("{:?}", files);
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
