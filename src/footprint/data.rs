// (c) 2016-2017 Productize SPRL <joost@productize.be>

use {Adjust, Bound, BoundingBox};
use symbolic_expressions::iteratom::SResult;

/// a Kicad module, with a name and a list of elements
#[derive(Debug,Clone)]
pub struct Module {
    /// name of the Kicad Module
    pub name: String,
    /// elements contained within the Kicad Module
    pub elements: Vec<Element>,
}

trait Named {
    fn name(&self) -> &'static str;
}

impl Module {
    /// create a Module
    pub fn new(name: String) -> Module {
        Module {
            name: name,
            elements: vec![],
        }
    }
    /// append an Element to a Module
    pub fn append(&mut self, e: Element) {
        self.elements.push(e)
    }
    /// check if a Module has a reference Element with the specified name
    pub fn is_reference_with_name(&self, reference: &str) -> bool {
        for element in &self.elements[..] {
            if let Element::FpText(ref fp_text) = *element {
                if fp_text.name == "reference" && fp_text.value == *reference {
                    return true;
                }
            }
        }
        false
    }

    /// check if a Module has a reference Element with the specified name
    pub fn get_reference(&self) -> Option<&String> {
        for element in &self.elements[..] {
            if let Element::FpText(ref fp_text) = *element {
                if fp_text.name == "reference" {
                    return Some(&fp_text.value);
                }
            }
        }
        None
    }

    /// update the name of the reference element specified by name, if found
    pub fn set_reference(&mut self, reference: &str, reference2: &str) {
        // println!("debug: searching '{}'", reference);
        for ref mut element in &mut self.elements[..] {
            if let Element::FpText(ref mut fp_text) = **element {
                if fp_text.name == "reference" && fp_text.value == *reference {
                    fp_text.value.clear();
                    fp_text.value.push_str(reference2);
                }
            }
        }
    }
    /// check if there is an At element and return the coordinates found
    /// returns the default of (0.0,0.0) if not found
    pub fn at(&self) -> (f64, f64) {
        for element in &self.elements[..] {
            if let Element::At(ref at) = *element {
                return (at.x, at.y);
            }
        }
        (0.0, 0.0)
    }

    /// adjust the At element contained in the module
    pub fn adjust_at(&mut self, x: f64, y: f64) {
        for element in &mut self.elements[..] {
            if let Element::At(ref mut at) = *element {
                at.x += x;
                at.y += y;
            }
        }
    }

    /// check if the Module is on the front layer
    pub fn is_front(&self) -> bool {
        for element in &self.elements[..] {
            if let Element::Layer(ref layer) = *element {
                return &layer[..] == "F.Cu";
            }
        }
        true
    }

    /// rename a net
    pub fn rename_net(&mut self, old_name: &str, new_name: &str) {
        for element in &mut self.elements[..] {
            if let Element::Pad(ref mut pad) = *element {
                pad.rename_net(old_name, new_name)
            }
        }
    }
}

impl BoundingBox for Module {
    fn bounding_box(&self) -> Bound {
        let (x, y) = self.at();
        let mut b = Bound::new(x, y, x, y);
        for element in &self.elements {
            let mut b2 = element.bounding_box();
            b2.x1 += x;
            b2.y1 += y;
            b2.x2 += x;
            b2.y2 += y;
            // trace!("{}: {:?}", element.name(), b2);
            b.update(&b2);
        }
        b.swap_if_needed();
        // trace!("Module {} bb: {:?}", self.name, b);
        b
    }
}

impl Adjust for Module {
    fn adjust(&mut self, x: f64, y: f64) {
        self.adjust_at(x, y)
    }
}

/// elements that can be found in a Module
#[derive(Debug,Clone)]
pub enum Element {
    /// solder mask margin
    SolderMaskMargin(f64),
    /// layer name
    Layer(String), // TODO: use Layer type
    /// description
    Descr(String),
    /// Tags element
    Tags(String),
    /// Attr element
    Attr(String),
    /// text
    FpText(FpText),
    /// pad
    Pad(Pad),
    /// polygon
    FpPoly(FpPoly),
    /// line
    FpLine(FpLine),
    /// circle
    FpCircle(FpCircle),
    /// arc
    FpArc(FpArc),
    /// edited time stamp
    TEdit(String),
    /// time stamp
    TStamp(String),
    /// Path element
    Path(String),
    /// location of module in layout
    At(At),
    /// 3D model information
    Model(Model),
    /// Clearance override for module
    Clearance(f64),
    /// is the module locked
    Locked,
}

impl BoundingBox for Element {
    fn bounding_box(&self) -> Bound {
        let bb = match *self {
            Element::Pad(ref x) => x.bounding_box(),
            Element::FpPoly(ref x) => x.bounding_box(),
            Element::FpLine(ref x) => x.bounding_box(),
            Element::FpCircle(ref x) => x.bounding_box(),
            Element::FpText(ref x) => x.bounding_box(),
            Element::FpArc(ref x) => x.bounding_box(),
            Element::At(_) |
            Element::Layer(_) |
            Element::TEdit(_) |
            Element::Descr(_) |
            Element::Path(_) |
            Element::Model(_) |
            Element::Attr(_) |
            Element::SolderMaskMargin(_) |
            Element::Clearance(_) |
            Element::Tags(_) |
            Element::Locked |
            Element::TStamp(_) => Bound::default(),
        };
        bb
    }
}

impl Named for Element {
    fn name(&self) -> &'static str {
        match *self {
            Element::Pad(_) => "Pad",
            Element::FpPoly(_) => "FpPoly",
            Element::FpLine(_) => "FpLine",
            Element::FpCircle(_) => "FpCircle",
            Element::FpArc(_) => "FpArc",
            Element::FpText(_) => "FpText",
            Element::At(_) => "At",
            Element::Layer(_) => "Layer",
            Element::TEdit(_) => "TEdit",
            Element::Descr(_) => "Descr",
            Element::Path(_) => "Path",
            Element::Model(_) => "Model",
            Element::TStamp(_) => "Tstamp",
            Element::SolderMaskMargin(_) => "SolderMaskMargin",
            Element::Clearance(_) |
            Element::Tags(_) => "Tags",
            Element::Attr(_) => "Attr",
            Element::Locked => "Locked",
        }
    }
}

/// text element
#[derive(Debug,Clone)]
pub struct FpText {
    /// name
    pub name: String,
    /// text
    pub value: String,
    /// location
    pub at: At,
    /// layer
    pub layer: Layer,
    /// text effects
    pub effects: Effects,
    /// is it a hidden text
    pub hide: bool,
}

impl BoundingBox for FpText {
    fn bounding_box(&self) -> Bound {
        let (x, y) = (self.at.x, self.at.y);
        debug!("bound for FpText is poor");
        Bound::new(x, y, x, y)
    }
}

impl FpText {
    /// create a text with given name and value
    pub fn new(name: String, value: String) -> FpText {
        FpText {
            name: name,
            value: value,
            at: At::default(),
            layer: Layer::default(),
            effects: Effects::default(),
            hide: false,
        }
    }
    /// set the text effects of the text
    pub fn set_effects(&mut self, effects: &Effects) {
        self.effects.clone_from(effects)
    }
    /// set the layer of the text
    pub fn set_layer(&mut self, layer: &Layer) {
        self.layer.clone_from(layer)
    }
}

/// a location and rotation in a layout
#[derive(Debug,Clone,Default)]
pub struct At {
    /// x coordinate
    pub x: f64,
    /// y coordinate
    pub y: f64,
    /// rotation
    pub rot: f64,
}

impl Adjust for At {
    fn adjust(&mut self, x: f64, y: f64) {
        self.x += x;
        self.y += y
    }
}

impl At {
    /// create a location
    pub fn new(x: f64, y: f64, rot: f64) -> At {
        At {
            x: x,
            y: y,
            rot: rot,
        }
    }
}

/// font attributes for text
#[derive(Debug,Clone,Default)]
pub struct Font {
    /// size of the font
    pub size: Xy,
    /// thickness of the font
    pub thickness: f64,
    /// if it is italic
    pub italic: bool,
}

/// text effects
#[derive(Debug,Clone,Default)]
pub struct Effects {
    /// the font used
    pub font: Font,
    /// the text justification
    pub justify: Option<Justify>,
}

impl Effects {
    /// create a text effects element from font and justification
    pub fn from_font(font: Font, justify: Option<Justify>) -> Effects {
        Effects {
            font: font,
            justify: justify,
        }
    }
}

/// text justification
#[derive(Debug,Clone)]
pub enum Justify {
    /// the text is mirrored
    Mirror,
    /// the text is left-justified
    Left,
    /// the text is right-justified
    Right,
}

/// the type of X-Y element
#[derive(Debug,Clone,PartialEq)]
pub enum XyType {
    /// regular
    Xy,
    /// starting point
    Start,
    /// ending point
    End,
    /// size
    Size,
    /// center point
    Center,
    /// rectangular delta
    RectDelta,
}

impl Default for XyType {
    fn default() -> XyType {
        XyType::Xy
    }
}

/// X-Y element
#[derive(Debug,Clone,Default)]
pub struct Xy {
    /// x coordinate
    pub x: f64,
    /// y coorginate
    pub y: f64,
    /// the type of X-Y
    pub t: XyType,
}

impl Adjust for Xy {
    fn adjust(&mut self, x: f64, y: f64) {
        self.x += x;
        self.y += y
    }
}

impl Xy {
    /// create a new X-Y coordinate
    pub fn new(x: f64, y: f64, t: XyType) -> Xy {
        Xy { x: x, y: y, t: t }
    }
    /// create a new default X-Y coordinate of a certain type
    pub fn new_empty(t: XyType) -> Xy {
        Xy {
            x: 0.0,
            y: 0.0,
            t: t,
        }
    }
}

/// a list of X-Y coordinates
#[derive(Debug,Clone,Default)]
pub struct Pts {
    /// the list of X-Y coordinates
    pub elements: Vec<Xy>,
}

impl Adjust for Pts {
    fn adjust(&mut self, x: f64, y: f64) {
        for e in &mut self.elements {
            e.adjust(x, y)
        }
    }
}

impl BoundingBox for Pts {
    fn bounding_box(&self) -> Bound {
        let mut b = Bound::default();
        for e in &self.elements {
            let b2 = Bound::new(e.x, e.y, e.x, e.y);
            b.update(&b2);
        }
        b.swap_if_needed();
        b
    }
}

/// a drill
#[derive(Clone,Debug,Default)]
pub struct Drill {
    /// shape of the drill
    pub shape: Option<String>,
    /// width of the drill
    pub width: f64,
    /// height of the drill
    pub height: f64,
    /// x-offset of the drill
    pub offset_x: f64,
    /// y-offset of the drill
    pub offset_y: f64,
}

/// type of a Pad
#[derive(Debug,Clone)]
pub enum PadType {
    /// surface mount
    Smd,
    /// through-hole
    Pth,
    /// non-plated through-hole
    NpPth,
}

impl PadType {
    /// convert a &str to a pad type
    pub fn from_string(s: &str) -> SResult<PadType> {
        match s {
            "smd" => Ok(PadType::Smd),
            "thru_hole" => Ok(PadType::Pth),
            "np_thru_hole" => Ok(PadType::NpPth),
            x => Err(format!("unknown PadType {}", x).into()),
        }
    }
}

/// shape of a pad
#[derive(Debug,Clone)]
pub enum PadShape {
    /// rectangular
    Rect,
    /// circular
    Circle,
    /// oval
    Oval,
    /// trapezoid
    Trapezoid, // TODO
}

impl PadShape {
    /// convert a &str to a pad shape
    pub fn from_string(s: &str) -> SResult<PadShape> {
        match s {
            "rect" => Ok(PadShape::Rect),
            "circle" => Ok(PadShape::Circle),
            "oval" => Ok(PadShape::Oval),
            "trapezoid" => Ok(PadShape::Trapezoid),
            x => Err(format!("unknown PadShape: {}", x).into()),
        }
    }
}

/// side of a layer
#[derive(Debug,Clone)]
pub enum LayerSide {
    /// front side
    Front,
    /// back side
    Back,
    /// Dwgs side
    Dwgs,
    /// Cmts side
    Cmts,
    /// Eco1 side
    Eco1,
    /// Eco2 side
    Eco2,
    /// edge of the board
    Edge,
    /// both sides
    Both,
    /// Inner layer 1
    In1,
    /// Inner layer 2
    In2,
    /// no side
    None,
}

impl Default for LayerSide {
    fn default() -> LayerSide {
        LayerSide::Front
    }
}

/// type of a layer
#[derive(Debug,Clone)]
pub enum LayerType {
    /// copper layer
    Cu,
    /// paste layer
    Paste,
    /// solder mask layer
    Mask,
    /// silk screen layer
    SilkS,
    /// user layer
    User,
    /// adhesive layer
    Adhes,
    /// cuts layer
    Cuts,
    /// CrtYd layer
    CrtYd,
    /// fabrication layer
    Fab,
    /// margin layer
    Margin,
    /// an other custom named layer
    Other(String),
}

impl Default for LayerType {
    fn default() -> LayerType {
        LayerType::Cu
    }
}

/// a pcb layer, with a side and a type
#[derive(Debug,Clone,Default)]
pub struct Layer {
    /// side of the layer
    pub side: LayerSide,
    /// type of the layer
    pub t: LayerType,
}

impl Layer {
    /// create a layer from a String
    pub fn from_string(s: &str) -> SResult<Layer> {
        let sp: Vec<&str> = s.split('.').collect();
        let mut side = LayerSide::None;
        let s_t = if sp.len() == 2 {
            side = match sp[0] {
                "F" => LayerSide::Front,
                "B" => LayerSide::Back,
                "Dwgs" => LayerSide::Dwgs,
                "Cmts" => LayerSide::Cmts,
                "Eco1" => LayerSide::Eco1,
                "Eco2" => LayerSide::Eco2,
                "Edge" => LayerSide::Edge,
                "In1" => LayerSide::In1,
                "In2" => LayerSide::In2,
                "*" => LayerSide::Both,
                x => return Err(format!("unknown layer side {}", x).into()),
            };
            sp[1]
        } else {
            sp[0]
        };

        let t = match s_t {
            "Cu" => LayerType::Cu,
            "Paste" => LayerType::Paste,
            "Mask" => LayerType::Mask,
            "SilkS" => LayerType::SilkS,
            "User" => LayerType::User,
            "Adhes" => LayerType::Adhes,
            "Cuts" => LayerType::Cuts,
            "CrtYd" => LayerType::CrtYd,
            "Fab" => LayerType::Fab,
            "Margin" => LayerType::Margin,
            x => LayerType::Other(String::from(x)),
        };
        Ok(Layer { side: side, t: t })
    }
}

/// a list of layers
#[derive(Debug,Clone,Default)]
pub struct Layers {
    /// a list of layers
    pub layers: Vec<Layer>,
}

impl Layers {
    /// append a layer to a list of layers
    pub fn append(&mut self, layer: Layer) {
        self.layers.push(layer)
    }
}

/// a pad
#[derive(Debug,Clone)]
pub struct Pad {
    /// name
    pub name: String,
    /// type
    pub t: PadType,
    /// shape
    pub shape: PadShape,
    /// size
    pub size: Xy,
    /// offset
    pub rect_delta: Option<Xy>,
    /// location
    pub at: At,
    /// layers
    pub layers: Layers,
    /// associated net
    pub net: Option<Net>,
    /// drill
    pub drill: Option<Drill>,
    /// solder paste margin
    pub solder_paste_margin: Option<f64>,
    /// solder mask margin
    pub solder_mask_margin: Option<f64>,
    /// clearance
    pub clearance: Option<f64>,
    /// thermal gap
    pub thermal_gap: Option<f64>,
}

impl Pad {
    /// create a pad with a name, type and shape
    pub fn new(name: String, t: PadType, shape: PadShape) -> Pad {
        Pad {
            name: name,
            t: t,
            shape: shape,
            size: Xy::new_empty(XyType::Size),
            rect_delta: None,
            at: At::default(),
            layers: Layers::default(),
            net: None,
            drill: None,
            solder_paste_margin: None,
            solder_mask_margin: None,
            clearance: None,
            thermal_gap: None,
        }
    }

    /// rename the net of a pad
    pub fn rename_net(&mut self, old_name: &str, new_name: &str) {
        let new_net = if let Some(ref net) = self.net {
            if &net.name == old_name {
                Some(Net { name: new_name.to_string(), ..*net })
            } else {
                Some(net.clone())
            }
        } else {
            None
        };
        self.net = new_net
    }

    /// set the net of a pad
    pub fn set_net(&mut self, net: Net) {
        self.net = Some(net)
    }

    /// set the drill of a pad
    pub fn set_drill(&mut self, drill: Drill) {
        self.drill = Some(drill)
    }
}

impl BoundingBox for Pad {
    fn bounding_box(&self) -> Bound {
        let x = self.at.x;
        let y = self.at.y;
        let (dx, dy) = if self.at.rot < 0.1 {
            (self.size.x, self.size.y)
        } else {
            (self.size.y, self.size.x)
        };
        Bound::new(x - dx / 2.0, y - dy / 2.0, x + dx / 2.0, y + dy / 2.0)
    }
}

/// a polygon
#[derive(Debug,Clone,Default)]
pub struct FpPoly {
    /// points
    pub pts: Pts,
    /// width
    pub width: f64,
    /// layer
    pub layer: Layer,
}

impl BoundingBox for FpPoly {
    fn bounding_box(&self) -> Bound {
        let mut b = Bound::default();
        for p in &self.pts.elements {
            let b2 = Bound::new(p.x, p.y, p.x, p.y);
            b.update(&b2);
        }
        b.swap_if_needed();
        b
    }
}

/// a line
#[derive(Debug,Clone)]
pub struct FpLine {
    /// start point
    pub start: Xy,
    /// end point
    pub end: Xy,
    /// layer
    pub layer: Layer,
    /// width
    pub width: f64,
}

impl Default for FpLine {
    fn default() -> FpLine {
        FpLine {
            start: Xy::new_empty(XyType::Start),
            end: Xy::new_empty(XyType::End),
            layer: Layer::default(),
            width: 0.0,
        }
    }
}

impl BoundingBox for FpLine {
    fn bounding_box(&self) -> Bound {
        Bound::new(self.start.x, self.start.y, self.end.x, self.end.y)
    }
}

/// a circle
#[derive(Debug,Clone)]
pub struct FpCircle {
    /// center point
    pub center: Xy,
    /// end point
    pub end: Xy,
    /// layer
    pub layer: Layer,
    /// width
    pub width: f64,
}

impl Default for FpCircle {
    fn default() -> FpCircle {
        FpCircle {
            center: Xy::new_empty(XyType::Center),
            end: Xy::new_empty(XyType::End),
            layer: Layer::default(),
            width: 0.0,
        }
    }
}

impl BoundingBox for FpCircle {
    fn bounding_box(&self) -> Bound {
        let dx = self.center.x - self.end.x;
        let dy = self.center.y - self.end.y;
        let d2 = dx * dx + dy * dy;
        let d = d2.sqrt();
        Bound::new(self.center.x - d,
                   self.center.y - d,
                   self.center.x + d,
                   self.center.y + d)
    }
}

/// an arc
#[derive(Debug,Clone)]
pub struct FpArc {
    /// start point
    pub start: Xy,
    /// end point
    pub end: Xy,
    /// angle
    pub angle: f64,
    /// layer
    pub layer: Layer,
    /// width
    pub width: f64,
}

impl BoundingBox for FpArc {
    fn bounding_box(&self) -> Bound {
        // perhaps not correct
        Bound::new(self.start.x,
                   self.start.y,
                   self.end.x,
                   self.end.y)
    }
}

impl Default for FpArc {
    fn default() -> FpArc {
        FpArc {
            start: Xy::new_empty(XyType::Start),
            end: Xy::new_empty(XyType::End),
            angle: 0.0,
            layer: Layer::default(),
            width: 0.0,
        }
    }
}

/// a net
#[derive(Debug,Clone)]
pub struct Net {
    /// net number
    pub num: i64,
    /// net name
    pub name: String,
}

/// a 3D model
#[derive(Debug,Clone)]
pub struct Model {
    /// name
    pub name: String,
    /// location
    pub at: Xyz,
    /// scale
    pub scale: Xyz,
    /// rotation
    pub rotate: Xyz,
}

/// a 3D X-Y-Z coordinate
#[derive(Debug,Clone)]
pub struct Xyz {
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
    /// Z coordinate
    pub z: f64,
}

impl Xyz {
    /// create a X-Y-Z coordinate
    pub fn new(x: f64, y: f64, z: f64) -> Xyz {
        Xyz { x: x, y: y, z: z }
    }
}
