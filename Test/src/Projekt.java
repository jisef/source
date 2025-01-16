import java.util.Objects;
import java.util.TreeMap;

public class Projekt implements Comparable<Projekt>{
    private String name;
    TreeMap<String, Person> zeiteintrag;

    public Projekt(String name) {
        this.name = name;
        this.zeiteintrag = new TreeMap<String, Person>();
    }

    public String getName() {
        return name;
    }

    public boolean addPerson(Person person, Zeiteintrag zeiteintrag){
        return this.zeiteintrag.putIfAbsent(person.getName(), person) != null;
    }

    @Override
    public int compareTo(Projekt o) {
        return this.name.compareTo(((Projekt)o).getName());
    }

    @Override
    public boolean equals(Object o) {
        return ((Projekt)o).getName().equals(this.getName());
    }

    @Override
    public int hashCode() {
        return Objects.hashCode(name);
    }
}