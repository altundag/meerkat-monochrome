const fn make_header() -> [u8; 4] {
    [
        b'I', b'I', // Little endian
        0x2a, 0x00, // TIFF stamp
    ]
}

const fn make_short_ifd_entry(tag: u16, count: u32, value: u16) -> [u8; 12] {
    let [tag0, tag1] = tag.to_le_bytes();
    let [type0, type1] = 3u16.to_le_bytes();
    let [count0, count1, count2, count3] = count.to_le_bytes();
    let [value0, value1] = value.to_le_bytes();
    [
        tag0, tag1, type0, type1, count0, count1, count2, count3, value0, value1, 0, 0,
    ]
}

const fn make_long_ifd_entry(
    tag: u16,
    entry_type: u16,
    count: u32,
    value_or_offset: u32,
) -> [u8; 12] {
    let [tag0, tag1] = tag.to_le_bytes();
    let [type0, type1] = entry_type.to_le_bytes();
    let [count0, count1, count2, count3] = count.to_le_bytes();
    let [
        value_or_offset0,
        value_or_offset1,
        value_or_offset2,
        value_or_offset3,
    ] = value_or_offset.to_le_bytes();
    [
        tag0,
        tag1,
        type0,
        type1,
        count0,
        count1,
        count2,
        count3,
        value_or_offset0,
        value_or_offset1,
        value_or_offset2,
        value_or_offset3,
    ]
}

pub fn write_single_directory_monochrome_tiff<W, E>(
    mut write_all: W,
    width: u16,
    height: u16,
    bpp: u16,
    image: &[u8],
) -> Result<(), E>
where
    W: FnMut(&[u8]) -> Result<(), E>,
{
    let make = b"Meerkat\0";
    let model = b"Monochrome 1\0";

    // Make and write header
    let header = make_header();
    write_all(&header)?;

    // Write IFD offset
    let ifd_offset =
        ((header.len() + 4 + image.len() + make.len() + model.len()) as u32).to_le_bytes();
    write_all(&ifd_offset)?;

    // Write the image data
    let image_offset = (header.len() + 4) as u32;
    write_all(image)?;

    let make_offset = image_offset + image.len() as u32;
    write_all(make)?;

    let model_offset = make_offset + make.len() as u32;
    write_all(model)?;

    // Make and write entries
    let entries = [
        // Width
        make_short_ifd_entry(256, 1, width),
        // Height
        make_short_ifd_entry(257, 1, height),
        // BitsPerSample
        make_short_ifd_entry(258, 1, bpp),
        // Compression (no compression)
        make_short_ifd_entry(259, 1, 1),
        // PhotometricInterpretation (BlackIsZero)
        make_short_ifd_entry(262, 1, 1),
        // FillOrder
        make_short_ifd_entry(266, 1, 1),
        // Make
        make_long_ifd_entry(271, 2, make.len() as u32, make_offset),
        // Model
        make_long_ifd_entry(272, 2, model.len() as u32, model_offset),
        // StripOffsets
        make_long_ifd_entry(273, 4, 1, image_offset),
        // SamplesPerPixel
        make_short_ifd_entry(277, 1, 1),
        // StripByteCounts
        make_long_ifd_entry(279, 4, 1, image.len() as u32),
        // SampleFormat
        make_short_ifd_entry(339, 1, 1),
    ];
    write_all(&(entries.len() as u16).to_le_bytes())?;
    for entry in entries {
        write_all(&entry)?;
    }

    // No next IFD exists
    write_all(&0u32.to_le_bytes())?;

    Ok(())
}
