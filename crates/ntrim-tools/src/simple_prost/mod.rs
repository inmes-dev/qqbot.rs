#[macro_export]
macro_rules! protobuf {
    // Handle the base case of an empty map
    (
        $buf_name:expr, {}
    ) => {};

    ($buf_name:ident, {
        $field_number:expr => $value_type:ident => $value:expr
        $(, $($rest_tag:expr => $rest_type:ident => $rest_value:expr),+ $(,)?)?
    }) => {
        prost::encoding::$value_type::encode($field_number, &$value, &mut $buf_name);
        protobuf!($buf_name, { $($($rest_tag => $rest_type => $rest_value),*)? });
    };
}

#[test]
fn encode_prost_with_macro() {
    let mut buf = Vec::new();
    let mut buf1281 = Vec::new();
    protobuf!(buf1281, {
        1 => int64 => 1,
        2 => int32 => 0,
        3 => int32 => 16,
        4 => int32 => 1,
        6 => int32 => 3,
        10 => int32 => 9,
    });
    prost::encoding::int32::encode_repeated(7, &[1, 5, 10, 21], &mut buf1281);
    protobuf!(buf, {
        1281 => bytes => buf1281
    });

    // Now `buf` contains the serialized data
    println!("{:?}", hex::encode(buf));
}
