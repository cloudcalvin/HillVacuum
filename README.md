[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/IvoryDuke/HillVacuum#license)

## What is HillVacuum?
HillVacuum is a Doom Builder and TrenchBroom inspired editor that allows the creation of bidimensional maps through the manipulation of convex polygons, placement of items and entities, and texture mapping.

## Why is HillVacuum?
- I wanted to learn Rust;
- I wanted to create my editor;
- I got tired of manually typing the coordinates of the collision polygons in a text file.

## How is HillVacuum?
For those who do not mind reading, HV features a built in manual which can be opened through the keyboard key `~`.  
For the rest, there's this [video](https://youtu.be/c5lakP_V1n0).

## Keywords
### Brushes
Brushes are convex polygonal surfaces. They can have an associated texture which can either be drawn filling their area or as a sprite. The sprite can be displaced independently of the brush surface.  
Brushes can also be assigned a path that describes how it moves in the bidimensional space and that can be edited with the Path tool.  
Finally, brushes have a built-in property, `collision`, which determines whether they should represent a clipping surface or not. It can be edited in the properties window.

### Things
Things are objects which can be placed around the map. They area characterized by an ID, a width and height, a name, and a texture which represents them.  
Things can also be assigned a path that describes how it moves in the bidimensional space and that can be edited with the Path tool.  
Things can either be defined in one or many .ini files to be placed in the `assets/things/` folder or, if HillVacuum is used as a library, implementing the `MapThing` interface for the structs representing an object to be placed in the map and using the `hardcoded_things` macro to insert them in the bevy App.  
If defined in the .ini files, the things must follow a similar format:
```ini
[Name]
width = N
height = M
id = ID
preview = TEX
```
Where ID is an unique identifier between 0 and 65534, and TEX is the name of the texture to be drawn along with the bounding box.  
If a thing defined through the MapThing interface has the same ID as one loaded from file, the latter will overwrite the former.   
Finally, things have two built-in properties, `angle` and `draw height`. The orientation of the arrow drawn on top of the things will change based on the value of `angle`, and `draw height` determines its draw order. They can be edited in the properties window.
     
Things can be reloaded while the application is running through the UI button in the Options menu.  

### Properties
Properties are custom user defined values which can be associated to brushes and things.   
Such values can be inserted through the `brush_properties` and `thing_properties` macros by specifying the pairs `(name, default_value)` of the properties.   
Properties can be edited per-entity using the properties window.   
Currently supported value types are `bool`, `u8`, `u16`, `u32`, `u64`, `u128`, `i8`, `i16`, `i32`, `i64`, `i128`, `f32`, `f64`, and `String`.   
   
!!! If a saved map contains properties that differ in type and/or name from the ones defined in the aforementioned resources, a warning window will appear on screen when trying to load the .hv file, asking whether you'd like to use the app or map ones.   

### Textures
Textures must be placed in the `assets/textures/` folder to be loaded.  
The texture editor can be opened at any time to edit the properties of the textures of the selected brushes.  
Entity, scale, and rotate tool also feature texture editing capabilities. These capabilities can be either enabled through the dedicated "Target" UI element in the bottom left area, or by pressing Alt + texture editor bind.  
Textures can have an associated animation which can either consist of a list of textures to display, each one for a specific time, or an atlas of textures generated by subdividing the textures in subareas. The animations can be applied to the texture as a default or to the texture of the selected brushes only.  
When editing a list type animation, it is possible to add a texture by clicking it with the left mouse button.   
To edit the animation of a texture that is not the one of the selected brushes, it needs to be pressed with the right mouse button.   
   
Textures can be reloaded while the application is running through the UI button in the Options menu.  
Default textures animation can be exported and imported between map files. The file extension of the animations files is .anms.

### Props
A prop is a collection of entities which can be painted around the map like the brushes of an image editing tool.  
Each prop has a pivot, the point relative to which the it is painted onto the map.  
Props can be imported and exported between map files. The file extension of the props files is .prps.

## Files
HV creates three types of files, all of which are relatively simple:
- .hv is the regular map file;
```
------------------------------
| Header (4 usize)           |
| brushes amount             |
| things amount              |
| animations amount          |
| props amount               |
------------------------------
| Brushes default properties |
------------------------------
| Things default properties  |
------------------------------
| Animations                 |
------------------------------
| Brushes                    |
------------------------------
| Things                     |
------------------------------
| Props                      |
------------------------------
```
- .anms is the "animations only" file, which can be used to exchange animations between maps;
```
------------------------------
| animations amount (usize)  |
------------------------------
| Animations                 |
------------------------------
```
- .prps is the "props only" file, which can be used to exchange props between maps.
```
------------------------------
| props amount (usize)       |
------------------------------
| Props                      |
------------------------------
```

## Getting started
HV can be compiled as a standalone executable simply compiling the source code (Linux distributions may require the installation of extra libraries).
```sh
cargo run
```

Otherwise it can be integrated in your own project as such:
```rust
fn main()
{
    bevy::prelude::App::new()
        .add_plugins(hill_vacuum::HillVacuumPlugin)
        .run();
}
```

Map files can be read through the Exporter struct that will return lists of all the brushes and things, which can then be exported as desired.
Assuming the path of the map file was passed as an argument to the exporting executable the code will look something like this:
```rust
fn main()
{
    let exporter = hill_vacuum::Exporter::new(&std::env::args().collect::<Vec<_>>()[0]);
    // Your code.
}
```
The map being edited can be exported through such an executable through the File->Export command in the editor.
The executable can be picked through Options->Exporter.

## Features
- `debug`: enables dynamic linking for faster compile times and some debug lines of the editor;
- `arena_alloc`: enables the usage of an arena allocator for fast allocation times. Requires nightly compiler.

## !! WARNING
[The only thing I know for real](https://youtu.be/T928kJvqTlo?si=2_YnB2pEuFSKKq-j), there will be bugs.  
HV has been thoroughly tested but is still in its early releases, so there might be issues that lead to crashes due to unrecoverable errors. It is strongly recommended to save often.

## Known issues
On Windows, the things and props gallery of the Thing and Paint tools are incorretly drawn. This does not occur on Linux.

## Misc
In order to close the in-editor windows through the keyboard the F4 key needs to be pressed (similar to pressing Alt+F4 to close OS windows).

## FAQ
### It's "vertices", not "vertexes"
First of all https://dictionary.cambridge.org/dictionary/english/vertexes.  
Finally, [if popular culture has taught us anything is that 'c's are not extreme](https://youtu.be/mols06iqcfA?si=ijCNCojPOR4rC8n1&t=336).

### Actually if you read the "Rust programming language" book it clearly says: "It’s good style to place the opening curly bracket on the same line as the function declaration, adding one space in between."
There's a lot of talk around the internet about respecting each other's differences, and rightfully so. So I would appreciate it if people could respect this difference of mine.

## To do
Complete code documentation.
