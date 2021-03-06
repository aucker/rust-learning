# 2022 Daily schedule of OS Tranining Camp

## Timeline

*July*

| Mon               | Tues              | Wed                          | Thur                         | Fri                          | Sat               | Sun               |
| ----------------- | ----------------- | ---------------------------- | ---------------------------- | ---------------------------- | ----------------- | ----------------- |
|                   |                   |                   |                   | 1 <br> ([D1](#day-1-202271)) | 2 <br> ([D2](#day-2-202272)) | 3 <br> ([D3](#day-3-202273)) | 
|4 <br> ([D4](#day-4-202274)) | 5 <br> ([D5](#day-5-202275)) | 6 <br> ([D6](#day-6-202276)) | 7 <br> ([D7](#day-7-202277)) | 8 <br> ([D8](#day-8-202278))       | 9 <br> ([D9](#day-9-202279))            | 10 <br> ([D10](#day-10-2022710))         | 
|11  <br>  ([D11](#day-11-2022711))             | 12      <br>    ([D12](#day-12-2022712))       | 13    <br>    ([D13](#day-13-2022713))             | 14         <br>    ([D14](#day-14-2020711))        | 15        <br>    ([D15](#day-15-2022715))                    | 16    <br>     ([D16](#day-16-2022716))                       | 17    <br>      ([D17](#day-17-2022717))                       |
|18    <br>    ([D18](#day-18-2020718))            | 19   <br>     ([D19](#day-19-2022719))            | 20   <br>    ([D20](#day-20-2022720))            | 21       <br>    ([D21](#day-21-2022721))         | 22     <br>    ([D22](#day-22-2022722))                         | 23     <br>    ([D23](#day-23-2022723))                         | 24    <br>    ([D24](#day-24-2022724))                        | 
|25      <br>    ([D25](#day-25-2022725))             | 26         <br>    ([D26](#day-26-2022726))           | 27         <br>    ([D27](#day-27-2022727))           | 28       <br>    ([D28](#day-28-2022728))           | 29         <br>    ([D29](#day-29-2022729))                    | 30        <br>    ([D30](#day-30-2022730))                     | 31     <br>    ([D31](#day-31-2022731))                           |

------

## Day 1 2022/7/1

### OS Tranining Camp

???Telegram???????????????????????????????????????OS????????????????????????????????????????????????OS????????????????????????????????????????????????????????????????????????????????????????????????????????????Rust????????????????????????????????????OS???????????????????????????Rust??????????????????steve klabnik?????????TRPL???????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????TRPL?????????Rust?????????????????????????????????????????????????????????????????????????????????????????????

????????????????????????????????????OS????????????Rust????????????

?????????GitHub classroom??????????????????????????????codespace????????????rust???riscv??????????????????????????????????????????lab0-0????????????

codespace???
* Pros:
    * no need to configure the proxy
    * network speed is fast
* Cons:
    * some bugs
    * rust-analyzer doesn't work well

### Plans: 

??????schedule??????Rust?????????
????????????riscv????????????

## Day 2 2022/7/2

### OS Training Camp

??????codespace????????????????????????proxy???????????????????????????????????????rust-analyzer??????bug?????????????????????????????????????????????????????????????????????????????????Ubuntu??????????????????????????????codespace?????????????????????????????????????????????????????????????????????

???????????????????????????????????????????????????rusttoolchain???riscv?????????????????????????????????????????????????????????

* rust-toolchain
  * ??????ustc???
    
    ```shell
    <!-- export RUSTUP_DIST_SERVER=https://mirrors.tuna.edu.cn/rustup
    export RUSTUP_UPDATE_ROOT=https://mirrors.tuna.edu.cn/rustup/rustup -->
    export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
    export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
    ```
  * ??????rust
    ```shell
    curl https://sh.rustup.rs -sSf | sh
    ```
  * ??????cargo??????`~/.cargo/config`????????????
    ```shell
    <!-- [source.crates-io]
    replace-with = 'tuna'

    [source.tuna]
    registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git" -->
    [source.crates-io]
    registry = "https://github.com/rust-lang/crates.io-index"
    replace-with = 'ustc'
    [source.ustc]
    registry = "git://mirrors.ustc.edu.cn/crates.io-index"
    ```
* riscv
  * ?????????????????????
    ```shell
    wget https://download.qemu.org/qemu-7.0.0.tar.xz
    ```
    *??????????????????* `tar xvJf qemu*`
  * ????????????????????????
    ```shell
    sudo apt install autoconf automake autotools-dev curl libmpc-dev libmpfr-dev libgmp-dev \
              gawk build-essential bison flex texinfo gperf libtool patchutils bc \
              zlib1g-dev libexpat-dev pkg-config  libglib2.0-dev libpixman-1-dev git tmux python3 ninja-build
    ```
  * ??????
    ```shell
    cd qemu-7.0.0
    ./configure --target-list=riscv64-softmmu,riscv64-linux-user
    make -j$(nproc)
    ```
  * ??????
    ```shell
    sudo make install
    ```
    ?????????qemu?????????`/usr/local/share`?????????,?????????`~/.zshrc`???????????????
    ```shell
    export PATH=$PATH:/home/aucker/Downloads/build/qemu-7.0.0/riscv64-softmmu
    export PATH=$PATH:/home/aucker/Downloads/build/qemu-7.0.0/riscv64-linux-user
    export PATH=$PATH:/home/aucker/Downloads/build/qemu-7.0.0
    ```
    ????????????riscv?????????
    ```shell
    ???  qemu-7.0.0 qemu-system-riscv64 --version
    QEMU emulator version 7.0.0
    Copyright (c) 2003-2022 Fabrice Bellard and the QEMU Project developers
    ???  qemu-7.0.0 qemu-riscv64 --version
    qemu-riscv64 version 7.0.0
    Copyright (c) 2003-2022 Fabrice Bellard and the QEMU Project developers
    ```

* rCore-Tutorial???

    ????????????????????????git????????????????????????????????????????????????????codespace??????:cry:??????WSL2???????????????????????????:yum:

```shell
???  qemu-7.0.0 realpath -s riscv64-mmu
/home/aucker/Downloads/build/qemu-7.0.0/riscv64-mmu
```
`realpath` can get the full path of the file.

## Day 3 2022/7/3

### OS Training Camp

**??????** ?????????????????????GitHub codespace??????????????????

???????????????????????????????????????????????????
![smart pointer in Rust](./../_images/rust_smart_pointer.jpg)

???????????????????????????????????????????????????

**????????????????????????**

?????????????????????(???Linux)??????????????????????????????????????????????????????????????????
![app software stack](../_images/app-software-stack.png)

????????????????????????????????????????????????????????????????????????????????????**??????Platform**????????????**???????????????Target Triplet**????????????????????????CPU??????????????????????????????????????????????????????

```shell
??? rustc --version --verbose
rustc 1.61.0 (fe5b13d68 2022-05-18)
binary: rustc
commit-hash: fe5b13d681f25ee6474be29d748c65adcd91f69e
commit-date: 2022-05-18
host: x86_64-unknown-linux-gnu
release: 1.61.0
LLVM version: 14.0.0
```
`host`???????????????????????????`x86_64-unknown-linux-gnu`,CPU?????????x86_64?????????unknown??????????????????Linux??????????????????gnu libc???

?????????????????????????????????????????????RISCV????????????`riscv64gc-unknown-none-elf`????????????

?????????`elf`?????????????????????????????????????????????????????????????????????????????????????????????ELF????????????????????????????????????`linux-gnu`?????????`riscv64gc-unknown-linux-gnu`???????????????????????????????????????????????????????????????Linux????????????????????????????????????

**??????????????????**

??????????????????????????????`riscv64gc-unknown-none-elf`??????????????????
```shell
???  os git:(main) ??? cargo run --target riscv64gc-unknown-none-elf
   Compiling os v0.1.0 (/home/aucker/rust-learning/OS_lab/my_os_toy/os)
error[E0463]: can't find crate for `std`
  |
  = note: the `riscv64gc-unknown-none-elf` target may not be installed
  = help: consider downloading the target with `rustup target add riscv64gc-unknown-none-elf`
```

?????????????????????????????????????????????Rust?????????std????????????????????????OS????????????????????????????????????????????????**???????????? bare-metal**???Luckily???Rust?????????????????????????????????????????????????????????**core**???????????????Rust???????????????????????????????????????????????????????????????????????????std???????????????????????????core???

???????????????

* **Remove the dependency of the std library**

  ?????????`os`???????????????`.cargo`????????????????????????????????????`config`????????????????????????

  ```toml
  [build]
  target = "riscv64gc-unknown-none-elf"
  ```

  ?????????cargo?????????os????????????????????????riscv64gc-unknown-none-elf???????????????????????????????????????????????????(x86_64)???????????????????????????????????????????????????????????????**???????????? Cross Compile**???

  * Remove `println!()`
  ?????????`main.rs`????????????`#![no_std]`??????????????????????????????????????????std,?????????????????????`core`???
  * Provide `panic_handler()` 
  ?????????std?????????Rust??????????????????#[panic_handler]???????????????????????????????????????????????????,???????????????????????????????????????????????????core???????????????????????????????????????????????????????????????????????????
    ```rust
    // os/src/lang_items.rs
    use core::panic::PanicInfo;

    #[panic_handler]
    fn panic_handler(_info: &PanicInfo) -> ! {
      loop {}
    }
    ```
  * Remove `main()`
  Then we build this program again, we got the following errors:
    > **Error**
    >
    > ```shell
    > ???  os git:(main) ??? cargo build
    > Compiling os v0.1.0 (/home/aucker/rust-learning/OS_lab/my_os_toy/os)
    > error: requires `start` lang_item
    > 
    > error: could not compile `os` due to previous error
    > ```
    we need `start`. `start` represents what `std` library needs to do some initialization work before executing a program.

    We add `#![no_main]` to inform the compiler that we don't have a `main` function. And compiler don't need to consider the initialization work. 

    Now, we finally remove all the `std` library dependencies, the codes look like this:
    ```rust
    // os/src/main.rs
    #![no_std]
    #![no_main]

    mod lang_items;

    // os/src/lang_items.rs
    use core::panic::PanicInfo;

    #[panic_handler]
    fn panic_handler(_info: &PanicInfo) -> ! {
      loop {}
    }
    ```

  * Analyze the program that was removed the `std` library
  
    We need `rust-readobj`, use `cargo install rust-readobj` to install it.
  
    We can use some tools to analyze the program:
  
    ```shell
    ???  os git:(main) ??? file target/riscv64gc-unknown-none-elf/debug/os           
    target/riscv64gc-unknown-none-elf/debug/os: ELF 64-bit LSB executable, UCB RISC-V, version 1 (SYSV), statically linked, with debug_info, not stripped
    ???  os git:(main) ??? rust-readobj -h target/riscv64gc-unknown-none-elf/debug/os

    File: target/riscv64gc-unknown-none-elf/debug/os
    Format: elf64-littleriscv
    Arch: riscv64
    AddressSize: 64bit
    LoadName: <Not found>
    ElfHeader {
      Ident {
        Magic: (7F 45 4C 46)
        Class: 64-bit (0x2)
        DataEncoding: LittleEndian (0x1)
        FileVersion: 1
        OS/ABI: SystemV (0x0)
        ABIVersion: 0
        Unused: (00 00 00 00 00 00 00)
      }
    Type: Executable (0x2)
    Machine: EM_RISCV (0xF3)
    Version: 1
    Entry: 0x0
    ProgramHeaderOffset: 0x40
    SectionHeaderOffset: 0x1B40
    Flags [ (0x5)
      EF_RISCV_FLOAT_ABI_DOUBLE (0x4)
      EF_RISCV_RVC (0x1)
    ]
    HeaderSize: 64
    ProgramHeaderEntrySize: 56
    ProgramHeaderCount: 3
    SectionHeaderEntrySize: 64
    SectionHeaderCount: 14
    StringTableSectionIndex: 12
    }
    ???  os git:(main) ??? rust-objdump -S target/riscv64gc-unknown-none-elf/debug/os

    target/riscv64gc-unknown-none-elf/debug/os:     file format elf64-littleriscv
    ```

    ??????`file`??????????????????`os`???????????????????????????????????????????????????RV64??????????????????`rust-readobj`????????????????????????Entry???`0`.?????????`rust-objdump`?????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????????`_start`???

* **???????????????????????????**

> **Note**
> QEMU has two running mode:
> `User mode`, such as `qemu-riscv64`, it can simulate different CPUs' User mode instructions execution, and directly parse ELF executable file, load and execute user mode Linux applications.
> `System mode`, such as `qemu-system-riscv64`, it can simulate a complete hardware system based on different CPUs, including CPU, memory, peripherals, and so on. 



### Day 4 2022/7/4

?????????????????????rustling

**Plans???** ??????rustling???????????????

### Day 5 2022/7/5

????????????????????????rustlings???????????????

* Shadowing in Rust
https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing

* CONSTANT
const must come with a TYPE annotation.
https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#constants


### Day 6 2022/7/6

[Ownership in Rust: References and Borrowing](https://doc.rust-lang.org/stable/book/ch04-02-references-and-borrowing.html)

```rust
// move_semantics.rs

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
```

In Rust, `String` doesn't implement `Copy` trait, so it can't be copied.

Get a slice out of an array:
```rust
// primitive_types4.rs
#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice = &a[1 .. 4];

    assert_eq!([2, 3, 4], nice_slice)
}
```

The [Tuple Type](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type) is awesome???

```rust
// primitive_types5.rs
fn main() {
  let cat = ("Furry McFurson", 3.5);
  let (name, age)/* your pattern here */ = cat;

  println!("{} is {} years old.", name, age);
}
```

The different [Structs](https://doc.rust-lang.org/book/ch05-01-defining-structs.html) in Rust.

```rust
// structs.rs

struct ColorClassicStruct {
    name: String,
    hex: String,
}
struct ColorTupleStruct<'a, 'b>(&'a str, &'b str);
// well, well, well, here comes with lifetime variables
#[derive(Debug)]
struct UnitStruct;

let green = ColorClassicStruct {
    // name: "green".to_string(),
    // hex: "#00FF00".to_string(),
    name: String::from("green"),
    hex: String::from("#00FF00"),
};

let green = ColorTupleStruct("green", "#00FF00");

let unit_struct = UnitStruct;
```

enum3.rs in Rustlings is very interesting for reference. Use the [match](https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html?highlight=match#multiple-patterns) in Rust.

Plus, modules3.rs in Rustlings is also interesting to implement.

Remember the initialization of HashMap:
```rust
// hashmap.rs
let mut basket = HashMap::new();
basket.insert("apple", 1);
```

**Rustlings**: finish the 66% of rustlings exercises.

### Day 7 2022/7/7

**Rustlings**: I'll finish the rest of the rustlings exercises.

The trait `std::fmt::Display` is not implemented for `HashMap` and `Option<Option<i8>>`.

So in format strings use `{:?}` instead of `{}`.

Option is so hard.

### Day 8 2022/7/8

Finished the rustlings tests, but some questions still not quite understood.

Plus, I finished lab0-1

### Day 9 2022/7/9

* ?????????rustlings???????????????????????????????????????lab0???
* ?????????WSL2????????????ubuntu20.04???vscode???????????????????????????????????????16G??????????????????vscode????????????????????????????????????vemm???????????????????????????????????????????????????????????????github????????????????????????ubuntu??????????????????????????????
* All in ubuntu?????????ubuntu?????????????????????????????????????????????

### Day 10 2022/7/10

* ????????????lab1?????????????????????????????????lab1??????????????????????????????????????????
* ??????????????????risc-v?????????????????????????????????
* ??????rust???

### Day 11 2022/7/11

```rust
// syscall/process.rs
/// YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    // -1
    set_task_info(ti);
    0
}
```

* ?????????lab1???CI??????

![lab1-os3](../_images/lab1-os3.png)

### Day 12 2022/7/12

### Day 13 2022/7/13

### Day 14 2022/7/14

### Day 15 2022/7/15

```rust
// syscall/process.rs
// YOUR JOB: ???????????????????????? sys_get_time
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    let us = get_time_us();
    // unsafe {
    //     *ts = TimeVal {
    //         sec: us / 1_000_000,
    //         usec: us % 1_000_000,
    //     };
    // }
    translated_assign_ptr(
        current_user_token(),
        ts,
        TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        }
    );
    0
}

...

// YOUR JOB: ????????????????????? sys_mmap ??? sys_munmap
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    // -1
    mmap_in_current_memory_set(start, len, port)
}

pub fn sys_munmap(start: usize, len: usize) -> isize {
    // -1
    munmap_in_current_memory_set(start, len)
}

// YOUR JOB: ???????????????????????? sys_task_info
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    // -1
    translated_assign_ptr(
        current_user_token(),
        ti,
        get_task_info()
    );
    0
}
```

### Day 16 2022/7/16

* ?????????lab2???CI??????

![lab2-os4](../_images/lab2-os4.png)

### Day 17 2022/7/17

### Day 18 2022/7/18

### Day 19 2022/7/19

### Day 20 2022/7/20

### Day 21 2022/7/21

### Day 22 2022/7/22

### Day 23 2022/7/23

### Day 24 2022/7/24

### Day 25 2022/7/25

### Day 26 2022/7/26

### Day 27 2022/7/27

```rust
/// YOUR JOB: ???????????????????????? sys_get_time
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    let us = get_time_us();
    let bufs = translated_large_type::<TimeVal>(current_user_token(), ts);
    unsafe {
        copy_type_into_bufs::<TimeVal>(
            &TimeVal {
                sec: us / 1_000_000,
                usec: us % 1_000_000,
            },
            bufs
        );
    }
    0
}

...

// YOUR JOB: ???????????????????????? sys_task_info
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
  ...

// YOUR JOB: ??????sys_set_priority???????????????????????????
pub fn sys_set_priority(prio: isize) -> isize {
  ...

// YOUR JOB: ????????????????????? sys_mmap ??? sys_munmap
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
  ...
pub fn sys_munmap(start: usize, len: usize) -> isize {
  ...

// YOUR JOB: ?????? sys_spawn ????????????
// ALERT: ??????????????? SPAWN ??????????????????????????????????????????SPAWN != FORK + EXEC 
pub fn sys_spawn(path: *const u8) -> isize {
  ...
```

### Day 28 2022/7/28

* ?????????lab3???CI??????

![lab3-os5](../_images/lab3-os5.png)

### Day 29 2022/7/29

### Day 30 2022/7/30

### Day 31 2022/7/31
