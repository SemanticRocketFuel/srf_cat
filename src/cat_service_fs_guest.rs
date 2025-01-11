

use std::collections::HashMap;

use infograph_lib::triple::info_table;
use infograph_lib::{edge_service_fs::EdgeServiceFS, tr_service_fs, desc_service_fs};
use infograph_lib::{NameOrPoint, Point, LabelOrPoint, InfoTable, InfoTriple, Descriptor};

use crate::cat::Cat;
use crate::cat_element::CatElement;
use crate::cat_element_list::CatElementList;
use crate::cat_list::CatList;

//this file may be unused/deprecated...

pub struct CatServiceFS {
    table_name: String,
    cat_tag: String,
    canceled_tag: String,
    space_id: String,
}

impl CatServiceFS {

    pub fn new() -> Self{
        CatServiceFS {
           table_name: "".to_string(),
           cat_tag: "5fe9374c5e9e27ebb78f8bf7cd78bbb23ee51e672dc54c603ec1c5b3eef33feb".to_string(),
           canceled_tag: "1c1ca1cef969f76f757fdf7ff3ff9f365a864ef579f2d45866f8d0d5ef9f01df".to_string(),
        }
    }
    pub fn new_w_space() -> Self{
        CatServiceFS {
           table_name: "".to_string(),
           cat_tag: "5fe9374c5e9e27ebb78f8bf7cd78bbb23ee51e672dc54c603ec1c5b3eef33feb".to_string(),
           canceled_tag: "1c1ca1cef969f76f757fdf7ff3ff9f365a864ef579f2d45866f8d0d5ef9f01df".to_string(),
        }
    }

    ///
    /// Loads and returns a cat_list object for data storage specified in the parameter graph_name.
    ///
    pub fn get_all_cats(&self, graph_name: String) -> CatList {

        let info_table = tr_service_fs::TrServiceFS::get_all_info_triples_from_info_table(graph_name.clone());
        let mut info_table = InfoTable::from_iter(info_table);    

//        let mut info_table = info_table.get_neighbors_as_triples(self.cat_tag.clone());

        let cat_table = info_table.get_neighbors_except_decorated(self.cat_tag.clone(), self.canceled_tag.clone());
        let cat_list = cat_table.get_neighbor_ids(self.cat_tag.clone());

        let descs = desc_service_fs::DescServiceFS::get_descs_hashmap_for_list(cat_list);
//        let descs = desc_service_fs::DescServiceFS::get_descs_hashmap_for_list(info_table.get_neighbor_ids(self.cat_tag.clone()));

        self.populate_cat_list(cat_table, descs)
    }

    pub fn get_all_space_cats(&self, space_id: String) -> CatList {

        let info_table = tr_service_fs::TrServiceFS::get_all_info_triples_from_space_info_table("main_table".to_string(), space_id.clone());
        let mut info_table = InfoTable::from_iter(info_table);    

//        let mut info_table = info_table.get_neighbors_as_triples(self.cat_tag.clone());

        let cat_table = info_table.get_neighbors_except_decorated(self.cat_tag.clone(), self.canceled_tag.clone());
        let cat_list = cat_table.get_neighbor_ids(self.cat_tag.clone());

        let descs = desc_service_fs::DescServiceFS::get_descs_hashmap_for_space_list(cat_list, space_id);
//        println!("descs for all spacecats {:?}", descs);
//        let descs = desc_service_fs::DescServiceFS::get_descs_hashmap_for_list(info_table.get_neighbor_ids(self.cat_tag.clone()));

        self.populate_cat_list(cat_table, descs)
    }

    ///
    /// Returns a CatList containing a Cat object for each row in the InfoTable provided as
    /// parameter. This method is private because it does not check if the InfoTriple objects are indeed cats.
    /// The cat objects are enriched with data from both the info triples in the parameter info_table and the descriptor notes from the parameter desc.
    /// Desc is a HashMap<String,Descriptor> which is expected to contain pairings of descriptor's
    /// points and the Descriptor objects themselves.
    ///
    fn populate_cat_list(&self, mut info_table: InfoTable, descs: HashMap<String, Descriptor>) -> CatList {

        let mut result = CatList::new();
        info_table
            .get_info_triples()
            .iter()
            .for_each(|it|{
                let point =  &it.other_half(self.cat_tag.clone()).unwrap();
                let desc = descs.get(point)
                    .unwrap();
                result.add(Cat{
                    triple_id: it.id.clone(),
                    point: point.to_string(),
                    type_id: self.cat_tag.clone(),
                    desc_id: desc.desc_id.clone(),
                    name: desc.name.clone(),
                    icon_uri: desc.label.clone(),
                    description: desc.description.clone(),
                });
            });

        result
        
    }

    ///
    /// This is supposed to replace get_all_cat_elements and return a cat_element_list instead of a
    /// HashMap.
    ///
    pub fn get_cat_elements(&self, graph_name: String, cat_id: String) -> CatElementList {
        let mut result = CatElementList::new();

        let info_table = tr_service_fs::TrServiceFS::get_all_info_triples_from_info_table(graph_name.clone());
        let mut info_table = InfoTable::from_iter(info_table);    
        
        let element_table = info_table.get_neighbors_except_decorated_and_not(cat_id.clone(), self.canceled_tag.clone(), self.cat_tag.clone());

        element_table
            .clone()
            .into_iter()
            .for_each(|it|{
                let ce = CatElement::from_triple(cat_id.clone(), it);
                if let Ok(..) = ce {
                    result.add(ce.unwrap());
                }
            });

        result
    }

    ///
    /// Returns a list of InfoEdge objects for a given cat graph_name referring to a specific
    /// category type.
    /// This means triples and their descriptor notes.
    ///
    pub fn get_all_cat_elements(&self, graph_name: String, cat_id: String) -> HashMap<Point, NameOrPoint> {

        let mut result:HashMap<Point, NameOrPoint> = HashMap::new();

        let info_table = tr_service_fs::TrServiceFS::get_all_info_triples_from_info_table(graph_name.clone());
        let mut info_table = InfoTable::from_iter(info_table);    

//println!("getting elements for cat:{}",cat_id);
        
        let element_table = info_table.get_neighbors_except_decorated_and_not(cat_id.clone(), self.canceled_tag.clone(), self.cat_tag.clone());
        let element_points = element_table.get_neighbor_ids(cat_id.clone());

        let descs = desc_service_fs::DescServiceFS::get_descs_hashmap_for_list(element_points.clone());

        element_points
            .iter()
            .for_each(|point|{
                let desc = descs.get(point);
                if desc.is_some(){
                    let name = &desc.unwrap().name;
                    if name.is_empty() {
//                        println!("found an element with name but empty");
                        result.insert(point.to_string(), point.to_string());
                    } else {
//                        println!("found an element with name");
                        result.insert(point.to_string(), name.to_string());
                    }
                } else {
//                        println!("found an element without name");
                    result.insert(point.to_string(), point.to_string());
                }
            });

        result

    }

   pub fn create_cat(cat_point: &str, cat_name: &str, cat_icon: &str, cat_desc: &str) {
    
        let id2 = "5fe9374c5e9e27ebb78f8bf7cd78bbb23ee51e672dc54c603ec1c5b3eef33feb".to_string();
        let id1 = cat_point.to_string(); 
        let descid = desc_service_fs::DescServiceFS::create_desc(cat_point.to_string(), 
                                                              cat_name.to_string(), cat_icon.to_string(), cat_desc.to_string()).desc_id;
       tr_service_fs::TrServiceFS::create_infotriple(id1, id2);
   }

   pub fn delete_cat(cat: Cat) {
        let id2 = "1c1ca1cef969f76f757fdf7ff3ff9f365a864ef579f2d45866f8d0d5ef9f01df".to_string();
        let id1 = cat.triple_id.to_string(); 

        tr_service_fs::TrServiceFS::create_infotriple(id1, id2);
   }

    pub fn create_cat_element(cat: Cat, id_to_tag: String) {
        tr_service_fs::TrServiceFS::create_infotriple(cat.point, id_to_tag);
    } 
}

#[test]
fn populate_cat_list_test() {

    let cat_tag= "5fe9374c5e9e27ebb78f8bf7cd78bbb23ee51e672dc54c603ec1c5b3eef33feb".to_string();
    let mut info_table: InfoTable = InfoTable::new();
    let mut descs: HashMap<String, Descriptor> = HashMap::new();
    let desc1 = Descriptor {
     point: "a".to_string(),
     desc_id: "d1".to_string(),
     name: "name1".to_string(),
     label: "uri1".to_string(),
     description: "cat no. 1".to_string(),
    };
    descs.insert(desc1.point.clone(), desc1);
    let it1 = InfoTriple {
        id: "3-1".to_string(),
        id1: "a".to_string(),
        id2: cat_tag.to_string(),
    };
    assert!(info_table.add_triple(it1).is_ok());
    
    let desc2 = Descriptor {
     point: "b".to_string(),
     desc_id: "d2".to_string(),
     name: "name2".to_string(),
     label: "uri2".to_string(),
     description: "cat no. 2".to_string(),
    };
    descs.insert(desc2.point.clone(), desc2);
    let it2 = InfoTriple {
        id: "3-2".to_string(),
        id1: "b".to_string(),
        id2: cat_tag.to_string(),
    };
    info_table.add_triple(it2);

    let desc3 = Descriptor {
     point: "c".to_string(),
     desc_id: "d3".to_string(),
     name: "name3".to_string(),
     label: "uri3".to_string(),
     description: "cat no. 3".to_string(),
    };
    descs.insert(desc3.point.clone(), desc3);
    let it3 = InfoTriple {
        id: "3-3".to_string(),
        id1: "c".to_string(),
        id2: cat_tag.to_string(),
    };
    info_table.add_triple(it3);

    let cat_service = CatServiceFS::new();
    let cats: CatList = cat_service.populate_cat_list(info_table, descs);
    assert!(!cats.cats.is_empty());

    let mut it = cats.into_iter();
    let cat = it.next().unwrap();
    assert_eq!(cat.triple_id,"3-1");
    assert_eq!(cat.point,"a");
    assert_eq!(cat.type_id,cat_tag);
    assert_eq!(cat.desc_id,"d1");
    assert_eq!(cat.name,"name1");
    assert_eq!(cat.icon_uri,"uri1");
    assert_eq!(cat.description,"cat no. 1");
}
