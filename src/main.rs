use raylib::prelude::*;

pub fn line(
    image: &mut Image,
    start: Vector2,
    end: Vector2,
    color: Color,
) {
    let mut x0 = start.x as i32;
    let mut y0 = start.y as i32;
    let x1 = end.x as i32;
    let y1 = end.y as i32;

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();

    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };

    let mut err = dx - dy;

    loop {
        image.draw_pixel(x0, y0, color);

        if x0 == x1 && y0 == y1 {
            break;
        }

        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x0 += sx;
        }
        if e2 < dx {
            err += dx;
            y0 += sy;
        }
    }
}

// Estructura para representar una interseccion con una linea horizontal
struct Intersection {
    x: f32,
    slope: f32,
}

pub fn fill_polygon(
    image: &mut Image,
    vertices: &[Vector2],
    fill_color: Color,
) {
    // Verificar que tenemos al menos 3 vertices para formar un poligono
    if vertices.len() < 3 {
        return;
    }

    // Paso 1: Encontrar los limites verticales del poligono
    // Buscamos el valor minimo y maximo de y entre todos los vertices
    let mut min_y = vertices[0].y;
    let mut max_y = vertices[0].y;
    
    for vertex in vertices {
        min_y = min_y.min(vertex.y);
        max_y = max_y.max(vertex.y);
    }

    // Paso 2: Recorrer cada linea horizontal (scan-line) desde min_y hasta max_y
    for y in min_y as i32..=max_y as i32 {
        let mut intersections = Vec::new();
        
        // Paso 3: Para cada arista del poligono, verificar si la scan-line la cruza
        for i in 0..vertices.len() {
            let current = vertices[i];
            let next = vertices[(i + 1) % vertices.len()];
            
            // Verificar si la linea horizontal actual cruza esta arista
            // Esto ocurre cuando un punto esta arriba y otro abajo de la scan-line
            if (current.y <= y as f32 && next.y > y as f32) || 
               (next.y <= y as f32 && current.y > y as f32) {
                
                // Paso 4: Calcular el punto exacto de interseccion en x
                // Usamos interpolacion lineal para encontrar donde cruza
                let t = (y as f32 - current.y) / (next.y - current.y);
                let x = current.x + t * (next.x - current.x);
                
                intersections.push(x);
            }
        }
        
        // Paso 5: Ordenar las intersecciones de menor a mayor en x
        intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());
        
        // Paso 6: Rellenar entre pares de intersecciones
        // Cada par representa un segmento dentro del poligono
        for i in (0..intersections.len()).step_by(2) {
            if i + 1 < intersections.len() {
                let start_x = intersections[i] as i32;
                let end_x = intersections[i + 1] as i32;
                
                // Dibujar una linea horizontal entre las dos intersecciones
                for x in start_x..=end_x {
                    if x >= 0 && x < image.width() && y >= 0 && y < image.height() {
                        image.draw_pixel(x, y, fill_color);
                    }
                }
            }
        }
    }
}

pub fn fill_polygon_with_hole(
    image: &mut Image,
    exterior: &[Vector2],
    hole: &[Vector2],
    fill_color: Color,
    bg_color: Color,
) {
    fill_polygon(image, exterior, fill_color);
    fill_polygon(image, hole, bg_color);
}

fn main() {
    let image_width = 800;
    let image_height = 600;
    let mut image = Image::gen_image_color(image_width, image_height, Color::BLACK);

    let vertices1 = vec![
        Vector2 { x: 165.0, y: 380.0 },
        Vector2 { x: 185.0, y: 360.0 },
        Vector2 { x: 180.0, y: 330.0 },
        Vector2 { x: 207.0, y: 345.0 },
        Vector2 { x: 233.0, y: 330.0 },
        Vector2 { x: 230.0, y: 360.0 },
        Vector2 { x: 250.0, y: 380.0 },
        Vector2 { x: 220.0, y: 385.0 },
        Vector2 { x: 205.0, y: 410.0 },
        Vector2 { x: 193.0, y: 383.0 },
    ];

    fill_polygon(&mut image, &vertices1, Color::ORANGE);
    for i in 0..vertices1.len() - 1 {
        line(&mut image, vertices1[i], vertices1[i + 1], Color::YELLOW);
    }
    line(&mut image, vertices1[9], vertices1[0], Color::YELLOW);

    let vertices2 = vec![
        Vector2 { x: 321.0, y: 335.0 },
        Vector2 { x: 288.0, y: 286.0 },
        Vector2 { x: 339.0, y: 251.0 },
        Vector2 { x: 374.0, y: 302.0 },
    ];

    fill_polygon(&mut image, &vertices2, Color::BLUE);
    for i in 0..vertices2.len() - 1 {
        line(&mut image, vertices2[i], vertices2[i + 1], Color::WHITE);
    }
    line(&mut image, vertices2[vertices2.len() - 1], vertices2[0], Color::WHITE);

    let vertices3 = vec![
        Vector2 { x: 377.0, y: 249.0 },
        Vector2 { x: 411.0, y: 197.0 },
        Vector2 { x: 436.0, y: 249.0 },
    ];

    fill_polygon(&mut image, &vertices3, Color::GREEN);
    for i in 0..vertices3.len() - 1 {
        line(&mut image, vertices3[i], vertices3[i + 1], Color::WHITE);
    }
    line(&mut image, vertices3[vertices3.len() - 1], vertices3[0], Color::WHITE);

    let vertices4 = vec![
        Vector2 { x: 413.0, y: 177.0 },
        Vector2 { x: 448.0, y: 159.0 },
        Vector2 { x: 502.0, y: 88.0 },
        Vector2 { x: 553.0, y: 53.0 },
        Vector2 { x: 535.0, y: 36.0 },
        Vector2 { x: 676.0, y: 37.0 },
        Vector2 { x: 660.0, y: 52.0 },
        Vector2 { x: 750.0, y: 145.0 },
        Vector2 { x: 761.0, y: 179.0 },
        Vector2 { x: 672.0, y: 192.0 },
        Vector2 { x: 659.0, y: 214.0 },
        Vector2 { x: 615.0, y: 214.0 },
        Vector2 { x: 632.0, y: 230.0 },
        Vector2 { x: 580.0, y: 230.0 },
        Vector2 { x: 597.0, y: 215.0 },
        Vector2 { x: 552.0, y: 214.0 },
        Vector2 { x: 517.0, y: 144.0 },
        Vector2 { x: 466.0, y: 180.0 },
    ];

    let vertices5 = vec![
        Vector2 { x: 682.0, y: 175.0 },
        Vector2 { x: 708.0, y: 120.0 },
        Vector2 { x: 735.0, y: 148.0 },
        Vector2 { x: 739.0, y: 170.0 },
    ];

    fill_polygon_with_hole(&mut image, &vertices4, &vertices5, Color::BLUE, Color::BLACK);
    for i in 0..vertices4.len() - 1 {
        line(&mut image, vertices4[i], vertices4[i + 1], Color::RED);
    }
    line(&mut image, vertices4[vertices4.len() - 1], vertices4[0], Color::RED);

    for i in 0..vertices5.len() - 1 {
        line(&mut image, vertices5[i], vertices5[i + 1], Color::WHITE);
    }
    line(&mut image, vertices5[vertices5.len() - 1], vertices5[0], Color::WHITE);

    image.export_image("out.bmp");
}
