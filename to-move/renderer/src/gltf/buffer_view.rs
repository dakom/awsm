use gltf::buffer::View;

pub fn get_buffer_view_data <'a>(view:&View, buffers:&'a Vec<Vec<u8>>) -> &'a [u8] {
    let byte_offset = view.offset();
    let byte_length = view.length();
    let byte_end = byte_offset + byte_length;
    let full_buffer_data = &buffers[view.buffer().index()];

    let data = &full_buffer_data[byte_offset..byte_end];

    //log::info!("buffer view: target length {} start {} end {} data {:?}", full_buffer_data.len(), byte_offset, byte_end, data);

    data
}