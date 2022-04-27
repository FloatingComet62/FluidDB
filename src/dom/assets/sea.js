const River = require('./river');

class Sea{
    constructor(database, groupName, name){
        this.groupName = groupName;
        this.name = name;
        this.database = database;
        this.database.addSea(this.groupName, this.name);
    }
    createRiver(name){
        return new River(this.database, this.groupName, this.name, name);
    }
}

module.exports = Sea;