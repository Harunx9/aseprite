//! An importer for aseprite's image+json sprite animations.
//!
//! In aseprite, go to file->export sprite sheet,
//! choose to export json data, and select "array" rather than "hash"
//! because that makes sense.
//!
//! Tested with aseprite 1.1.6, as on Debian Stretch.

#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Rect {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Dimensions {
    w: u32,
    h: u32,
}


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Frame {
    filename: String,
    frame: Rect,
    rotated: bool,
    trimmed: bool,
    #[serde(rename = "spriteSourceSize")]
    sprite_source_size: Rect,
    #[serde(rename = "sourceSize")]
    source_size: Dimensions,
    duration: u32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    #[serde(rename="forward")]
    Forward,
    #[serde(rename="backward")]
    Backward,
    #[serde(rename="pingpong")]
    Pingpong
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Frametag {
    name: String,
    from: u32,
    to: u32,
    direction: Direction,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy)]
pub enum BlendMode {
    #[serde(rename="normal")]
    Normal,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Layer {
    name: String,
    opacity: u32,
    #[serde(rename = "blendMode")]
    blend_mode: BlendMode,
}


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Metadata {
    app: String,
    version: String,
    format: String,
    size: Dimensions,
    scale: String, // Surely this should be a number?
    #[serde(rename = "frameTags")]
    frame_tags: Option<Vec<Frametag>>,
    layers: Option<Vec<Layer>>,
}


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct SpritesheetData {
    frames: Vec<Frame>,
    meta: Metadata,
}


#[cfg(test)]
mod tests {
    extern crate serde_json;

    const S: &'static str = r##"{ "frames": [
   {
    "filename": "boonga 0.ase",
    "frame": { "x": 1, "y": 1, "w": 18, "h": 18 },
    "rotated": false,
    "trimmed": false,
    "spriteSourceSize": { "x": 0, "y": 0, "w": 16, "h": 16 },
    "sourceSize": { "w": 16, "h": 16 },
    "duration": 250
   },
   {
    "filename": "boonga 1.ase",
    "frame": { "x": 20, "y": 1, "w": 18, "h": 18 },
    "rotated": false,
    "trimmed": false,
    "spriteSourceSize": { "x": 0, "y": 0, "w": 16, "h": 16 },
    "sourceSize": { "w": 16, "h": 16 },
    "duration": 250
   }
 ],
 "meta": {
  "app": "http://www.aseprite.org/",
  "version": "1.1.6-dev",
  "image": "boonga.png",
  "format": "RGBA8888",
  "size": { "w": 39, "h": 20 },
  "scale": "1",
  "frameTags": [
   { "name": "testtag", "from": 0, "to": 1, "direction": "forward" }
  ],
  "layers": [
   { "name": "Layer 1", "opacity": 255, "blendMode": "normal" }
  ]
 }
}
"##;

    
    const S_NO_META: &'static str = r##"{ "frames": [
   {
    "filename": "boonga 0.ase",
    "frame": { "x": 1, "y": 1, "w": 18, "h": 18 },
    "rotated": false,
    "trimmed": false,
    "spriteSourceSize": { "x": 0, "y": 0, "w": 16, "h": 16 },
    "sourceSize": { "w": 16, "h": 16 },
    "duration": 250
   },
   {
    "filename": "boonga 1.ase",
    "frame": { "x": 20, "y": 1, "w": 18, "h": 18 },
    "rotated": false,
    "trimmed": false,
    "spriteSourceSize": { "x": 0, "y": 0, "w": 16, "h": 16 },
    "sourceSize": { "w": 16, "h": 16 },
    "duration": 250
   }
 ],
 "meta": {
  "app": "http://www.aseprite.org/",
  "version": "1.1.6-dev",
  "image": "boonga.png",
  "format": "RGBA8888",
  "size": { "w": 39, "h": 20 },
  "scale": "1"
 }
}
"##;

    
    #[test]
    fn test_sprite_load_save() {
        let deserialized: super::SpritesheetData = serde_json::from_str(S).unwrap();

        let serialized = serde_json::to_string(&deserialized).unwrap();
        let deserialized_again: super::SpritesheetData = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized, deserialized_again);
    }

    
    #[test]
    fn test_less_metadata() {
        let deserialized: super::SpritesheetData = serde_json::from_str(S_NO_META).unwrap();

        let serialized = serde_json::to_string(&deserialized).unwrap();
        let deserialized_again: super::SpritesheetData = serde_json::from_str(&serialized).unwrap();

        assert_eq!(deserialized, deserialized_again);
    }

}
