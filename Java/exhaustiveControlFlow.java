public class exhaustiveControlFlow { 
    // Enum with the variant LOCKED for Case 3
    enum Status { OPEN, CLOSED, PENDING, LOCKED }

    public static void main(String[] args) {
        Status myStatus = Status.OPEN;

        // CASE 1: ALL VARIANTS COVERED (Perfect)
        String case1 = switch (myStatus) {
            case OPEN    -> "The gate is open.";
            case CLOSED  -> "The gate is locked.";
            case PENDING -> "The gate is stuck...";
            case LOCKED  -> "The gate is bolted shut.";
        };
        System.out.println("Case 1 Result: " + case1);

        // CASE 2: DELIBERATELY MISSING A VARIANT (Compiler Error)
        /* 
        String case2 = switch (myStatus) {
            case OPEN    -> "Open";
            case CLOSED  -> "Closed";
            case LOCKED  -> "Locked";
        };
        */

        // CASE 3: ADDED A NEW VARIANT WITHOUT UPDATING (Compiler Error)
        /* 
        String case3 = switch (myStatus) {
            case OPEN    -> "Open";
            case CLOSED  -> "Closed";
            case PENDING -> "Pending";
        };
        */
        
        System.out.println("\n[Note] Cases 2 and 3 are commented out because they prevent compilation.");
        System.out.println("This demonstrates Java's strict exhaustive checking.");
    }
} 
