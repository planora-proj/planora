const SERVER_IP = 'http://localhost:8080';
const WEBSOCKET = 'ws://localhost:8080/ws';

const health = document.getElementById('health')

function ws_connection() {
    try {
        const socket = new WebSocket(WEBSOCKET);

        socket.addEventListener('open', event => {
            console.log('Websocket connection opened:', event);
            socket.send("ping");
        })

        socket.addEventListener('error', e => {
            console.error("webSocket error:", e);
        })

        socket.addEventListener('message', event => {
            console.log("message received from server:", event.data);
        })

        socket.addEventListener('close', event => {
            console.log("webSocket connection closed:", event);
        })
    } catch (err) {
        console.log(`failed to connect to the websocket ${err}`)
    }
}

async function health_check() {
    try {
        const response = await fetch(`${SERVER_IP}/health`);
        if (response.ok) {
            const data = await response.json();
            if (data.status == "ok") {
                health.innerHTML = 'server is running'
                return
            }
        }
        throw new Error('failed to connect to the server');
    } catch (err) {
        health.innerHTML = 'cannot reach the server'
    }
}

health_check()
    .then(() => {
        // if the health request is ok, then try to connect it to the websocket
        ws_connection();
    })
    .catch(err => console.error(err))
