

#[derive(Debug)]
enum Status {
    Open,
    Closed,
    Pending,
    Locked,
   
}

fn main() {
    let my_status = Status::Open;


    let case1 = match my_status {
        Status::Open    => "The gate is open.",
        Status::Closed  => "The gate is locked.",
        Status::Pending => "The gate is stuck...",
        Status::Locked  => "The gate is bolted shut.",
    };
    println!("Case 1 Result: {}", case1);

   

    println!("\n[Note] Cases 2 and 3 are commented out because they prevent compilation.");
    println!("This demonstrates Rust's strict exhaustive checking.");

    
    println!("\n--- Key Difference from Java ---");
    println!("Java: exhaustiveness is only enforced on switch EXPRESSIONS");
    println!("      (the arrow -> syntax, Java 14+).");
    println!("      Classic switch STATEMENTS silently ignore missing cases.");
    println!();
    println!("Rust: ALL match expressions are always exhaustive.");
    println!("      There is no 'classic' match that falls through silently.");
    println!("      A wildcard `_ =>` must be explicit if you want a catch-all.");

    println!("\n--- Wildcard ( _ ) as catch-all ---");
    let my_status2 = Status::Pending;

    let result = match my_status2 {
        Status::Open => "Open",
        Status::Closed => "Closed",
        _ => "Some other status (Pending or Locked)", // catches remaining variants
    };
    println!("Wildcard result: {}", result);

    println!("\n[Note] Using _ silences the exhaustiveness check for those arms.");
    println!("Unlike Java's default, Rust forces you to write _ explicitly.");
    println!("You cannot accidentally forget a case and have it silently ignored.");
}