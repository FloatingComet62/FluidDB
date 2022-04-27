import ocean from './assets/ocean.js';
import sea from './assets/sea.js';
import river from './assets/river.js';
import low from './low.js';

export default {
    getAll(){
        return low.getData();
    },
    reset(){
        low.setData(`{}`);
    },
    addOcean(name){
        ocean.addOcean(name);
    },
    removeOcean(name){
        ocean.removeOcean(name);
    },
    getOcean(name = null){
        return ocean.getOcean(name);
    },
    addSea(groupName, name){
        sea.addSea(groupName, name);
    },
    removeSea(groupName, name){
        sea.removeSea(groupName, name);
    },
    getSea(groupName, name = null){
        return sea.getSea(groupName, name);
    },
    addRiver(groupName, seaName, name){
        river.addRiver(groupName, seaName, name);
    },
    removeRiver(groupName, seaName, name){
        river.removeRiver(groupName, seaName, name);
    },
    getRiver(groupName, seaName, name = null){
        return river.getRiver(groupName, seaName, name);
    },
    addWave(groupName, seaName, name, waveName, waveValue){
        river.addWave(groupName, seaName, name, waveName, waveValue);
    },
    getWave(groupName, seaName, name, waveName){
        return river.getWave(groupName, seaName, name, waveName);
    },
    removeWave(groupName, seaName, name, waveName){
        river.removeWave(groupName, seaName, name, waveName);
    }
}