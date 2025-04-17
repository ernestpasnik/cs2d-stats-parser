# CS2D Stats Parser
This tool parses CS2D stats and exports them as **HTML**, **JSON**, **CSV**, **Markdown**, or **XML**.
Use the `-h` option to display all available arguments and options.

### Demo Output
- [HTML Example](https://htmlpreview.github.io/?https://github.com/ernestpasnik/cs2d-stats-parser/blob/main/example-output/demo.html)
- [Markdown Example](https://github.com/ernestpasnik/cs2d-stats-parser/blob/main/example-output/demo.md)
- [CSV Example](https://github.com/ernestpasnik/cs2d-stats-parser/blob/main/example-output/demo.csv)
- [XML Example](https://github.com/ernestpasnik/cs2d-stats-parser/blob/main/example-output/demo.xml)
- [JSON Example](https://github.com/ernestpasnik/cs2d-stats-parser/blob/main/example-output/demo.json)

## Usage
```
Usage: cs2d-stats-parser [OPTIONS] <folder> <output>

Arguments:
  <folder>  Path to the folder containing 'userstats.dat'
  <output>  Output file (must end with .html, .json, .csv, .md, or .xml)

Options:
  -s, --sort <sort>    Sort leaderboard:
                       0 = score+kills-deaths
                       1 = assists+kills-deaths
                       2 = score+assists+deaths [default: 1]
  -l, --limit <limit>  Limit players in the generated output [default: 100]
  -t, --title <title>  Title to display in the HTML/Markdown report [default: "CS2D Server"]
  -p, --pretty-print   Enable pretty-printing for JSON output to improve readability
  -w, --watch          Monitor 'userstats.dat' for changes and regenerate output when modified
  -h, --help           Print help
  -V, --version        Print version
```

### Examples
Generate a **JSON** file on **Windows**:
```bash
cs2d-stats-parser.exe "C:\Program Files (x86)\Steam\steamapps\common\CS2D\sys\stats" "report.json"
```
Generate a **HTML** file with a custom title on **Linux**:
```bash
./cs2d-stats-parser "/home/cs2d/sys/stats" "/var/www/html/dm.html" --title "Deathmatch Server"
```
