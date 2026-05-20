/// OHOS-specific initialiser — libelectron.so namespace fix + dlopen override.
///
/// On OHOS the dynamic linker uses namespace isolation: dlopen("libelectron.so")
/// fails from inside our .node because the app lib dir is not on the default
/// search path. The C shim then falls back to dlsym(RTLD_DEFAULT, "napi_*"),
/// which finds libace_napi.z.so (ArkTS) instead of Electron's V8 napi → crash.
///
/// Fix:
/// 1. .init_array constructor finds libelectron.so via /proc/self/maps and
///    loads it with its full absolute path before any napi_* call happens.
/// 2. Local dlopen override intercepts the shim's dlopen("libelectron.so") and
///    returns the pre-loaded handle. -Bsymbolic-functions makes all dlopen
///    calls within our .so go directly to this override (no PLT).

use std::ffi::{CStr, CString};
use std::io::Write;
use std::os::raw::{c_char, c_int, c_void};
use std::sync::OnceLock;

extern "C" {
  fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
  fn dlerror() -> *mut c_char;
}

const RTLD_LAZY: c_int = 0x1;
const RTLD_GLOBAL: c_int = 0x100;
const RTLD_NOLOAD: c_int = 0x4;
/// RTLD_DEFAULT: search the global symbol table (NULL handle on POSIX).
const RTLD_DEFAULT: *mut c_void = std::ptr::null_mut();

static ELECTRON_HANDLE: OnceLock<usize> = OnceLock::new();
static REAL_DLOPEN: OnceLock<usize> = OnceLock::new();

// ── .init_array constructor ────────────────────────────────────────────────

#[used]
#[link_section = ".init_array"]
static _OHOS_INIT: extern "C" fn() = ohos_init;

extern "C" fn ohos_init() {
  // Write diagnostics to a file — eprintln! does not reach hilog on OHOS.
  let mut log = std::fs::OpenOptions::new()
    .create(true)
    .append(true)
    .open("/data/storage/el1/base/nscr.log")
    .ok();
  macro_rules! L {
    ($($t:tt)*) => {
      if let Some(f) = log.as_mut() { let _ = writeln!(f, $($t)*); }
    };
  }

  L!("ohos_init: start");

  unsafe {
    // ── 1. Get the real system dlopen ─────────────────────────────────────
    // Our dlopen override is marked "local" in the version script, so it is
    // NOT in .dynsym. dlsym(RTLD_DEFAULT, "dlopen") therefore finds the
    // system libc dlopen, not our override.
    let real_ptr = dlsym(RTLD_DEFAULT, b"dlopen\0".as_ptr() as _);
    L!("ohos_init: real dlopen = {:p}", real_ptr);
    if real_ptr.is_null() {
      L!("ohos_init: RTLD_DEFAULT dlopen not found — aborting");
      return;
    }
    let _ = REAL_DLOPEN.set(real_ptr as usize);
    type DlopenFn = unsafe extern "C" fn(*const c_char, c_int) -> *mut c_void;
    let real_dlopen: DlopenFn = std::mem::transmute(real_ptr);

    // ── 2. Find libelectron.so full path via /proc/self/maps ──────────────
    // More reliable than dladdr: works regardless of symlinks or OHOS vnode
    // aliasing. libelectron.so must already be mapped since Electron is running.
    let electron_path = find_in_maps("libelectron.so");
    L!("ohos_init: libelectron.so from maps = {:?}", electron_path);

    // ── 3. Open with full path (bypasses OHOS namespace name lookup) ──────
    let h = if let Some(ref path) = electron_path {
      match CString::new(path.as_str()) {
        Ok(c) => {
          let h = real_dlopen(c.as_ptr(), RTLD_LAZY | RTLD_GLOBAL);
          L!("ohos_init: dlopen({}) = {:p}", path, h);
          if h.is_null() {
            let err = dlerror();
            let msg = if err.is_null() {
              "(no dlerror)".to_owned()
            } else {
              CStr::from_ptr(err).to_str().unwrap_or("?").to_owned()
            };
            L!("ohos_init: dlopen FAILED: {}", msg);
          }
          h
        }
        Err(_) => std::ptr::null_mut(),
      }
    } else {
      std::ptr::null_mut()
    };

    // ── 4. Fallback: RTLD_NOLOAD — retrieve handle of already-loaded lib ──
    // If the full-path open failed (OHOS namespace blocks even that), try
    // asking for the already-loaded library by short name with RTLD_NOLOAD.
    // This succeeds when the library IS loaded in our namespace but cannot be
    // found by name for a fresh open.
    let h = if h.is_null() {
      let h2 = real_dlopen(
        b"libelectron.so\0".as_ptr() as _,
        RTLD_NOLOAD | RTLD_LAZY,
      );
      L!("ohos_init: RTLD_NOLOAD fallback = {:p}", h2);
      h2
    } else {
      h
    };

    if h.is_null() {
      L!("ohos_init: could not obtain libelectron.so handle — shim will use RTLD_DEFAULT");
      return;
    }

    // Verify we got Electron's napi, not libace_napi.z.so.
    let sym = dlsym(h, b"napi_create_string_utf8\0".as_ptr() as _);
    L!("ohos_init: handle={:p} napi_create_string_utf8={:p}", h, sym);

    let _ = ELECTRON_HANDLE.set(h as usize);
    L!("ohos_init: ELECTRON_HANDLE set OK");
  }
}

/// Scan /proc/self/maps for the first mapping whose pathname contains `name`.
/// Returns the full filesystem path (last whitespace-delimited field of the line).
fn find_in_maps(name: &str) -> Option<String> {
  let maps = std::fs::read_to_string("/proc/self/maps").ok()?;
  for line in maps.lines() {
    if line.contains(name) {
      let path = line.split_whitespace().last()?;
      if path.starts_with('/') {
        return Some(path.to_owned());
      }
    }
  }
  None
}

// ── dlopen override ────────────────────────────────────────────────────────
// Marked "local" in scripts/ohos_napi.map → not in .dynsym.
// -Bsymbolic-functions makes all dlopen() calls from within this .so (including
// the statically-linked shim) resolve directly here without PLT interposition.
#[no_mangle]
pub unsafe extern "C" fn dlopen(filename: *const c_char, flag: c_int) -> *mut c_void {
  if !filename.is_null() && CStr::from_ptr(filename).to_bytes() == b"libelectron.so" {
    if let Some(&h) = ELECTRON_HANDLE.get() {
      return h as *mut c_void;
    }
    // ELECTRON_HANDLE not set yet (shouldn't happen after .init_array ran).
  }

  type DlopenFn = unsafe extern "C" fn(*const c_char, c_int) -> *mut c_void;
  let raw = match REAL_DLOPEN.get().copied() {
    Some(p) => p,
    None => {
      // ohos_init hasn't run yet: get real dlopen via RTLD_DEFAULT.
      let p = dlsym(RTLD_DEFAULT, b"dlopen\0".as_ptr() as _);
      if p.is_null() {
        return std::ptr::null_mut();
      }
      let _ = REAL_DLOPEN.set(p as usize);
      p as usize
    }
  };
  let real: DlopenFn = std::mem::transmute(raw);
  real(filename, flag)
}
