

use std::collections::HashMap;

use infograph::types::{NameOrPoint, Point};
use infograph::{InfoTriple, Descriptor, InfoTable};

use crate::model::cat::Cat;
use crate::model::cat_element_list::CatElementList;
use crate::model::cat_list::CatList;
use crate::cat_service_fs::CatServiceFS;

#[derive(Clone)]
pub struct CatSingleServiceFS {
}

impl CatSingleServiceFS {

    ///
    /// Loads and returns a cat_list object for data storage specified in the parameter graph_name.
    ///
    pub fn get_all_cats(space_id: String, graph_name: String) -> CatList {

        CatServiceFS::new(space_id).get_all_cats(graph_name)

    }

    pub fn get_all_cats_w_conditions(space_id: String, graph_name: String) -> CatList {

        CatServiceFS::new(space_id).get_all_cats_w_conditions(graph_name)

    }

    ///
    /// This is supposed to replace get_all_cat_elements and return a cat_element_list instead of a
    /// HashMap.
    ///
    pub fn get_cat_elements(space_id: String, graph_name: String, cat_id: String) -> CatElementList {

        CatServiceFS::new(space_id).get_cat_elements(graph_name, cat_id)
    }

    ///
    /// Returns a list of InfoEdge objects for a given cat graph_name referring to a specific
    /// category type.
    /// This means triples and their descriptor notes.
    ///
    pub fn get_all_cat_elements(space_id: String, graph_name: String, cat_id: String) -> HashMap<Point, NameOrPoint> {
    
        CatServiceFS::new(space_id).get_all_cat_elements(graph_name, cat_id)
    }

   pub fn create_cat(space_id:String, cat_point: &str, cat_name: &str, cat_icon: &str, cat_desc: &str) {

        CatServiceFS::new(space_id).create_cat(cat_point, cat_name, cat_icon, cat_desc)
   }

   pub fn delete_cat(space_id: String, cat: Cat) {

        CatServiceFS::new(space_id).delete_cat(cat)
            
   }

    pub fn create_cat_element(space_id: String, cat: Cat, id_to_tag: String) {
        CatServiceFS::new(space_id).create_cat_element(cat, id_to_tag)
    } 
}

#[test]
fn populate_cat_list_test() {

    let cat_id= "5fe9374c5e9e27ebb78f8bf7cd78bbb23ee51e672dc54c603ec1c5b3eef33feb".to_string();
    let mut info_table: InfoTable = InfoTable::new();
    let mut descs: HashMap<String, Descriptor> = HashMap::new();
    let desc1 = Descriptor {
     point: "a".to_string(),
     desc_id: "d1".to_string(),
     name: "name1".to_string(),
     label: "uri1".to_string(),
     description: "button no. 1".to_string(),
    };
    descs.insert(desc1.point.clone(), desc1);
    let it1 = InfoTriple {
        id: "3-1".to_string(),
        id1: "a".to_string(),
        id2: cat_id.to_string(),
    };
    assert!(info_table.add_triple(it1).is_ok());
    
    let desc2 = Descriptor {
     point: "b".to_string(),
     desc_id: "d2".to_string(),
     name: "name2".to_string(),
     label: "uri2".to_string(),
     description: "button no. 2".to_string(),
    };
    descs.insert(desc2.point.clone(), desc2);
    let it2 = InfoTriple {
        id: "3-2".to_string(),
        id1: "b".to_string(),
        id2: cat_id.to_string(),
    };
    let _ = info_table.add_triple(it2);

    let desc3 = Descriptor {
     point: "c".to_string(),
     desc_id: "d3".to_string(),
     name: "name3".to_string(),
     label: "uri3".to_string(),
     description: "button no. 3".to_string(),
    };
    descs.insert(desc3.point.clone(), desc3);
    let it3 = InfoTriple {
        id: "3-3".to_string(),
        id1: "c".to_string(),
        id2: cat_id.to_string(),
    };
    let _ = info_table.add_triple(it3);

    let cat_service = CatServiceFS::new("test".to_string());
    let cats: CatList = cat_service.populate_cat_list(info_table, descs);
    assert!(!cats.cats.is_empty());

    let mut it = cats.into_iter();
    let cat = it.next().unwrap();
    assert_eq!(cat.triple_id,"3-1");
    assert_eq!(cat.point,"a");
    assert_eq!(cat.type_id,cat_id);
    assert_eq!(cat.desc_id,"d1");
    assert_eq!(cat.name,"name1");
    assert_eq!(cat.icon_uri,"uri1");
    assert_eq!(cat.description,"button no. 1");
}
