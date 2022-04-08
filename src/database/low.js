const fs = require('fs');
const dotenv = require('dotenv');
const crypter = require('../crypter');
dotenv.config();

module.exports = {
    getData(){
        const encryptedData = fs.readFileSync('database.txt', 'utf8');
        const rawData = crypter.cryptr.decrypt(encryptedData);
        const data = JSON.parse(rawData);
        return data;
    },
    setData(data){
        const encryptedData = crypter.cryptr.encrypt(data, process.env.KEY);
        fs.writeFileSync('database.txt', encryptedData);
    }
}