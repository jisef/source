import java.util.ArrayList;
import java.util.Hashtable;
import java.util.Map;

public class Command {
    private String command;
    private String[] commandAsArray;
    // Stores from index zero every word
    private ArrayList<Map.Entry<String, Integer>> subcommands;
    
    
    public Command(String command) {
        this.command = command;
        this.commandAsArray = command.split(" ");
        this.subcommands = new ArrayList<Map.Entry<String, Integer>>();
        for (int i = 0; i < commandAsArray.length; i++) {
            this.subcommands.add(new Map.Entry<String, Integer>());
        }
        Hashtable hashtable  = new Hashtable();
        
    }
}
