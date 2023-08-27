# geoconverter
A simple CLI tool for fast CRS transformation of geographic coordinates.

<p align="center">
  <img src="docs/terminal.gif" alt="Tool preview">
</p>

## Quick examples
- Transform coordinates passed directly as a CLI argument from EPSG:4326 to EPSG:3857.
```sh
./geoconverter -i 4326 -o 3857 -c "[[21.00017724164357, 52.22746422475734], [20.99920222484198, 52.22725414500391]]"
```
- Transform file coordinates from EPSG:4326 to EPSG:3857.
```sh
./geoconverter -i 4326 -o 3857 -f line.json
```
As a result you get a prettiefied JSON with transformed coordinates:
```
[
  [
    [
      5813934.7229239,
      2391899.722129077
    ]
  ],
  [
    [
      5813911.336952722,
      2391783.462034956
    ]
  ]
]
```
