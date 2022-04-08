/*
    TODO userauth
    TODO api routes
    TODO database (low.js for low level interaction) (high.js for high level interaction)
    TODO encrytion(toggleable)
    ? dom?

    * Database > Ocean(clusters) > Sea(collections) > River(documents)
*/

const high = require('./database/high');
const low = require('./database/low');

const main = async() => {
    console.log(high.getOcean());
}


main();