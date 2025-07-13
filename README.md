# Proyecto Polygon

Este laboratorio es una aplicación en Rust que utiliza la librería Raylib para dibujar y rellenar polígonos en imágenes. Implementa el algoritmo scan-line para rellenar polígonos definidos por sus vértices.

## ¿Qué hace?
- Dibuja polígonos a partir de listas de puntos (vértices).
- Rellena los polígonos con el color que elijas.
- Permite dibujar el borde de cada polígono con otro color.
- Puedes definir varios polígonos y exportar cada imagen como un archivo PNG.

## Requisitos
- Rust (https://www.rust-lang.org/)
- Raylib para Rust (`raylib = "5.5.1"` en `Cargo.toml`)

## ¿Cómo usar?
1. Clona este repositorio o descarga los archivos.
2. Entra a la carpeta `polygon`.
3. Ejecuta:
   ```
   cargo run
   ```
4. Se generarán imágenes PNG con los polígonos dibujados en la carpeta del proyecto.

## ¿Cómo agregar más polígonos?
- Abre el archivo `src/main.rs`.
- Agrega un nuevo vector de vértices, por ejemplo:
  ```rust
  let vertices_nuevo = vec![
      Vector2 { x: 100.0, y: 100.0 },
      Vector2 { x: 200.0, y: 100.0 },
      Vector2 { x: 200.0, y: 200.0 },
      Vector2 { x: 100.0, y: 200.0 },
  ];
  fill_polygon(&mut image, &vertices_nuevo, Color::GREEN);
  for i in 0..vertices_nuevo.len() - 1 {
      line(&mut image, vertices_nuevo[i], vertices_nuevo[i + 1], Color::WHITE);
  }
  line(&mut image, vertices_nuevo[vertices_nuevo.len() - 1], vertices_nuevo[0], Color::WHITE);
  ```
- Exporta la imagen con un nombre diferente si quieres guardar el resultado:
  ```rust
  image.export_image("nuevo_poligono.png");
  ```

## Notas
- El algoritmo funciona bien para polígonos simples (no autointersectados).
- Si quieres crear hoyos, puedes usar la función `fill_polygon_with_hole`.
- Puedes cambiar los colores modificando los parámetros de las funciones.

## Autor
- Luis Gonzalez 