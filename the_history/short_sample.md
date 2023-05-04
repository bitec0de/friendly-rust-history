Version 1.69.0 (2023-04-20)
==========================

<a id="1.69.0-Language"></a>

Language
--------

- [Deriving built-in traits on packed structs works with `Copy` fields.](https://github.com/rust-lang/rust/pull/104429/)
- [Stabilize the `cmpxchg16b` target feature on x86 and x86_64.](https://github.com/rust-lang/rust/pull/106774/)
- [Improve analysis of trait bounds for associated types.](https://github.com/rust-lang/rust/pull/103695/)
- [Allow associated types to be used as union fields.](https://github.com/rust-lang/rust/pull/106938/)
- [Allow `Self: Autotrait` bounds on dyn-safe trait methods.](https://github.com/rust-lang/rust/pull/107082/)
- [Treat `str` as containing `[u8]` for auto trait purposes.](https://github.com/rust-lang/rust/pull/107941/)

<a id="1.69.0-Compiler"></a>

Compiler
--------

- [Upgrade `*-pc-windows-gnu` on CI to mingw-w64 v10 and GCC 12.2.](https://github.com/rust-lang/rust/pull/100178/)
- [Rework min_choice algorithm of member constraints.](https://github.com/rust-lang/rust/pull/105300/)
- [Support `true` and `false` as boolean flags in compiler arguments.](https://github.com/rust-lang/rust/pull/107043/)
- [Default `repr(C)` enums to `c_int` size.](https://github.com/rust-lang/rust/pull/107592/)

<a id="1.69.0-Libraries"></a>

Libraries
---------

- [Implement the unstable `DispatchFromDyn` for cell types, allowing downstream experimentation with custom method receivers.](https://github.com/rust-lang/rust/pull/97373/)
- [Document that `fmt::Arguments::as_str()` may return `Some(_)` in more cases after optimization, subject to change.](https://github.com/rust-lang/rust/pull/106823/)
- [Implement `AsFd` and `AsRawFd` for `Rc`.](https://github.com/rust-lang/rust/pull/107317/)

<a id="1.69.0-Stabilized-APIs"></a>

Stabilized APIs
---------------

- [`CStr::from_bytes_until_nul`](https://doc.rust-lang.org/stable/core/ffi/struct.CStr.html#method.from_bytes_until_nul)
- [`core::ffi::FromBytesUntilNulError`](https://doc.rust-lang.org/stable/core/ffi/struct.FromBytesUntilNulError.html)

These APIs are now stable in const contexts:

- [`SocketAddr::new`](https://doc.rust-lang.org/stable/std/net/enum.SocketAddr.html#method.new)
- [`SocketAddr::ip`](https://doc.rust-lang.org/stable/std/net/enum.SocketAddr.html#method.ip)
- [`SocketAddr::port`](https://doc.rust-lang.org/stable/std/net/enum.SocketAddr.html#method.port)
- [`SocketAddr::is_ipv4`](https://doc.rust-lang.org/stable/std/net/enum.SocketAddr.html#method.is_ipv4)
- [`SocketAddr::is_ipv6`](https://doc.rust-lang.org/stable/std/net/enum.SocketAddr.html#method.is_ipv6)
- [`SocketAddrV4::new`](https://doc.rust-lang.org/stable/std/net/struct.SocketAddrV4.html#method.new)
- [`SocketAddrV4::ip`](https://doc.rust-lang.org/stable/std/net/struct.SocketAddrV4.html#method.ip)
- [`SocketAddrV4::port`](https://doc.rust-lang.org/stable/std/net/struct.SocketAddrV4.html#method.port)
- [`SocketAddrV6::new`](https://doc.rust-lang.org/stable/std/net/struct.SocketAddrV6.html#method.new)
- [`SocketAddrV6::ip`](https://doc.rust-lang.org/stable/std/net/struct.SocketAddrV6.html#method.ip)
- [`SocketAddrV6::port`](https://doc.rust-lang.org/stable/std/net/struct.SocketAddrV6.html#method.port)
- [`SocketAddrV6::flowinfo`](https://doc.rust-lang.org/stable/std/net/struct.SocketAddrV6.html#method.flowinfo)
- [`SocketAddrV6::scope_id`](https://doc.rust-lang.org/stable/std/net/struct.SocketAddrV6.html#method.scope_id)

<a id="1.69.0-Cargo"></a>

Cargo
-----

- [Cargo now suggests `cargo fix` or `cargo clippy --fix` when compilation warnings are auto-fixable.](https://github.com/rust-lang/cargo/pull/11558/)
- [Cargo now suggests `cargo add` if you try to install a library crate.](https://github.com/rust-lang/cargo/pull/11410/)
- [Cargo now sets the `CARGO_BIN_NAME` environment variable also for binary examples.](https://github.com/rust-lang/cargo/pull/11705/)

<a id="1.69.0-Rustdoc"></a>

Rustdoc
-----

- [Vertically compact trait bound formatting.](https://github.com/rust-lang/rust/pull/102842/)
- [Only include stable lints in `rustdoc::all` group.](https://github.com/rust-lang/rust/pull/106316/)
- [Compute maximum Levenshtein distance based on the query.](https://github.com/rust-lang/rust/pull/107141/)
- [Remove inconsistently-present sidebar tooltips.](https://github.com/rust-lang/rust/pull/107490/)
- [Search by macro when query ends with `!`.](https://github.com/rust-lang/rust/pull/108143/)

<a id="1.69.0-Compatibility-Notes"></a>

Compatibility Notes
-------------------

- [The `rust-analysis` component from `rustup` now only contains a warning placeholder.](https://github.com/rust-lang/rust/pull/101841/) This was primarily intended for RLS, and the corresponding `-Zsave-analysis` flag has been removed from the compiler as well.
- [Unaligned references to packed fields are now a hard error.](https://github.com/rust-lang/rust/pull/102513/) This has been a warning since 1.53, and denied by default with a future-compatibility warning since 1.62.
- [Update the minimum external LLVM to 14.](https://github.com/rust-lang/rust/pull/107573/)
- [Cargo now emits errors on invalid characters in a registry token.](https://github.com/rust-lang/cargo/pull/11600/)
- [When `default-features` is set to false of a workspace dependency, and an inherited dependency of a member has `default-features = true`, Cargo will enable default features of that dependency.](https://github.com/rust-lang/cargo/pull/11409/)
- [Cargo denies `CARGO_HOME` in the `[env]` configuration table. Cargo itself doesn't pick up this value, but recursive calls to cargo would, which was not intended.](https://github.com/rust-lang/cargo/pull/11644/)
- [Debuginfo for build dependencies is now off if not explicitly set. This is expected to improve the overall build time.](https://github.com/rust-lang/cargo/pull/11252/)
- [The Rust distribution no longer always includes rustdoc](https://github.com/rust-lang/rust/pull/106886)
  If `tools = [...]` is set in config.toml, we will respect a missing rustdoc in that list. By
  default rustdoc remains included. To retain the prior behavior explicitly add `"rustdoc"` to the
  list.
  
<a id="1.69.0-Internal-Changes"></a>

Internal Changes
----------------

These changes do not affect any public interfaces of Rust, but they represent
significant improvements to the performance or internals of rustc and related
tools.

- [Move `format_args!()` into AST (and expand it during AST lowering)](https://github.com/rust-lang/rust/pull/106745/)

Version 1.13.0 (2016-11-10)
===========================

Language
--------

* [Stabilize the `?` operator][36995]. `?` is a simple way to propagate
  errors, like the `try!` macro, described in [RFC 0243].
* [Stabilize macros in type position][36014]. Described in [RFC 873].
* [Stabilize attributes on statements][36995]. Described in [RFC 0016].
* [Fix `#[derive]` for empty tuple structs/variants][35728]
* [Fix lifetime rules for 'if' conditions][36029]
* [Avoid loading and parsing unconfigured non-inline modules][36482]

Compiler
--------

* [Add the `-C link-arg` argument][36574]
* [Remove the old AST-based backend from rustc_trans][35764]
* [Don't enable NEON by default on armv7 Linux][35814]
* [Fix debug line number info for macro expansions][35238]
* [Do not emit "class method" debuginfo for types that are not
  DICompositeType][36008]
* [Warn about multiple conflicting #[repr] hints][34623]
* [When sizing DST, don't double-count nested struct prefixes][36351]
* [Default RUST_MIN_STACK to 16MiB for now][36505]
* [Improve rlib metadata format][36551]. Reduces rlib size significantly.
* [Reject macros with empty repetitions to avoid infinite loop][36721]
* [Expand macros without recursing to avoid stack overflows][36214]

Diagnostics
-----------

* [Replace macro backtraces with labeled local uses][35702]
* [Improve error message for misplaced doc comments][33922]
* [Buffer unix and lock windows to prevent message interleaving][35975]
* [Update lifetime errors to specifically note temporaries][36171]
* [Special case a few colors for Windows][36178]
* [Suggest `use self` when such an import resolves][36289]
* [Be more specific when type parameter shadows primitive type][36338]
* Many minor improvements

Compile-time Optimizations
--------------------------

* [Compute and cache HIR hashes at beginning][35854]
* [Don't hash types in loan paths][36004]
* [Cache projections in trans][35761]
* [Optimize the parser's last token handling][36527]
* [Only instantiate #[inline] functions in codegen units referencing
  them][36524]. This leads to big improvements in cases where crates export
  define many inline functions without using them directly.
* [Lazily allocate TypedArena's first chunk][36592]
* [Don't allocate during default HashSet creation][36734]

Stabilized APIs
---------------

* [`checked_abs`]
* [`wrapping_abs`]
* [`overflowing_abs`]
* [`RefCell::try_borrow`]
* [`RefCell::try_borrow_mut`]

Libraries
---------

* [Add `assert_ne!` and `debug_assert_ne!`][35074]
* [Make `vec_deque::Drain`, `hash_map::Drain`, and `hash_set::Drain`
  covariant][35354]
* [Implement `AsRef<[T]>` for `std::slice::Iter`][35559]
* [Implement `Debug` for `std::vec::IntoIter`][35707]
* [`CString`: avoid excessive growth just to 0-terminate][35871]
* [Implement `CoerceUnsized` for `{Cell, RefCell, UnsafeCell}`][35627]
* [Use arc4rand on FreeBSD][35884]
* [memrchr: Correct aligned offset computation][35969]
* [Improve Demangling of Rust Symbols][36059]
* [Use monotonic time in condition variables][35048]
* [Implement `Debug` for `std::path::{Components,Iter}`][36101]
* [Implement conversion traits for `char`][35755]
* [Fix illegal instruction caused by overflow in channel cloning][36104]
* [Zero first byte of CString on drop][36264]
* [Inherit overflow checks for sum and product][36372]
* [Add missing Eq implementations][36423]
* [Implement `Debug` for `DirEntry`][36631]
* [When `getaddrinfo` returns `EAI_SYSTEM` retrieve actual error from
  `errno`][36754]
* [`SipHasher`] is deprecated. Use [`DefaultHasher`].
* [Implement more traits for `std::io::ErrorKind`][35911]
* [Optimize BinaryHeap bounds checking][36072]
* [Work around pointer aliasing issue in `Vec::extend_from_slice`,
  `extend_with_element`][36355]
* [Fix overflow checking in unsigned pow()][34942]

Cargo
-----

* This release includes security fixes to both curl and OpenSSL.
* [Fix transitive doctests when panic=abort][cargo/3021]
* [Add --all-features flag to cargo][cargo/3038]
* [Reject path-based dependencies in `cargo package`][cargo/3060]
* [Don't parse the home directory more than once][cargo/3078]
* [Don't try to generate Cargo.lock on empty workspaces][cargo/3092]
* [Update OpenSSL to 1.0.2j][cargo/3121]
* [Add license and license_file to cargo metadata output][cargo/3110]
* [Make crates-io registry URL optional in config; ignore all changes to
  source.crates-io][cargo/3089]
* [Don't download dependencies from other platforms][cargo/3123]
* [Build transitive dev-dependencies when needed][cargo/3125]
* [Add support for per-target rustflags in .cargo/config][cargo/3157]
* [Avoid updating registry when adding existing deps][cargo/3144]
* [Warn about path overrides that won't work][cargo/3136]
* [Use workspaces during `cargo install`][cargo/3146]
* [Leak mspdbsrv.exe processes on Windows][cargo/3162]
* [Add --message-format flag][cargo/3000]
* [Pass target environment for rustdoc][cargo/3205]
* [Use `CommandExt::exec` for `cargo run` on Unix][cargo/2818]
* [Update curl and curl-sys][cargo/3241]
* [Call rustdoc test with the correct cfg flags of a package][cargo/3242]

Tooling
-------

* [rustdoc: Add the `--sysroot` argument][36586]
* [rustdoc: Fix a couple of issues with the search results][35655]
* [rustdoc: remove the `!` from macro URLs and titles][35234]
* [gdb: Fix pretty-printing special-cased Rust types][35585]
* [rustdoc: Filter more incorrect methods inherited through Deref][36266]

Misc
----

* [Remove unmaintained style guide][35124]
* [Add s390x support][36369]
* [Initial work at Haiku OS support][36727]
* [Add mips-uclibc targets][35734]
* [Crate-ify compiler-rt into compiler-builtins][35021]
* [Add rustc version info (git hash + date) to dist tarball][36213]
* Many documentation improvements

Compatibility Notes
-------------------

* [`SipHasher`] is deprecated. Use [`DefaultHasher`].
* [Deny (by default) transmuting from fn item types to pointer-sized
  types][34923]. Continuing the long transition to zero-sized fn items,
  per [RFC 401].
* [Fix `#[derive]` for empty tuple structs/variants][35728].
  Part of [RFC 1506].
* [Issue deprecation warnings for safe accesses to extern statics][36173]
* [Fix lifetime rules for 'if' conditions][36029].
* [Inherit overflow checks for sum and product][36372].
* [Forbid user-defined macros named "macro_rules"][36730].

[33922]: https://github.com/rust-lang/rust/pull/33922
[34623]: https://github.com/rust-lang/rust/pull/34623
[34923]: https://github.com/rust-lang/rust/pull/34923
[34942]: https://github.com/rust-lang/rust/pull/34942
[35021]: https://github.com/rust-lang/rust/pull/35021
[35048]: https://github.com/rust-lang/rust/pull/35048
[35074]: https://github.com/rust-lang/rust/pull/35074
[35124]: https://github.com/rust-lang/rust/pull/35124
[35234]: https://github.com/rust-lang/rust/pull/35234
[35238]: https://github.com/rust-lang/rust/pull/35238
[35354]: https://github.com/rust-lang/rust/pull/35354
[35559]: https://github.com/rust-lang/rust/pull/35559
[35585]: https://github.com/rust-lang/rust/pull/35585
[35627]: https://github.com/rust-lang/rust/pull/35627
[35655]: https://github.com/rust-lang/rust/pull/35655
[35702]: https://github.com/rust-lang/rust/pull/35702
[35707]: https://github.com/rust-lang/rust/pull/35707
[35728]: https://github.com/rust-lang/rust/pull/35728
[35734]: https://github.com/rust-lang/rust/pull/35734
[35755]: https://github.com/rust-lang/rust/pull/35755
[35761]: https://github.com/rust-lang/rust/pull/35761
[35764]: https://github.com/rust-lang/rust/pull/35764
[35814]: https://github.com/rust-lang/rust/pull/35814
[35854]: https://github.com/rust-lang/rust/pull/35854
[35871]: https://github.com/rust-lang/rust/pull/35871
[35884]: https://github.com/rust-lang/rust/pull/35884
[35911]: https://github.com/rust-lang/rust/pull/35911
[35969]: https://github.com/rust-lang/rust/pull/35969
[35975]: https://github.com/rust-lang/rust/pull/35975
[36004]: https://github.com/rust-lang/rust/pull/36004
[36008]: https://github.com/rust-lang/rust/pull/36008
[36014]: https://github.com/rust-lang/rust/pull/36014
[36029]: https://github.com/rust-lang/rust/pull/36029
[36059]: https://github.com/rust-lang/rust/pull/36059
[36072]: https://github.com/rust-lang/rust/pull/36072
[36101]: https://github.com/rust-lang/rust/pull/36101
[36104]: https://github.com/rust-lang/rust/pull/36104
[36171]: https://github.com/rust-lang/rust/pull/36171
[36173]: https://github.com/rust-lang/rust/pull/36173
[36178]: https://github.com/rust-lang/rust/pull/36178
[36213]: https://github.com/rust-lang/rust/pull/36213
[36214]: https://github.com/rust-lang/rust/pull/36214
[36264]: https://github.com/rust-lang/rust/pull/36264
[36266]: https://github.com/rust-lang/rust/pull/36266
[36289]: https://github.com/rust-lang/rust/pull/36289
[36338]: https://github.com/rust-lang/rust/pull/36338
[36351]: https://github.com/rust-lang/rust/pull/36351
[36355]: https://github.com/rust-lang/rust/pull/36355
[36369]: https://github.com/rust-lang/rust/pull/36369
[36372]: https://github.com/rust-lang/rust/pull/36372
[36423]: https://github.com/rust-lang/rust/pull/36423
[36482]: https://github.com/rust-lang/rust/pull/36482
[36505]: https://github.com/rust-lang/rust/pull/36505
[36524]: https://github.com/rust-lang/rust/pull/36524
[36527]: https://github.com/rust-lang/rust/pull/36527
[36551]: https://github.com/rust-lang/rust/pull/36551
[36574]: https://github.com/rust-lang/rust/pull/36574
[36586]: https://github.com/rust-lang/rust/pull/36586
[36592]: https://github.com/rust-lang/rust/pull/36592
[36631]: https://github.com/rust-lang/rust/pull/36631
[36721]: https://github.com/rust-lang/rust/pull/36721
[36727]: https://github.com/rust-lang/rust/pull/36727
[36730]: https://github.com/rust-lang/rust/pull/36730
[36734]: https://github.com/rust-lang/rust/pull/36734
[36754]: https://github.com/rust-lang/rust/pull/36754
[36995]: https://github.com/rust-lang/rust/pull/36995
[RFC 0016]: https://github.com/rust-lang/rfcs/blob/master/text/0016-more-attributes.md
[RFC 0243]: https://github.com/rust-lang/rfcs/blob/master/text/0243-trait-based-exception-handling.md
[RFC 1506]: https://github.com/rust-lang/rfcs/blob/master/text/1506-adt-kinds.md
[RFC 401]: https://github.com/rust-lang/rfcs/blob/master/text/0401-coercions.md
[RFC 873]: https://github.com/rust-lang/rfcs/blob/master/text/0873-type-macros.md
[cargo/2818]: https://github.com/rust-lang/cargo/pull/2818
[cargo/3000]: https://github.com/rust-lang/cargo/pull/3000
[cargo/3021]: https://github.com/rust-lang/cargo/pull/3021
[cargo/3038]: https://github.com/rust-lang/cargo/pull/3038
[cargo/3060]: https://github.com/rust-lang/cargo/pull/3060
[cargo/3078]: https://github.com/rust-lang/cargo/pull/3078
[cargo/3089]: https://github.com/rust-lang/cargo/pull/3089
[cargo/3092]: https://github.com/rust-lang/cargo/pull/3092
[cargo/3110]: https://github.com/rust-lang/cargo/pull/3110
[cargo/3121]: https://github.com/rust-lang/cargo/pull/3121
[cargo/3123]: https://github.com/rust-lang/cargo/pull/3123
[cargo/3125]: https://github.com/rust-lang/cargo/pull/3125
[cargo/3136]: https://github.com/rust-lang/cargo/pull/3136
[cargo/3144]: https://github.com/rust-lang/cargo/pull/3144
[cargo/3146]: https://github.com/rust-lang/cargo/pull/3146
[cargo/3157]: https://github.com/rust-lang/cargo/pull/3157
[cargo/3162]: https://github.com/rust-lang/cargo/pull/3162
[cargo/3205]: https://github.com/rust-lang/cargo/pull/3205
[cargo/3241]: https://github.com/rust-lang/cargo/pull/3241
[cargo/3242]: https://github.com/rust-lang/cargo/pull/3242
[`checked_abs`]: https://doc.rust-lang.org/std/primitive.i32.html#method.checked_abs
[`wrapping_abs`]: https://doc.rust-lang.org/std/primitive.i32.html#method.wrapping_abs
[`overflowing_abs`]: https://doc.rust-lang.org/std/primitive.i32.html#method.overflowing_abs
[`RefCell::try_borrow`]: https://doc.rust-lang.org/std/cell/struct.RefCell.html#method.try_borrow
[`RefCell::try_borrow_mut`]: https://doc.rust-lang.org/std/cell/struct.RefCell.html#method.try_borrow_mut
[`SipHasher`]: https://doc.rust-lang.org/std/hash/struct.SipHasher.html
[`DefaultHasher`]: https://doc.rust-lang.org/std/collections/hash_map/struct.DefaultHasher.html
