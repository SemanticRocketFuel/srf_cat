use std::cmp::Ordering;

use infograph::types::Point;

///
/// This struct represents a category.
/// The idea is that the library users can create their own categories for tagging stuff.
/// Behind the scenes a category is modeled using one descriptor note and one info triple.
///
#[derive(Clone, Debug, Eq)]
pub struct Cat {

    ///
    /// This ID is to be used to represent the category for further specifications. For example in case of cancellation.
    /// In infograph terms this ID is used for decorations.
    ///
    pub triple_id: String,
    
    ///
    /// This ID is referenced in the descriptor note (as point) and in the category triple.
    /// References aside it is nothing, but an ID.
    ///
    pub point: Point,
    
    ///
    /// This is the common ID across all categories of a domain specific type. 
    /// For example if the category were to represent a button then you would have a type_id for a
    /// button.
    /// It is included here for convenience in case info triple work is needed.
    /// On the infograph side its the id being paired with the point.
    ///
    pub type_id: String,
    
    ///
    /// This is the ID of the Descriptor Note itself and will probably not be used directly.
    /// The field is added here for future extensions. 
    ///
    pub desc_id: String,
    
    ///
    /// The name of the category and the name field in its descriptor note.
    ///
    pub name: String,
    
    ///
    /// Alternative to name. This could be the path of an icon associated with this category. 
    /// I could also be an emoji.
    /// This is the label field in the descriptor note.
    ///
    pub icon_uri: String,
    
    ///
    /// A description of the purpose of this specific category. It uses the description field of the
    /// descriptor note.
    ///
    pub description: String,

    ///
    /// Only show the category for elements already tagged with an element from this list.
    ///
    pub conditioned_list: Vec<String>,
}

impl Default for Cat {

    fn default() -> Self {
        Self::new()
    }
}

impl Cat {
    pub fn new() -> Self {
        Cat { 
                triple_id: "".to_string(), 
                point: "".to_string(), 
                type_id: "".to_string(), 
                desc_id: "".to_string(), 
                name: "".to_string(), 
                icon_uri: "".to_string(), 
                description: "".to_string(), 
                conditioned_list: Vec::new(),
        }
    }

    pub fn has_conditions(&self) -> bool {
        !self.conditioned_list.is_empty()
    }
}

impl Ord for Cat {
    fn cmp(&self, other: &Self) -> Ordering {
       self.point.cmp(&other.point) 
    }
}

impl PartialOrd for Cat {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.point.cmp(&other.point))
    }
}

impl PartialEq for Cat {
    fn eq(&self, other: &Self) -> bool {
        self.point == other.point
    }
}
