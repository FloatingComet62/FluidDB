import low from '../low.js';

export default {
    addOcean(name){
        const data = low.getData();
        data[name] = {};
        low.setData(JSON.stringify(data));
    },
    removeOcean(name){
        const data = low.getData();
        delete data[name];
        low.setData(JSON.stringify(data));
    },
    getOcean(name = null){
        if(name === null){
            const data = low.getData();
            const oceans = Object.getOwnPropertyNames(data);
            return oceans;
        }else{
            const data = low.getData();
            return data[name];
        }
    }
}