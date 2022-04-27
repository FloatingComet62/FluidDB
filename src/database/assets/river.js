import low from '../low.js';

export default {
    addRiver(groupName, seaName, name){
        const data = low.getData();
        data[groupName][seaName][name] = {};
        low.setData(JSON.stringify(data));
    },
    removeRiver(groupName, seaName, name){
        const data = low.getData();
        delete data[groupName][seaName][name];
        low.setData(JSON.stringify(data));
    },
    getRiver(groupName, seaName, name = null){
        if(name === null){
            const data = low.getData();
            const rivers = Object.getOwnPropertyNames(data[groupName][seaName]);
            return rivers;
        }else{
            const data = low.getData();
            return data[groupName][seaName][name];
        }
    },
    addWave(groupName, seaName, name, waveName, waveValue){
        const data = low.getData();
        data[groupName][seaName][name][waveName] = waveValue;
        low.setData(JSON.stringify(data));
    },
    getWave(groupName, seaName, name, waveName){
        const data = low.getData();
        return data[groupName][seaName][name][waveName]
    },
    removeWave(groupName, seaName, name, waveName){
        const data = low.getData();
        delete data[groupName][seaName][name][waveName]
        low.setData(JSON.stringify(data));
    }
}