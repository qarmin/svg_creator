# SVG Creator
This is simple tool to create valid or unvalid svg files to test svg importers.

## How to use it?

It doesn't work out of box and needs to be configured and compiled.

First choose path to svg tool - variable `svg_tool`

Next change output filename - variable `file_name`(path must exists)

Tweak settings, you can change number of executed loops
- first loop - how much of svg files will be created
- second - how much of things like `<image>` or `<path>` will be inside files(this is random value inside some range)
- third - how much arguments things like `image` will have e.g. `<image d="4em" source="32">`

## License
MIT