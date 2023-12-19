use std::fmt::Debug;

use dyn_clone::DynClone;

pub trait Statement: Debug + DynClone {
    
}

pub trait Expression: Debug + DynClone  {
    
}

dyn_clone::clone_trait_object!(Expression);
dyn_clone::clone_trait_object!(Statement);