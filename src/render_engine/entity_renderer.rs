use cgmath;
use cgmath::num_traits::real::Real;
use cgmath::prelude::*;
use cgmath::vec3;
use gl::types::*;

use std::collections::HashMap;
use std::ptr;

use crate::entities::camera::Camera;
use crate::entities::entity::Entity;
use crate::models::raw_model::RawModel;
use crate::models::textured_model::TexturedModel;
use crate::render_engine::display_manager::DisplayManager;
use crate::shaders::shader_program::ShaderProgram;
use crate::toolbox::maths::*;

type P3CG = cgmath::Point3<f32>;
type V3CG = cgmath::Vector3<f32>;
type M4CG = cgmath::Matrix4<f32>;

pub struct Renderer {
    projection_matrix: cgmath::Matrix4<f32>,
    shader: ShaderProgram,
}

impl Renderer {
    pub fn new(shader: ShaderProgram, dm: &DisplayManager) -> Renderer {
        unsafe {
            gl::Enable(gl::CULL_FACE);
            gl::CullFace(gl::BACK);
        }
        let projection_matrix =
            create_projection_matrix_perspective(dm.width as f32, dm.height as f32);
        shader.start();
        shader.load_projection_matrix(&projection_matrix);
        shader.stop();
        Renderer {
            projection_matrix,
            shader,
        }
    }

    //Llamado una vez cada frame. Prepara OpenGL para renderizar el juego.
    pub fn prepare(&mut self) {
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);


            gl::ClearColor(0.0, 0.3, 0.0, 1.0);
        }
    }

    pub fn render(&mut self, entities: &Vec<Vec<Entity>>) {
        for i in 0..entities.len() {
            let vv = entities[i].clone();
            for e in vv {
                self.prepare_textured_model(&e.model);
                //println!("e = {:#?} ", e);
                self.prepare_instance(e);
                //dbg!(e.get_model().get_raw_model().get_vertex_count());
                unsafe {
                    gl::DrawElements(
                        gl::TRIANGLES,// modo
                        e // Entity
                            .get_model()
                            .get_raw_model()
                            .get_vertex_count(),// número de índices a
                        // renderizar
                        gl::UNSIGNED_INT,
                        ptr::null());
                }
            }
        }

//        let model = entity.get_model();//Ahora el modelo hay que leerlo de entity
//        let raw_model = model.get_raw_model();
//
//        unsafe {
//            shader.start();
//            let vm = create_view_matrix(camera);
//            shader.load_view_matrix(&vm);
//
//            gl::BindVertexArray(raw_model.get_vao_id());
//            //Activa VAO 0 (vértices).
//            gl::EnableVertexAttribArray(0);
//            //Activa VAO 1 (coordenadas de textura).
//            gl::EnableVertexAttribArray(1);
//            //Activa VAO 2 (coordenadas de normals).
//            gl::EnableVertexAttribArray(2);
//
//            //Crea matriz de transformación con los datos de la entity
//            let transformation_matrix = create_transformation_matrix(
//                entity.get_position(),
//                entity.get_rotation_x(), entity.get_rotation_y(), entity.get_rotation_z(),
//            );
//
//            //dbg!(&transformation_matrix);
//            //Envia la matriz de transformación de la entity al shader
//            shader.load_transformation_matrix(&transformation_matrix);
//
//            let texture = model.get_texture();
//            shader.load_shine_variables(texture.get_shine_damper(), texture.get_reflectivity());
//
//            gl::ActiveTexture(gl::TEXTURE0);
////            //dbg!(tex_model.get_model_texture().get_id());
//            gl::BindTexture(gl::TEXTURE_2D, model.get_texture().get_id());
//
//            //dbg!(raw_model.get_vertex_count());
//            gl::DrawElements(
//                gl::TRIANGLES,// modo
//                raw_model.get_vertex_count(),// número de índices a renderizar
//                gl::UNSIGNED_INT,
//                ptr::null());
//
//
//            gl::DisableVertexAttribArray(0);
//            gl::DisableVertexAttribArray(1);
//            gl::DisableVertexAttribArray(2);
//            gl::BindVertexArray(0);
//        }
    }

    pub fn unbind_textured_model(&mut self) {
        unsafe {
            gl::DisableVertexAttribArray(0);
            gl::DisableVertexAttribArray(1);
            gl::DisableVertexAttribArray(2);
            gl::BindVertexArray(0);
        }
    }

    pub fn prepare_instance(&mut self, entity: Entity) {
        //Crea matriz de transformación con los datos de la entity
        let transformation_matrix = create_transformation_matrix(
            entity.get_position(),
            entity.get_rotation_x(), entity.get_rotation_y(), entity.get_rotation_z(),
            entity.get_scale(),
        );

        //Envia la matriz de transformación de la entity al shader
        self.shader.load_transformation_matrix(&transformation_matrix);
    }


    pub fn prepare_textured_model(&mut self, model: &TexturedModel) {
        let raw_model = model.get_raw_model();
        unsafe {
            gl::BindVertexArray(raw_model.get_vao_id());
            //Activa VAO 0 (vértices).
            gl::EnableVertexAttribArray(0);
            //Activa VAO 1 (coordenadas de textura).
            gl::EnableVertexAttribArray(1);
            //Activa VAO 2 (coordenadas de normals).
            gl::EnableVertexAttribArray(2);

            //Luz reflejada
            let texture = model.get_texture();
            self.shader.load_shine_variables(texture.get_shine_damper(), texture.get_reflectivity());

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, model.get_texture().get_id());
        }
    }
}
