#!/bin/bash

#Establaciendo cantidad de contenedores
NO_CONTENEDORES=10

#Obteniendo num aleatorio
get_name() {
    echo "contenedor_$(cat /dev/urandom | tr -dc 'a-z0-9' | fold -w 8 | head -n 1)"
}

#Creación contenedores
for i in $(seq 1 $NO_CONTENEDORES)
do
    NAME_CONTENEDOR=$(get_name)
    docker run -d --name $NAME_CONTENEDOR alpine sleep 3600
    echo "Contenedor $NAME_CONTENEDOR creado."
done
