extern crate exif;
extern crate rexiv2;

fn main() {

    // O codigo descomentado eh relativo ao rexiv2

    let file = "images/andre.jpg";

    // Essa Tag que diferencia em quais dados ele pega
    let tag = "Iptc.Application2.Keywords";
    let meta = rexiv2::Metadata::new_from_path(&file).unwrap();
    println!("{:?}", meta.get_tag_multiple_strings(tag));

    // Código comentado abaixo eh relativo ao exif

    // let file = std::fs::File::open("images/andre.jpg").unwrap();
    // let reader = exif::Reader::new(
    //     &mut std::io::BufReader::new(&file)).unwrap();
    //
    // for f in reader.fields() {
    //     println!("{} {} {:?}", f.tag, f.thumbnail, f.value);
    // }
}