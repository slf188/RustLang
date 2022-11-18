# Notes

- Todas las organizaciones mencionadas en el ranking utilizan Rust sus proyectos
- Go tambien tiene algo similar a rustfmt llamado gofmt
- Abogar a Rust
- Es impulsado completamente por la comunidad, hoare se salio

### Ownership

- Veremos conceptos de manejo de memoria
- Veremos por que rust garantiza seguridad a la memoria sin el uso de un recolector de basura
- Ownership tiene que ver con como el programa maneja la memoria
- En Rust y entre otros lenguajes vemos que se almacena la memoria en dos lugares
    - Stack:
        - LIFO
        - Es como un contenedor de bolas de tenis
        - Tomemos como ejemplo lo siguiente
        - Rust almacena datos con un tamano fixed
        - Sabemos cuantos bits de espacio necesita cuando compilemos el programa
        - La variable y el valor en la variable se almacenan en el stack
        - Ahora veamos un vector que es capaz de cambiar de tamano en ejecucion
        - En este caso como puede cambiar almacenamos los datos en el stack y en el heap
        - En el stack almacenamos la variable vector junto a un puntero a direccion de memoria en el heap
        - Cuando utilizamos drop borramos el acceso que teniamos originalmente en el stack y en heap
        - Hasta ahora esto no es nada nuevo, muchos lenguajes almacenan datos en el stack y en el heap al mismo tiempo, pero en lo que se diferencian es en como ellos manejan la memoria, aqui es donde Rust se destaca
- Ahora veamos con una string
- Una string se almacena en el stack y en el heap, es mutable
- Pero ahora pensemos que pasaria si asignamos a una nueva variable say2 el valor de nuestra variable say
- Creariamos una copia de say en el stack llamada say2 y apuntariamos a la misma direccion de memoria en el heap que apunta say
- Pero esto genera una pregunta, que pasaria si eliminariamos say?
- Pareceria que los valores en el stack y en el heap para say dejarian de ser validos, pero sera que say2 apunta a la misma direccion de memoria en el heap?
- Este problema se le llama Puntero colgante o Dangling Pointer donde el valor que se halla en stack apunta a una direccion que ya no es valida en el heap
- Esto hace que se crashee nuestro programa y es un problema de seguridad

### Qué problemas ayuda a evitar el ownership?

1. Puntero Colgante
2. Doble liberación
3. Fuga de memoria

### Borrowing

- Podemos clonar una variable, pero esto creara una copia completa en el stack y en el heap
- Podemos evitar esto utilizando la tecnica de borrowing, esto crea una referencia al valor de esa variable. No pasa ownership y tampoco creamos una copia
- Es necesario pasar referencias validas

### Consumo de energia

- Investigacion realizada por investigadores portugueses
- Ejecutaron las soluciones a 10 problemas de programación escritos en 27 idiomas diferentes, mientras monitoreaban cuidadosamente cuánta electricidad usaba cada uno, así como su velocidad y uso de memoria.
- El compilador esencialmente es un colector de basura
- Con otros lenguajes estamos acostumbrados a ver a nuestros programas en relacion a como se relacionan con el cpu pero en el caso de rust nos fuerza a pensar mas en como la memoria esta siendo manejada
- El compilador es muy estricto

### Historia

- La historia de rust es de graydon liderando el lenguaje por los primeros 6 anios, desde ahi ha sido un lenguaje que ha sido desarrollado por la comunidad
- Evoluciono mucho en los primeros anios, inicialmente era oop pero ahora explicitamente no lo es, depende mucho de la definicion que uno tenga de oop
- Ya que graydon salio del proyecto el liderazgo es mas distribuido y discutido
- Inicialmente tuvo un gc
- **Cargo literally just calls rustc for you with all the compiler flags and whatnot to make it easier on the user to manage large projects.**

### Configuration file

- TOml es un archivo para configurar, agregamos dependencias y se ajustan o actualizan corriendo cargo build
- La gente de javascript introdujo como tema de discusion hacer que cargo instale paquetes como npm
- U of pen, mit, iowa, stanford

### Rust foundation

- Mozilla despidio al equipo servo que manejaba buena parte de rust por el covid, esto planteo preguntas del futuro de rust, asi se formo la fundacion rust, que es una organizacion fundada por meta, microsoft, google, huawei, aws, mozilla)

### Datos

- Mencionar la primera parte del por qué de rust de tirón arón
- compilador utiliza llvm soporta arquitecturas x86 y arm
- Al final recapitular
- If a language must have inheritance to be an object-oriented language, then Rust is not one.
- When i ask to translate to rust with inheritance i get nothing
- Tony hoare created quicksort and null pointer