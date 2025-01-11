
use std::collections::HashMap;

use infograph::infotriple::Filesystem as tr_service_fs;
use infograph::descnote::Filesystem as desc_service_fs;
use infograph::types::{NameOrPoint, Point};
use infograph::{InfoTriple, InfoTable, Descriptor};

use crate::model::cat::Cat;
use crate::model::cat_element::CatElement;
use crate::model::cat_element_list::CatElementList;
use crate::model::cat_list::CatList;

#[derive(Clone)]
pub struct CatServiceFS {
    table_name: String,
    cat_tag: String,
    canceled_tag: String,
    conditioned_tag: String,
    pub space_id: String,
    tr_service: tr_service_fs,
    desc_service: desc_service_fs,
}

impl CatServiceFS {

    pub fn new(space_id: String) -> Self{

//println!("Cat::new was called!!! for space_id {}", space_id);

        CatServiceFS {
            space_id: space_id.clone(),
            table_name: "".to_string(),
            cat_tag: "5fe9374c5e9e27ebb78f8bf7cd78bbb23ee51e672dc54c603ec1c5b3eef33feb".to_string(),
            canceled_tag: "1c1ca1cef969f76f757fdf7ff3ff9f365a864ef579f2d45866f8d0d5ef9f01df".to_string(),
            conditioned_tag: "9b2478b1a6e89633eede884aa57fd1a1fb2e4ed279b58e9d1cd15010a103c5bc".to_string(), 
            tr_service: tr_service_fs::new(space_id.clone()),
            desc_service: desc_service_fs::new(space_id.clone()),
        }
    }

    ///
    /// Loads and returns a cat_list object for data storage specified in the parameter graph_name.
    ///
    pub fn get_all_cats(&mut self, graph_name: String) -> CatList {
//println!("Cat::get_all_cats was called!!! for graph_name {}", graph_name);

        let info_table = self.tr_service.get_all_info_triples_from_info_table(graph_name.clone());
        let mut info_table = InfoTable::from_iter(info_table);    

        let cat_table = info_table.get_neighbors_except_decorated(self.cat_tag.clone(), self.canceled_tag.clone());
        let cat_list = cat_table.get_neighbor_ids(self.cat_tag.clone());

        let descs = self.desc_service.get_descs_hashmap_for_list(cat_list);

        self.populate_cat_list(cat_table, descs)
        
    }
    
    ///
    /// Loads and returns a cat_list object for data storage specified in the parameter graph_name.
    /// Populate the cats with a list of individual conditions for the use of each cat if any.
    ///
    pub fn get_all_cats_w_conditions(&mut self, graph_name: String) -> CatList {
//println!("Cat::get_all_cats_w_conditions was called!!! for graph_name {}", graph_name);

        let info_table = self.tr_service.get_all_info_triples_from_info_table(graph_name.clone());
        let mut info_table = InfoTable::from_iter(info_table);    

        let cat_table = info_table.get_neighbors_except_decorated(self.cat_tag.clone(), self.canceled_tag.clone());
        let cat_list = cat_table.get_neighbor_ids(self.cat_tag.clone());

        let descs = self.desc_service.get_descs_hashmap_for_list(cat_list);

        let cats = self.populate_cat_list(cat_table.clone(), descs);

        // all triples that is paired with conditioned
        let conditioned_table = info_table.clone().get_neighbors_except_decorated(self.conditioned_tag.clone(), self.canceled_tag.clone());

//println!("conditioned_table: {:?}", conditioned_table);

        let mut cat_list = CatList::new();

        cats.into_iter()
            .map(|cat|{
                let mut cat_w_condi = cat.clone();
                cat_w_condi.conditioned_list = self.get_condition_list(cat.triple_id.clone(), info_table.clone(), conditioned_table.clone());
//println!("cat_w_condi:{:?}",cat_w_condi);
                cat_w_condi
            })
            .for_each(|cat_w_condi|{
                cat_list.add(cat_w_condi);
            });

        cat_list
    }


/// Returning a list of cat_ids (list can be empty) that a cat_id is conditioned by.   
pub fn get_condition_list(&self, cat_triple_id: String, info_table: InfoTable, conditioned_table: InfoTable) -> Vec<String> {
//println!("Cat::get_condition_list was called!!! for cat_triple_id {}", cat_triple_id);
        
    let conditioned_id = self.get_conditioned_decoration_id(cat_triple_id, conditioned_table);

//println!("conditioned_id: {:?}", conditioned_id);

    if conditioned_id.is_none() {
//println!("No thing");        
        Vec::new()
    } else {
        info_table.get_neighbor_ids(conditioned_id.unwrap())
    }
}

/// Helper method that finds the triple_id (if any) used for adding conditions to a certain cat_id
pub fn get_conditioned_decoration_id(&self, cat_triple_id: String, conditioned_table: InfoTable) -> Option<String>{

    conditioned_table
        .get_neighbor_triple_ids_only(cat_triple_id)
        .pop()
}

    pub fn populate_cat_list(&self, mut info_table: InfoTable, descs: HashMap<String, Descriptor>) -> CatList {

//println!("Cat::populate_cat_list was called!!!");

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
                    conditioned_list: Vec::new(),
                });
            });

        result
        
    }

    ///
    /// This is supposed to replace get_all_cat_elements and return a cat_element_list instead of a
    /// HashMap.
    ///
    pub fn get_cat_elements(&mut self, graph_name: String, cat_id: String) -> CatElementList {
//println!("Cat::get_cat_elements was called!!! for cat_id {}", cat_id);
        let mut result = CatElementList::new();

        let info_table = self.tr_service.get_all_info_triples_from_info_table(graph_name.clone());
        let mut info_table = InfoTable::from_iter(info_table);    
        
        let element_table = info_table.get_neighbors_except_decorated_and_not(cat_id.clone(), self.canceled_tag.clone(), self.cat_tag.clone());

        element_table
            .clone()
            .into_iter()
            .for_each(|it|{
//println!("cat_element found: {:?}",it);                
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
    pub fn get_all_cat_elements(&mut self, graph_name: String, cat_id: String) -> HashMap<Point, NameOrPoint> {
//println!("Cat::get_all_cat_elements was called!!! for cat_id {}", cat_id);

        let mut result:HashMap<Point, NameOrPoint> = HashMap::new();

        let info_table = self.tr_service.get_all_info_triples_from_info_table(graph_name.clone());
        let mut info_table = InfoTable::from_iter(info_table);    

//println!("getting elements for cat:{}",cat_id);
        
        let element_table = info_table.get_neighbors_except_decorated_and_not(cat_id.clone(), self.canceled_tag.clone(), self.cat_tag.clone());
        let element_points = element_table.get_neighbor_ids(cat_id.clone());

        let descs = self.desc_service.get_descs_hashmap_for_list(element_points.clone());

        element_points
            .iter()
            .for_each(|point|{
                let desc = descs.get(point);
                if desc.is_some(){
                    let name = &desc.unwrap().name;
                    if name.is_empty() {
                        println!("found an element with name but empty");
                        result.insert(point.to_string(), point.to_string());
                    } else {
                        println!("found an element with name");
                        result.insert(point.to_string(), name.to_string());
                    }
                } else {
                        println!("found an element without name");
                    result.insert(point.to_string(), point.to_string());
                }
            });

        result

    }

   pub fn create_cat(&mut self, cat_point: &str, cat_name: &str, cat_icon: &str, cat_desc: &str) {
    
        let id1 = cat_point.to_string(); 
        let descid = self.desc_service.create_desc(cat_point.to_string(), 
                                                              cat_name.to_string(), cat_icon.to_string(), cat_desc.to_string()).desc_id;
       self.tr_service.create_infotriple(id1, self.cat_tag.to_string());
   }

   pub fn delete_cat(&mut self, cat: Cat) {
        let id1 = cat.triple_id.to_string(); 

        self.tr_service.create_infotriple(id1, self.canceled_tag.to_string());
   }

    pub fn create_cat_element(&mut self, cat: Cat, id_to_tag: String) {
        self.tr_service.create_infotriple(cat.point, id_to_tag);
    } 
    
    pub fn create_cat_condi(&mut self, super_cat_id: String, sub_cat_id: String) {
//println!("When a note has been marked as {} we will show the cat type {}",super_cat_id, sub_cat_id);
        //set sub_cat_id as conditioned
        let conditioned = self.tr_service.create_infotriple(sub_cat_id, self.conditioned_tag.clone());
//println!("created conditioned: {:?}",conditioned);    
        //pair super_cat_id with conditioned triple id
        let result = self.tr_service.create_infotriple(conditioned.id, super_cat_id);
//println!("created actual condition {:?}", result);
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
    let _ = info_table.add_triple(it2);

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
    let _ = info_table.add_triple(it3);

    let cat_service = CatServiceFS::new("test".to_string());
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
