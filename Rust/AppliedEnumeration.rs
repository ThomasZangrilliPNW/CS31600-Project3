#[derive(Debug, PartialEq, Clone, Copy)]
enum ThermostatState {
    Heating,
    Cooling,
    Idle,
}

impl ThermostatState {
    fn should_stop(&self, current: f64, target: f64) -> bool {
        match self {
            ThermostatState::Heating => current >= target, 
            ThermostatState::Cooling => current <= target, 
            ThermostatState::Idle    => true,           
        }
    }

    fn label(&self) -> &str {
        match self {
            ThermostatState::Heating => "HEATING",
            ThermostatState::Cooling => "COOLING",
            ThermostatState::Idle    => "IDLE",
        }
    }
}

fn run_thermostat(mut current_temp: f64, target_temp: f64, initial_state: ThermostatState) {
    let mut state = initial_state;

    println!("System Start: {}", state.label());
    println!("Current: {:.1}F | Target: {:.1}F\n", current_temp, target_temp);

    while !state.should_stop(current_temp, target_temp) {
        let delta: f64 = match state {
            ThermostatState::Heating =>  2.0,  
            ThermostatState::Cooling => -2.0,  
            ThermostatState::Idle    =>  0.0,  
        };

        println!("  ...Still {}. Temp: {:.1}F", state.label(), current_temp);
        current_temp += delta;
    }

    state = ThermostatState::Idle;
    println!("\nTarget reached! System is now: {}", state.label());
    println!("Final temp: {:.1}F", current_temp);
}

fn main() {
    println!("==========================================");
    println!("  Scenario 1: Heating");
    println!("==========================================");
    run_thermostat(65.0, 72.0, ThermostatState::Heating);

    println!("\n==========================================");
    println!("  Scenario 2: Cooling");
    println!("==========================================");
    run_thermostat(85.0, 72.0, ThermostatState::Cooling);

    println!("\n==========================================");
    println!("  Scenario 3: Already at target");
    println!("==========================================");
    run_thermostat(72.0, 72.0, ThermostatState::Heating);

    println!("\n==========================================");
    println!("  Beyond Normal Usage");
    println!("==========================================");

    let maybe_state: Option<ThermostatState> = None;
    match maybe_state {
        Some(s) => println!("  State: {}", s.label()),
        None    => println!("  No state set — handled safely. No crash possible."),
    }

    let result = std::panic::catch_unwind(|| {
        let no_state: Option<ThermostatState> = None;
        let s = no_state.unwrap(); // panics here
        s.should_stop(65.0, 72.0)
    });
    match result {
        Ok(_)  => println!("  No panic (unexpected)"),
        Err(_) => {
            println!("  RUNTIME PANIC: unwrap() on a None ThermostatState");
            println!("  Caught at: RUNTIME");
            println!("  In Java: NullPointerException at runtime.");
            println!("  In Rust: only happens if you explicitly call .unwrap().");
        }
    }

    println!("\n  Exhaustiveness test: uncomment Emergency in the enum,");
    println!("  then compile. Rust REFUSES until every match is updated.");
    println!("  Java compiles silently and returns wrong results at runtime.");

    // ── WHY ENUMS ARE ESSENTIAL ───────────────────────────────
    println!("\n==========================================");
    println!("  Why This Needs Enums");
    println!("==========================================");
    println!("  should_stop() is encoded IN each state variant.");
    println!("  The while loop never manually checks which state it is.");
    println!("  Replacing enums with strings would require:");
    // FIXED: Escaped the braces below using {{ and }}
    println!("    if state == HEATING {{ ... }} else if state == COOLING {{ ... }}");
    println!("  No compile-time safety, no exhaustiveness guarantee.");
}
