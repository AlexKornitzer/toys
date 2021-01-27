use std::collections::HashMap;
use std::mem::{size_of, MaybeUninit};
use std::time::Instant;

use libc::{getrusage, rusage, RUSAGE_SELF};

#[derive(Default)]
struct A {
    pub a: u32,
    pub b: u32,
    pub c: u32,
    pub d: D,
    pub k: u32,
    pub l: u32,
    pub m: u32,
    pub n: u32,
    pub o: u32,
    pub p: u32,
    pub q: u32,
}

#[derive(Default)]
struct D {
    pub e: u32,
    pub f: u32,
    pub g: u32,
    pub h: u32,
    pub i: u32,
    pub j: u32,
}

struct Container {
    raw: Box<str>,
    offsets: A,
}

impl Container {
    pub fn get(&self, key: &str) -> Option<String> {
        match key {
            "a" => {
                if self.offsets.a == 0 {
                    return None;
                }
                Some(self.raw[0..self.offsets.a as usize].to_string())
            }
            "b" => {
                if self.offsets.b == self.offsets.a {
                    return None;
                }
                Some(self.raw[self.offsets.a as usize..self.offsets.b as usize].to_string())
            }
            "c" => {
                if self.offsets.c == self.offsets.b {
                    return None;
                }
                Some(self.raw[self.offsets.b as usize..self.offsets.c as usize].to_string())
            }
            _ => None,
        }
    }
}

fn main() {
    println!("Container is {}", size_of::<Container>());

    let start = Instant::now();
    let mut map: HashMap<u64, Container> = HashMap::with_capacity(1_000_000);
    for i in 0..1_000_000 {
        map.insert(
            i,
            Container {
                raw: String::from("abcdefghijklmnopqrstuvwxyz").into_boxed_str(),
                offsets: A {
                    a: 1,
                    b: 2,
                    c: 3,
                    d: D {
                        e: 4,
                        f: 5,
                        g: 6,
                        h: 7,
                        i: 8,
                        j: 9,
                    },
                    k: 10,
                    l: 11,
                    m: 12,
                    n: 13,
                    o: 14,
                    p: 15,
                    q: 16,
                },
            },
        );
    }
    let duration = start.elapsed();
    println!("duration {:?}", duration);

    let container = Container {
        raw: String::from("abcdefghijklmnopqrstuvwxyz").into_boxed_str(),
        offsets: A {
            a: 1,
            b: 1,
            c: 3,
            d: D {
                e: 4,
                f: 5,
                g: 6,
                h: 7,
                i: 8,
                j: 9,
            },
            k: 10,
            l: 11,
            m: 12,
            n: 13,
            o: 14,
            p: 15,
            q: 16,
        },
    };
    println!("{:?}", container.get("a"));
    println!("{:?}", container.get("b"));
    println!("{:?}", container.get("c"));

    let mut usage = MaybeUninit::<rusage>::uninit();
    unsafe {
        getrusage(RUSAGE_SELF, usage.as_mut_ptr());
    }
    let usage = unsafe { usage.assume_init() };
    println!("{}kb", usage.ru_maxrss);
}
