/*
    TODO userauth
    TODO api routes
    TODO database (low.js for low level interaction) (high.js for high level interaction)
    TODO encrytion(toggleable)
    ? dom?

    * Database > Ocean(clusters) > Sea(collections) > River(documents)
*/

import './api.js';

import DOM from './dom/index.js';
const database = new DOM(true, 'admin', 'admin');
await database.reset();
await database.createOcean('test');
console.log(await database.getEverything());