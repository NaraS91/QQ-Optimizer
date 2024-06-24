macro_rules! flat_value {
    ($x: expr) => {
        |_, _, _, _, _, _| ($x)
    };
}
pub(crate) use flat_value;
macro_rules! value_with_buffer {
    ($x: expr) => {
        |_, buffer: &Unit, _, _, _, _| ($x)(buffer)
    };
}

pub(crate) use value_with_buffer;
