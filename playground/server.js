const express = require('express');
const path = require('path');

const app = express();
const PORT = 4000;

app.use(express.static(path.join(__dirname, 'public')));

app.listen(PORT, () => {
    console.log(`Server running at http://localhost:${PORT}`);
});
