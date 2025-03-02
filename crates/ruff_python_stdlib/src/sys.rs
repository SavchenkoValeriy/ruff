//! This file is generated by `scripts/generate_known_standard_library.py`

pub fn is_known_standard_library(minor_version: u32, module: &str) -> bool {
    matches!(
        (minor_version, module),
        (
            _,
            "_ast"
                | "_thread"
                | "abc"
                | "aifc"
                | "argparse"
                | "array"
                | "ast"
                | "asyncio"
                | "atexit"
                | "audioop"
                | "base64"
                | "bdb"
                | "binascii"
                | "bisect"
                | "builtins"
                | "bz2"
                | "cProfile"
                | "calendar"
                | "cgi"
                | "cgitb"
                | "chunk"
                | "cmath"
                | "cmd"
                | "code"
                | "codecs"
                | "codeop"
                | "collections"
                | "colorsys"
                | "compileall"
                | "concurrent"
                | "configparser"
                | "contextlib"
                | "contextvars"
                | "copy"
                | "copyreg"
                | "crypt"
                | "csv"
                | "ctypes"
                | "curses"
                | "dataclasses"
                | "datetime"
                | "dbm"
                | "decimal"
                | "difflib"
                | "dis"
                | "doctest"
                | "email"
                | "encodings"
                | "ensurepip"
                | "enum"
                | "errno"
                | "faulthandler"
                | "fcntl"
                | "filecmp"
                | "fileinput"
                | "fnmatch"
                | "fractions"
                | "ftplib"
                | "functools"
                | "gc"
                | "getopt"
                | "getpass"
                | "gettext"
                | "glob"
                | "grp"
                | "gzip"
                | "hashlib"
                | "heapq"
                | "hmac"
                | "html"
                | "http"
                | "imaplib"
                | "imghdr"
                | "importlib"
                | "inspect"
                | "io"
                | "ipaddress"
                | "itertools"
                | "json"
                | "keyword"
                | "lib2to3"
                | "linecache"
                | "locale"
                | "logging"
                | "lzma"
                | "mailbox"
                | "mailcap"
                | "marshal"
                | "math"
                | "mimetypes"
                | "mmap"
                | "modulefinder"
                | "msilib"
                | "msvcrt"
                | "multiprocessing"
                | "netrc"
                | "nis"
                | "nntplib"
                | "ntpath"
                | "numbers"
                | "operator"
                | "optparse"
                | "os"
                | "ossaudiodev"
                | "pathlib"
                | "pdb"
                | "pickle"
                | "pickletools"
                | "pipes"
                | "pkgutil"
                | "platform"
                | "plistlib"
                | "poplib"
                | "posix"
                | "posixpath"
                | "pprint"
                | "profile"
                | "pstats"
                | "pty"
                | "pwd"
                | "py_compile"
                | "pyclbr"
                | "pydoc"
                | "queue"
                | "quopri"
                | "random"
                | "re"
                | "readline"
                | "reprlib"
                | "resource"
                | "rlcompleter"
                | "runpy"
                | "sched"
                | "secrets"
                | "select"
                | "selectors"
                | "shelve"
                | "shlex"
                | "shutil"
                | "signal"
                | "site"
                | "smtplib"
                | "sndhdr"
                | "socket"
                | "socketserver"
                | "spwd"
                | "sqlite3"
                | "sre"
                | "sre_compile"
                | "sre_constants"
                | "sre_parse"
                | "ssl"
                | "stat"
                | "statistics"
                | "string"
                | "stringprep"
                | "struct"
                | "subprocess"
                | "sunau"
                | "symtable"
                | "sys"
                | "sysconfig"
                | "syslog"
                | "tabnanny"
                | "tarfile"
                | "telnetlib"
                | "tempfile"
                | "termios"
                | "test"
                | "textwrap"
                | "threading"
                | "time"
                | "timeit"
                | "tkinter"
                | "token"
                | "tokenize"
                | "trace"
                | "traceback"
                | "tracemalloc"
                | "tty"
                | "turtle"
                | "turtledemo"
                | "types"
                | "typing"
                | "unicodedata"
                | "unittest"
                | "urllib"
                | "uu"
                | "uuid"
                | "venv"
                | "warnings"
                | "wave"
                | "weakref"
                | "webbrowser"
                | "winreg"
                | "winsound"
                | "wsgiref"
                | "xdrlib"
                | "xml"
                | "xmlrpc"
                | "zipapp"
                | "zipfile"
                | "zipimport"
                | "zlib"
        ) | (
            7,
            "_dummy_thread"
                | "asynchat"
                | "asyncore"
                | "binhex"
                | "distutils"
                | "dummy_threading"
                | "formatter"
                | "imp"
                | "macpath"
                | "parser"
                | "smtpd"
                | "symbol"
        ) | (
            8,
            "_dummy_thread"
                | "asynchat"
                | "asyncore"
                | "binhex"
                | "distutils"
                | "dummy_threading"
                | "formatter"
                | "imp"
                | "parser"
                | "smtpd"
                | "symbol"
        ) | (
            9,
            "asynchat"
                | "asyncore"
                | "binhex"
                | "distutils"
                | "formatter"
                | "graphlib"
                | "imp"
                | "parser"
                | "smtpd"
                | "symbol"
                | "zoneinfo"
        ) | (
            10,
            "asynchat"
                | "asyncore"
                | "binhex"
                | "distutils"
                | "graphlib"
                | "idlelib"
                | "imp"
                | "smtpd"
                | "zoneinfo"
        ) | (
            11,
            "asynchat"
                | "asyncore"
                | "distutils"
                | "graphlib"
                | "idlelib"
                | "imp"
                | "smtpd"
                | "tomllib"
                | "zoneinfo"
        ) | (12, "graphlib" | "idlelib" | "tomllib" | "zoneinfo")
    )
}
