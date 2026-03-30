public class extendedEnumBehavior
{
    public enum MessageType {
		    INFO("Info"),
		    WARNING("Warning"),
		    ERROR("Error"),
		    FATAL("Fatal");
		    
		    public String message; //you would want the message to be a public final so that you're not modifing the enum itself
		    private MessageType(String message){
		        this.message = message;
		    }
	}
	
	public static void main(String[] args) {
		MessageType Info = MessageType.INFO;
		System.out.println(Info);
		System.out.println(Info.message);
		Info.message = "Hello World";
		System.out.println(Info.message);
		MessageType Info2 = MessageType.INFO;
		System.out.println(Info2.message);  //This shows that the INFO enum object stores the message and not the variable that you made to transport the enum
	}
}