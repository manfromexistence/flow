PS F:\cli\tui> cargo run
    Updating crates.io index
     Locking 1 package to latest compatible version
      Adding yazi-prebuilt v0.1.0
warning: declaration of an `unsafe` method
  --> src\file_browser\shared\src\loc\able.rs:67:2
   |
67 |     unsafe fn from_encoded_bytes_unchecked(bytes: &'p [u8]) -> Self;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: requested on the command line with `-W unsafe-code`

warning: implementation of an `unsafe` method
  --> src\file_browser\shared\src\loc\able.rs:93:2
   |
93 | /     unsafe fn from_encoded_bytes_unchecked(bytes: &'p [u8]) -> Self {
94 | |         std::path::Path::new(unsafe { OsStr::from_encoded_bytes_unchecked(bytes) })
95 | |     }
   | |_____^

warning: usage of an `unsafe` block
  --> src\file_browser\shared\src\loc\able.rs:94:24
   |
94 |         std::path::Path::new(unsafe { OsStr::from_encoded_bytes_unchecked(bytes) })
   |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
   --> src\file_browser\shared\src\loc\able.rs:125:8
    |
125 |         Some(unsafe { Self::from_encoded_bytes_unchecked(b) })
    |              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: implementation of an `unsafe` method
   --> src\file_browser\shared\src\loc\able.rs:140:2
    |
140 | /     unsafe fn from_encoded_bytes_unchecked(bytes: &'p [u8]) -> Self {
141 | |         typed_path::UnixPath::new(bytes)
142 | |     }
    | |_____^

warning: declaration of an `unsafe` method
   --> src\file_browser\shared\src\loc\able.rs:180:2
    |
180 |     unsafe fn from_encoded_bytes_unchecked(bytes: Vec<u8>) -> Self;
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: implementation of an `unsafe` method
   --> src\file_browser\shared\src\loc\able.rs:194:2
    |
194 | /     unsafe fn from_encoded_bytes_unchecked(bytes: Vec<u8>) -> Self {
195 | |         Self::from(unsafe { OsString::from_encoded_bytes_unchecked(bytes) })
196 | |     }
    | |_____^

warning: usage of an `unsafe` block
   --> src\file_browser\shared\src\loc\able.rs:195:14
    |
195 |         Self::from(unsafe { OsString::from_encoded_bytes_unchecked(bytes) })
    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: implementation of an `unsafe` method
   --> src\file_browser\shared\src\loc\able.rs:211:2
    |
211 |     unsafe fn from_encoded_bytes_unchecked(bytes: Vec<u8>) -> Self { bytes.into() }
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
  --> src\file_browser\shared\src\loc\buf.rs:92:17
   |
92 |         Self { inner: unsafe { P::from_encoded_bytes_unchecked(bytes) }, uri, urn }
   |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
  --> src\file_browser\shared\src\loc\loc.rs:93:20
   |
93 |           let prefix_len = unsafe {
   |  __________________________^
94 | |             name.as_encoded_bytes().as_ptr().offset_from_unsigned(path.as_encoded_bytes().as_ptr())
95 | |         };
   | |_________^

warning: usage of an `unsafe` block
  --> src\file_browser\shared\src\loc\loc.rs:99:14
   |
99 |             inner:    unsafe { P::from_encoded_bytes_unchecked(bytes) },
   |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
   --> src\file_browser\shared\src\loc\loc.rs:108:3
    |
108 | /         unsafe {
109 | |             P::from_encoded_bytes_unchecked(
110 | |                 self.inner.as_encoded_bytes().get_unchecked(..self.inner.len() - self.uri),
111 | |             )
112 | |         }
    | |_________^

warning: usage of an `unsafe` block
   --> src\file_browser\shared\src\loc\loc.rs:164:3
    |
164 | /         unsafe {
165 | |             P::from_encoded_bytes_unchecked(
166 | |                 self.inner.as_encoded_bytes().get_unchecked(..self.inner.len() - self.urn),
167 | |             )
168 | |         }
    | |_________^

warning: usage of an `unsafe` block
   --> src\file_browser\shared\src\loc\loc.rs:179:3
    |
179 | /         unsafe {
180 | |             (
181 | |                 P::from_encoded_bytes_unchecked(self.inner.as_encoded_bytes().get_unchecked(base)),
182 | |                 P::from_encoded_bytes_unchecked(self.inner.as_encoded_bytes().get_unchecked(rest)),
...   |
185 | |         }
    | |_________^

warning: usage of an `unsafe` block
   --> src\file_browser\shared\src\loc\loc.rs:190:3
    |
190 | /         unsafe {
191 | |             P::from_encoded_bytes_unchecked(
192 | |                 self.inner.as_encoded_bytes().get_unchecked(self.inner.len() - self.uri..),
193 | |             )
194 | |         }
    | |_________^

warning: usage of an `unsafe` block
   --> src\file_browser\shared\src\loc\loc.rs:199:3
    |
199 | /         unsafe {
200 | |             P::from_encoded_bytes_unchecked(
201 | |                 self.inner.as_encoded_bytes().get_unchecked(self.inner.len() - self.urn..),
202 | |             )
203 | |         }
    | |_________^

warning: implementation of an `unsafe` method
  --> src\file_browser\shared\src\path\buf.rs:70:2
   |
70 | /     pub unsafe fn from_encoded_bytes<K>(kind: K, bytes: Vec<u8>) -> Self
71 | |     where
72 | |         K: Into<PathKind>,
...  |
78 | |     }
   | |_____^

warning: usage of an `unsafe` block
  --> src\file_browser\shared\src\path\buf.rs:75:29
   |
75 |             PathKind::Os => Self::Os(unsafe { OsString::from_encoded_bytes_unchecked(bytes) }.into()),
   |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: implementation of an `unsafe` method
   --> src\file_browser\shared\src\path\path.rs:93:2
    |
 93 | /     pub unsafe fn from_encoded_bytes<K>(kind: K, bytes: &'p [u8]) -> Self
 94 | |     where
 95 | |         K: Into<PathKind>,
...   |
101 | |     }
    | |_____^

warning: usage of an `unsafe` block
  --> src\file_browser\shared\src\path\path.rs:98:29
   |
98 |             PathKind::Os => Self::Os(unsafe { OsStr::from_encoded_bytes_unchecked(bytes) }.as_ref()),
   |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
   --> src\file_browser\shared\src\path\path.rs:152:8
    |
152 |           Some(unsafe {
    |  ______________^
153 | |             (Self::from_encoded_bytes(self.kind(), a), Self::from_encoded_bytes(self.kind(), b))
154 | |         })
    | |_________^

warning: usage of an `unsafe` block
   --> src\file_browser\shared\src\path\path.rs:228:6
    |
228 |           Ok(unsafe {
    |  ____________^
229 | |             (Self::from_encoded_bytes(self.kind(), a), Self::from_encoded_bytes(self.kind(), b))
230 | |         })
    | |_________^

warning: implementation of an `unsafe` trait
 --> src\file_browser\shared\src\pool\ptr.rs:8:1
  |
8 | unsafe impl Send for SymbolPtr {}
  | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: implementation of an `unsafe` trait
  --> src\file_browser\shared\src\pool\ptr.rs:10:1
   |
10 | unsafe impl Sync for SymbolPtr {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
  --> src\file_browser\shared\src\pool\ptr.rs:35:40
   |
35 |     pub(super) fn bytes(&self) -> &[u8] { unsafe { self.0.as_ref() } }
   |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: implementation of an `unsafe` trait
  --> src\file_browser\shared\src\pool\symbol.rs:12:1
   |
12 | unsafe impl<T: ?Sized> Send for Symbol<T> {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: implementation of an `unsafe` trait
  --> src\file_browser\shared\src\pool\symbol.rs:14:1
   |
14 | unsafe impl<T: ?Sized> Sync for Symbol<T> {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
  --> src\file_browser\shared\src\pool\symbol.rs:37:11
   |
37 |                     drop(unsafe { Box::from_raw(self.ptr.as_ptr()) });
   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
  --> src\file_browser\shared\src\pool\symbol.rs:50:29
   |
50 |     fn as_ref(&self) -> &str { unsafe { str::from_utf8_unchecked(self.ptr.bytes()) } }
   |                                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe extern` block
  --> src\file_browser\shared\src\shell\windows.rs:82:2
   |
82 | /     unsafe extern "C" {
83 | |         fn wcslen(s: PCWSTR) -> usize;
84 | |     }
   | |_____^

warning: usage of an `unsafe` block
  --> src\file_browser\shared\src\shell\windows.rs:87:15
   |
87 |     let argv_p = unsafe { CommandLineToArgvW(s.as_ptr(), argc.as_mut_ptr()) };
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
  --> src\file_browser\shared\src\shell\windows.rs:92:13
   |
92 |     let argv = unsafe { std::slice::from_raw_parts(argv_p, argc.assume_init() as usize) };
   |                ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
  --> src\file_browser\shared\src\shell\windows.rs:95:13
   |
95 |         let len = unsafe { wcslen(arg) };
   |                   ^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
  --> src\file_browser\shared\src\shell\windows.rs:96:37
   |
96 |         res.push(String::from_utf16_lossy(unsafe { std::slice::from_raw_parts(arg, len) }));
   |                                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
  --> src\file_browser\shared\src\shell\windows.rs:99:2
   |
99 |     unsafe { LocalFree(argv_p as _) };
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: implementation of an `unsafe` method
  --> src\file_browser\shared\src\strand\buf.rs:66:2
   |
66 | /     pub unsafe fn from_encoded_bytes(kind: impl Into<StrandKind>, bytes: Vec<u8>) -> Self {
67 | |         match kind.into() {
68 | |             StrandKind::Utf8 => Self::Utf8(unsafe { String::from_utf8_unchecked(bytes) }),
69 | |             StrandKind::Os => Self::Os(unsafe { OsString::from_encoded_bytes_unchecked(bytes) }),
...  |
72 | |     }
   | |_____^

warning: usage of an `unsafe` block
  --> src\file_browser\shared\src\strand\buf.rs:68:35
   |
68 |             StrandKind::Utf8 => Self::Utf8(unsafe { String::from_utf8_unchecked(bytes) }),
   |                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
  --> src\file_browser\shared\src\strand\buf.rs:69:31
   |
69 |             StrandKind::Os => Self::Os(unsafe { OsString::from_encoded_bytes_unchecked(bytes) }),
   |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
  --> src\file_browser\shared\src\strand\buf.rs:85:25
   |
85 |                 Cow::Borrowed(_) => unsafe { String::from_utf8_unchecked(s.into_encoded_bytes()) },
   |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
  --> src\file_browser\shared\src\strand\buf.rs:90:25
   |
90 |                 Cow::Borrowed(_) => unsafe { String::from_utf8_unchecked(b) },
   |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
  --> src\file_browser\shared\src\strand\extensions.rs:29:3
   |
29 |         unsafe { StrandBuf::from_encoded_bytes(kind, buf) }
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
  --> src\file_browser\shared\src\strand\extensions.rs:59:3
   |
59 |         unsafe { StrandBuf::from_encoded_bytes(kind, buf) }
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
   --> src\file_browser\shared\src\strand\strand.rs:128:3
    |
128 |         unsafe { StrandBuf::from_encoded_bytes(self.kind(), out) }.into()
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: implementation of an `unsafe` method
   --> src\file_browser\shared\src\strand\strand.rs:165:2
    |
165 | /     pub unsafe fn from_encoded_bytes(kind: impl Into<StrandKind>, bytes: &'a [u8]) -> Self {
166 | |         match kind.into() {
167 | |             StrandKind::Utf8 => Self::Utf8(unsafe { str::from_utf8_unchecked(bytes) }),
168 | |             StrandKind::Os => Self::Os(unsafe { OsStr::from_encoded_bytes_unchecked(bytes) }),
...   |
171 | |     }
    | |_____^

warning: usage of an `unsafe` block
   --> src\file_browser\shared\src\strand\strand.rs:167:35
    |
167 |             StrandKind::Utf8 => Self::Utf8(unsafe { str::from_utf8_unchecked(bytes) }),
    |                                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
   --> src\file_browser\shared\src\strand\strand.rs:168:31
    |
168 |             StrandKind::Os => Self::Os(unsafe { OsStr::from_encoded_bytes_unchecked(bytes) }),
    |                                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
   --> src\file_browser\shared\src\translit\table.rs:757:16
    |
757 |         192..=687 => unsafe { Some(TABLE_0.get_unchecked((c as u16 - 192) as usize)) },
    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
   --> src\file_browser\shared\src\translit\table.rs:758:18
    |
758 |         7684..=7929 => unsafe { Some(TABLE_1.get_unchecked((c as u16 - 7684) as usize)) },
    |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
   --> src\file_browser\shared\src\translit\table.rs:759:20
    |
759 |         64257..=64258 => unsafe { Some(TABLE_2.get_unchecked((c as u16 - 64257) as usize)) },
    |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
  --> src\file_browser\shared\src\translit\traits.rs:13:25
   |
13 |             return Cow::Borrowed(unsafe { str::from_utf8_unchecked(self) });
   |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
  --> src\file_browser\shared\src\translit\traits.rs:23:16
   |
23 |         out.push_str(unsafe { str::from_utf8_unchecked(ascii) });
   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
  --> src\file_browser\shared\src\wtf8\wtf8.rs:39:8
   |
39 |                 Ok(unsafe { Self::from_encoded_bytes_unchecked(wtf8) })
   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
  --> src\file_browser\shared\src\wtf8\wtf8.rs:68:8
   |
68 |                 Ok(unsafe { Self::from_encoded_bytes_unchecked(wtf8) })
   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
  --> src\file_browser\shared\src\chars.rs:65:12
   |
65 |     result += unsafe { src.get_unchecked(..first_idx) };
   |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
  --> src\file_browser\shared\src\chars.rs:70:13
   |
70 |         result += unsafe { src.get_unchecked(last..idx) };
   |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
  --> src\file_browser\shared\src\chars.rs:75:22
   |
75 |     Cow::Owned(result + unsafe { src.get_unchecked(last..) })
   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
  --> src\file_browser\shared\src\debounce.rs:33:49
   |
33 |           let (mut stream, interval, mut sleep, last) = unsafe {
   |  _______________________________________________________^
34 | |             let me = self.get_unchecked_mut();
35 | |             (Pin::new(&mut me.stream), me.interval, Pin::new_unchecked(&mut me.sleep), &mut me.last)
36 | |         };
   | |_________^

warning: usage of an `unsafe` block
  --> src\file_browser\shared\src\natsort.rs:29:26
   |
29 |                 return_unless_equal!(unsafe { l.unwrap_unchecked().cmp(r.unwrap_unchecked()) })
   |                                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
  --> src\file_browser\shared\src\natsort.rs:54:13
   |
54 |                     bias = unsafe { l.unwrap_unchecked().cmp(r.unwrap_unchecked()) };
   |                            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: implementation of an `unsafe` trait
  --> src\file_browser\shared\src\ro_cell.rs:12:1
   |
12 | unsafe impl<T> Sync for RoCell<T> {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
  --> src\file_browser\shared\src\ro_cell.rs:35:3
   |
35 | /         unsafe {
36 | |             #[cfg(debug_assertions)]
37 | |             assert!(!self.initialized.get().replace(true));
38 | |             *self.inner.get() = MaybeUninit::new(value);
39 | |         }
   | |_________^

warning: usage of an `unsafe` block
  --> src\file_browser\shared\src\ro_cell.rs:52:3
   |
52 | /         unsafe {
53 | |             #[cfg(debug_assertions)]
54 | |             assert!(self.initialized.get().replace(false));
55 | |             self.inner.get().replace(MaybeUninit::uninit()).assume_init()
56 | |         }
   | |_________^

warning: usage of an `unsafe` block
  --> src\file_browser\shared\src\ro_cell.rs:68:3
   |
68 | /         unsafe {
69 | |             #[cfg(debug_assertions)]
70 | |             assert!(*self.initialized.get());
71 | |             (*self.inner.get()).assume_init_ref()
72 | |         }
   | |_________^

warning: implementation of an `unsafe` trait
  --> src\file_browser\shared\src\sync_cell.rs:11:1
   |
11 | unsafe impl<T: ?Sized + Sync> Sync for SyncCell<T> {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
  --> src\file_browser\shared\src\utf8.rs:45:3
   |
45 |         unsafe { lower_bound + new_index.unwrap_unchecked() }
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: `fb-shared` (lib) generated 66 warnings
warning: usage of an `unsafe` block
  --> src\file_browser\shim\src\ratatui\paragraph.rs:35:19
   |
35 |                     out.push_str(unsafe { span.content.get_unchecked(start..idx) });
   |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: requested on the command line with `-W unsafe-code`

warning: usage of an `unsafe` block
  --> src\file_browser\shim\src\ratatui\paragraph.rs:44:19
   |
44 |                     out.push_str(unsafe { span.content.get_unchecked(start..) });
   |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: `fb-shim` (lib) generated 2 warnings
warning: usage of an `unsafe` block
  --> src\file_browser\tty\src\handle.rs:25:4
   |
25 |             unsafe { windows_sys::Win32::Foundation::CloseHandle(self.inner) };
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: requested on the command line with `-W unsafe-code`

warning: usage of an `unsafe` block
  --> src\file_browser\tty\src\handle.rs:43:16
   |
43 |             let mut f = unsafe { std::fs::File::from_raw_handle(self.inner) };
   |                         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
  --> src\file_browser\tty\src\handle.rs:65:17
   |
65 |                 let mut f = unsafe { std::fs::File::from_raw_handle(self.inner) };
   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
   --> src\file_browser\tty\src\handle.rs:137:16
    |
137 |           let result = unsafe {
    |  ______________________^
138 | |             CreateFileW(
139 | |                 name.as_ptr(),
140 | |                 GENERIC_READ | GENERIC_WRITE,
...   |
147 | |         };
    | |_________^

warning: usage of an `unsafe` block
   --> src\file_browser\tty\src\handle.rs:153:22
    |
153 |                 out_utf8:        unsafe { GetConsoleOutputCP() } == CP_UTF8,
    |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
   --> src\file_browser\tty\src\handle.rs:166:21
    |
166 |             out_utf8:        unsafe { GetConsoleOutputCP() } == CP_UTF8,
    |                              ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
   --> src\file_browser\tty\src\handle.rs:175:9
    |
175 |         match unsafe { WaitForSingleObject(self.inner, millis as u32) } {
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
   --> src\file_browser\tty\src\handle.rs:187:17
    |
187 |         let success = unsafe { ReadFile(self.inner, &mut buf, 1, &mut bytes, std::ptr::null_mut()) };
    |                       ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
   --> src\file_browser\tty\src\windows.rs:100:22
    |
100 |       let utf16: &[u16] = unsafe {
    |  _________________________^
...   |
115 | |         &*(&utf16[..result as usize] as *const [MaybeUninit<u16>] as *const [u16])
116 | |     };
    | |_____^

warning: usage of an `unsafe` block
   --> src\file_browser\tty\src\windows.rs:155:15
    |
155 |       let result = unsafe {
    |  __________________^
156 | |         WriteConsoleW(handle, data.as_ptr(), data.len() as u32, &mut written, std::ptr::null_mut())
157 | |     };
    | |_____^

warning: `fb-tty` (lib) generated 10 warnings
warning: usage of an `unsafe` block
   --> src\file_browser\fs\src\provider\local\casefold.rs:189:13
    |
189 |           let len = unsafe {
    |  ___________________^
190 | |             GetFinalPathNameByHandleW(
191 | |                 file.as_raw_handle() as HANDLE,
192 | |                 buf.as_mut_ptr(),
...   |
196 | |         };
    | |_________^
    |
    = note: requested on the command line with `-W unsafe-code`

warning: usage of an `unsafe` block
   --> src\file_browser\fs\src\provider\local\casefold.rs:215:18
    |
215 |         let mut data = unsafe { mem::zeroed::<WIN32_FIND_DATAW>() };
    |                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
   --> src\file_browser\fs\src\provider\local\casefold.rs:216:9
    |
216 |         match unsafe { FindFirstFileW(wide.as_ptr(), &mut data) } {
    |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
   --> src\file_browser\fs\src\provider\local\casefold.rs:218:18
    |
218 |             handle => _ = unsafe { FindClose(handle) },
    |                           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
  --> src\file_browser\fs\src\provider\local\identical.rs:37:32
   |
37 |         let mut info: FILE_ID_INFO = unsafe { mem::zeroed() };
   |                                      ^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
  --> src\file_browser\fs\src\provider\local\identical.rs:38:13
   |
38 |           let ret = unsafe {
   |  ___________________^
39 | |             GetFileInformationByHandleEx(
40 | |                 file.as_raw_handle() as HANDLE,
41 | |                 FileIdInfo,
...  |
45 | |         };
   | |_________^

warning: usage of an `unsafe` block
   --> src\file_browser\fs\src\cwd.rs:101:4
    |
101 |             unsafe { std::env::set_var("PWD", path.as_ref()) }
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
   --> src\file_browser\fs\src\cwd.rs:108:5
    |
108 |                 unsafe { std::env::set_var("PWD", path.as_ref()) }
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: `fb-fs` (lib) generated 8 warnings
warning: usage of an `unsafe` block
   --> src\file_browser\emulator\src\emulator.rs:178:16
    |
178 |         let (w, h) = unsafe { (String::from_utf8_unchecked(w), String::from_utf8_unchecked(h)) };
    |                      ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: requested on the command line with `-W unsafe-code`

warning: `fb-emulator` (lib) generated 1 warning
warning: usage of an `unsafe` block
  --> src\file_browser\sftp\src\fs\file.rs:77:12
   |
77 |         let me = unsafe { self.get_unchecked_mut() };
   |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: requested on the command line with `-W unsafe-code`

warning: usage of an `unsafe` block
   --> src\file_browser\sftp\src\fs\file.rs:130:12
    |
130 |         let me = unsafe { self.get_unchecked_mut() };
    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
   --> src\file_browser\sftp\src\fs\file.rs:143:28
    |
143 |             let Ok(result) = ready!(unsafe { Pin::new_unchecked(rx) }.poll(cx)) else {
    |                                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
   --> src\file_browser\sftp\src\fs\file.rs:180:12
    |
180 |         let me = unsafe { self.get_unchecked_mut() };
    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
   --> src\file_browser\sftp\src\fs\file.rs:206:12
    |
206 |         let me = unsafe { self.get_unchecked_mut() };
    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
   --> src\file_browser\sftp\src\fs\file.rs:216:12
    |
216 |         let rx = unsafe { Pin::new_unchecked(me.flush_rx.as_mut().unwrap()) };
    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
   --> src\file_browser\sftp\src\fs\file.rs:233:12
    |
233 |         let me = unsafe { self.get_unchecked_mut() };
    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
   --> src\file_browser\sftp\src\fs\file.rs:240:12
    |
240 |         let rx = unsafe { Pin::new_unchecked(me.close_rx.as_mut().unwrap()) };
    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: `fb-sftp` (lib) generated 8 warnings
warning: `fb-shared` (lib) generated 66 warnings (66 duplicates)
warning: usage of an `unsafe` block
   --> src\file_browser\adapter\src\drivers\kgp.rs:360:6
    |
360 |                     unsafe { str::from_utf8_unchecked(first) },
    |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: requested on the command line with `-W unsafe-code`

warning: usage of an `unsafe` block
   --> src\file_browser\adapter\src\drivers\kgp.rs:365:81
    |
365 |                   write!(buf, "{START}_Gm={};{}{ESCAPE}\\{CLOSE}", it.peek().is_some() as u8, unsafe {
    |  _____________________________________________________________________________________________^
366 | |                     str::from_utf8_unchecked(chunk)
367 | |                 })?;
    | |_________________^

warning: usage of an `unsafe` block
  --> src\file_browser\adapter\src\drivers\kgp_old.rs:49:6
   |
49 |                     unsafe { str::from_utf8_unchecked(first) },
   |                     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
  --> src\file_browser\adapter\src\drivers\kgp_old.rs:54:81
   |
54 |                   write!(buf, "{START}_Gm={};{}{ESCAPE}\\{CLOSE}", it.peek().is_some() as u8, unsafe {
   |  _____________________________________________________________________________________________^
55 | |                     str::from_utf8_unchecked(chunk)
56 | |                 })?;
   | |_________________^

warning: `fb-adapter` (lib) generated 4 warnings
   Compiling fb-boot v26.2.2 (F:\cli\tui\src\file_browser\boot)
warning: usage of an `unsafe` block
  --> src\file_browser\binding\src\input.rs:33:20
   |
33 |             let mut inner = unsafe { Pin::new_unchecked(&mut me.inner) };
   |                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: requested on the command line with `-W unsafe-code`

warning: usage of an `unsafe` block
  --> src\file_browser\binding\src\utils.rs:7:32
   |
 7 |       let (_, mt): (Value, Table) = unsafe {
   |  ___________________________________^
 8 | |         lua.exec_raw(value.into_lua(lua)?, |state| {
 9 | |             mlua::ffi::lua_getmetatable(state, -1);
10 | |         })
11 | |     }?;
   | |_____^

   Compiling fb-dds v26.2.2 (F:\cli\tui\src\file_browser\dds)
warning: `fb-binding` (lib) generated 2 warnings
warning: dx-tui@26.2.2: Embedded 113 figlet fonts (221566 bytes compressed)
   Compiling fb-parser v26.2.2 (F:\cli\tui\src\file_browser\parser)
   Compiling fb-proxy v26.2.2 (F:\cli\tui\src\file_browser\proxy)
warning: usage of an `unsafe` block
  --> src\file_browser\dds\src\lib.rs:25:2
   |
25 | /     unsafe {
26 | |         if let Some(s) = std::env::var("YAZI_ID").ok().filter(|s| !s.is_empty()) {
27 | |             std::env::set_var("YAZI_PID", s);
...  |
33 | |         );
34 | |     }
   | |_____^
   |
   = note: requested on the command line with `-W unsafe-code`

   Compiling fb-plugin v26.2.2 (F:\cli\tui\src\file_browser\plugin)
   Compiling fb-watcher v26.2.2 (F:\cli\tui\src\file_browser\watcher)
warning: usage of an `unsafe` block
   --> src\file_browser\plugin\src\external\highlighter.rs:188:3
    |
188 |         unsafe { String::from_utf8_unchecked(buf) }
    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    |
    = note: requested on the command line with `-W unsafe-code`

warning: usage of an `unsafe` block
  --> src\file_browser\plugin\src\process\child.rs:24:4
   |
24 |             unsafe { windows_sys::Win32::Foundation::CloseHandle(h) };
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: usage of an `unsafe` block
  --> src\file_browser\plugin\src\process\command.rs:62:4
   |
62 | /             unsafe {
63 | |                 let job = CreateJobObjectW(std::ptr::null_mut(), std::ptr::null());
64 | |                 if job.is_null() {
65 | |                     return Err(io::Error::last_os_error());
...  |
86 | |             }
   | |_____________^

warning: `fb-dds` (lib) generated 1 warning
   Compiling fb-scheduler v26.2.2 (F:\cli\tui\src\file_browser\scheduler)
warning: `fb-plugin` (lib) generated 3 warnings
   Compiling fb-core v26.2.2 (F:\cli\tui\src\file_browser\core)
   Compiling fb-actor v26.2.2 (F:\cli\tui\src\file_browser\actor)
warning: usage of an `unsafe` block
 --> src\file_browser\actor\src\lives\ptr.rs:8:37
  |
8 |     fn deref(&self) -> &Self::Target { unsafe { &*self.0 } }
  |                                        ^^^^^^^^^^^^^^^^^^^
  |
  = note: requested on the command line with `-W unsafe-code`

warning: usage of an `unsafe` block
  --> src\file_browser\actor\src\lives\ptr.rs:33:49
   |
33 |     pub(super) fn as_static(&self) -> &'static T { unsafe { &*self.0 } }
   |                                                    ^^^^^^^^^^^^^^^^^^^

warning: `fb-actor` (lib) generated 2 warnings
   Compiling dx-tui v26.2.2 (F:\cli\tui)
    Building [=======================> ] 743/744: dx(bin)