import javax.swing.text.html.HTMLDocument;
import java.util.ArrayList;
import java.util.Iterator;
import java.util.stream.Collectors;

public class Projektmanagement {
    private ArrayList<Projekt> projekte;

    public Projektmanagement() {
        this.projekte = new ArrayList<Projekt>();
    }

    public ArrayList<Projekt> getProjekte() {
        return projekte;
    }


    void addData(String[][] data) {
        for (String[] line : data ) {
            Projekt prj = new Projekt(line[1]);
            if (this.projekte.contains(prj)) {
                Zeiteintrag zeiteintrag = new Zeiteintrag(line[2],line[3], line[4], line[5]);
                Projekt containingProjekt = this.projekte.get(this.projekte.indexOf(prj));

                if (containingProjekt.getZeiteintrag().containsKey(line[0])) {
                    containingProjekt.getZeiteintrag().get(line[0]).addZeiteintrag(zeiteintrag);
                } else {
                    Person person = new Person(line[0], zeiteintrag);
                    containingProjekt.addPerson(person, zeiteintrag);
                }




                // zeiteintrag needs to be added to person and projekt
                this.projekte.get(this.projekte.indexOf(prj)).addPerson(new Person(line[0],zeiteintrag), zeiteintrag);
            } else { // nicht drinnen
                Projekt projekt = new Projekt(line[1]);
                Zeiteintrag zeiteintrag = new Zeiteintrag(line[2],line[3], line[4], line[5]);
                Person person = new Person(line[0], zeiteintrag);
                projekt.addPerson(person, zeiteintrag);
                this.projekte.add(projekt);
            }
        }
    }

    void printProjektPlan(String projektname) {
        ArrayList<Projekt> list = this.getProjekte().stream().filter(x -> x.getName().equals(projektname)).collect(Collectors.toCollection(ArrayList<Projekt>::new));
        Projekt element = list.get(0);
        if (list.isEmpty() || list == null) {
            System.out.println("Das Projekt " + projektname + " existiert nicht.");
        } else {
            System.out.println("Projekt: " + element.getName());
            for (Person person : element.zeiteintrag.values()) {
                System.out.println("Person : " + person.getName());
                for (Zeiteintrag ze : person.getZeiteintrag()) {
                    System.out.println("    Phase: " + ze.getPhase() + " Datum: " + ze.getDatum() + " Zeit von: " + ze.getZeitVon() + " bis: " + ze.getZeitBis());
                }
            }
        }

    }

    void printProjektAll() {
        for (Projekt projekt : this.getProjekte()) {
            printProjektPlan(projekt.getName());
            System.out.println("############################################");

        }
    }


    void getTimeTreeOfPerson(String namePerson) {
        System.out.println(namePerson + ": ");
        StringBuilder sb = new StringBuilder();
        Iterator<Projekt> projects = this.getProjekte().iterator();
        while (projects.hasNext()) {
            Projekt ss = projects.next();
            Person dd = ss.getZeiteintrag().get(namePerson);
            if (dd == null) {
                continue;
            }

            for (Zeiteintrag ze : dd.getZeiteintrag()) {
                if (dd.getName() != null) {
                    sb.append("  " + ss.getName()+ " ").append(ze.getPhase()).append(" ").append(ze.getDatum()).append(" ").append(ze.getZeitVon()).append(" bis: ").append(ze.getZeitBis()).append("\n");
                }
            }
        }

        System.out.println(sb.toString());

    }
}
