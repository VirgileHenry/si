pub trait TimeUnit {
    const SYMBOL: &'static str;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Second;

impl TimeUnit for Second {
    const SYMBOL: &'static str = "s";
}
