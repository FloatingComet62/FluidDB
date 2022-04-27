import low from '../low.js';

export default {
    addSea(groupName, name){
        const data = low.getData();
        data[groupName][name] = {};
        low.setData(JSON.stringify(data));
    },
    removeSea(groupName, name){
        const data = low.getData();
        delete data[groupName][name];
        low.setData(JSON.stringify(data));
    },
    getSea(groupName, name = null){
        if(name === null){
            const data = low.getData();
            const seas = Object.getOwnPropertyNames(data[groupName]);
            return seas;
        }else{
            const data = low.getData();
            return data[groupName][name];
        }
    }
}