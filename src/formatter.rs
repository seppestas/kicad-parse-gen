// (c) 2016 Productize SPRL <joost@productize.be>

use std::io;

use Sexp;
use symbolic_expressions::Result;
use symbolic_expressions::Formatter;

// custom symbolic_expressions formatter that aims to be
// kicad compatible

pub struct KicadFormatter;

impl Default for KicadFormatter {
    fn default() -> KicadFormatter {
        KicadFormatter {}
    }
}

impl Formatter for KicadFormatter {
    fn open<W>(&mut self, writer: &mut W, _value:Option<&Sexp>) -> Result<()>
        where W: io::Write
    {
        writer.write_all(b"(").map_err(From::from)
    }
    fn element<W>(&mut self, writer: &mut W, _value:&Sexp) -> Result<()>
        where W: io::Write
    {
        writer.write_all(b" ").map_err(From::from)
    }
    
    fn close<W>(&mut self, writer: &mut W) -> Result<()>
        where W: io::Write
    {
        writer.write_all(b")").map_err(From::from)
    }
}
