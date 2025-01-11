use std::cmp::Ordering;
use std::{error::Error, fmt};

use infograph::types::Point; 
use infograph::{InfoTriple, Descriptor};

#[derive(Clone, Debug, Eq)]
pub struct CatElement {
    ///
    /// The ID of the category this element has been tagged as.
    ///
    pub cat_id: Point,

    ///
    /// The unique ID of this categorized element. 
    /// Behind the scenes it's the triple ID of the triple representing this cat element.
    ///
    pub element_id: Point,
    
    ///
    /// This identifies the object that has been categorized.
    ///
    pub point: Point,
    
    ///
    /// The name field in the Descriptor Note for this CatElement's point, if such Descriptor Note
    /// exists.
    ///
    pub name: String,
    
    ///
    /// The label field in the Descriptor Note for this CatElement's point, if such Descriptor Note
    /// exists.
    ///
    pub label: String,
    
    ///
    /// The description field in the Descriptor Note for this CatElement's point, if such Descriptor Note
    /// exists.
    ///
    pub description: String,
    
    ///
    /// This is the ID of the descriptor note, if such Descriptor Note exists.
    ///
    pub desc_id: String,

}

impl fmt::Display for CatElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(Id: {}, Cat type: {}, Tagged NoteId: {})", self.element_id, self.cat_id, self.point)
    }
}

impl Error for CatElement {}

impl CatElement {

    fn add_desc(&mut self, desc: Descriptor) {
        self.name = desc.name;
        self.label = desc.label;
        self.description = desc.description;
        self.desc_id = desc.desc_id;
    }

    pub fn from_triple(cat_id: String, value: InfoTriple) -> Result<CatElement, Box<dyn Error>> {
        let other_half = value.clone().other_half(cat_id.clone()); 
        if other_half.is_err() {
            return Err("Not a CatElement".to_string().into())
        } else {
            return Ok(CatElement { cat_id: cat_id.clone(), element_id: value.id.clone(), 
                point: other_half.unwrap(), 
                name: "".to_string(), 
                label: "".to_string(), 
                description: "".to_string(), 
                desc_id: "".to_string() 
            })    
        }
    }

}

//TODO: In the following methods point is compared, but it may be that element_id is the correct
// variable to use. Review in the future.
impl Ord for CatElement {
    fn cmp(&self, other: &Self) -> Ordering {
       self.point.cmp(&other.point)       
    }
}

impl PartialOrd for CatElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.point.cmp(&other.point))
    }
}

impl PartialEq for CatElement {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point
    }
}
