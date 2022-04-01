struct TGAHeader {
    id_length : u8,
    color_map_type : u8,
    data_type_code : u8,
    color_map_origin : u16,
    color_map_length : u16,
    color_map_depth : u8,
    x_origin : u16,
    y_origin : u16,
    width : u16,
    height : u16,
    bits_per_pixel : u8,
    image_descriptor : u8
}

