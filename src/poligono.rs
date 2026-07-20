// Para dibujar los poligonos usando el algoritmo de Baresenham y el scanline

use crate::framebuffer::Framebuffer;
use crate::line::linea;

//Usa de parametros los vertices del poligono 
pub fn poligono(framebuffer: &mut Framebuffer,vertices: &[(i32, i32)],) {
    let n = vertices.len();

    for i in 0..n {

        let (x0, y0) = vertices[i];
        let (x1, y1) = vertices[(i + 1) % n];

        linea(framebuffer, x0, y0, x1, y1,);

    }

}

// Para rellenarlo
// Se usa el de scanline ya que es mas eficiente y sencillo de implementar 
// Se usa de parametros los vertices del poligono
// Scanline: linea horizontal

pub fn rellenar_poligono(framebuffer: &mut Framebuffer,vertices: &[(i32, i32)],) {
    let n = vertices.len();

    //Primer paso es encontrar el y mínimo y máximo para saber que rellenar
    let mut y_min = vertices[0].1;
    let mut y_max = vertices[0].1;

    // Recorrer los vertices para encontrar el y mínimo y máximo
    for i in 1..n {
        if vertices[i].1 < y_min {
            y_min = vertices[i].1;
        }

        if vertices[i].1 > y_max {
            y_max = vertices[i].1;
        }
    }

    // Recorrer cada scanline
    for y in y_min..=y_max {

        let mut intersecciones: Vec<i32> = Vec::new();

        // Buscar intersecciones con cada arista
        for i in 0..n {

            let (x0, y0) = vertices[i];
            let (x1, y1) = vertices[(i + 1) % n];

            if (y0 <= y && y < y1) || (y1 <= y && y < y0) {

                // Calcular la intersección usando decimales
                // Cambie a decimales porque con enteros se perdia presición y se salia
                let x = x0 as f32
                    + ((y - y0) as f32 * (x1 - x0) as f32)
                        / (y1 - y0) as f32;

                intersecciones.push(x.round() as i32);
            }
        }

        // Ordenar las intersecciones
        intersecciones.sort();

        // Rellenar entre pares de intersecciones
        let mut i = 0;

        while i + 1 < intersecciones.len() {

            let x_inicio = intersecciones[i];
            let x_fin = intersecciones[i + 1];

            // No use el ..= para no pintar un píxel extra (si pasa y se sale)
            for x in x_inicio..x_fin {
                framebuffer.point(x, y);
            }

            // Para que no se repita el mismo par de intersecciones
            i += 2;
        }
    }
}