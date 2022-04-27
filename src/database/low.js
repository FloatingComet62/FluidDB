import fs from 'fs';
import dotenv from 'dotenv';
import crypter from '../crypter.js';
dotenv.config();

export default {
    getData(){
        const encryptedData = fs.readFileSync('database.txt', 'utf8');
        const rawData = crypter.decrypt(encryptedData);
        const data = JSON.parse(rawData);
        return data;
    },
    setData(data){
        const encryptedData = crypter.encrypt(data, process.env.KEY);
        fs.writeFileSync('database.txt', encryptedData);
    }
}