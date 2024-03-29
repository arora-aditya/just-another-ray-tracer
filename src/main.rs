mod render;
mod tracer;
mod objects;
mod material;
mod camera;
mod utils;
mod textures;



fn main() {
    // render::ppm_test();
    // render::vec_test();
    // render::ray_test();
    // render::blue_shader();
    // render::blue_shader_with_sphere();
    // render::blue_shader_with_sphere_shading();
    // render::blue_shader_with_2_sphere_shading();
    // render::blue_shader_with_2_sphere_shading_with_anti_aliasing();
    // render::diffused_shader_with_2_spheres();
    // render::material_diffused_shader_with_3_spheres();
    // render::dielectric_with_3_spheres();
    // render::change_camera_dielectric_with_2_spheres();
    render::book_cover();
}
