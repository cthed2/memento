1. Listar todas las particiones disponibles para montar: lsblk
2. Sintaxis de enlace en Markdown: [texto del enlace](http://www.url-del-sitio.com)

# Guía de Comandos Docker

## Configuración Inicial y Ayuda:
- Iniciar el demonio de Docker:               `docker -d`
- Ayuda de Docker:                            `docker --help`
- Información del sistema:                    `docker info`

## Trabajar con Imágenes:
- Construir una imagen:                       `docker build -t <image_name>`
- Construir imagen sin caché:                 `docker build -t <image_name> . --no-cache`
- Listar imágenes locales:                    `docker images`
- Eliminar imagen específica:                 `docker rmi <image_name>`
- Eliminar imágenes no utilizadas:            `docker image prune`
- Buscar imagen en Docker Hub:                `docker search <image_name>`
- Descargar imagen de Docker Hub:             `docker pull <image_name>`
- Subir imagen a Docker Hub:                  `docker push <username>/<image_name>`

## Manejo de Contenedores:
- Crear y ejecutar con nombre personalizado:  `docker run --name <container_name> <image_name>`
- Ejecutar y publicar puerto:                 `docker run -p <host_port>:<container_port> <image_name>`
- Ejecutar en segundo plano:                  `docker run -d <image_name>`
- Iniciar/detener contenedor:                 `docker start|stop <container_name>`
- Eliminar contenedor:                        `docker rm <container_name>`
- Abrir shell en contenedor:                  `docker exec -it <container_name> sh`
- Ver registros del contenedor:               `docker logs -f <container_name>`
- Inspeccionar contenedor:                    `docker inspect <container_name>`
- Listar contenedores en ejecución:           `docker ps`
- Listar todos los contenedores:              `docker ps --all`

## Estadísticas y Recursos:
- Estadísticas de contenedores:               `docker container stats`

## Autenticación:
- Iniciar sesión en Docker Hub:               `docker login -u <username>`


funciones/funcion: updateUpstream, gitix, gitcth
