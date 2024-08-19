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

macro_rules! lc_superimposition {
    ($unit: ident, $lc_store: ident) => {
        $lc_store
            .get($unit.unique_data.light_cone.unwrap())
            .unwrap()
            .superimposition
    };
}
pub(crate) use lc_superimposition;

macro_rules! value_with_lc_s {
    ($x: expr) => {
        |_, buffer: &Unit, _, _, lc_store, _| {
            ($x)(
                lc_store
                    .get(buffer.unique_data.light_cone.unwrap())
                    .unwrap()
                    .superimposition as usize,
            )
        }
    };
}

pub(crate) use value_with_lc_s;
