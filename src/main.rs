
#![allow(unused_variables)]
#![allow(unused_parens)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![feature(slice_concat_ext)]

extern crate argparse;

use std::process::{exit};
use std::path::{Path, PathBuf};
use std::io::{Error, ErrorKind};
use std::result::{Result};
use std::option::{Option};
use std::vec::{Vec, IntoIter as VecIntoIter};
use std::fs::{ReadDir, DirEntry, Metadata, Permissions};
use std::iter::{Filter, Map};
use std::os::unix::fs::{PermissionsExt, MetadataExt};
use std::collections::{HashMap};
use std::slice::{SliceConcatExt};
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

const COLDEF_TARGETNAME: u8 = 0;
const COLDEF_SRCNAME_DIR: u8 = 1;
const COLDEF_SRCNAME_FILE: u8 = 2;
const COLDEF_TIME: u8 = 3;
const COLDEF_SIZE_FILECOUNT: u8 = 4;
const COLDEF_SIZE_BYTES: u8 = 5;
const COLDEF_ACLS: u8 = 6;
const COLDEF_OWNER: u8 = 7;
const COLDEF_SIZE: u8 = 8;
const COLDEF_FILETYPE: u8 = 9;
const COLDEF_PREVIEW: u8 = 10;
const COLDEF_DEFAULT: u8 = 11;

const FIELDNAME_ACLS: u8 = 0;
const FIELDNAME_OWNER: u8 = 1;
const FIELDNAME_FILETYPE: u8 = 2;
const FIELDNAME_SIZE: u8 = 3;
const FIELDNAME_TIMEISO: u8 = 4;
const FIELDNAME_SRCNAME: u8 = 5;
const FIELDNAME_TARGETNAME: u8 = 6;
const FIELDNAME_PREVIEW: u8 = 7;

const CLR_RED: &'static str = "\033[31m";
const CLR_MAGENTA: &'static str = "\033[35m";
const CLR_LIGHT_MAGENTA: &'static str = "\033[95m";
const CLR_GREEN: &'static str = "\033[32m";
const CLR_LIGHT_GREEN: &'static str = "\033[92m";
const CLR_LIGHT_CYAN: &'static str = "\033[96m";
const CLR_LIGHT_YELLOW: &'static str = "\033[93m";
const CLR_LIGHT_GRAY: &'static str = "\033[37m";
const CLR_DARK_GRAY: &'static str = "\033[30m";
const CLR_LIGHT_RED: &'static str = "\033[91m";
const CLR_LIGHT_BLUE: &'static str = "\033[31m";
const CLR_BLUE: &'static str = "\033[34m";
const CLR_END: &'static str = "\033[0m";

const CLRID_RED: u8 = 0;
const CLRID_MAGENTA: u8 = 1;
const CLRID_LIGHT_MAGENTA: u8 = 2;
const CLRID_GREEN: u8 = 3;
const CLRID_LIGHT_GREEN: u8 = 4;
const CLRID_LIGHT_CYAN: u8 = 5;
const CLRID_LIGHT_YELLOW: u8 = 6;
const CLRID_LIGHT_GRAY: u8 = 7;
const CLRID_DARK_GRAY: u8 = 8;
const CLRID_LIGHT_RED: u8 = 9;
const CLRID_LIGHT_BLUE: u8 = 10;
const CLRID_BLUE: u8 = 11;

const CLRVAL_TARGETNAME: u8 = CLRID_LIGHT_CYAN;
const CLRVAL_SRCNAME_DIR: u8 = CLRID_LIGHT_RED;
const CLRVAL_SRCNAME_FILE: u8 = CLRID_LIGHT_GREEN;
const CLRVAL_TIME: u8 = CLRID_BLUE;
const CLRVAL_SIZE_FILECOUNT: u8 = CLRID_MAGENTA;
const CLRVAL_SIZE_BYTES: u8 = CLRID_GREEN;
const CLRVAL_ACLS: u8 = CLRID_DARK_GRAY;
const CLRVAL_OWNER: u8 = CLRID_DARK_GRAY;
const CLRVAL_FILETYPE: u8 = CLRID_DARK_GRAY;
const CLRVAL_PREVIEW: u8 = CLRID_DARK_GRAY;
const CLRVAL_DEFAULT: u8 = CLRID_LIGHT_MAGENTA;


struct RowInfo {
    fname: String,
    stat_res: Metadata,
    ftype: u8,
    contenttype: u8
}


#[derive(Debug)]
struct RowRendered {
    acls: String,
    owner: String,
    filetype: String,
    size: String,
    timeiso: String,
    srcname: String,
    targetname: String,
    preview: String
}

#[derive(Debug)]
struct RowPadding {
    acls: String,
    owner: String,
    filetype: String,
    size: String,
    timeiso: String,
    srcname: String,
    targetname: String,
    preview: String
}


struct AllRowDefs {
    acls: RowDef,
    owner: RowDef,
    filetype: RowDef,
    size: RowDef,
    timeiso: RowDef,
    srcname: RowDef,
    targetname: RowDef,
    preview: RowDef
}


struct Row {
    info: RowInfo,
    render: RowRendered
}


struct RowDef {
    name: u8,
    onlyfull: bool,
    align: u8,
    func: (fn (&RowInfo) -> Result<String, Error>)
}


#[derive(Debug)]
struct Options {
    start: String,
    full: bool,
    filtres: String
}


impl<'a> IntoIterator for &'a RowRendered {
    type Item = (u8, &'a String);
    type IntoIter = VecIntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        vec![
            (FIELDNAME_ACLS, &self.acls),
            (FIELDNAME_OWNER, &self.owner),
            (FIELDNAME_FILETYPE, &self.filetype),
            (FIELDNAME_SIZE, &self.size),
            (FIELDNAME_TIMEISO, &self.timeiso),
            (FIELDNAME_SRCNAME, &self.srcname),
            (FIELDNAME_TARGETNAME, &self.targetname),
            (FIELDNAME_PREVIEW, &self.preview)
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
    let ipaths: VecIntoIter<String> = vpaths.into_iter();
    let fpaths: Filter<VecIntoIter<String>, _> = ipaths.filter(&filesfilt);
    let paths: Vec<String> = fpaths.collect();
    return Ok(paths);
}


fn col_acls (rowinfo: &RowInfo) -> Result<String, Error> {
    let all_acls_mode: String = rowinfo.stat_res.mode().to_string();
    //let me_acls_mode: String =  match get_acls_me(&fname, &stat_res) {
    //    Ok(v) => v,
    //    Err(e) => return Err(e)
    //};
    //let ret = ' '.join([all_acls_mode, me_acls_mode])
    let ret: String = all_acls_mode;
    return Ok(ret);
}


fn col_owner (rowinfo: &RowInfo) -> Result<String, Error> {
    let ret: String = "OWNER".to_string();
    return Ok(ret);
}


fn col_filetype (rowinfo: &RowInfo) -> Result<String, Error> {
    let ret: String = "FILETYPE".to_string();
    return Ok(ret);
}


fn col_size (rowinfo: &RowInfo) -> Result<String, Error> {
    let ret: String = "SIZE".to_string();
    return Ok(ret);
}


fn col_timeiso (rowinfo: &RowInfo) -> Result<String, Error> {
    let ret: String = "TIMEISO".to_string();
    return Ok(ret);
}


fn col_srcname (rowinfo: &RowInfo) -> Result<String, Error> {
    let ret: String = "SRCNAME".to_string();
    return Ok(ret);
}


fn col_targetname (rowinfo: &RowInfo) -> Result<String, Error> {
    let ret: String = "TARGETNAME".to_string();
    return Ok(ret);
}


fn col_preview (rowinfo: &RowInfo) -> Result<String, Error> {
    let ret: String = "PREVIEW".to_string();
    return Ok(ret);
}


fn getrowdefs () -> Result<AllRowDefs, Error> {
    let rowdefs: AllRowDefs = AllRowDefs {
        acls: RowDef {
            name: FIELDNAME_ACLS,
            func: col_acls,
            onlyfull: true,
            align: ROWDEF_ALIGN_LEFT
        },
        owner: RowDef {
            name: FIELDNAME_OWNER,
            func: col_owner,
            onlyfull: true,
            align: ROWDEF_ALIGN_LEFT
        },
        filetype: RowDef {
            name: FIELDNAME_FILETYPE,
            func: col_filetype,
            onlyfull: true,
            align: ROWDEF_ALIGN_LEFT
        },
        size: RowDef {
            name: FIELDNAME_SIZE,
            func: col_size,
            onlyfull: true,
            align: ROWDEF_ALIGN_LEFT
        },
        timeiso: RowDef {
            name: FIELDNAME_TIMEISO,
            func: col_timeiso,
            onlyfull: true,
            align: ROWDEF_ALIGN_LEFT
        },
        srcname: RowDef {
            name: FIELDNAME_SRCNAME,
            func: col_srcname,
            onlyfull: true,
            align: ROWDEF_ALIGN_LEFT
        },
        targetname: RowDef {
            name: FIELDNAME_TARGETNAME,
            func: col_targetname,
            onlyfull: true,
            align: ROWDEF_ALIGN_LEFT
        },
        preview: RowDef {
            name: FIELDNAME_PREVIEW,
            func: col_preview,
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
    let val_acls: String = match (fdefs.acls.func)(&rowinfo) {
        Ok(v) => v,
        Err(e) => return Err(e)
    };
    let val_owner: String = match (fdefs.owner.func)(&rowinfo) {
        Ok(v) => v,
        Err(e) => return Err(e)
    };
    let val_filetype: String = match (fdefs.filetype.func)(&rowinfo) {
        Ok(v) => v,
        Err(e) => return Err(e)
    };
    let val_size: String = match (fdefs.size.func)(&rowinfo) {
        Ok(v) => v,
        Err(e) => return Err(e)
    };
    let val_timeiso: String = match (fdefs.timeiso.func)(&rowinfo) {
        Ok(v) => v,
        Err(e) => return Err(e)
    };
    let val_srcname: String = match (fdefs.srcname.func)(&rowinfo) {
        Ok(v) => v,
        Err(e) => return Err(e)
    };
    let val_targetname: String = match (fdefs.targetname.func)(&rowinfo) {
        Ok(v) => v,
        Err(e) => return Err(e)
    };
    let val_preview: String = match (fdefs.preview.func)(&rowinfo) {
        Ok(v) => v,
        Err(e) => return Err(e)
    };
    // Build output struct
    let ret: RowRendered = RowRendered {
        acls: val_acls,
        owner: val_owner,
        filetype: val_filetype,
        size: val_size,
        timeiso: val_timeiso,
        srcname: val_srcname,
        targetname: val_targetname,
        preview: val_preview
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
    let files_iter: VecIntoIter<String> = files.into_iter();
    let files_build: Map<VecIntoIter<String>, _> = files_iter.map(&func_build);
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


fn getcolpaddings (rows: &Vec<Row>) -> Result<RowPadding, Error> {
    //
    // This capacity needs to be increased to match the number
    // of fields in RowPadding struct.
    //
    let mut longest: HashMap<&str, u8> = HashMap::with_capacity(1);
    //
    // Initialize values
    //
    longest.insert(FIELDNAME_ACLS, 0);
    longest.insert(FIELDNAME_OWNER, 0);
    longest.insert(FIELDNAME_FILETYPE, 0);
    longest.insert(FIELDNAME_SIZE, 0);
    longest.insert(FIELDNAME_TIMEISO, 0);
    longest.insert(FIELDNAME_SRCNAME, 0);
    longest.insert(FIELDNAME_TARGETNAME, 0);
    longest.insert(FIELDNAME_PREVIEW, 0);
    //
    // Cycle through paddings
    //
    for row in rows {
        for col in &row.render {
            let colname: u8 = col.0;
            let colval: &String = &col.1;
            let collen: u8 = (colval.len() as u8);
            //println!("COLNAME: {}; COLVAL: {}; COLLEN: {}", &colname, &colval, &collen);
            if let Some(x) = longest.get_mut(colname) {
                let tmp_collen: u8 = (collen as u8);
                let tmp_x: u8 = (*x as u8);
                //println!("tmp_collen: {:?}; tmp_x: {:?}", tmp_collen, tmp_x);
                if tmp_collen > tmp_x {
                    *x = collen;
                }
            }
        }
    }
    //
    // Convert hashmap to struct
    //
    let ret: RowPadding = RowPadding {
        //
        // ACLS
        //
        acls: match longest.get(FIELDNAME_ACLS) {
            Some(v) => *v,
            None => 0
        },
        owner: match longest.get(FIELDNAME_OWNER) {
            Some(v) => *v,
            None => 0
        },
        filetype: match longest.get(FIELDNAME_FILETYPE) {
            Some(v) => *v,
            None => 0
        },
        size: match longest.get(FIELDNAME_SIZE) {
            Some(v) => *v,
            None => 0
        },
        timeiso: match longest.get(FIELDNAME_TIMEISO) {
            Some(v) => *v,
            None => 0
        },
        srcname: match longest.get(FIELDNAME_SRCNAME) {
            Some(v) => *v,
            None => 0
        },
        targetname: match longest.get(FIELDNAME_TARGETNAME) {
            Some(v) => *v,
            None => 0
        },
        preview: match longest.get(FIELDNAME_PREVIEW) {
            Some(v) => *v,
            None => 0
        }
    };
    return Ok(ret);
}


fn getcolslisting (full: bool) -> Result<Vec<u8>, Error> {
    let mut out: Vec<u8> = vec![];
    // This is just temporary
    if full {
         out.push(FIELDNAME_ACLS);
         out.push(FIELDNAME_OWNER);
         out.push(FIELDNAME_FILETYPE);
    }
    out.push(FIELDNAME_SIZE);
    out.push(FIELDNAME_TIMEISO);
    out.push(FIELDNAME_SRCNAME);
    out.push(FIELDNAME_TARGETNAME);
    if full {
        out.push(FIELDNAME_PREVIEW);
    }
    let ret: Vec<u8> = out;
    return Ok(ret);
}


fn get_field_from_fdefs (
    field: u8,
    fdefs: &AllRowDefs
) -> Result<&RowDef, Error> {
    if field == FIELDNAME_ACLS {
        return Ok(&fdefs.acls);
    }
    if field == FIELDNAME_OWNER {
        return Ok(&fdefs.owner);
    }
    if field == FIELDNAME_FILETYPE {
        return Ok(&fdefs.filetype);
    }
    if field == FIELDNAME_SIZE {
        return Ok(&fdefs.size);
    }
    if field == FIELDNAME_TIMEISO {
        return Ok(&fdefs.timeiso);
    }
    if field == FIELDNAME_SRCNAME {
        return Ok(&fdefs.srcname);
    }
    if field == FIELDNAME_TARGETNAME {
        return Ok(&fdefs.targetname);
    }
    if field == FIELDNAME_PREVIEW {
        return Ok(&fdefs.preview);
    }
    return Err(Error::new(ErrorKind::Other, "Bad Fdef!"));
}


fn getcolefs (row: &Row, field: u8) -> Result<u8, Error> {
    if field == FIELDNAME_TARGETNAME {
        let col: u8 = COLDEF_TARGETNAME;
    }
    else if field == FIELDNAME_SRCNAME {
        if row.info.ftype == FTYPE_DIR {
            let col: u8 = COLDEF_SRCNAME_DIR;
        }
        else {
            let col: u8 = COLDEF_SRCNAME_FILE;
        }
    }
    else if field == FIELDNAME_TIMEISO {
        let col: u8 = COLDEF_TIME;
    }
    else if field == FIELDNAME_SIZE {
        if row.info.ftype == FTYPE_DIR {
            let col: u8 = COLDEF_SIZE_FILECOUNT;
        }
        else {
            let col: u8 = COLDEF_SIZE_BYTES;
        }
    }
    else if field == FIELDNAME_ACLS {
        let col: u8 = COLDEF_ACLS;
    }
    else if field == FIELDNAME_OWNER {
        let col: u8 = COLDEF_OWNER;
    }
    else if field == FIELDNAME_SIZE {
        let col: u8 = COLDEF_SIZE;
    }
    else if field == FIELDNAME_FILETYPE {
        let col: u8 = COLDEF_FILETYPE;
    }
    else if field == FIELDNAME_PREVIEW {
        let col: u8 = COLDEF_PREVIEW;
    }
    else {
        let col: u8 = COLDEF_DEFAULT;
    }
    return Ok(col);
}


fn getcolorval (coldef: u8) -> Result<u8, Error> {
    if (coldef == COLDEF_TARGETNAME) {
        let val: u8 = CLRVAL_TARGETNAME;
    }
    else if (coldef == COLDEF_SRCNAME_DIR) {
        let val: u8 = CLRVAL_SRCNAME_DIR;
    }
    else if (coldef == COLDEF_SRCNAME_FILE) {
        let val: u8 = CLRVAL_SRCNAME_FILE;
    }
    else if (coldef == COLDEF_TIME) {
        let val: u8 = CLRVAL_TIME;
    }
    else if (coldef == COLDEF_SIZE_FILECOUNT) {
        let val: u8 = CLRVAL_SIZE_FILECOUNT;
    }
    else if (coldef == COLDEF_SIZE_BYTES) {
        let val: u8 = CLRVAL_SIZE_BYTES;
    }
    else if (coldef == COLDEF_ACLS) {
        let val: u8 = CLRVAL_ACLS;
    }
    else if (coldef == CLRDEF_OWNER) {
        let val: u8 = CLRVAL_OWNER;
    }
    else if (coldef == COLDEF_FILETYPE) {
        let val: u8 = CLRVAL_FILETYPE;
    }
    else if (coldef == COLDEF_PREVIEW) {
        let val: u8 = CLRVAL_PREVIEW;
    }
    else if (coldef == COLDEF_DEFAULT) {
        let val: u8 = CLRVAL_DEFAULT;
    }
    return Ok(val);
}


fn get_rowrenderedfield (
    rowrendered: &RowRendered,
    field: u8
) -> Result<&String, Error> {
    if (field == FIELDNAME_ACLS) {
        return Ok(&rowrendered.acls);
    }
    else if (field == FIELDNAME_OWNER) {
        return Ok(&rowrendered.owner);
    }
    else if (field == FIELDNAME_FILETYPE) {
        return Ok(&rowrendered.filetype);
    }
    else if (field == FIELDNAME_SIZE) {
        return Ok(&rowrendered.size);
    }
    else if (field == FIELDNAME_TIMEISO) {
        return Ok(&rowrendered.timeiso);
    }
    else if (field == FIELDNAME_SRCNAME) {
        return Ok(&rowrendered.srcname);
    }
    else if (field == FIELDNAME_TARGETNAME) {
        return Ok(&rowrendered.targetname);
    }
    else if (field == FIELDNAME_PREVIEW) {
        let Ok(&rowrendered.preview);
    }
    return Err(Error::new(ErrorKind::Other, "Bad field"));
}


fn addpadding (
    field: u8,
    val: &String,
    colpaddings: &RowPadding,
    align: u8
) -> Result<&String, Error> {
    if val.len() == 0 {
        return Ok(" ".to_string())
    }
    let alignchar: &'static str = {
        if align == ROWDEF_ALIGN_RIGHT {
            ">"
        }
        else {
            "<"
        }
    }
    padlen = colpaddings[field]
    let padstr: String = ''.join(['{:', alignchar, str(padlen), 's}'])
    ret = padstr.format(val)
    return ret
}


fn makepretty (
    row: &Row,
    field: u8,
    colpaddings: &RowPadding,
    fdefs: &AllRowDefs
) {
    let fdef_field: &RowDef = get_field_from_fdefs(field).unwrap();
    let align: u8 = fdef_field.align;
    let col: u8 = getcoldefs(row, field).unwrap();
    let clrval: u8 = getcolorval(col).unwrap();
    let textval: &String = get_rowrenderedfield(&row.render, field).unwrap();
    let paddedval: &String = addpadding(field, textval, colpaddings, align);
    let colorval = addcolor(paddedval, clrval);
    return colorval;
}


fn structurecols (
    row: &Row,
    colpaddings: &RowPadding,
    fdefs: &AllRowDefs,
    full: bool
) {
    let colslisting: Vec<u8> = getcolslisting(full).unwrap();
    let func = |name: u8| ->  {
        makepretty(row, name, colpaddings, fdefs)
    };
    let ret = map(func, colslisting);
    return ret;
}


fn rendercols (
    row: &Row,
    colpaddings: &RowPadding,
    fdefs: &AllRowDefs,
    full: bool
) -> Result<String, Error> {
    //margin = '  ';
    let structcols = structurecols(row, colpaddings, fdefs, full);
    //ret = ''.join([margin, margin.join(structcols)]);
    //return ret;
    return Ok("TEST".to_string());
}


fn renderrows (files: Vec<Row>, full: bool) -> Result<String, Error> {
    let colpaddings: RowPadding = getcolpaddings(&files).unwrap();
    let fdefs: AllRowDefs = match getrowdefs() {
        Ok(v) => v,
        Err(e) => return Err(e)
    };
    let renderer = |row: Row| -> String {
        rendercols(
            &row,
            &colpaddings,
            &fdefs,
            full
        ).unwrap()
    };
    let files_iter: VecIntoIter<Row> = files.into_iter();
    let rendered: Vec<String> = files_iter.map(&renderer).collect();
    let out: String = rendered.join("\n");
    return Ok(out);
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
