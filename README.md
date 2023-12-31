# Memento - Search Tool

Esta herramienta de búsqueda de línea de comandos permite buscar palabras clave en un archivo de texto.

## Instalación

Puedes clonar este repositorio y compilar el programa o descargar el binario directamente.

### Clonar y compilar

Requisitos previos: asegúrate de tener [Rust](https://www.rust-lang.org/tools/install) y Cargo instalados.

```sh
git clone https://github.com/cthed2/memento.git
cd memento 
cargo build --release
```
El ejecutable estará disponible en target/release/memento

### Descargar binario

Si prefieres no compilar el código fuente, puedes descargar el binario más reciente desde la sección [Releases](https://github.com/cthed2/memento/releases/tag/v1.0.0) en la página del repositorio de GitHub.

# Uso

Después de compilar el proyecto o descargar el binario, puedes usar la herramienta de la siguiente manera:

```sh
./memento ruta/al/archivo.md palabraclave1 palabraclave2
```
Nota: Recuerda que puedes buscar las palabras que quieras, en la salida solo aparecen las lineas que contienen las palabras que has buscado.

Puedes descargar el archivo memento.md, en caso aún no tengas tu propio "cheat sheet".

## Configuración para .zsh

1. El binario `memento` lo movemos a la ruta `/usr/local/bin/`
```sh
sudo mv memento /usr/local/bin/
```
2. Le damos el permiso con:
```sh
sudo chmod 775 memento
```
3. Abrimos .zshrc
```
sudo vim .zshrc
```
4. Agregamos un alias
```
alias memento='/usr/local/bin/memento ~/memento.md'
```
Nota: Recuerda que el archivo de texto memento.md lo ubicamos en /home/user/ . Si la ubicación de tu archivo de texto es diferente recuerda cambiar la ruta.

# Contribuciones

Las contribuciones son bienvenidas. Por favor, haz un fork del repositorio, crea tus características o correcciones en una rama separada, y envía tus pull requests para revisión.

# Licencia

Este proyecto está licenciado bajo la Licencia MIT - ve el archivo [LICENSE.txt](https://github.com/cthed2/memento/blob/master/LICENSE.txt) para detalles.

Este archivo README proporciona una visión general clara de lo que hace la herramienta, cómo se instala y se utiliza, cómo otros pueden contribuir al proyecto y la licencia bajo la cual se distribuye. 

Una vez que hayas terminado de editar el `README.md`, asegúrate de añadirlo a tu repositorio local, hacer commit y luego empujarlo al repositorio remoto con los comandos:

```sh
git add README.md
git commit -m "Add README with installation and usage instructions"
git push origin master
```

Finalmente, sigue las instrucciones para crear una release en GitHub, que te permitirá adjuntar binarios compilados para que otros puedan descargarlos directamente sin necesidad de compilar el código fuente ellos mismos.
