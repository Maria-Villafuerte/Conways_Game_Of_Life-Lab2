# Conway's Game of Life - Lab 2

Este labortorio implementa el Juego de "la Vida de Conway" en Rust. El Juego de la Vida es un autómata celular desarrollado por el matemático John Conway. Es un juego de cero jugadores, lo que significa que su evolución está determinada por su estado inicial, sin necesidad de más entradas. 

## Descripción

En este trabajo, se ha utilizado Rust para implementar el Juego de la Vida, usando un framebuffer para representar el estado de la cuadrícula en la pantalla. Se han incluido varios patrones predefinidos que se pueden posicionar en cualquier lugar del tablero.

### Características

- **Implementación en Rust**: Uso eficiente de memoria y velocidad.
- **Patrones Predefinidos**: Incluye patrones como Ten Cell Row, Exploder, Small Exploder, Glider, y Spaceship.
- **Visualización en Tiempo Real**: Muestra la evolución del juego en tiempo real utilizando un framebuffer.

## Uso

Para ejecutar el programa, asegúrate de tener Rust instalado en tu sistema. Luego, clona este repositorio y ejecuta los siguientes comandos en tu terminal:

```bash
git clone https://github.com/Maria-Villafuerte/Conways_Game_Of_Life-Lab2.git
cd Conways_Game_Of_Life-Lab2
cargo run
```

## Patrones Disponibles

- Ten Cell Row: Una fila de diez células vivas.
- Exploder: Un patrón que explota en una estructura más grande.
- Small Exploder: Una versión más pequeña del Exploder.
- Glider: Un patrón que se mueve diagonalmente a través del tablero.
- Spaceship: Un patrón que se mueve horizontalmente a través del tablero.

## Demo

Aquí puedes ver una demostración en funcionamiento:

![Demo GIF](./lab2-Graficas.gif)

Para esta demostración si utilizó una pantalla de 100*100 y el siguiente código:
 
```rust
    ten_cell_row(framebuffer, 5, 0);
    ten_cell_row(framebuffer, 15, 0);
    ten_cell_row(framebuffer, 25, 0);
    ten_cell_row(framebuffer, 35, 0);
    ten_cell_row(framebuffer, 45, 0);
    ten_cell_row(framebuffer, 55, 0);
    ten_cell_row(framebuffer, 65, 0);
    ten_cell_row(framebuffer, 75, 0);
    ten_cell_row(framebuffer, 85, 0);
    ten_cell_row(framebuffer, 95, 0);
```