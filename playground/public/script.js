const SERVER_IP = "http://localhost:8080";

const health = document.getElementById('health')

async function health_check() {
    try {
        const response = await fetch(`${SERVER_IP}/health`);
        if (response.ok) {
            const data = await response.json();
            if (data.status == "ok") {
                health.innerHTML = 'server is running'
            }
        } else {
            health.innerHTML = 'server is stopped'
        }
    } catch (err) {
        health.innerHTML = 'server is stopped'
    }
}

health_check()
    .catch(err => console.error(err))
