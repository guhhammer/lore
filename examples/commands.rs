//! Prints every command lore lists on this machine, `name<TAB>kind`, one per
//! line: the input of scripts/content/man.py and fix.py.
//!
//!     cargo run --release --example commands > .content-draft/commands.tsv

fn main() {
    for command in lore_tui::commands::collect() {
        println!("{}\t{:?}", command.name, command.kind);
    }
}
