use raylib::prelude::*;

fn main() {
    let mut rl = Raylib::init_window(100, 100, "", 60);
    let mut mat = raylib::model::Material::load_default(&mut rl);
    dbg!(mat.get_texture_mut(MaterialMapIndex::Albedo));
}
