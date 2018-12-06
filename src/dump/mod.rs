//! TODO

use crate::direction::Output;
use crate::{Ieee1164, Port};

use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::Path;

use crate::logicbit::LogicVector;
use chrono::Local;

/// A trait for iterating over the containing [`Port`]s of a `Model`.
///
/// Instead of using (non-exiting) reflection, you have to pass all Ports you want to export to the
/// argument `FnMut`.
///
/// This is mainly used for dumping purposes, because this operations can be quiet expensive.
//TODO: Is this really needed? Let's rethink dumping values.
pub trait IterPorts {
    /// See [`IterPorts] for a good description.
    ///
    /// The implementor should pass a short, descripting `&str` as long with the `Port`.
    /// Currently only [`Ieee1164`] is supported, in the future other values will be supported too.
    fn iter_ports<F>(&self, f: F)
    where
        F: FnMut(&str, &Port<Ieee1164, Output>);
}

//TODO: Is this really needed? Let's rethink dumping values.
/// Iterates over the values of a struct. This can either be a single value or multiple, depending
/// on the struct itself.
pub trait IterValues {
    /// FooBar
    // TODO!
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

/// This is a dumper which will output a `.vcd` file. You can than view the waveform in programs,
/// e.g. [GtkWave](http://gtkwave.sourceforge.net/).
#[derive(Debug, Default)]
pub struct Vcd {
    module_name: String,
    tags: BTreeMap<u32, Vec<(Ident, String)>>, //Do we need more than 4x10^9 timestamps? I don't think so :/
    identifier: HashMap<String, Ident>,
    timestamp: u32,
}

impl Vcd {
    /// Create a new `Vcd` dumper that will be able to serialize an `Ieee1164` or a `LogicVector`.
    pub fn new(module_name: &str) -> Self {
        let mut tags = BTreeMap::new();
        tags.insert(0, vec![]);
        Self {
            module_name: module_name.into(),
            tags,
            ..Default::default()
        }
    }

    // TODO: replace this by a trait function
    /// Serializes a struct which holds `Port`s. This function will dump all ports it contains.
    pub fn serialize_ports(&mut self, ports: &impl IterPorts) {
        ports.iter_ports(|n, p: &Port<Ieee1164, Output>| {
            p.iter_values(|v| self.serialize_ieee1164(n, *v));
        });
    }

    /// Ticks this dumper. This will increment the inner time to the next value.
    pub fn tick(&mut self) {
        self.timestamp += 1;
        self.tags.insert(self.timestamp, vec![]);
    }

    /// Serializes a `LogicVector`, but won't write anything to a file. It just stores the value
    /// in memory and a call to [`Vcd::dump`] will actually write the values to disk in the proper
    /// format.
    pub fn serialize_logivector(&mut self, identifier: &str, value: &LogicVector) {
        let ident = self
            .identifier
            .entry(identifier.to_string())
            .or_insert_with(|| Ident {
                ty: Type::Register,
                width: value.width(),
                ident: gen_ident(),
                name: identifier.to_string(),
            })
            .clone();

        self.tags
            .get_mut(&self.timestamp)
            .unwrap()
            .push((ident, value.to_string()));
    }

    /// Serializes an `Ieee1164`, but won't write anything to a file yet. It just stores the value
    /// in memory and a call to [`Vcd::dump`] will actually write the values to disk in the proper
    /// format.
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
    /// Dumps the recorded values to the file at `path`. In any case of an error, an `std::io::Error`
    /// will be returned.
    /// The file will not be overwritten if it already exists.
    pub fn dump<A: AsRef<Path>>(&mut self, path: A) -> io::Result<()> {
        let mut file = OpenOptions::new().write(true).create(true).truncate(true).open(path)?; // FIXME: do not truncate

        // header
        writeln!(file, "$date\n {date}\n$end", date = Local::now())?;
        writeln!(file, "$version\n Logical-rs VCD dumper\n$end")?;
        writeln!(file, "$timescale 1ps $end")?;

        // vars
        writeln!(file, "$scope module {module_name} $end", module_name = self.module_name)?;
        for i in self.identifier.values() {
            // TODO: recursive structures
            writeln!(
                file,
                "$var {typ} {width} {ident} {name} $end",
                typ = i.ty.to_string(),
                width = i.width,
                ident = i.ident,
                name = i.name
            )?;
        }
        writeln!(file, "$upscope $end")?;
        writeln!(file, "$enddefinitions $end")?;

        // dump
        writeln!(file, "$dumpvars")?;
        for (ts, values) in &self.tags {
            writeln!(file, "#{timestamp}", timestamp = ts)?;
            for (i, v) in values {
                match i.ty {
                    Type::Wire => writeln!(file, "{value}{ident}", value = v, ident = i.ident)?,
                    Type::Register => writeln!(file, "b{value} {ident}", value = v, ident = i.ident)?,
                }
            }
        }

        Ok(())
    }
}
