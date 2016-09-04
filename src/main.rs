
#![allow(unused_variables)]
#![allow(unused_parens)]
#![allow(dead_code)]
#![allow(unused_imports)]


extern crate argparse;

use std::process::{exit};
use std::path::{Path, PathBuf};
use std::io::{Error, ErrorKind};
use std::result::{Result};
use std::option::{Option};
use std::vec::{Vec, IntoIter};
use std::fs::{ReadDir, DirEntry, Metadata, Permissions};
use std::iter::{Filter, Map};
use std::os::unix::fs::{PermissionsExt, MetadataExt};
use std::collections::{HashMap};
use std::fs;

use argparse::{ArgumentParser, StoreTrue as ArgStoreTrue, Store as ArgStore};

const ROWDEF_ALIGN_LEFT: u8 = 0;
const ROWDEF_ALIGN_RIGHT: u8 = 1;

const FTYPE_DIR: u8 = 0;
const FTYPE_FILE: u8 = 1;

const CONTYPE_DIR: u8 = 0;
const CONTYPE_UNREADABLE: u8 = 1;
const CONTYPE_EMPTY: u8 = 2;
const CONTYPE_BINEXEC: u8 = 3;
const CONTYPE_BINOTHER: u8 = 4;
const CONTYPE_TEXT: u8 = 5;
const CONTYPE_OTHER: u8 = 6;


struct RowInfo {
    fname: String,
    stat_res: Metadata,
    ftype: u8,
    contenttype: u8
}


#[derive(Debug)]
struct RowRendered {
    acls: String
    // owner: String,
    // filetype: String,
    // size: String,
    // timeiso: String,
    // srcname: String,
    // targetname: String,
    // preview: String
}


struct Row {
    info: RowInfo,
    render: RowRendered
}


struct RowDef {
    name: &'static str,
    onlyfull: bool,
    align: u8,
    func: (fn (&RowInfo) -> Result<String, Error>)
}


struct AllRowDefs {
    acls: RowDef
    // owner: RowDef,
    // filetype: RowDef,
    // size: RowDef,
    // timeiso: RowDef,
    // srcname: RowDef,
    // targetname: RowDef,
    // preview: RowDef
}


#[derive(Debug)]
struct Options {
    start: String,
    full: bool,
    filtres: String
}


impl IntoIterator for RowRendered {
    pub fn into_iter(self) -> IntoIter {
        [
            ("acls", &self.acls)
        ].into_iter()
    }
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


fn get_dir_listing (start: &String, filtres: &String) -> Result<Vec<String>, Error> {
    let relstart: String = {
        if start != "./" {
            match path_canonicalize(&start) {
                Ok(v) => v,
                Err(e) => return Err(e)
            }
        }
        else {
            start.to_string()
        }
    };
    let meta: Metadata = match fs::metadata(&relstart) {
        Ok(v) => v,
        Err(e) => return Err(e)
    };
    if !meta.is_dir() {
        return Err(Error::new(ErrorKind::Other, "Not a directory!"));
    }
    let rfiles: ReadDir = match fs::read_dir(&relstart) {
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


fn get_acls_all (fname: &String, stat_res: &Metadata) -> Result<String, Error> {
    //let all_acls_mode = str(oct(stat.S_IMODE(stat_res.st_mode)))[-3:];
    let all_acls_mode: String = stat_res.mode().to_string();
    return Ok(all_acls_mode);
}


fn col_acls (rowinfo: &RowInfo) -> Result<String, Error> {
    let all_acls_mode: String = {
        match get_acls_all(&rowinfo.fname, &rowinfo.stat_res) {
            Ok(v) => v,
            Err(e) => return Err(e)
        }
    };
    //let me_acls_mode: String =  match get_acls_me(&fname, &stat_res) {
    //    Ok(v) => v,
    //    Err(e) => return Err(e)
    //};
    //let ret = ' '.join([all_acls_mode, me_acls_mode])
    let ret: String = all_acls_mode;
    return Ok(ret);
}



fn getrowdefs () -> Result<AllRowDefs, Error> {
    let rowdefs: AllRowDefs = AllRowDefs {
        acls: RowDef {
            name: "acls",
            func: col_acls,
            onlyfull: true,
            align: ROWDEF_ALIGN_LEFT
        }
    };
    println!("{}", ROWDEF_ALIGN_RIGHT);
    return Ok(rowdefs);
}


fn getrowinfo (fname: String) -> Result<RowInfo, Error> {
    let stat_res: Metadata = match fs::metadata(&fname) {
        Ok(v) => v,
        Err(e) => return Err(e)
    };
    let ftype: u8 = FTYPE_DIR;
    let contenttype: u8 = CONTYPE_DIR;
    let ret: RowInfo = RowInfo {
        fname: fname,
        stat_res: stat_res,
        ftype: ftype,
        contenttype: contenttype
    };
    return Ok(ret);
}


fn get_fileinfo_rendered (fdefs: &AllRowDefs, rowinfo: &RowInfo) -> Result<RowRendered, Error> {
    // func = (
    //     lambda rec: (
    //         rec['name'],
    //         (
    //             rec['func'](rowinfo) if
    //             shouldbuild(rec, full=full) else
    //             ' '
    //         )
    //     )
    // )
    // return dict(map(func, fdefs.values()))
    let acls: String = match (fdefs.acls.func)(&rowinfo) {
        Ok(v) => v,
        Err(e) => return Err(e)
    };
    let ret: RowRendered = RowRendered {
        acls: acls
    };
    return Ok(ret);
}


fn buildrow (fname: String, fdefs: &AllRowDefs, full: bool) -> Result<Row, Error> {
    let rowinfo: RowInfo = match getrowinfo(fname) {
        Ok(v) => v,
        Err(e) => return Err(e)
    };
    let rowrender: RowRendered = match get_fileinfo_rendered(&fdefs, &rowinfo) {
        Ok(v) => v,
        Err(e) => return Err(e)
    };
    let row: Row = Row {
        info: rowinfo,
        render: rowrender
    };
    return Ok(row);
}


fn processrows (files: Vec<String>, full: bool) -> Result<Vec<Row>, Error> {
    let fdefs: AllRowDefs = match getrowdefs() {
        Ok(v) => v,
        Err(e) => return Err(e)
    };
    //
    // There should probably be a better way to handle this?
    //
    let func_build = |fname: String| -> Row {
        buildrow(fname, &fdefs, full).unwrap()
    };
    let files_iter: IntoIter<String> = files.into_iter();
    let files_build: Map<IntoIter<String>, _> = files_iter.map(&func_build);
    let out: Vec<Row> = files_build.collect();
    return Ok(out);
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


fn getcolpaddings (rows: Vec<Row>) -> HashMap<&str, u8> {
    let mut collen: u8 = 0;
    let mut longest: HashMap<&str, u8> = HashMap::new();
    for row in rows:
        for col in row.render.into_iter() {
            let colname: &str = col.0;
            let colval: &str = col.1;
            if !longest.contains_key(&colname) {
                longest.insert(&colname, 0);
            }
            collen = colval.len();
            if let Some(x) = longest.get_mut(&colname) {
                if collen > x {
                    *x = collen;
                }
            }
        }
    return longest;
}


fn renderrows (files: Vec<Row>, full: bool) -> Result<String, Error> {
    let colpaddings: HashMap<&str, u8> = getcolpaddings(files);
    //fdefs = getrowdefs();
    //renderer = lambda r: rendercols(r, colpaddings, fdefs, full=full);
    //out = '\n'.join(map(renderer, files));
    //return out;
    return Ok("Testing".to_string());
}


fn display (outdata: String) -> Result<&'static str, Error> {
    println!("{}", &outdata);
    return Ok("");
}


fn run (start: &String, full: bool, filtres: &String) -> Result<&'static str, Error> {
    let filesres: Result<Vec<Row>, Error> = getfiles(&start, full, &filtres);
    if let Err(e) = filesres {
        println!("Error!!!");
        //rendererror();
        return Err(Error::new(ErrorKind::Other, ""));
    }
    let files: Vec<Row> = filesres.unwrap();
    let outdata: String = renderrows(files, full).unwrap();
    let dispres: Result<&'static str, Error> = display(outdata);
    return Ok("");
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
    let ret: Result<&str, Error> = run(&start, full, &filtres);
    match ret {
        Ok(_) => exit(0),
        Err(_) => exit(1)
    }
}
