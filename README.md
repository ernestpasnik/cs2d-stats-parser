# CS2D Stats Parser
This tool processes the `userstats.dat` file from your **CS2D** server to generate outputs in **HTML**, **JSON**, or **CSV** format. Use the `-h` option to display all available arguments and options.

## Example Usage
Generate a **JSON** file on **Windows**:
```bash
cs2d-stats-parser.exe "C:\Program Files (x86)\Steam\steamapps\common\CS2D\sys\stats" "report.json"
```
Generate a **HTML** file with a custom title on **Linux**:
```bash
./cs2d-stats-parser "/home/cs2d/sys/stats" "/var/www/html/deathmatch.html" --title "Deathmatch Server"
```

## Example Output
![Output](https://i.imgur.com/fOKdcgq.gif)
