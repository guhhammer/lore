# egrep

Busca patrones en archivos utilizando expresiones regulares extendidas. Nota: Este comando es un alias de `grep --extended-regexp`.

## Usage
- `egrep 'a+' ruta/al/archivo` : busca uno o más caracteres repetidos
- `egrep 'a?' ruta/al/archivo` : busca cero o una aparición de un carácter (coincidencia opcional)
- `egrep 'a{10}' ruta/al/archivo` : busca 10 repeticiones de un carácter
- `egrep 'a{3,7}' ruta/al/archivo` : busca entre 3 y 7 repeticiones de un carácter
- `egrep 'gato|perro|ratón' ruta/al/archivo` : busca una de las opciones enumeradas
- `egrep 'c(a|o|u)p' ruta/al/archivo` : busca una de las opciones enumeradas dentro de un patrón más amplio
- `egrep '(aeiou)+' ruta/al/archivo` : busca un grupo de caracteres que se repita una o más veces
- `egrep [[:alnum:|:alpha:|:space:|...]] ruta/al/archivo` : busca utilizando clases de caracteres estándar (más información: <https://www.regular-expressions.info/posixbrackets.html>)

## Links
- More information: https://manned.org/egrep
