import jdk.swing.interop.SwingInterOpUtils;

import java.io.*;
import java.util.LinkedList;

public class Main {
    public static void main(String[] args) {
        
        
        readHistory("/Users/josef/.zsh_history");
    }

    public static void readHistory(String path ) {
        LinkedList linkedList = new LinkedList();
        try (BufferedReader inputStream = new BufferedReader(new FileReader(path))) {
            String line;
            while ((line = inputStream.readLine()) != null) {
                linkedList.add(line.split(" "));
            }
        } catch (FileNotFoundException fx) {
            System.out.println(path + " konnte nicht gefunden werden");
            System.out.println(fx.getMessage());
            return;
        } catch (IOException io) {
            System.out.println(io.getMessage());
        }

    }
    
    
}