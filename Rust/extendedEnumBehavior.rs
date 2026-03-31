

#[derive(Debug, Clone, PartialEq)]
enum MessageType {
    Info(String),
    Warning(String),
    Error(String),
    Fatal(String),
}

impl MessageType {
    
    fn message(&self) -> &str {
        match self {
            MessageType::Info(m)    => m,
            MessageType::Warning(m) => m,
            MessageType::Error(m)   => m,
            MessageType::Fatal(m)   => m,
        }
    }

    
    fn name(&self) -> &str {
        match self {
            MessageType::Info(_)    => "INFO",
            MessageType::Warning(_) => "WARNING",
            MessageType::Error(_)   => "ERROR",
            MessageType::Fatal(_)   => "FATAL",
        }
    }

 
    fn with_message(self, new_message: &str) -> MessageType {
        match self {
            MessageType::Info(_)    => MessageType::Info(new_message.to_string()),
            MessageType::Warning(_) => MessageType::Warning(new_message.to_string()),
            MessageType::Error(_)   => MessageType::Error(new_message.to_string()),
            MessageType::Fatal(_)   => MessageType::Fatal(new_message.to_string()),
        }
    }
}

fn main() {
   
    println!("=== Normal Usage ===\n");

    let info = MessageType::Info(String::from("Info"));
    println!("{}", info.name());       // prints: INFO   (Java: System.out.println(Info))
    println!("{}", info.message());    // prints: Info   (Java: System.out.println(Info.message))

   

    println!("\n=== Beyond Normal: Mutation ===\n");

    let mut info_mut = MessageType::Info(String::from("Info"));
    let info2 = MessageType::Info(String::from("Info")); // separate copy

    println!("Before mutation:");
    println!("  info_mut  = {}", info_mut.message());  // Info
    println!("  info2     = {}", info2.message());     // Info

   
    info_mut = info_mut.with_message("Hello World");

    println!("\nAfter info_mut = info_mut.with_message(\"Hello World\"):");
    println!("  info_mut  = {}", info_mut.message());  // Hello World
    println!("  info2     = {}", info2.message());     // Still "Info" -- NOT affected


    println!("\n--- Key Finding ---");
    println!("Java:  mutating Info.message changes Info2.message too");
    println!("       because both point to the SAME singleton object.");
    println!("Rust:  mutating info_mut has no effect on info2");
    println!("       because each binding is an independent VALUE.");

   
    println!("\n=== Beyond Normal: Null vs Option<T> ===\n");

   
    let maybe: Option<MessageType> = None;

    match &maybe {
        Some(m) => println!("  Message: {}", m.message()),
        None    => println!("  No message set — handled safely, no crash."),
    }

  
    let result = std::panic::catch_unwind(|| {
        let no_msg: Option<MessageType> = None;
        let m = no_msg.unwrap(); // panics
        m.message().to_string()
    });

    match result {
        Ok(msg) => println!("  Got: {} (unexpected)", msg),
        Err(_)  => {
            println!("  RUNTIME PANIC: unwrap() on None MessageType");
            println!("  Caught at: RUNTIME");
            println!("  Java equivalent: NullPointerException at runtime.");
            println!("  In Rust this only happens if you explicitly call .unwrap().");
        }
    }

   
    println!("\n=== Beyond Normal: Accessing data from wrong variant ===\n");

  

    let msg_type = MessageType::Fatal(String::from("Fatal"));

    if let MessageType::Fatal(text) = &msg_type {
        println!("  Fatal message text: {}", text);
    }

 
    if let MessageType::Info(text) = &msg_type {
        println!("  Info text: {}", text);
    } else {
        println!("  Not an Info variant — safely skipped.");
    }

    println!("\n=== Summary ===\n");
    println!("  Scenario                        Java              Rust");
    println!("  -------------                   ----              ----");
    println!("  Enum with associated data       Shared field      Per-variant value");
    println!("  Mutation of associated data     Mutates singleton Independent copy");
    println!("  Null enum variable              Allowed (NPE)     Impossible (Option<T>)");
    println!("  Accessing data on wrong type    Same field always Pattern match required");
}