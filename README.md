# CS2D Stats Parser
This tool processes the `userstats.dat` file from your **CS2D** server to generate outputs in **HTML**, **JSON**, or **CSV** format. Use the `-h` option to display all available arguments and options.

![Output](https://i.imgur.com/fOKdcgq.gif)


## Usage
```
Usage: cs2d-stats-parser [OPTIONS] <folder> <output>

Arguments:
  <folder>  Path to the folder containing 'userstats.dat'
  <output>  Output file (filename with extension html, json, or csv)

Options:
  -s, --sort <sort>    Sort leaderboard:
                       0 = score+kills-deaths
                       1 = assists+kills-deaths
                       2 = score+assists+deaths [default: 1]
  -l, --limit <limit>  Limit players in the generated output [default: 100]
  -t, --title <title>  Title to display in the HTML report [default: "CS2D Server"]
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
