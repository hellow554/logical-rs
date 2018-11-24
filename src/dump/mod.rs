//! TODO

use crate::direction::Output;
use crate::{Ieee1164, Port};

use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::Path;

use chrono::Local;

pub trait IterPorts {
    fn iter_ports<F>(&self, f: F)
    where
        F: FnMut(&str, &Port<Ieee1164, Output>);
}

pub trait IterValues {
    fn iter_values<F>(&self, f: F)
    where
        F: FnMut(&Ieee1164);
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Type {
    Wire,
    Register,
}

fn gen_ident() -> char {
    static mut IDENT: u8 = b'!';
    unsafe {
        assert!(IDENT >= b'!', "Invalid start of identifier!");
        assert!(IDENT <= b'~', "Ran out of identifier!");
        IDENT += 1;
        IDENT as char
    }
} //FIXME: thread safety?!

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", if let Type::Wire = self { "wire" } else { "reg" })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Ident {
    ty: Type,
    width: u8,
    ident: char,
    name: String,
}

#[derive(Debug, Default)]
pub struct Vcd {
    module_name: String,
    tags: BTreeMap<u32, Vec<(Ident, String)>>, //Do we need more than 4x10^9 timestamps? I don't think so :/
    identifier: HashMap<String, Ident>,
    timestamp: u32,
}

impl Vcd {
    pub fn new(module_name: &str) -> Self {
        let mut tags = BTreeMap::new();
        tags.insert(0, vec![]);
        Self {
            module_name: module_name.into(),
            tags,
            ..Default::default()
        }
    }

    pub fn serialize_ports(&mut self, ports: &impl IterPorts) {
        ports.iter_ports(|n, p: &Port<Ieee1164, Output>| {
            p.iter_values(|v| self.serialize_ieee1164(n, *v));
        });
    }

    pub fn tick(&mut self) {
        self.timestamp += 1;
        self.tags.insert(self.timestamp, vec![]);
    }

    //    pub fn serialize_port<D>(&mut self, name: &str, port: &Port<Ieee1164, D>)
    //    where
    //        D: PortDirection,
    //    {
    //        println!("{}", name);
    //        print!("\t");
    //        port.iter_values(|n, v| self.serialize_ieee1164(n, *v));
    //    }

    pub fn serialize_ieee1164(&mut self, identifier: &str, value: Ieee1164) {
        let ident = self
            .identifier
            .entry(identifier.to_string())
            .or_insert_with(|| Ident {
                ty: Type::Wire,
                width: 1,
                ident: gen_ident(),
                name: identifier.to_string(),
            })
            .clone();

        self.tags
            .get_mut(&self.timestamp)
            .unwrap()
            .push((ident, value.to_string()));
    }
}

impl Vcd {
    pub fn dump<A: AsRef<Path>>(&mut self, path: A) -> io::Result<()> {
        let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(path)?; // FIXME: do not truncate
                                                                                               // header
        writeln!(file, "$date\n    {date}\n$end", date = Local::now())?;
        writeln!(file, "$version\n    Logical-rs VCD dumper\n$end")?;
        writeln!(file, "$comment\n    I don't like diz!\n$end")?;
        writeln!(file, "$timescale 1ps $end")?;

        // vars
        //$var wire 8 # data $end
        writeln!(file, "$scope module {module_name} $end", module_name = self.module_name)?;
        for i in self.identifier.values() {
            writeln!(file, "$var {type} {width} {ident} {name} $end", type=i.ty.to_string(), width=i.width, ident=i.ident, name=i.name)?;
        }
        writeln!(file, "$upscope $end")?;
        writeln!(file, "$enddefinitions $end")?;

        // dump
        writeln!(file, "$dumpvars")?;
        for (ts, values) in &self.tags {
            writeln!(file, "#{timestamp}", timestamp = ts)?;
            for (i, v) in values {
                writeln!(file, "{value}{ident}", value = v, ident = i.ident)?;
            }
            if (*ts as usize) < self.tags.len() - 2 {
                writeln!(file, "$end")?;
            }
        }

        Ok(())
    }
}
