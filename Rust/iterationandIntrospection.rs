
#[derive(Debug, PartialEq, Clone, Copy)]
enum Color {
    Red,
    Crimson, // "alias": same associated integer (1) as Red
    Blue,
    Navy,    // "alias": same associated integer (2) as Blue
}

impl Color {
    
    fn value(&self) -> u32 {
        match self {
            Color::Red     => 1,
            Color::Crimson => 1, // same as Red — the "alias"
            Color::Blue    => 2,
            Color::Navy    => 2, // same as Blue — the "alias"
        }
    }

   
    fn name(&self) -> &str {
        match self {
            Color::Red     => "RED",
            Color::Crimson => "CRIMSON",
            Color::Blue    => "BLUE",
            Color::Navy    => "NAVY",
        }
    }


    fn all() -> &'static [Color] {
        &[Color::Red, Color::Crimson, Color::Blue, Color::Navy]
    }
}

fn main() {
   
    println!("ITERATION TEST: ALIASED DATA\n");

    for c in Color::all() {
        println!("Found Name: {:5} | Associated Integer: {}", c.name(), c.value());
    }

   
    println!("\nEQUALITY TEST");
    println!("Does RED == CRIMSON? {}", Color::Red == Color::Crimson);

    // ── KEY FINDINGS ──────────────────────────────────────────
    println!("\n--- Key Findings ---");

    println!("\n[Iteration]");
    println!("  Java: Color.values() is built in — one line, no setup.");
    println!("  Rust: No built-in equivalent. Color::all() must be");
    println!("        defined manually (or use the `strum` crate).");
    println!("  Both: All four variants appear — RED, CRIMSON, BLUE, NAVY.");
    println!("        Aliased values do NOT collapse into one entry.");

    println!("\n[Equality]");
    println!("  Java: RED == CRIMSON is false.");
    println!("        Java compares by OBJECT IDENTITY — two distinct objects.");
    println!("  Rust: Red == Crimson is false.");
    println!("        Rust compares by VARIANT IDENTITY via PartialEq.");
    println!("        Same result, different reason.");

    println!("\n[Aliased Values]");
    println!("  Both languages allow two variants to share the same integer.");
    println!("  Java:  custom field via constructor — Color(int value)");
    println!("  Rust:  return value from match — no constructor needed.");
    println!("  In both: the variants remain DISTINCT despite sharing a value.");

    // ── BEYOND NORMAL: what if we compare values, not variants? ─
    println!("\n--- Beyond Normal: Comparing Values vs Variants ---");
    println!("  Color::Red.value() == Color::Crimson.value() → {}",
             Color::Red.value() == Color::Crimson.value());
    println!("  Color::Red == Color::Crimson                 → {}",
             Color::Red == Color::Crimson);
    println!();
    println!("  Comparing the associated INTEGER gives true (both are 1).");
    println!("  Comparing the VARIANTS gives false (they are distinct).");
    println!("  Java behaves identically:");
    println!("    RED.getValue() == CRIMSON.getValue() → true");
    println!("    RED == CRIMSON                       → false");
}