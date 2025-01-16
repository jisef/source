
public class Zeiteintrag implements Comparable{
    private String phase;
    private String datum;
    private String zeitVon;
    private String zeitBis;

    public String getDatum() {
        return datum;
    }

    public Zeiteintrag(String phase, String datum, String zeitVon, String zeitBis) {
        this.phase = phase;
        this.datum = datum;
        this.zeitVon = zeitVon;
        this.zeitBis = zeitBis;
    }

    public String getPhase() {
        return phase;
    }

    public String getZeitVon() {
        return zeitVon;
    }

    public String getZeitBis() {
        return zeitBis;
    }

    @Override
    public int compareTo(Object o) {
        // Compare by phase
        int phaseComparison = phase.compareTo(((Zeiteintrag)o).getPhase());
        if (phaseComparison != 0) {
            return phaseComparison;
        }

        // Compare by datum
        int datumComparison = datum.compareTo(((Zeiteintrag)o).getDatum());
        if (datumComparison != 0) {
            return datumComparison;
        }

        // Compare by zeitVon
        int zeitVonComparison = zeitVon.compareTo(((Zeiteintrag)o).getZeitVon());
        if (zeitVonComparison != 0) {
            return zeitVonComparison;
        }

        // Compare by zeitBis
        return zeitBis.compareTo(((Zeiteintrag)o).getPhase());
    }
}
