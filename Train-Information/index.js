function load_proginfo(zugnr,datum,avg,pwert) {
    if(document.getElementById("detail_"+datum) == null)
    {
        http_request = false;

        if (window.XMLHttpRequest) { // Mozilla, Safari,...
            http_request = new XMLHttpRequest();
            if (http_request.overrideMimeType) {
                http_request.overrideMimeType('application/json');
                // zu dieser Zeile siehe weiter unten
            }
        } else if (window.ActiveXObject) { // IE
            try {
                http_request = new ActiveXObject("Msxml2.XMLHTTP");
            } catch (e) {
                try {
                    http_request = new ActiveXObject("Microsoft.XMLHTTP");
                } catch (e) {}
            }
        }

        if (!http_request) {
            alert('Ende :( Kann keine XMLHTTP-Instanz erzeugen');
            return false;
        }

        http_request.onreadystatechange=function()
        {
            if (http_request.readyState==4 && http_request.status==200) {
                setTimeout(show_progdaten(datum,zugnr,pwert),2000);
                setTimeout(() => document.getElementById("form_"+datum).setAttribute("class", "hiddeninfo_show"), 1);
            }
        }
        http_request.open('GET', '/js/zuginfo_json.php?z='+zugnr+'&p='+pwert+'&a='+avg, true);
        http_request.send();

    } else if(document.getElementById("detail_"+datum).style.display == 'table-row') {
        document.getElementById("form_"+datum).setAttribute("class", "hiddeninfo");
        setTimeout(() => {
            document.getElementById("detail_"+datum).style.display = 'none';
            newtr = document.getElementById("detail_"+datum);
            document.getElementById('speicher').appendChild(newtr);
            document.getElementById(datum).firstElementChild.firstElementChild.style.transform = "rotate(0deg)";
        },900);
    } else if(document.getElementById("detail_"+datum).style.display == 'none') {
        newtr = document.getElementById("detail_"+datum);
        document.getElementById(datum).parentNode.insertBefore(newtr, document.getElementById(datum).nextSibling);
        document.getElementById("detail_"+datum).style.display = 'table-row';
        document.getElementById(datum).firstElementChild.firstElementChild.style.transform = "rotate(90deg)";
        setTimeout(() => document.getElementById("form_"+datum).setAttribute("class", "hiddeninfo_show"), 1);
    }
}

function load_zuginfo(zugnr,datum) {
    if(document.getElementById("detail_"+datum) == null)
    {
        http_request = false;

        if (window.XMLHttpRequest) { // Mozilla, Safari,...
            http_request = new XMLHttpRequest();
            if (http_request.overrideMimeType) {
                http_request.overrideMimeType('application/json');
                // zu dieser Zeile siehe weiter unten
            }
        } else if (window.ActiveXObject) { // IE
            try {
                http_request = new ActiveXObject("Msxml2.XMLHTTP");
            } catch (e) {
                try {
                    http_request = new ActiveXObject("Microsoft.XMLHTTP");
                } catch (e) {}
            }
        }

        if (!http_request) {
            alert('Ende :( Kann keine XMLHTTP-Instanz erzeugen');
            return false;
        }

        http_request.onreadystatechange=function()
        {
            if (http_request.readyState==4 && http_request.status==200) {
                setTimeout(show_daten(datum,zugnr),2000);
                setTimeout(() => document.getElementById("form_"+datum).setAttribute("class", "hiddeninfo_show"), 1);
            }
        }
        http_request.open('GET', '/js/zuginfo_json.php?z='+zugnr+'&d='+datum, true);
        http_request.send();

    } else if(document.getElementById("detail_"+datum).style.display == 'table-row') {
        document.getElementById("form_"+datum).setAttribute("class", "hiddeninfo");
        setTimeout(() => {
            document.getElementById("detail_"+datum).style.display = 'none';
            newtr = document.getElementById("detail_"+datum);
            document.getElementById('speicher').appendChild(newtr);
            document.getElementById(datum).firstElementChild.firstElementChild.style.transform = "rotate(0deg)";
        },900);
    } else if(document.getElementById("detail_"+datum).style.display == 'none') {
        newtr = document.getElementById("detail_"+datum);
        document.getElementById(datum).parentNode.insertBefore(newtr, document.getElementById(datum).nextSibling);
        document.getElementById("detail_"+datum).style.display = 'table-row';
        document.getElementById(datum).firstElementChild.firstElementChild.style.transform = "rotate(90deg)";
        setTimeout(() => document.getElementById("form_"+datum).setAttribute("class", "hiddeninfo_show"), 1);
    }
}

function show_daten(datum,zugnr) {
    var xmldoc = JSON.parse(http_request.responseText);

    if(document.getElementById("detail_"+datum) == null) { var newtr = document.createElement("tr"); }
    else {
        //TODO: NIX
    }
    newtr.id = "detail_"+datum;
    newtr.style.display = "table-row";
    var newtd = document.createElement("td");
    newtd.colSpan = "5";
    newtr.appendChild(newtd);
    var newform = document.createElement("form");
    newform.id = 'form_'+datum;
    newform.setAttribute("class", "hiddeninfo");
    var newtable = document.createElement("table");
    newtable.style.borderCollapse = "collapse";
    var formlink = document.createElement("a");
    formlink.innerHTML = "&nbsp; &#8627; "+l_verbindungspeichern;
    formlink.style.fontSize = ".9em";
    formlink.setAttribute("href", "javascript:void(0)");
    formlink.setAttribute("onClick", "var cb=document.querySelectorAll('.bhfselect:checked');opendetail('verbindungspeichern',encodeURI(cb[0].value)+'|'+encodeURI(cb[1].value)+'|"+datum+"|"+zugnr+"');");
    newform.appendChild(newtable);
    newform.appendChild(formlink);
    newtd.appendChild(newform);
    if(document.getElementById("detail_"+datum) == null) {
        document.getElementById(datum).parentNode.insertBefore(newtr, document.getElementById(datum).nextSibling);
        document.getElementById(datum).firstElementChild.firstElementChild.style.transform = "rotate(90deg)";
    }

    //daten aus xml in variablen lesen
    for (var i=0; i < xmldoc.length; ++i) {
        bhf = xmldoc[i].bhf;
        arr = xmldoc[i].arr;
        adelay = xmldoc[i].adelay;
        dep = xmldoc[i].dep;
        ddelay = xmldoc[i].ddelay;
        fontstyle = "";
        if(dep > "0") { checker = "<td style='padding:0;line-height:.4em;border:0'><input type='checkbox' onclick='journeyselector(\""+newform.id+"\")' class='bhfselect' value='"+bhf+"|"+arr+"|"+adelay+"|"+dep+"|"+ddelay+"' /></td>"; }
        else { checker = ""; }

        if(dep == 'live' && adelay == '-1') { adelay = " "+l_ausfall; }
        else if(dep == 'live' && ddelay < 6) { adelay = " <span style='color:#009337'>(+"+adelay+")</span>"; }
        else if(dep == 'live' && ddelay > 5 && ddelay < 16) { adelay = " <span style='color:#e4903b'>(+"+adelay+")</span>"; }
        else if(dep == 'live' && ddelay > 15) { adelay = " <span style='color:#E43B3B'>(+"+adelay+")</span>"; }
        if(dep == 'live') { dep = l_beschreibung_live; ddelay = ""; fontstyle = "; font-style: italic" }

        if(adelay == '0') { adelay = ""; }
        else if(adelay == '-1') { adelay = " "+l_ausfall; }
        else if(adelay > 0 && adelay < 6) { adelay = " <span style='color:#009337'>(+"+adelay+")</span>"; }
        else if(adelay > 5 && adelay < 16) { adelay = " <span style='color:#e4903b'>(+"+adelay+")</span>"; }
        else if(adelay > 15) { adelay = " <span style='color:#E43B3B'>(+"+adelay+")</span>"; }
        if(ddelay == '0') { ddelay = ""; }
        else if(ddelay == '-1') { ddelay = " "+l_ausfall; }
        else if(ddelay > 0 && ddelay < 6) { ddelay = " <span style='color:#009337'>(+"+ddelay+")</span>"; }
        else if(ddelay > 5 && ddelay < 16) { ddelay = " <span style='color:#e4903b'>(+"+ddelay+")</span>"; }
        else if(ddelay > 15) { ddelay = " <span style='color:#E43B3B'>(+"+ddelay+")</span>"; }

        if(arr == '99:99') { arr = "" ;adelay = ""; }
        if(arr == 'proaccount') { arr = l_proaccount; }
        if(dep == '99:99') { dep = "" ;ddelay = ""; }

        var newbhf = document.createElement("tr");
        newbhf.style.fontSize = ".9em";
        newbhf.style.background = "transparent";
        newbhf.innerHTML = checker+"<td style='padding-left:1em;line-height:.9em;border:0"+fontstyle+"'>"+bhf+"</td><td style='padding-left:1em;line-height:.8em;border:0;max-width:200px"+fontstyle+"'>"+arr+adelay+"</td><td style='padding-left:1em;line-height:.8em;border:0'>"+dep+ddelay+"</td>";

        newtable.appendChild(newbhf);
    }
}

function show_progdaten(datum,zugnr,pwert) {
    var xmldoc = JSON.parse(http_request.responseText);

    if(document.getElementById("detail_"+datum) == null) { var newtr = document.createElement("tr"); }
    else {
        //TODO: NIX
    }
    newtr.id = "detail_"+datum;
    newtr.style.display = "table-row";
    var newtd = document.createElement("td");
    newtd.colSpan = "5";
    newtr.appendChild(newtd);
    var newform = document.createElement("form");
    newform.id = 'form_'+datum;
    newform.setAttribute("class", "hiddeninfo");
    var newtable = document.createElement("table");
    newtable.style.borderCollapse = "collapse";
    var information = document.createElement("p");
    information.style.whiteSpace = "normal";
    information.style.fontSize = "0.9em";

    if(pwert < 2.5) { information.innerHTML = l_beschreibung_achtung + " " + l_beschreibung_sehr_gering; }
    else if(pwert >= 2.5 && pwert < 4.0) { information.innerHTML = l_beschreibung_achtung + " " + l_beschreibung_gering; }
    else if(pwert >= 4.0 && pwert < 6.8) { information.innerHTML = l_beschreibung_achtung + " " + l_beschreibung_mittel; }
    else if(pwert >= 6.8 && pwert < 12.0) { information.innerHTML = l_beschreibung_achtung + " " + l_beschreibung_hoch; }
    else if(pwert >= 12) { information.innerHTML = l_beschreibung_achtung + " " + l_beschreibung_sehr_hoch; }
    newform.appendChild(information);
    newform.appendChild(newtable);
    newtd.appendChild(newform);
    if(document.getElementById("detail_"+datum) == null) {
        document.getElementById(datum).parentNode.insertBefore(newtr, document.getElementById(datum).nextSibling);
        document.getElementById(datum).firstElementChild.firstElementChild.style.transform = "rotate(90deg)";
    }

    var json_arr = {};
    //daten aus xml in variablen lesen
    for (var i=0; i < xmldoc.length; ++i) {
        bhf = xmldoc[i].bhf;
        progdelay = xmldoc[i].progdelay;
        avgdelay = Math.round(xmldoc[i].avgdelay);
        //prog = xmldoc[i].prog;
        if(progdelay >= 6.8 && avgdelay > 3) { avgdelay = (avgdelay-4); }
        else if(progdelay >= 6.8 && avgdelay < 4) { avgdelay = 0; }
        if(progdelay >= 2.5 && progdelay < 4.0) { avgdelay = (avgdelay+4); }

        if((avgdelay-10) < 0) { minavg = 0; } else { minavg = (avgdelay-10); }
        if(avgdelay < 5) { maxavg = 5; } else { maxavg = avgdelay; }

        if(progdelay > 0) {
            var newbhf = document.createElement("tr");
            newbhf.style.fontSize = ".9em";
            newbhf.style.background = "transparent";

            if(progdelay < 2.5) { prog = " <span style='color:#009337'>"+l_sehr_gering+" (<5 min)</span>"; rmin = 0; rmax = 5; }
            else if(progdelay >= 2.5 && progdelay < 4.0) { prog = " <span style='color:#009337'>"+l_gering+" (<"+maxavg+" min)</span>"; rmin = 0; rmax = maxavg; }
            else if(progdelay >= 4.0 && progdelay < 6.8) { prog = " <span style='color:#e4903b'>"+l_mittel+" ("+minavg+"-"+(avgdelay+10)+" min)</span>"; rmin = minavg; rmax = (avgdelay+10); }
            else if(progdelay >= 6.8 && progdelay < 12.0) { prog = " <span style='color:#E43B3B'>"+l_hoch+" (>"+avgdelay+" min)</span>"; rmin = avgdelay; rmax = 180; }
            else if(progdelay >= 12) { prog = " <span style='color:#E43B3B'>"+l_sehr_hoch+" (>"+avgdelay+" min)</span>"; rmin = avgdelay; rmax = 255; }

            newbhf.innerHTML = "<td style='padding-left:1em;line-height:.9em;border:0'>"+bhf+"</td><td style='padding-left:1em;line-height:.8em;border:0;max-width:200px'>"+prog+"</td>";
            newtable.appendChild(newbhf);

            json_arr[i] = {};
            json_arr[i]["zugnr"] = zugnr;
            json_arr[i]["datum"] = datum;
            json_arr[i]["bhf"] = bhf;
            json_arr[i]["von"] = rmin;
            json_arr[i]["bis"] = rmax;

        }
    }
    var json_string = JSON.stringify(json_arr);
    var xhr = new XMLHttpRequest();
    xhr.open("POST", "https://www.zugfinder.net/js/post_prognose.php", true);
    xhr.setRequestHeader("Content-Type", "application/json; charset=UTF-8");
    xhr.send(json_string);
}
load_zuginfo(761, "")
