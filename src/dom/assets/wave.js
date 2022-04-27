class Wave{
    constructor(database, groupName, seaName, riverName, name){
        this.groupName = groupName;
        this.seaName = seaName;
        this.riverName = riverName;
        this.name = name;
        this.database = database;
        this.database.addWave(this.groupName, this.seaName, this.riverName, this.name);
    }
}

module.exports = Wave;