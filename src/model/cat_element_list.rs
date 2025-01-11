
use std::{collections::HashMap, vec::IntoIter};

use infograph::types::{NameOrPoint, Point};

use crate::model::cat_element::CatElement;

#[derive(Clone,Debug)]
pub struct CatElementList {
    cat_elements: Vec<CatElement>,
}

impl IntoIterator for CatElementList {
    type Item = CatElement;
    type IntoIter = IntoIter<CatElement>;

    fn into_iter(self) -> IntoIter<CatElement> {
    
        let mut vec: Vec<CatElement> = self.cat_elements.to_vec();
        vec.sort();
        vec.into_iter()
    }
}

impl Default for CatElementList {
   fn default() -> Self {
        Self::new()
    } 
}

impl CatElementList {

    pub fn new() -> Self{
        CatElementList {cat_elements: Vec::new()}
    }

    ///
    /// Returns a list of InfoEdge objects for a given cat graph_name referring to the "is category" tag.
    /// This means triples and their descriptor notes.
    ///
    pub fn get_all_cat_element_names(&self) -> HashMap<Point, NameOrPoint> {
        let mut result: HashMap<Point, NameOrPoint> = HashMap::new();
        self.cat_elements
            .iter()
            .for_each(|cat_element|{
                if cat_element.name.is_empty() {
                    result.insert(cat_element.point.clone(), cat_element.point.clone());
                } else {
                    result.insert(cat_element.point.clone(), cat_element.name.clone());
                }  
            });

        result
    }

    pub fn get_point_ids(&self) -> Vec<String> {
        self.cat_elements
            .iter()
            .map(|ce|ce.point.clone())
            .collect()
    } 

    pub fn add(&mut self, cat_element: CatElement) {
        self.cat_elements.push(cat_element);
    }

}

#[test]
fn get_all_cat_element_names_test() {

    let ce1 = CatElement {
     element_id: "3".to_string(),
     point: "a".to_string(),
     cat_id: "cat1".to_string(),
     desc_id: "d1".to_string(),
     name: "name1".to_string(),
     label: "label1".to_string(),
     description: "button no. 1".to_string(),
    };
    let ce2 = CatElement {
     element_id: "6".to_string(),
     point: "b".to_string(),
     cat_id: "cat2".to_string(),
     desc_id: "d2".to_string(),
     name: "".to_string(),
     label: "label2".to_string(),
     description: "button no. 2".to_string(),
    };
    let ce3 = CatElement {
     element_id: "9".to_string(),
     point: "c".to_string(),
     cat_id: "cat2".to_string(),
     desc_id: "d3".to_string(),
     name: "name3".to_string(),
     label: "label3".to_string(),
     description: "button no. 3".to_string(),
    };

    let mut cat_elements: CatElementList = CatElementList::new();
    cat_elements.cat_elements.push(ce1);
    cat_elements.cat_elements.push(ce2);
    cat_elements.cat_elements.push(ce3);

    let it = cat_elements.get_all_cat_element_names();
    let mut it: Vec<(String,String)> = it.iter()
        .map(|(k,v)|(k.to_string(),v.to_string()))
        .collect();
    it.sort();
    let mut it = it.iter();
    
    assert_eq!(it.next().unwrap(), &("a".to_string() ,"name1".to_string()));
    assert_eq!(it.next().unwrap(), &("b".to_string() ,"b".to_string()));
    assert_eq!(it.next().unwrap(), &("c".to_string() ,"name3".to_string()));
    assert!(it.next().is_none());
}

