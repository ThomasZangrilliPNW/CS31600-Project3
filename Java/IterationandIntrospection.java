public class IterationandIntrospection {

    enum Color {
        RED(1),
        CRIMSON(1), // ALIAS: Same integer (1) as RED
        BLUE(2),
        NAVY(2);    // ALIAS: Same integer (2) as BLUE

        private final int value;

        // Constructor to attach the integer
        Color(int value) {
            this.value = value;
        }

        public int getValue() { return value; }
    }

    public static void main(String[] args) {
        System.out.println("ITERATION TEST: ALIASED DATA");

        // 1. Iterate over all values
        // We are checking if both RED and CRIMSON appear in the list
        for (Color c : Color.values()) {
            System.out.println("Found Name: " + c.name() + " | Associated Integer: " + c.getValue());
        }

        // 2. Comparison Test
        System.out.println("\n EQUALITY TEST");
        System.out.println("Does RED == CRIMSON? " + (Color.RED == Color.CRIMSON));
    }
}
