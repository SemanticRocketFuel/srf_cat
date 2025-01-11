use std::{collections::HashMap, vec::IntoIter};

use infograph::types::{NameOrPoint, Point};

use crate::model::cat::Cat;

#[derive(Clone, Debug)]
pub struct CatList {
    pub cats: HashMap<Point,Cat>,
}

impl IntoIterator for CatList {
    type Item = Cat;
    type IntoIter = IntoIter<Cat>;

    fn into_iter(self) -> IntoIter<Cat> {
    
        let mut vec: Vec<Cat> = self.cats
            .values()
            .cloned()
            .collect();
        vec.sort();
        vec.into_iter()
    }
}

impl Default for CatList {
    fn default() -> Self {
        CatList::new()
    }
}

impl CatList {

    pub fn new() -> Self{
        CatList {cats: HashMap::new()}
    }

    pub fn is_empty(&self) -> bool {
        self.cats.is_empty()
    }

    pub fn get(&self, cat_id: String) -> Option<Cat> {
        self
            .cats
            .get(&cat_id)
            .cloned()
    }

    ///
    /// Returns a list of InfoEdge objects for a given cat graph_name referring to the "is category" tag.
    /// This means triples and their descriptor notes.
    ///
    pub fn get_all_cat_names(&self) -> HashMap<Point, NameOrPoint> {
        let mut result: HashMap<Point, NameOrPoint> = HashMap::new();
        self.cats
            .values()
            .for_each(|cat|{
                if cat.name.is_empty() {
                    result.insert(cat.point.clone(), cat.point.clone());
                } else {
                    result.insert(cat.point.clone(), cat.name.clone());
                }  
            });

        result
    }

    pub fn add(&mut self, cat: Cat) {
        self.cats.insert(cat.point.clone(), cat.clone());
    }
}

#[test]
fn get_all_cat_names_test() {

    let cat1 = Cat {
     triple_id: "3".to_string(),
     point: "a".to_string(),
     type_id: "cat".to_string(),
     desc_id: "d1".to_string(),
     name: "name1".to_string(),
     icon_uri: "uri1".to_string(),
     description: "button no. 1".to_string(),
     conditioned_list: Vec::new(),
    };
    let cat2 = Cat {
     triple_id: "6".to_string(),
     point: "b".to_string(),
     type_id: "cat".to_string(),
     desc_id: "d2".to_string(),
     name: "".to_string(),
     icon_uri: "uri2".to_string(),
     description: "button no. 2".to_string(),
     conditioned_list: Vec::new(),
    };
    let cat3 = Cat {
     triple_id: "9".to_string(),
     point: "c".to_string(),
     type_id: "cat".to_string(),
     desc_id: "d3".to_string(),
     name: "name3".to_string(),
     icon_uri: "uri3".to_string(),
     description: "button no. 3".to_string(),
     conditioned_list: Vec::new(),
    };

    let mut cats: CatList = CatList::new();
    cats.add(cat1);
    cats.add(cat2);
    cats.add(cat3);

    let it = cats.get_all_cat_names();
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

#[test]
fn into_iter_test() {
    let cat1 = Cat {
     triple_id: "3".to_string(),
     point: "a".to_string(),
     type_id: "cat".to_string(),
     desc_id: "d1".to_string(),
     name: "name1".to_string(),
     icon_uri: "uri1".to_string(),
     description: "button no. 1".to_string(),
     conditioned_list: Vec::new(),
    };
    let cat2 = Cat {
     triple_id: "6".to_string(),
     point: "b".to_string(),
     type_id: "cat".to_string(),
     desc_id: "d2".to_string(),
     name: "".to_string(),
     icon_uri: "uri2".to_string(),
     description: "button no. 2".to_string(),
     conditioned_list: Vec::new(),
    };
    let cat3 = Cat {
     triple_id: "9".to_string(),
     point: "c".to_string(),
     type_id: "cat".to_string(),
     desc_id: "d3".to_string(),
     name: "name3".to_string(),
     icon_uri: "uri3".to_string(),
     description: "button no. 3".to_string(),
     conditioned_list: Vec::new(),
    };

    let mut cats: CatList = CatList::new();
    cats.add(cat1.clone());
    cats.add(cat2.clone());
    cats.add(cat3.clone());
        
    for cat in cats.clone(){
        println!("this is just to prove the for loop works cat:{:?}", cat);
    }    
    
    let mut iter = cats.clone().into_iter();
    assert_eq!(cat1.point,iter.next().unwrap().point);
    assert_eq!(cat2.point,iter.next().unwrap().point);
    assert_eq!(cat3.point,iter.next().unwrap().point);
    assert!(iter.next().is_none());   
}

