# Net accumulator program

Small playground to learn rust programming with basic network communications over TCP and UDP protocols

## Build project

For rust setup, follor intructions found on https://rustup.rs/

To build, simple execute `cargo build`

## Running programs

### TCP

The net accumulator over TCP generates an accumulator instance for each incoming client connections. Numbers are 32 bit signed intergers encoded in little-endian order. Each group of numbers send over the tcp stream is preceded by a 32 bits unsigned integer (also encoded in little-endian order) containing the number of numbers sent on this group.

* Server

    ```shell
    # target/debug/tcp1ser <server listening port>
    target/debug/tcp1ser 1234
    2022-03-10T19:55:40.748Z INFO [tcp1ser] Server listening to incoming connection on port 1234
    ```

    For testing purposes, you can send numbers to the server without using a client with the help of `netcat` like in the following example:

    ```shell
    # Sending the numbers 100 and 1
    # Prefixed by 2 (0x02,0x00,0x00,0x00)
    # 100 (0x64,0x00,0x00,0x00)
    # 1 (0x01,0x00,0x00,0x00)
    echo -n -e '\x02\x00\x00\x00\x64\x00\x00\x00\x01\x00\x00\x00' | netcat 127.0.0.1 1234
    ```

* Client

    The client provides an cli utility to recover, process and send the groups of numbers. If the first number of a sequence is 0, the client will stop and exit

    ```shell
    # target/debug/tcp1cli <server ip address> <server listening port>
    target/debug/tcp1cli 127.0.0.1 1234
    >> Type a integer number to send to the server
    1 2 3 4 5
    2022-03-10T19:55:40.748Z DEBUG [tcp1cli] Sending numbers [1, 2, 3, 4, 5] to server 127.0.0.1:1235
    >> Received 15 from server
    >> Type a integer number to send to the server
    0  
    First number typed is a 0. Exiting program...
    2022-03-10T19:56:01.945Z INFO [tcp1cli] Closed connection with server 127.0.0.1:1235
    ```

### UDP

The net accumulator over UDP will generate an accumulator instance for each client, understanding a client as the pair <ip address> and <port>, and will mantain the accumulator state for the following client's requests. Numbers are 32 bit signed intergers encoded in little-endian order. Each UDP datagram will contain maximun payload of 512 numbers in message (using buffer of 2048 bytes)

* Server

    ```shell
    # target/debug/udpser <server listening port>
    target/debug/udpser 1234
    2022-03-10T19:55:40.748Z INFO [udpser] Server listening to incoming connection on port 1234
    ```

    For testing purposes, you can send numbers to the server without using a client with the help of `netcat` like in the following example:

    ```shell
    # Sending the numbers 100 and 1
    # 100 (0x64,0x00,0x00,0x00)
    # 1 (0x01,0x00,0x00,0x00)
    echo -n -e '\x64\x00\x00\x00\x01\x00\x00\x00' | netcat -u 127.0.0.1 1234
    ```

* Client

    The client provides an cli utility to recover, process and send the groups of numbers. If the first number of a sequence is 0, the client will stop and exit

    ```shell
    # target/debug/udpcli <server ip address> <server listening port> <optional client port bind>
    target/debug/udpcli 127.0.0.1 1234
    >> Type a integer number to send to the server
    1 2 3 4 5
    2022-03-10T19:55:40.748Z DEBUG [udpcli] Sending numbers [1, 2, 3, 4, 5] to server 127.0.0.1:1235
    >> Received 15 from server
    >> Type a integer number to send to the server
    0  
    First number typed is a 0. Exiting program...
    2022-03-10T19:56:01.945Z INFO [udpcli] Closed connection with server 127.0.0.1:1235
    ```