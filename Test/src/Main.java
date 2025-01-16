public class Main {
    public static void main(String[] args) {

        Projektmanagement pm = new Projektmanagement();
        String[][] datenArray = {
                {"HELF", "Eduway", "Analyse", "01.02.2017", "08:02", "10:00"},
                {"GAME", "Eduway", "Analyse", "01.02.2017", "08:02", "10:00"},
                {"HELF", "Eduway", "Analyse", "01.02.2017", "10:00", "12:15"},
                {"HELT", "Homepage", "Update", "01.02.2017", "07:00", "12:15"},
                {"HELT", "Homepage", "Update", "01.02.2017", "13:00", "15:15"},
                {"GAME", "Sigma", "Implementierung", "01.02.2017", "10:00", "12:30"},
                {"GAME", "Sigma", "Implementierung", "01.02.2017", "12:30", "17:00"},
                {"GAME", "Eduway", "Spezifikation", "02.02.2017", "07:00", "13:00"},
                {"HELF", "Eduway", "Spezifikation", "02.02.2017", "07:00", "13:00"},
                {"SANH", "Eduway", "Spezifikation", "02.02.2017", "07:00", "13:00"},
                {"GAME", "Eduway", "Spezifikation", "02.02.2017", "14:00", "17:00"},
                {"HELF", "Eduway", "Spezifikation", "02.02.2017", "14:00", "18:00"},
                {"SANH", "Eduway", "Spezifikation", "02.02.2017", "14:00", "18:05"},
                {"GAME", "Sigma", "Implementierung", "04.02.2017", "08:00", "17:00"},
                {"HELF", "Sigma", "Implementierung", "02.02.2017", "07:00", "18:00"},
                {"SANH", "Sigma", "Implementierung", "02.02.2017", "09:00", "18:05"},
                {"GAME", "Administration", "Reiseabrechnung", "03.02.2017", "08:00", "08:30"},
                {"HELF", "Sigma", "Implementierung", "03.02.2017", "07:00", "18:00"},
                {"SANH", "Sigma", "Implementierung", "03.02.2017", "09:00", "18:05"},
                {"HELT", "Homepage", "Update", "03.02.2017", "09:00", "12:15"},
                {"GAME", "Homepage", "Update", "05.02.2017", "06:00", "15:30"},
                {"HELF", "Sigma", "Implementierung", "03.02.2017", "07:00", "18:00"},
                {"SANH", "Administration", "Reiseabrechnung", "03.02.2017", "09:00", "10:00"},
                {"HELT", "Homepage", "Update", "03.02.2017", "09:00", "12:15"},
                {"SANH", "Netzwerk", "Administration", "03.02.2017", "10:00", "17:00"}
        };


        pm.addData(datenArray);
//        pm.printProjektPlan("Sigma");
//        System.out.println("############################################");
//        pm.printProjektPlan("Eduway");
        pm.printProjektAll();
        System.out.println("########################################################################################");

        pm.getTimeTreeOfPerson("GAME");
    }
}