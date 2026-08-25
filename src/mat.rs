use std::marker::PhantomData;

use crate::channel::Channel;

pub struct Mat<T, C:Channel>{
    weight:T,
    height:T,

    data:Vec<u8>,

    _phantom:PhantomData<C>,
}