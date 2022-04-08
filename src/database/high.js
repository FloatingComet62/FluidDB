const { addOcean, getOcean, removeOcean } = require('./structure/ocean');
const { addSea, getSea, removeSea } = require('./structure/sea');
const { addRiver, getRiver, removeRiver, addWave } = require('./structure/river');
const low = require('./low');

module.exports = {
    getAll(){
        return low.getData();
    },
    reset(){
        low.setData(`{}`);
    },
    addOcean,
    getOcean,
    removeOcean,
    addSea,
    getSea,
    removeSea,
    addRiver,
    getRiver,
    removeRiver,
    addWave
}