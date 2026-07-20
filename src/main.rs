// Main

// Se usa mod para llamar a los otros archivos
mod framebuffer;
mod line;
mod poligono;

// Se usan use para llamar a las funciones de los otros archivos
use framebuffer::Framebuffer;
use line::linea;
use poligono::{poligono, rellenar_poligono};

use raylib::prelude::*;


fn main() {

    let mut framebuffer = Framebuffer::new(800, 600);

    framebuffer.set_background_color(Color::BLACK);
    framebuffer.clear();

    framebuffer.set_current_color(Color::WHITE);

    // Dibujar los poligonos con sus vertices

    // Poligono 1
    let poligono1 = vec![ 
    (165, 380),
    (185, 360),
    (180, 330),
    (207, 345),
    (233, 330),
    (230, 360),
    (250, 380),
    (220, 385),
    (205, 410),
    (193, 383)];

    framebuffer.set_current_color(Color::YELLOW);
    rellenar_poligono(&mut framebuffer,&poligono1,);

    framebuffer.set_current_color(Color::WHITE);
    poligono( &mut framebuffer, &poligono1,);

    

    // Poligono 2
    let poligono2 = vec![ (321, 335), (288, 286), (339, 251), (374, 302)];

    framebuffer.set_current_color(Color::RED);
    rellenar_poligono(&mut framebuffer,&poligono2,);

    framebuffer.set_current_color(Color::WHITE);
    poligono( &mut framebuffer, &poligono2,);

    // Poligono 3
    let poligono3 = vec![ (377, 249), (411, 197), (436, 249) ];

    framebuffer.set_current_color(Color::GREEN);
    rellenar_poligono(&mut framebuffer,&poligono3,);

    framebuffer.set_current_color(Color::WHITE);
    poligono( &mut framebuffer, &poligono3,);

    // Poligono 4
    let poligono4 = vec![ 
    (413, 177), (448, 159), (502, 88), (553, 53), (535, 36), (676, 37), (660, 52),
    (750, 145), (761, 179), (672, 192), (659, 214), (615, 214), (632, 230), (580, 230),
    (597, 215), (552, 214), (517, 144), (466, 180)];

    //Primero se pinta todo 
    framebuffer.set_current_color(Color::BLUE);
    rellenar_poligono(&mut framebuffer,&poligono4,);

    framebuffer.set_current_color(Color::WHITE);
    poligono( &mut framebuffer, &poligono4,);


    //Este es un agujero dentro del polígono 4 
    let poligono5 = vec![ (682, 175), (708, 120), (735, 148), (739, 170)];

    // Se pinta de negro para que se vea como un agujero
    framebuffer.set_current_color(Color::BLACK);
    rellenar_poligono(&mut framebuffer,&poligono5,);

    framebuffer.set_current_color(Color::WHITE);
    poligono( &mut framebuffer, &poligono5,);


    framebuffer.render_to_file("poligono.png");

}