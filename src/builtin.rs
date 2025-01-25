use crate::builtin::Builtin::Echo;

pub enum Builtin {
    Echo,
}

impl Builtin {
    pub fn handle(&self, args: Vec<&str>) {
        match self {
            Echo => println!("{}", args.join(" ")),
        }
    }
}