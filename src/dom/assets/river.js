const Wave = require('./wave');

class River{
    constructor(database, groupName, seaName, name){
        this.groupName = groupName;
        this.seaName = seaName;
        this.name = name;
        this.database = database;
        this.database.addRiver(this.groupName, this.seaName, this.name);
    }
    createWave(name){
        return new Wave(this.database, this.groupName, this.seaName, this.name, name);
    }
}

module.exports = River;