const low = require('../low');

module.exports = {
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
    }
}