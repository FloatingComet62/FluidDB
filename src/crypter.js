const Cryptr = require('cryptr');
const fs = require('fs');
const dotenv = require('dotenv');
dotenv.config();
let cryptr;
if(process.env.KEY){
    cryptr = new Cryptr(process.env.KEY);
}else{
    genkey();
    cryptr = new Cryptr(process.env.KEY);
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

module.exports = {
    cryptr
};