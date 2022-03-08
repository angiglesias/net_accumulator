# Ejercicio 2 prácticas programación RO

## Compilar proyecto

```shell
cargo build
```

## Ejecutar programas

* Servidor

    ```shell
    target/debug/tcp1ser 1234
    ```
    
    Para enviar números de pruebas sin utilizar el cliente, utilizar echo junto con netcat (ejemplo enviamos los números 100 y 1, codificados en extremista menor, sin signo, 32 bits)

    ```shell
    echo -n -e '\x64\x00\x00\x00\x01\x00\x00\x00' | nc localhost 1234
    ```

