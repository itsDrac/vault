use std::collections::HashMap;
use super::types::GitObject;

/* SAMPLE TREE FILE

blob 103\0fileA.txt\0123456789abcdef0123456789abcdef01234567
tree 41\0folderA\0ab89abcdef0123456789abcdef0123456789ab
 */


#[derive(Debug)]
pub struct Tree {
    entries: Vec<TreeEntry>,
}

#[derive(Debug)]
pub struct TreeEntry {
    pub name: String,
    pub object: GitObject,
    pub hashedPath: String
}

impl Tree {
    //@TODO
    fn new_entry() {

    }
}