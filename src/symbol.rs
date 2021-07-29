
#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
pub enum Symbol {
    Empty,
    Char(char),
}


// Ranges? We'd rewrite Symbol so that Symbol::Char was just Symbol::Range.  Or, perhaps we'd
// merely extend Symbol with the Range(Range) item?

