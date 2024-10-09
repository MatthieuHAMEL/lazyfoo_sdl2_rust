Sprite management was always a big mess in my first projects (coordinates hardcoded directly in the CPP, map of enum values to file names also hardcoded, global objects to store textures, loading images anywhere in the middle of the mainloop, etc.)

Here, with project scaling-up still in mind, I'm going beyond the spirit of the tutorial and proposing a very simple json format for describing sprites, with suitable data structures on the code side. I use the **serde** crate to deserialize the json directly into my structs. 
