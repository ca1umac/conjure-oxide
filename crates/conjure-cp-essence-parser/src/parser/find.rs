#![allow(clippy::legacy_numeric_constants)]

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use tree_sitter::Node;

use super::domain::parse_domain;
use super::util::named_children;
use crate::EssenceParseError;
use conjure_cp_core::ast::{DomainPtr, Name, SymbolTable};


// Implement the parse_find_statement function.
// You'll need to create and return a new BTreeMap that maps variables to domains
// Enabling the calling function to know which variables it must decide on.
//
// There is a list of top-level statements, e.g. 'find_statement', 'letting_statement', etc.,  and the 
//  calling function wants to know the semantices of the statement. The syntax of a find_statement
//  is: 
//          $find_statement -> $variable_list ":" $domain
//
//  suggesting a sementic meaning that there exists one or more variables with a single specified domain.
//  The 'find_statement' node will have two children: "variable_list", and "domain". 
//     The "variable_list" child will also have children for each variable in the list.
//
// e.g, for an essence file
//      letting x be 1
//      find y,z : int(1..3)
//      such that x < y, x < z
//
//  Would have a tree:
//  
//                     domain (1..3)
//                   /  
//  find_statement =                     <start char 19, end char 19>
//                   \                 /
//                     variable_list =
//                                     \
//                                       <start char 21, end char 21>
//
//  Note that "y" and "z" are not in the tree, rather their positions in the string are.
//  Note how there is no mention of the variable x, or it's position: this is because that is in a letting statement, not a find_statement.
//
//  HINTS:
//      > There is a useful function on the Node type `child_by_field_name` which you can use if you already know the name of the child (e.g. "domain" or "variable list")
//      > There are useful functions on the Node type `start_byte` and `end_byte` which specify where in the string the Node corresponds to, which is useful for getting variable names
//      > There is a iterator `named_children` which you will need if you don't know the child name (e.g. "y" or "z")

pub fn parse_find_statement(                                // Given
    find_statement: Node,                                   // ... a tree-sitter node in the Concrete Syntax Tree representing a find_statement
    source_code: &str,                                      // ... an essence string, for retrieving the variable names
    symbols: Option<Rc<RefCell<SymbolTable>>>,              // ... the symbol table, needed for the `parse_domain` function 

) -> Result<BTreeMap<Name, DomainPtr>, EssenceParseError>   // Return a result of either the new tree, or propagate errors back up
{

    // TODO: Create a new BTreeMap to populate and return
    // let mut vars = ??? ;

    // TODO: Parse the domain. 
    //      > This is a child of the find statement.
    //      > You can expect the "domain" child node to exist, given the function signature requests a "find_statement", so .unwrap() or .expect() are valid.
    //      > There is already a `parse_domain` function, which has been brough into scope above. This may raise an EssenceParseError.
    // let dom_node = ??? ;
    // let domain = ??? ;


    // TODO: Parse the variable list.
    //      > Once again, a child of the find statement
    
    // let var_list = ???;


    // TODO: Populate your BTreeMap
    //      > Iterate over the children nodes in the variable list
    //      > The "name" of the var is not in the node, the node says where in string the var appears.
    //      > You may find `&source_code[..]` with start and end slices specified using the .start_byte() and .end_byte() useful 
    //      > Note the variable name should be constructed with Name::user(), because:
    //          - The BTreeMap type is <Name, DomainPtr>, so we need a Name type for our key
    //          - and "User" is the variant of the Name enum which says the name came from the input, and "user" is the function that takes a &str
    // for var in named_children(&var_list) {
        // let var_name = ???;
        // vars.insert(???);
    // }

    // TODO: Return your BTreeMap


}
