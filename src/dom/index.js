const fetch = (...args) => import('node-fetch').then(({default: fetch}) => fetch(...args));
import database from '../database/index.js';

class Database{
    constructor(encrypted, username, password){
        this.encrypted = encrypted;
        this.username = username;
        this.password = password;
        this.database = database;
    }
    async reset(){
        await fetch(`http://localhost:3000/reset`);
    }
    async createOcean(name){
        return (await fetch(`http://localhost:3000/${name}`, {
            method: 'POST'
        })).json();
    }
    async getEverything(){
        return (await fetch(`http://localhost:3000/`)).json();
    }
}

export default Database;