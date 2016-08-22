
#![allow(unused_parens)]


extern crate argparse;

use std::process::{exit};
use std::path::{Path, PathBuf};
use std::io::{Error, ErrorKind};
use std::result::{Result};
use std::option::{Option};
use std::vec::{Vec, IntoIter};
use std::fs::{ReadDir, DirEntry, Metadata};
use std::iter::{Filter};
//use std::ops::{Fn, FnMut};
//use std::collections::{HashMap};
use std::fs;

use argparse::{ArgumentParser, StoreTrue as ArgStoreTrue, Store as ArgStore};

const ROWDEF_ALIGN_LEFT: u8 = 0;
const ROWDEF_ALIGN_RIGHT: u8 = 1;


#[derive(Debug)]
#[allow(dead_code)]
struct FileInfo {
    fname: bool,
    stat_res: bool,
    ftype: bool,
    contenttype: bool,
    timeepoch: bool
}


#[derive(Debug)]
#[allow(dead_code)]
struct RowDef<'a> {
    name: &'a str,
    onlyfull: bool,
    align: u8,
    func: (fn (FileInfo) -> String)
}


#[derive(Debug)]
#[allow(dead_code)]
struct AllRowDefs<'a> {
    acls: RowDef<'a>
    // owner: RowDef<'a>,
    // filetype: RowDef<'a>,
    // size: RowDef<'a>,
    // timeiso: RowDef<'a>,
    // srcname: RowDef<'a>,
    // targetname: RowDef<'a>,
    // preview: RowDef<'a>
}


#[derive(Debug)]
#[allow(dead_code)]
struct Options {
    start: String,
    full: bool,
    filtres: String
}


#[derive(Debug)]
#[allow(dead_code)]
struct Row {

}


fn path_canonicalize (start: &String) -> Result<String, Error> {
    let can: Result<PathBuf, Error> = fs::canonicalize(&start);
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


fn build_error (msg: &str) -> Result<Vec<String>, Error> {
    return Err(Error::new(ErrorKind::Other, msg));
}


fn get_dir_listing (start: &String, filtres: &String) -> Result<Vec<String>, Error> {
    let relstart: String = {
        if start != "./" {
            let canouter: Result<String, Error> = path_canonicalize(&start);
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
    let metares: Result<Metadata, Error> = fs::metadata(&relstart);
    let meta: Metadata = match metares {
        Ok(v) => v,
        Err(e) => return Err(e)
    };
    if !meta.is_dir() {
        return build_error("Not a directory!");
    }
    let dirres: Result<ReadDir, Error> = fs::read_dir(&relstart);
    let rfiles: ReadDir = match dirres {
        Ok(v) => v,
        Err(e) => return Err(e)
    };
    let filesmapper = |fpath: Result<DirEntry, Error>| -> String {
        let pbpath: PathBuf = fpath.unwrap().path();
        let ppath: &Path = pbpath.as_path();
        let spath: Option<&str> = ppath.to_str();
        let upath: &str = spath.unwrap();
        // Strip off leading "./"
        let tpath: &str = &upath[2..];
        let path: String = tpath.to_string();
        path
    };
    let filesfilt = |f: &String| -> bool {
        filtres == &"".to_string() ||
            f.contains(filtres)
    };
    let vpaths: Vec<String> = rfiles.map(&filesmapper).collect();
    let ipaths: IntoIter<String> = vpaths.into_iter();
    let fpaths: Filter<IntoIter<String>, _> = ipaths.filter(&filesfilt);
    let paths: Vec<String> = fpaths.collect();
    return Ok(paths);
}


fn col_acls (rowinfo: FileInfo) -> String {
    // fname = rowinfo['fname']
    // stat_res = rowinfo['stat_res']
    // all_acls_mode = get_acls_all(fname, stat_res)
    // me_acls_mode = get_acls_me(fname, stat_res)
    // ret = ' '.join([all_acls_mode, me_acls_mode])
    // return ret
    println!("{:?}", rowinfo);
    return "".to_string();
}

fn getrowdefs () -> Result<AllRowDefs<'a>, Error> {
    let rowdefs: AllRowDefs = AllRowDefs {
        acls: RowDef {
            name: "acls",
            func: col_acls,
            onlyfull: true,
            align: ROWDEF_ALIGN_LEFT
        }
        // owner: RowDef {
        //
        // },
        // filetype: RowDef {
        //
        // },
        // size: RowDef {
        //
        // },
        // timeiso: RowDef {
        //
        // },
        // srcname: RowDef {
        //
        // },
        // targetname: RowDef {
        //
        // },
        // preview: RowDef {
        //
        // }
    };
    return Ok(rowdefs);
}


fn processrows (files: Vec<String>, full: bool) -> Result<Vec<Row>, Error> {
    let fdefs_res: Result<AllRowDefs, Error> = getrowdefs();
    //func = lambda fname: buildrow(fname, fdefs, full=full)
    //out = map(func, files)
    //return out
    println!("{:?}", fdefs_res);
    println!("{:?}", &files);
    println!("{}", full);
    return Ok(vec![]);
}


fn getfiles (start: &String, full: bool, filtres: &String) -> Result<Vec<Row>, Error> {
    let respaths: Result<Vec<String>, Error> = get_dir_listing(&start, &filtres);
    let paths: Vec<String> = match respaths {
        Ok(v) => v,
        Err(e) => return Err(e)
    };
    //if paths is None:
    //    return None
    let resproc: Result<Vec<Row>, Error> = processrows(paths, full);
    let processed: Vec<Row> = match resproc {
        Ok(v) => v,
        Err(e) => return Err(e)
    };
    println!("{}", full);
    return Ok(processed);
    //sfiles = sorted(processed, key=sortfile, reverse=False)
    //out = list(sfiles)
    //return out;
}


fn run (start: &String, full: bool, filtres: &String) -> Result<bool, Error> {
    let resfiles: Result<Vec<Row>, Error> = getfiles(&start, full, &filtres);
    let files: Vec<Row> = match resfiles {
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
    let ret: Result<bool, Error> = run(&start, full, &filtres);
    match ret {
        Ok(_) => exit(0),
        Err(_) => exit(1)
    }
}
