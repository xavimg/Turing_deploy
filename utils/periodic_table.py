import json

import requests
from bs4 import BeautifulSoup

data = requests.get('https://raw.githubusercontent.com/Bowserinator/Periodic-Table-JSON/master/PeriodicTableJSON.json')
data = data.json()["elements"]

response = requests.get('https://periodictable.com/Properties/A/UniverseAbundance.an.log.html')
html = response.content.decode()
soup = BeautifulSoup(html, 'html.parser')
elements = soup.find('table', { 'width': 748 }).find_all("tr")

for element in elements:
    names = element.find_all("td", { "align": "right" })
    pcts = element.find_all("td", { "align": "left" })

    for i in range(0, len(names)):
        name = names[i].text
        pct = pcts[i]

        for pt in data:
            if pt['name'] == name:
                value = str(pct.text[:-1])
                if value == "N/":
                    data.remove(pt)
                    break

                value = value.split("×", 1)
                if len(value) == 1:
                    value = float(value[0])
                else:
                    value = float(value[0]) / pow(10, int(value[1][3:]))

                pt["abundance"] = value
                print(name, value)
                break

for pt in data:
    if "abundance" not in pt:
        data.remove(pt)

json.dump({ "elements": data }, open("utils/periodic_table.json", "w"))
print("\nDone!")