use std::{
    env,
    fmt::{Display, Formatter},
    fs::OpenOptions,
    io::{self, BufWriter, Write},
    path::Path,
};

struct NSpace {
    n: usize,
}

impl Display for NSpace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for _ in 0..self.n {
            write!(f, " ")?;
        }
        Ok(())
    }
}

struct NExludeM {
    name: &'static str,
    n: usize,
    m: usize,
}

impl Display for NExludeM {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in (0..self.n).filter(|&i| i != self.m) {
            write!(f, "{}{i},", self.name, i = i)?;
        }
        Ok(())
    }
}

fn magic<W: Write>(mut w: W, n: usize, i: usize) -> Result<W, io::Error> {
    if i < n {
        writeln!(
            w,
            "{}match self.{i}.parse(input.clone()) {{",
            NSpace { n: (i + 2) * 2 },
            i = i
        )?;
        writeln!(
            w,
            "{}Ok((tail_{i}, output_{i})) => {{",
            NSpace { n: (i + 3) * 2 },
            i = i
        )?;
        writeln!(
            w,
            "{}let (tail, ({})) = ({}).permutation(tail_{})?;",
            NSpace { n: (i + 4) * 2 },
            NExludeM {
                name: "output_",
                n: n,
                m: i,
            },
            NExludeM {
                name: "&mut self.",
                n: n,
                m: i,
            },
            i,
        )?;
        writeln!(
            w,
            "{}Ok((tail, ({})))",
            NSpace { n: (i + 4) * 2 },
            NTuple {
                name: "output_",
                n: n,
            }
        )?;
        writeln!(w, "{}}}", NSpace { n: (i + 3) * 2 })?;
        writeln!(
            w,
            "{}Err(nom::Err::Error(error)) => {{",
            NSpace { n: (i + 3) * 2 }
        )?;
        let mut w = magic(w, n, i + 1)?;
        writeln!(w, "{}}}", NSpace { n: (i + 3) * 2 })?;
        writeln!(w, "{}Err(error) => {{", NSpace { n: (i + 3) * 2 })?;
        writeln!(w, "{}Err(error)", NSpace { n: (i + 4) * 2 })?;
        writeln!(w, "{}}}", NSpace { n: (i + 3) * 2 })?;
        writeln!(w, "{}}}", NSpace { n: (i + 2) * 2 })?;

        Ok(w)
    } else {
        writeln!(w, "{}Err(nom::Err::Error(Error::append(input, nom::error::ErrorKind::Permutation, error)))", NSpace { n: (i + 3) * 2 })?;

        Ok(w)
    }
}

struct NParser {
    n: usize,
}

impl Display for NParser {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.n {
            write!(
                f,
                "Output{i}, Fn{i}: nom::Parser<Input, Output{i}, Error>,",
                i = i
            )?;
        }
        Ok(())
    }
}

struct NTuple {
    name: &'static str,
    n: usize,
}

impl Display for NTuple {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.n {
            write!(f, "{}{i},", self.name, i = i)?;
        }
        Ok(())
    }
}

fn permutation_impl<W: Write>(mut w: W, n: usize) -> Result<W, io::Error> {
    for i in 1..n {
        writeln!(
            w,
            "impl<Input: Clone, Error: nom::error::ParseError<Input>, {}> Permutation<Input, ({}), Error> for ({}) {{",
            NParser { n: i },
            NTuple { name: "Output", n: i },
            NTuple { name: "Fn", n: i },
        )?;
        writeln!(
            w,
            "  fn permutation(&mut self, input: Input) -> nom::IResult<Input, ({}), Error> {{",
            NTuple {
                name: "Output",
                n: i
            }
        )?;
        w = magic(w, i, 0)?;
        writeln!(w, "  }}")?;
        writeln!(w, "}}")?;
        writeln!(w, "")?;
    }

    Ok(w)
}

fn main() -> Result<(), io::Error> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("permutation.rs");
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(dest_path)?;

    permutation_impl(BufWriter::new(file), 8)?;
    println!("cargo:rerun-if-changed=build.rs");

    Ok(())
}
