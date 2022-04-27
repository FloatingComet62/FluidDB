import Cryptr from 'cryptr';
import fs from 'fs';
import dotenv from 'dotenv';
dotenv.config();
let crypter;
if(process.env.KEY){
    crypter = new Cryptr(process.env.KEY);
}else{
    genkey();
    crypter = new Cryptr(process.env.KEY);
}

function genkey(){
    let key = "";
    const possible = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    for (let i = 0; i < 50; i++){
        key += possible.charAt(Math.floor(Math.random() * possible.length));
    }
    
    const previousData = fs.readFileSync('.env');
    fs.writeFileSync('.env', previousData + "\nKEY=" + key);
    dotenv.config();
}

export default crypter