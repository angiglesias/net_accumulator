# Ejercicio 2 prácticas programación RO

## Compilar proyecto

Para instalar rust, seguir las instrucciones https://rustup.rs/

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
    echo -n -e '\x02\x00\x00\x00\x64\x00\x00\x00\x01\x00\x00\x00' | nc localhost 1234
    ```

* Cliente

    ```shell
    target/debug/tcp1cli 127.0.0.1 1234
    >> Type a integer number to send to the server
    2022-03-10T19:55:40.748Z DEBUG [tcp1cli] Sending numbers [1, 2, 3, 4, 5] to server 127.0.0.1:1235
    >> Received 15 from server
    >> Type a integer number to send to the server
    0  
    First number typed is a 0. Exiting program...
    2022-03-10T19:56:01.945Z INFO [tcp1cli] Closed connection with server 127.0.0.1:1235
    ```