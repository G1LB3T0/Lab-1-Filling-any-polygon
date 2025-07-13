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

fn main() {
    let image_width = 500;
    let image_height = 500;

    let mut image = Image::gen_image_color(image_width, image_height, Color::BLACK);


    let puntos = vec![
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

    for i in 0..puntos.len() - 1{
        line(&mut image, puntos[i], puntos[i + 1], Color::YELLOW); // aparentemente no se cierra la estrella asi que debo conectar el ultimo punto con el primero
    }

    line(&mut image, puntos[9], puntos[0], Color::YELLOW);

    image.export_image("linea.png");
    println!("{}", puntos.len());
}
