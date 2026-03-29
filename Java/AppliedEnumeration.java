public class AppliedEnumeration {

    // THE LOGIC-DRIVING ENUM
    enum ThermostatState {
        HEATING {
            @Override
            public boolean shouldStop(double current, double target) {
                return current >= target; // Stop if it's warm enough
            }
        },
        COOLING {
            @Override
            public boolean shouldStop(double current, double target) {
                return current <= target; // Stop if it's cool enough
            }
        },
        IDLE {
            @Override
            public boolean shouldStop(double current, double target) {
                return true; // Already stopped
            }
        };

        // Abstract method: Every state must define its own logic
        public abstract boolean shouldStop(double current, double target);
    }

    public static void main(String[] args) {
        double currentTemp = 65.0;
        double targetTemp = 72.0;

        // Start the system in HEATING mode
        ThermostatState currentState = ThermostatState.HEATING;

        System.out.println("System Start: " + currentState);
        System.out.println("Current: " + currentTemp + " | Target: " + targetTemp);

        // Simulated Logic Loop
        while (!currentState.shouldStop(currentTemp, targetTemp)) {
            System.out.println("...Still " + currentState + ". Increasing temp...");
            currentTemp += 2.0; // Simulate warming up
        }

        currentState = ThermostatState.IDLE;
        System.out.println("\nTarget reached! System is now: " + currentState);
    }
}
