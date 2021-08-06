use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;

use winapi::shared::minwindef::DWORD;
use winapi::um::winnls::CP_UTF8;

const CODEPAGE: DWORD = CP_UTF8;

pub fn rb_file_expand_path_internal<T>(
    fname: Option<&OsStr>,
    dname: Option<&OsStr>,
    abs_mode: bool,
    long_name: bool,
    result: (),
) -> Result<Vec<u8>, ()>
where
    T: AsRef<OsStr>,
{
    let mut wpath = None;

    let path = fname;
    let dir = dname;

    let cp = CODEPAGE;
    let path_cp = CODEPAGE;

    // convert to wide string
    if let Some(path) = path {
        wpath = path.encode_wide().collect::<Vec<_>>();
    }

    // determine if we need the user's home directory */
    // expand '~' only if NOT rb_file_absolute_path() where `abs_mode` is 1
    if !abs_mode
        && !wpath.is_empty()
        && wpath[0] == (b'~' as u16)
        && (wpath.len() == 1 || [b'/' as u16, b'\\' as u16].contains(wpath[1]))
    {
        let whome =
    }
}
