
#![allow(unused_parens)]


extern crate argparse;

use std::process::{exit};
use std::path::{Path, PathBuf};
use std::io::{Error, ErrorKind};
use std::result::{Result};
use std::option::{Option};
use std::fs;

use argparse::{ArgumentParser, StoreTrue as ArgStoreTrue, Store as ArgStore};


#[derive(Debug)]
struct Options {
    start: String,
    full: bool,
    filtres: String
}


#[derive(Debug)]
#[allow(dead_code)]
struct Row {

}


fn path_canonicalize (start: String) -> Result<String, Error> {
    let can: Result<PathBuf, Error> = fs::canonicalize(start);
    let foo: PathBuf = match can {
        Ok(v) => v,
        Err(e) => return Err(e)
    };
    let bar: &Path = foo.as_path();
    let faz: Option<&str> = bar.to_str();
    let baz: String = match faz {
        Some(gaf) => gaf.to_string(),
        None => return Err(Error::new(ErrorKind::Other, "Woops!"))
    };
    return Ok(baz);
}


fn get_dir_listing (start: String, filtres: String) -> Result<Vec<String>, Error> {
    let relstart: String = {
        if start != "./" {
            let canouter: Result<String, Error> = path_canonicalize(start);
            let caninner: String = match canouter {
                Ok(v) => v,
                Err(e) => return Err(e)
            };
            caninner
        }
        else {
            start.to_string()
        }
    };
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
    let files = match fs::read_dir(relstart) {
        Ok(v) => v,
        Err(e) => return Err(e)
    };
    for file in files {
        println!("File: {}", file.unwrap().path().display());
    }
    // fpaths = (
    //     iter(files) if
    //     (filtres is None) else
    //     filter(filterit, files)
    // )
    // paths = map(joinit, fpaths)
    // return paths;
    println!("{}", filtres);
    return Ok(vec![]);
}


fn getfiles (start: String, full: bool, filtres: String) -> Result<Vec<String>, Error> {
    let respaths: Result<Vec<String>, Error> = get_dir_listing(start, filtres);
    let paths: Vec<String> = match respaths {
        Ok(v) => v,
        Err(e) => return Err(e)
    };
    println!("{}", full);
    return Ok(paths);
    //if paths is None:
    //    return None
    //let processed: Vec<Row> = processrows(paths, full);
    //sfiles = sorted(processed, key=sortfile, reverse=False)
    //out = list(sfiles)
    //return out;
}


fn run (start: String, full: bool, filtres: String) -> Result<bool, Error> {
    let resfiles: Result<Vec<String>, Error> = getfiles(start, full, filtres);
    let files: Vec<String> = match resfiles {
        Ok(v) => v,
        Err(e) => return Err(e)
    };
    //if files is None:
    //    rendererror()
    //    return false;
    //rows = renderrows(files, full=full)
    //display(rows)
    println!("{:?}", files);
    return Ok(true);
}


fn getargs () -> Result<Options, Error> {
    let mut options: Options = Options {
        full: false,
        start: "./".to_string(),
        filtres: "".to_string()
    };
    {
        let mut aparse: ArgumentParser = ArgumentParser::new();
        aparse.set_description("Replacement for ls");
        aparse
            .refer(&mut options.start)
            .add_option(
                &["-s", "--start"],
                ArgStore,
                "Starting Path"
            );
        aparse
            .refer(&mut options.full)
            .add_option(
                &["-f", "--full"],
                ArgStoreTrue,
                "Full Output"
            );
        aparse
            .refer(&mut options.filtres)
            .add_option(
                &["-g", "--filter"],
                ArgStore,
                "Filter Results"
            );
        aparse.parse_args_or_exit();
    }
    return Ok(options);
}


fn main () -> () {
    let resoptions: Result<Options, Error> = getargs();
    let options: Options = match resoptions {
        Ok(v) => v,
        Err(_) => exit(1)
    };
    let start: String = options.start;
    let full: bool = options.full;
    let filtres: String = options.filtres;
    let ret: Result<bool, Error> = run(start, full, filtres);
    match ret {
        Ok(_) => exit(0),
        Err(_) => exit(1)
    }
}